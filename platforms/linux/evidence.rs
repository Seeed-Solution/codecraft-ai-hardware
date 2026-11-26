//! Offline host syntax/overlay evidence; does not establish device OS support.
use crate::util::*;
use anyhow::{ensure, Result};
use std::{fs, path::Path};
pub fn cases(root: &Path, t: &J) -> Result<Vec<String>> {
    let mut rows: Vec<_> = strings(&t["examples"])?
        .iter()
        .map(|ex| format!("{}/{ex}", t["id"].as_str().unwrap_or_default()))
        .collect();
    for entry in fs::read_dir(safe(root, s(&t["path"])?)?.join("overlays"))? {
        let p = entry?.path();
        if p.extension().and_then(|x| x.to_str()) == Some("dts") {
            rows.push(format!(
                "{}/overlay/{}",
                s(&t["id"])?,
                p.file_stem().unwrap().to_string_lossy()
            ));
        }
    }
    rows.sort();
    Ok(rows)
}
pub fn toolchain(_t: &J, a: &J) -> Result<()> {
    ensure!(
        a["status"] == "passed",
        "Linux evidence needs a passed probe"
    );
    ensure!(
        regex::Regex::new(r"^3\.[0-9]+\.[0-9]+")?.is_match(s(&a["python"])?),
        "Invalid Python identity"
    );
    ensure!(
        s(&a["dtc"])?.contains("DTC") && !s(&a["scope"])?.is_empty(),
        "Missing Linux overlay/scope identity"
    );
    Ok(())
}

/// Validate native source selection offline; syntax parsing uses the Rust parser.
pub fn validate_target(root: &Path, t: &J) -> Result<()> {
    use rustpython_parser::{ast, Parse};
    let base = s(&t["path"])?;
    for ex in strings(&t["examples"])? {
        let path = format!("{base}/demos/{ex}.py");
        let file = safe(root, &path)?;
        let inputs = strings(&t["example_inputs"][&ex])?;
        ensure!(
            inputs
                .iter()
                .any(|v| path == *v || path.starts_with(&format!("{v}/"))),
            "Linux demo must belong to its example source closure: {ex}"
        );
        ast::Suite::parse(&fs::read_to_string(file)?, &path)
            .map_err(|e| anyhow::anyhow!("Invalid native Python example: {e}"))?;
    }
    let overlays = safe(root, &format!("{base}/overlays"))?;
    let mut found = false;
    let inputs = strings(&t["inputs"])?;
    for entry in fs::read_dir(overlays)? {
        let path = entry?.path();
        if path.extension().and_then(|v| v.to_str()) == Some("dts") {
            found = true;
            let rel = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            ensure!(
                inputs
                    .iter()
                    .any(|v| rel == *v || rel.starts_with(&format!("{v}/"))),
                "Linux overlay missing from common source closure"
            );
        }
    }
    ensure!(found, "Linux overlay source required");
    let requirements = &t["toolchain"]["runtime_requirements"];
    ensure!(
        !requirements.is_null()
            && requirements != &serde_json::json!([])
            && requirements != &serde_json::json!(""),
        "Linux runtime prerequisites must be explicit"
    );
    Ok(())
}
