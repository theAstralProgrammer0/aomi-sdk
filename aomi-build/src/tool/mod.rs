use clap::Args;
use eyre::{Context, Result, bail};
use inquire::MultiSelect;
use std::path::PathBuf;

use crate::spec_load::{self, escape_keyword, pascal_case, snake_case};
use crate::specs::workspace_root;

mod render;

#[derive(Args, Debug)]
pub struct GenToolArgs {
    /// Platform name (looks for ext/specs/<platform>.yaml).
    pub platform: String,

    /// Override the spec path. Defaults to ext/specs/<platform>.yaml.
    #[arg(long)]
    pub spec: Option<PathBuf>,

    /// Override the output directory. Defaults to apps/<platform>/.
    #[arg(long)]
    pub out: Option<PathBuf>,

    /// Skip the interactive picker and include every operation.
    #[arg(long)]
    pub all: bool,

    /// Overwrite existing files (Cargo.toml, src/lib.rs, src/tool.rs).
    #[arg(long)]
    pub force: bool,
}

pub fn run(args: GenToolArgs) -> Result<()> {
    let root = workspace_root()?;
    let spec_path = args
        .spec
        .clone()
        .unwrap_or_else(|| root.join("ext").join("specs").join(format!("{}.yaml", args.platform)));
    let out_dir = args
        .out
        .clone()
        .unwrap_or_else(|| root.join("apps").join(&args.platform));

    println!("Reading spec: {}", spec_path.display());
    let spec = spec_load::load_and_preprocess(&spec_path)?;

    let ops = collect_ops(&spec);
    if ops.is_empty() {
        bail!("spec has no operations");
    }

    let chosen: Vec<Op> = if args.all {
        ops
    } else {
        let labels: Vec<String> = ops
            .iter()
            .map(|o| {
                format!(
                    "{} {} → {}_{}",
                    o.method.to_uppercase(),
                    o.path,
                    args.platform,
                    o.operation_id
                )
            })
            .collect();
        let picked = MultiSelect::new("Select operations to expose as Aomi tools:", labels.clone())
            .with_page_size(20)
            .prompt()
            .context("operation picker failed or was cancelled")?;
        picked
            .iter()
            .map(|p| {
                let idx = labels.iter().position(|l| l == p).expect("picked label exists");
                ops[idx].clone()
            })
            .collect()
    };

    if chosen.is_empty() {
        bail!("no operations selected");
    }

    let (chosen, skipped): (Vec<_>, Vec<_>) = chosen.into_iter().partition(|o| {
        !o.non_json_response && !o.params.iter().any(|p| p.kind == ParamKind::EnumString)
    });
    for op in &skipped {
        let reason = if op.non_json_response {
            "non-JSON response"
        } else {
            "has enum-typed params (progenitor generates Rust enums we can't synth from String)"
        };
        println!(
            "  skipped {} {} ({})",
            op.method.to_uppercase(),
            op.path,
            reason
        );
    }
    if chosen.is_empty() {
        bail!("all selected operations were skipped");
    }

    let src_dir = out_dir.join("src");
    std::fs::create_dir_all(&src_dir)
        .with_context(|| format!("failed to create {}", src_dir.display()))?;

    let cargo_toml_path = out_dir.join("Cargo.toml");
    let lib_rs_path = src_dir.join("lib.rs");
    let tool_rs_path = src_dir.join("tool.rs");

    for p in [&cargo_toml_path, &lib_rs_path, &tool_rs_path] {
        if p.exists() && !args.force {
            bail!(
                "{} already exists. Pass --force to overwrite.",
                p.display()
            );
        }
    }

    let app_struct = format!("{}App", pascal_case(&args.platform));
    let preamble = render::preamble_default(&args.platform);

    std::fs::write(&cargo_toml_path, render::cargo_toml(&args.platform))?;
    std::fs::write(&lib_rs_path, render::lib_rs(&args.platform, &app_struct, &chosen, &preamble))?;
    std::fs::write(&tool_rs_path, render::tool_rs(&args.platform, &app_struct, &chosen))?;

    println!("✓ wrote {}", cargo_toml_path.display());
    println!("✓ wrote {}", lib_rs_path.display());
    println!("✓ wrote {} ({} tools)", tool_rs_path.display(), chosen.len());
    println!();
    println!("Verify with:");
    println!("  cd apps/{} && cargo build", args.platform);
    Ok(())
}

