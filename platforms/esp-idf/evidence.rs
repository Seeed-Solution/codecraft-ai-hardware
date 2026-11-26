//! Offline evidence checks; never probe or install the reviewer's SDK.
use crate::util::*;
use anyhow::{ensure, Result};
use std::path::Path;
pub fn cases(_root: &Path, t: &J) -> Result<Vec<String>> {
    let variants: Vec<Option<&String>> = if obj(&t["variants"])?.is_empty() {
        vec![None]
    } else {
        obj(&t["variants"])?.keys().map(Some).collect()
    };
    let mut rows = vec![];
    for ex in strings(&t["examples"])? {
        for v in &variants {
            rows.push(format!(
                "{}/{}{}",
                s(&t["id"])?,
                ex,
                v.map(|x| format!("/{x}")).unwrap_or_default()
            ));
        }
    }
    Ok(rows)
}
pub fn toolchain(t: &J, actual: &J) -> Result<()> {
    ensure!(
        actual["status"] == "passed",
        "Evidence needs a passed ESP-IDF probe"
    );
    ensure!(
        super::esp_idf::version_matches(s(&actual["version"])?, s(&t["toolchain"]["version"])?),
        "ESP-IDF evidence version mismatch"
    );
    for k in ["compiler", "python", "sdk_commit", "sdk_changes_sha256"] {
        ensure!(!s(&actual[k])?.is_empty(), "Missing ESP-IDF identity {k}");
    }
    ensure!(
        regex::Regex::new(r"^[0-9a-f]{40}$")?.is_match(s(&actual["sdk_commit"])?),
        "Invalid SDK commit"
    );
    ensure!(
        regex::Regex::new(r"^[0-9a-f]{64}$")?.is_match(s(&actual["sdk_changes_sha256"])?),
        "Invalid SDK changes hash"
    );
    s(&actual["sdk_submodules"])?;
    Ok(())
}
