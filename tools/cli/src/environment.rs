//! Read-only environment inspection reuses the platform's actual build probe.
use crate::{catalog::Catalog, platform, util::*};
use anyhow::{Context, Result};
use serde_json::json;
use std::path::Path;

pub fn inspect(root: &Path, target: &str) -> Result<J> {
    let catalog = Catalog::load(root)?;
    let selected = catalog.targets.get(target).context("Unknown Target")?;
    let adapter = platform::adapter(s(&selected["platform"])?)?;
    let actual = adapter.probe(&catalog.root, selected).unwrap_or_else(|error| {
        json!({"status":"not_run","reason":format!("Environment probe could not complete: {error:#}")})
    });
    let ready = actual["status"] == "passed";
    Ok(json!({
        "target": target,
        "status": if ready { "passed" } else { "not_run" },
        "expected": selected["toolchain"],
        "actual": actual,
        "scope": "Activated host toolchain only; no compilation, lock update, device access or runtime compatibility verification.",
        "next_steps": if ready { vec![] } else { vec![json!({
            "action": "Read setup guidance, then install missing prerequisites or activate the declared existing toolchain in the build shell.",
            "command": ["codecraft", "context", "--target", target, "--task", "setup"],
            "retry": ["codecraft", "doctor", "--target", target]
        })] }
    }))
}
