use clap::{Args, ValueEnum};
use eyre::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::specs::workspace_root;

#[derive(Args, Debug)]
pub struct TestSchemaArgs {
    /// Platform name (looks for ext/specs/<platform>.yaml).
    pub platform: String,

    /// Override the spec path. Defaults to ext/specs/<platform>.yaml.
    #[arg(long)]
    pub spec: Option<PathBuf>,

    /// Override the base URL. Defaults to spec's first `servers` entry.
    #[arg(long)]
    pub base_url: Option<String>,

    /// Add an HTTP header (e.g. `--header X-API-Key:abc`). Repeatable.
    #[arg(long = "header", short = 'H')]
    pub headers: Vec<String>,

    /// Comma-separated checks to run. Defaults to a fast subset; use `all` to enable everything.
    #[arg(long, default_value = "not_a_server_error,status_code_conformance,content_type_conformance,response_schema_conformance")]
    pub checks: String,

    /// Cap on Hypothesis examples per operation. Default 25 keeps smoke tests fast.
    #[arg(long, default_value_t = 25)]
    pub max_examples: u32,

    /// Which schemathesis runner to use. Auto-detect by default.
    #[arg(long, value_enum, default_value_t = Runner::Auto)]
    pub runner: Runner,

    /// Extra args passed verbatim to schemathesis after `--`.
    #[arg(last = true)]
    pub extra: Vec<String>,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
pub enum Runner {
    /// Try `schemathesis` on PATH, then `uvx schemathesis`.
    Auto,
    /// `schemathesis run ...` (must be on PATH).
    Schemathesis,
    /// `uvx schemathesis run ...` (uv installs on demand).
    Uvx,
}

pub fn run(args: TestSchemaArgs) -> Result<()> {
    let root = workspace_root()?;
    let spec_path = args
        .spec
        .clone()
        .unwrap_or_else(|| root.join("ext").join("specs").join(format!("{}.yaml", args.platform)));
    if !spec_path.exists() {
        bail!("spec not found at {}", spec_path.display());
    }

    let base_url = match args.base_url.clone() {
        Some(u) => u,
        None => extract_first_server_url(&spec_path)?,
    };

    let runner = pick_runner(args.runner)?;
    println!("Validating {} against {}", spec_path.display(), base_url);
    println!("  runner: {runner:?}");
    println!("  checks: {}", args.checks);

    let mut cmd = build_command(runner, &spec_path, &base_url, &args)?;
    let status = cmd
        .status()
        .with_context(|| format!("failed to spawn {runner:?}"))?;

    if !status.success() {
        bail!(
            "schemathesis reported {} (exit {})",
            if status.code() == Some(1) { "violations" } else { "an error" },
            status.code().unwrap_or(-1)
        );
    }
    println!("✓ schema conforms");
    Ok(())
}

fn pick_runner(requested: Runner) -> Result<Runner> {
    if requested != Runner::Auto {
        return Ok(requested);
    }
    if which("schemathesis") {
        return Ok(Runner::Schemathesis);
    }
    if which("uvx") {
        return Ok(Runner::Uvx);
    }
    bail!(
        "schemathesis not found. Install with:\n  \
         pipx install schemathesis    (recommended — isolated venv)\n  \
         pip install schemathesis     (any Python env)\n  \
         brew install uv              (then `uvx schemathesis ...` works)"
    );
}

fn which(bin: &str) -> bool {
    Command::new(bin)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn build_command(
    runner: Runner,
    spec_path: &Path,
    base_url: &str,
    args: &TestSchemaArgs,
) -> Result<Command> {
    let mut cmd = match runner {
        Runner::Schemathesis => {
            let mut c = Command::new("schemathesis");
            c.arg("run").arg(spec_path);
            c
        }
        Runner::Uvx => {
            let mut c = Command::new("uvx");
            c.arg("schemathesis").arg("run").arg(spec_path);
            c
        }
        Runner::Auto => unreachable!("auto resolved earlier"),
    };

    cmd.arg("--url").arg(base_url);
    cmd.arg("--checks").arg(&args.checks);
    cmd.arg("--max-examples").arg(args.max_examples.to_string());
    for h in &args.headers {
        cmd.arg("-H").arg(h);
    }
    for extra in &args.extra {
        cmd.arg(extra);
    }
    Ok(cmd)
}

fn extract_first_server_url(spec_path: &Path) -> Result<String> {
    let text = std::fs::read_to_string(spec_path)
        .with_context(|| format!("failed to read {}", spec_path.display()))?;
    let doc: serde_yaml::Value = serde_yaml::from_str(&text).context("spec is not valid YAML")?;
    let url = doc
        .get("servers")
        .and_then(|s| s.as_sequence())
        .and_then(|s| s.first())
        .and_then(|s| s.get("url"))
        .and_then(|u| u.as_str())
        .ok_or_else(|| eyre::eyre!("spec has no `servers[0].url` — pass --base-url"))?
        .to_string();
    Ok(url)
}
