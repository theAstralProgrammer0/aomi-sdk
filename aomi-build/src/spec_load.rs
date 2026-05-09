//! Loading + preprocessing OpenAPI specs to satisfy progenitor.
//!
//! Real-world specs frequently violate progenitor's strict expectations.
//! Each helper here is a small, named patch with a printed summary so the
//! caller knows what was rewritten.

use eyre::{Context, Result};
use std::path::Path;

pub fn load_and_preprocess(spec_path: &Path) -> Result<openapiv3::OpenAPI> {
    let spec_text = std::fs::read_to_string(spec_path)
        .with_context(|| format!("failed to read {}", spec_path.display()))?;
    let spec_text = downgrade_to_30(&spec_text);
    let mut spec: openapiv3::OpenAPI =
        if spec_path.extension().and_then(|e| e.to_str()) == Some("json") {
            serde_json::from_str(&spec_text).context("spec is not valid JSON")?
        } else {
            serde_yaml::from_str(&spec_text).context("spec is not valid YAML")?
        };
    let n = fill_missing_operation_ids(&mut spec);
    if n > 0 {
        println!("  filled in {n} missing operationId(s)");
    }
    let n = rename_wildcard_content_types(&mut spec);
    if n > 0 {
        println!("  renamed {n} `*/*` content type(s) → application/json");
    }
    let n = dedupe_success_responses(&mut spec);
    if n > 0 {
        println!("  dropped {n} duplicate success response(s) (progenitor allows only one)");
    }
    let n = drop_param_name_collisions(&mut spec);
    if n > 0 {
        println!("  dropped {n} parameter(s) whose snake_case name collided with a path param");
    }
    let n = drop_multipart_ops(&mut spec);
    if n > 0 {
        println!("  dropped {n} operation(s) with multipart request bodies (progenitor doesn't support multipart)");
    }
    Ok(spec)
}

/// progenitor doesn't handle `multipart/form-data` request bodies. Drop those
/// operations entirely from the spec — they need hand-written wrappers.
fn drop_multipart_ops(spec: &mut openapiv3::OpenAPI) -> usize {
    use openapiv3::ReferenceOr;
    let mut dropped = 0;
    for item_ref in spec.paths.paths.values_mut() {
        let ReferenceOr::Item(item) = item_ref else {
            continue;
        };
        for op_slot in [
            &mut item.get,
            &mut item.put,
            &mut item.post,
            &mut item.delete,
            &mut item.patch,
            &mut item.head,
            &mut item.options,
            &mut item.trace,
        ] {
            if let Some(op) = op_slot.as_ref() {
                let has_multipart = match &op.request_body {
                    Some(ReferenceOr::Item(rb)) => rb.content.contains_key("multipart/form-data"),
                    _ => false,
                };
                if has_multipart {
                    *op_slot = None;
                    dropped += 1;
                }
            }
        }
    }
    dropped
}

