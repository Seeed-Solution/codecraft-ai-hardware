//! Offline Arduino dependency identity validation.
use crate::util::*;
use anyhow::{ensure, Result};
use std::path::Path;
pub fn cases(_root: &Path, t: &J) -> Result<Vec<String>> {
    strings(&t["examples"])?
        .iter()
        .map(|ex| Ok(format!("{}/{ex}", s(&t["id"])?)))
        .collect()
}
pub fn toolchain(t: &J, a: &J) -> Result<()> {
    let tc = &t["toolchain"];
    ensure!(
        a["status"] == "passed"
            && a["cli"] == tc["version"]
            && a["fqbn"] == tc["fqbn"]
            && a["core"]["version"] == tc["core_version"],
        "Arduino evidence toolchain mismatch"
    );
    let fqbn = s(&tc["fqbn"])?;
    ensure!(
        a["core"]["id"] == fqbn.split(':').take(2).collect::<Vec<_>>().join(":"),
        "Arduino core identity mismatch"
    );
    let hash = regex::Regex::new(r"^[0-9a-f]{64}$")?;
    ensure!(
        hash.is_match(s(&a["core"]["source_digest"])?),
        "Missing core fingerprint"
    );
    ensure!(
        obj(&a["libraries"])?.len() == obj(&tc["libraries"])?.len(),
        "Arduino library set mismatch"
    );
    for (name, version) in obj(&tc["libraries"])? {
        let l = &a["libraries"][name];
        ensure!(
            l["version"] == *version && hash.is_match(s(&l["source_digest"])?),
            "Arduino library identity mismatch: {name}"
        );
    }
    let bundled: std::collections::BTreeSet<_> =
        obj(&a["bundled_libraries"])?.keys().cloned().collect();
    ensure!(
        bundled == strings(&tc["bundled_libraries"])?.into_iter().collect(),
        "Bundled library mismatch"
    );
    ensure!(
        !obj(&a["tools"])?.is_empty(),
        "Native tool identities missing"
    );
    for v in obj(&a["tools"])?.values() {
        ensure!(
            !s(&v["version"])?.is_empty() && hash.is_match(s(&v["source_digest"])?),
            "Native tool fingerprint missing"
        );
    }
    Ok(())
}

/// Native metadata checks do not require an installed Arduino toolchain.
pub fn validate_target(root: &Path, t: &J) -> Result<()> {
    let tc = &t["toolchain"];
    ensure!(
        regex::Regex::new(r"^[\w-]+:[\w-]+:[\w-]+$")?.is_match(s(&tc["fqbn"])?),
        "Invalid Arduino FQBN"
    );
    let version = regex::Regex::new(r"^\d+\.\d+\.\d+(?:[-+][a-zA-Z0-9.]+)?$")?;
    for value in [&tc["version"], &tc["core_version"]]
        .into_iter()
        .chain(obj(&tc["libraries"])?.values())
    {
        ensure!(
            version.is_match(s(value)?),
            "Arduino versions must be pinned"
        );
    }
    ensure!(
        obj(&tc["libraries"])?.keys().all(|v| !v.trim().is_empty()),
        "Invalid library name"
    );
    let name = regex::Regex::new(r"^[\w-]+$")?;
    ensure!(
        strings(&tc["bundled_libraries"])?
            .iter()
            .all(|v| name.is_match(v)),
        "Invalid bundled library directory"
    );
    ensure!(
        strings(&tc["board_manager_urls"])?
            .iter()
            .all(|v| v.starts_with("https://")),
        "Invalid board-manager URL"
    );
    for ex in strings(&t["examples"])? {
        let dir = format!("{}/examples/{ex}", s(&t["path"])?);
        let sketch = format!("{dir}/{ex}.ino");
        safe(root, &sketch)?;
        let inputs = strings(&t["example_inputs"][&ex])?;
        ensure!(
            inputs.contains(&dir) || inputs.contains(&sketch),
            "Arduino sketch must belong to its example source closure: {ex}"
        );
    }
    Ok(())
}