#[derive(Debug, Clone)]
pub(crate) struct Op {
    pub operation_id: String,
    pub method: &'static str,
    pub path: String,
    pub server_url: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub params: Vec<Param>,
    pub has_request_body: bool,
    pub tool_marker: String,
    /// True when the success response is NOT JSON (e.g. text/csv, octet-stream).
    /// Such ops return ByteStream from progenitor — we skip JSON-style codegen.
    pub non_json_response: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Param {
    pub name: String,
    pub snake_name: String,
    pub location: ParamLoc,
    pub required: bool,
    pub kind: ParamKind,
    pub description: Option<String>,
    pub is_auth: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParamLoc {
    Path,
    Query,
    Header,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParamKind {
    String,
    Int32,
    Int64,
    Number,
    Boolean,
    /// String param with an `enum:` constraint — progenitor generates a typed
    /// enum that we can't synthesise from a plain String at the tool layer.
    /// We mark these and let the caller decide what to do.
    EnumString,
    /// Array, object, or anything else we don't try to type — surface as `String`
    /// in the generated Args (caller passes JSON-encoded text).
    Other,
}

fn collect_ops(spec: &openapiv3::OpenAPI) -> Vec<Op> {
    use openapiv3::ReferenceOr;
    let global_server = spec_load::default_server_url(spec);
    let mut out = Vec::new();
    for (path, item_ref) in spec.paths.paths.iter() {
        let ReferenceOr::Item(item) = item_ref else {
            continue;
        };
        let path_server = item
            .servers
            .first()
            .map(|s| s.url.clone())
            .unwrap_or_else(|| global_server.clone());
        for (method, op_opt) in [
            ("get", &item.get),
            ("put", &item.put),
            ("post", &item.post),
            ("delete", &item.delete),
            ("patch", &item.patch),
            ("head", &item.head),
            ("options", &item.options),
            ("trace", &item.trace),
        ] {
            let Some(op) = op_opt else { continue };
            let server_url = op
                .servers
                .first()
                .map(|s| s.url.clone())
                .unwrap_or_else(|| path_server.clone());
            let operation_id = op
                .operation_id
                .clone()
                .unwrap_or_else(|| format!("{method}_{}", snake_case(path)));
            let tool_marker = pascal_case(&operation_id);
            let mut params = Vec::new();
            for p_ref in &op.parameters {
                let ReferenceOr::Item(p) = p_ref else {
                    continue;
                };
                params.push(map_param(p));
            }
            // progenitor's positional ordering: path params in path-declaration order,
            // then everything else alphabetically by snake_case name.
            let path_order: Vec<&str> = path_param_order(path);
            let path_rank = |name: &str| -> usize {
                path_order
                    .iter()
                    .position(|p| *p == name)
                    .unwrap_or(usize::MAX)
            };
            params.sort_by(|a, b| match (a.location, b.location) {
                (ParamLoc::Path, ParamLoc::Path) => path_rank(&a.name).cmp(&path_rank(&b.name)),
                (ParamLoc::Path, _) => std::cmp::Ordering::Less,
                (_, ParamLoc::Path) => std::cmp::Ordering::Greater,
                _ => a.snake_name.cmp(&b.snake_name),
            });
            let non_json_response = first_success_content_type(op)
                .map(|ct| !ct.starts_with("application/json"))
                .unwrap_or(false);
            out.push(Op {
                operation_id,
                method,
                path: path.clone(),
                server_url,
                summary: op.summary.clone(),
                description: op.description.clone(),
                params,
                has_request_body: op.request_body.is_some(),
                tool_marker,
                non_json_response,
            });
        }
    }
    out
}

fn map_param(p: &openapiv3::Parameter) -> Param {
    use openapiv3::Parameter;
    let (data, location) = match p {
        Parameter::Path { parameter_data, .. } => (parameter_data, ParamLoc::Path),
        Parameter::Query { parameter_data, .. } => (parameter_data, ParamLoc::Query),
        Parameter::Header { parameter_data, .. } => (parameter_data, ParamLoc::Header),
        Parameter::Cookie { parameter_data, .. } => (parameter_data, ParamLoc::Header),
    };
    let kind = schema_kind(&data.format);
    let snake_name = escape_keyword(&snake_case(&data.name));
    let is_auth = header_looks_like_auth(&data.name);
    Param {
        name: data.name.clone(),
        snake_name,
        location,
        required: data.required,
        kind,
        description: data.description.clone(),
        is_auth,
    }
}

fn schema_kind(format: &openapiv3::ParameterSchemaOrContent) -> ParamKind {
    use openapiv3::{
        IntegerFormat, ParameterSchemaOrContent, ReferenceOr, SchemaKind, Type, VariantOrUnknownOrEmpty,
    };
    let ParameterSchemaOrContent::Schema(ReferenceOr::Item(schema)) = format else {
        return ParamKind::Other;
    };
    match &schema.schema_kind {
        SchemaKind::Type(Type::String(s)) => {
            if !s.enumeration.is_empty() {
                ParamKind::EnumString
            } else {
                ParamKind::String
            }
        }
        SchemaKind::Type(Type::Integer(i)) => match &i.format {
            VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32) => ParamKind::Int32,
            _ => ParamKind::Int64,
        },
        SchemaKind::Type(Type::Number(_)) => ParamKind::Number,
        SchemaKind::Type(Type::Boolean(_)) => ParamKind::Boolean,
        _ => ParamKind::Other,
    }
}

/// Extract the path-parameter names from a path template like
/// `/v1/{owner}/dashboards/{dashboard_id}` → `["owner", "dashboard_id"]`.
fn path_param_order(path: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'}' {
                j += 1;
            }
            if j < bytes.len() {
                if let Ok(name) = std::str::from_utf8(&bytes[start..j]) {
                    out.push(name);
                }
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn first_success_content_type(op: &openapiv3::Operation) -> Option<String> {
    use openapiv3::ReferenceOr;
    for (code, resp_ref) in &op.responses.responses {
        let is_2xx = matches!(code, openapiv3::StatusCode::Code(c) if (200..300).contains(c))
            || matches!(code, openapiv3::StatusCode::Range(2));
        if !is_2xx {
            continue;
        }
        let ReferenceOr::Item(resp) = resp_ref else {
            continue;
        };
        if let Some((ct, _)) = resp.content.iter().next() {
            return Some(ct.clone());
        }
    }
    None
}

fn header_looks_like_auth(name: &str) -> bool {
    let n: String = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    n.contains("apikey") || n == "authorization" || n.ends_with("apitoken") || n.ends_with("token")
}