fn downgrade_to_30(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut changed = false;
    for line in src.lines() {
        if let Some(rest) = line.strip_prefix("openapi: ") {
            let v = rest.trim().trim_matches(|c: char| c == '"' || c == '\'');
            if v.starts_with("3.1") {
                out.push_str("openapi: 3.0.3\n");
                changed = true;
                continue;
            }
        } else if let Some(rest) = line.strip_prefix("\"openapi\": ") {
            let v = rest.trim_end_matches(',').trim().trim_matches('"');
            if v.starts_with("3.1") {
                out.push_str("\"openapi\": \"3.0.3\",\n");
                changed = true;
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    if changed {
        println!("  downgraded openapi 3.1 → 3.0.3 (progenitor doesn't support 3.1)");
    }
    out
}

fn fill_missing_operation_ids(spec: &mut openapiv3::OpenAPI) -> usize {
    use openapiv3::ReferenceOr;
    let mut count = 0;
    for (path, item_ref) in spec.paths.paths.iter_mut() {
        let item = match item_ref {
            ReferenceOr::Item(i) => i,
            ReferenceOr::Reference { .. } => continue,
        };
        for (method, op_opt) in [
            ("get", &mut item.get),
            ("put", &mut item.put),
            ("post", &mut item.post),
            ("delete", &mut item.delete),
            ("patch", &mut item.patch),
            ("head", &mut item.head),
            ("options", &mut item.options),
            ("trace", &mut item.trace),
        ] {
            if let Some(op) = op_opt
                && op.operation_id.is_none()
            {
                op.operation_id = Some(synthesize_op_id(method, path));
                count += 1;
            }
        }
    }
    count
}

fn rename_wildcard_content_types(spec: &mut openapiv3::OpenAPI) -> usize {
    use openapiv3::ReferenceOr;
    let mut count = 0;
    fn fix(c: &mut indexmap::IndexMap<String, openapiv3::MediaType>, n: &mut usize) {
        if let Some(media) = c.shift_remove("*/*") {
            c.entry("application/json".to_string()).or_insert(media);
            *n += 1;
        }
    }
    for item_ref in spec.paths.paths.values_mut() {
        let ReferenceOr::Item(item) = item_ref else {
            continue;
        };
        for op in [
            &mut item.get,
            &mut item.put,
            &mut item.post,
            &mut item.delete,
            &mut item.patch,
            &mut item.head,
            &mut item.options,
            &mut item.trace,
        ]
        .into_iter()
        .flatten()
        {
            if let Some(ReferenceOr::Item(req)) = &mut op.request_body {
                fix(&mut req.content, &mut count);
            }
            for resp_ref in op.responses.responses.values_mut() {
                if let ReferenceOr::Item(resp) = resp_ref {
                    fix(&mut resp.content, &mut count);
                }
            }
            if let Some(ReferenceOr::Item(resp)) = op.responses.default.as_mut() {
                fix(&mut resp.content, &mut count);
            }
        }
    }
    count
}

fn dedupe_success_responses(spec: &mut openapiv3::OpenAPI) -> usize {
    use openapiv3::ReferenceOr;
    let mut dropped = 0;
    for item_ref in spec.paths.paths.values_mut() {
        let ReferenceOr::Item(item) = item_ref else {
            continue;
        };
        for op in [
            &mut item.get,
            &mut item.put,
            &mut item.post,
            &mut item.delete,
            &mut item.patch,
            &mut item.head,
            &mut item.options,
            &mut item.trace,
        ]
        .into_iter()
        .flatten()
        {
            let to_remove: Vec<openapiv3::StatusCode> = {
                let mut seen = false;
                op.responses
                    .responses
                    .iter()
                    .filter_map(|(code, r)| {
                        let has_body = matches!(r, ReferenceOr::Item(rr) if !rr.content.is_empty());
                        if !has_body {
                            return None;
                        }
                        if is_success(code) {
                            if seen {
                                Some(code.clone())
                            } else {
                                seen = true;
                                None
                            }
                        } else {
                            Some(code.clone())
                        }
                    })
                    .collect()
            };
            for code in &to_remove {
                op.responses.responses.shift_remove(code);
                dropped += 1;
            }
        }
    }
    dropped
}

fn drop_param_name_collisions(spec: &mut openapiv3::OpenAPI) -> usize {
    use openapiv3::{Parameter, ReferenceOr};
    let mut dropped = 0;
    for item_ref in spec.paths.paths.values_mut() {
        let ReferenceOr::Item(item) = item_ref else {
            continue;
        };
        for op in [
            &mut item.get,
            &mut item.put,
            &mut item.post,
            &mut item.delete,
            &mut item.patch,
            &mut item.head,
            &mut item.options,
            &mut item.trace,
        ]
        .into_iter()
        .flatten()
        {
            let path_keys: std::collections::HashSet<String> = op
                .parameters
                .iter()
                .filter_map(|p| match p {
                    ReferenceOr::Item(Parameter::Path { parameter_data, .. }) => {
                        Some(snake_case(&parameter_data.name))
                    }
                    _ => None,
                })
                .collect();
            let before = op.parameters.len();
            op.parameters.retain(|p| match p {
                ReferenceOr::Item(Parameter::Query { parameter_data, .. })
                | ReferenceOr::Item(Parameter::Header { parameter_data, .. })
                | ReferenceOr::Item(Parameter::Cookie { parameter_data, .. }) => {
                    !path_keys.contains(&snake_case(&parameter_data.name))
                }
                _ => true,
            });
            dropped += before - op.parameters.len();
        }
    }
    dropped
}

pub fn snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    let mut prev_lower = false;
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_lower = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_lower = true;
        } else {
            if !out.ends_with('_') {
                out.push('_');
            }
            prev_lower = false;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    out
}

/// Append `_` if `s` is a Rust 2024 keyword (matches progenitor's convention).
pub fn escape_keyword(s: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "do", "dyn", "else",
        "enum", "extern", "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match",
        "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super",
        "trait", "true", "try", "type", "typeof", "union", "unsafe", "unsized", "use", "virtual",
        "where", "while", "yield",
    ];
    if KEYWORDS.contains(&s) {
        format!("{s}_")
    } else {
        s.to_string()
    }
}

pub fn pascal_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper_next = true;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if upper_next {
                out.push(ch.to_ascii_uppercase());
            } else {
                out.push(ch);
            }
            upper_next = false;
        } else {
            upper_next = true;
        }
    }
    out
}

fn is_success(code: &openapiv3::StatusCode) -> bool {
    match code {
        openapiv3::StatusCode::Code(c) => (200..300).contains(c),
        openapiv3::StatusCode::Range(2) => true,
        _ => false,
    }
}

fn synthesize_op_id(method: &str, path: &str) -> String {
    let mut s = String::from(method);
    let mut sep = true;
    for ch in path.chars() {
        match ch {
            '/' | '{' | '}' | '-' | '.' => {
                if !sep {
                    s.push('_');
                    sep = true;
                }
            }
            c if c.is_ascii_alphanumeric() => {
                s.push(c.to_ascii_lowercase());
                sep = false;
            }
            _ => {}
        }
    }
    while s.ends_with('_') {
        s.pop();
    }
    s
}

pub fn default_server_url(spec: &openapiv3::OpenAPI) -> String {
    spec.servers
        .first()
        .map(|s| s.url.clone())
        .unwrap_or_default()
}
