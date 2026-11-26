//! Shared offline semantic contracts. No hardware catalog or service execution.
use crate::util::*;
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::collections::BTreeSet;

fn keys(v: &J) -> Result<BTreeSet<&str>> {
    Ok(obj(v)?.keys().map(String::as_str).collect())
}
fn exact(v: &J, names: &[&str]) -> Result<()> {
    ensure!(
        keys(v)? == names.iter().copied().collect(),
        "Unexpected contract fields"
    );
    Ok(())
}
pub fn version(v: &J) -> Result<Vec<u64>> {
    let text = s(v)?;
    ensure!(
        regex::Regex::new(r"^\d+\.\d+\.\d+$")?.is_match(text),
        "Version must be major.minor.patch"
    );
    text.split('.').map(|n| Ok(n.parse()?)).collect()
}
pub fn schema(v: &J) -> Result<()> {
    ensure!(
        v["description"].as_str().is_some_and(|s| !s.is_empty()),
        "Schema description required"
    );
    match s(&v["type"])? {
        "object" => {
            ensure!(
                v["additionalProperties"].is_boolean(),
                "Explicit properties policy required"
            );
            let p = obj(&v["properties"])?;
            for k in strings(&v["required"])? {
                ensure!(p.contains_key(&k), "Unknown required property");
            }
            for child in p.values() {
                schema(child)?;
            }
        }
        "array" => schema(&v["items"])?,
        "string" | "number" | "integer" | "boolean" | "null" => {}
        _ => anyhow::bail!("Unknown schema type"),
    }
    for (a, b) in [("minimum", "maximum"), ("minLength", "maxLength")] {
        if let (Some(a), Some(b)) = (v.get(a), v.get(b)) {
            ensure!(
                a.as_f64().context("Invalid bound")? <= b.as_f64().context("Invalid bound")?,
                "Inverted schema range"
            );
        }
    }
    if let Some(p) = v.get("pattern") {
        regex::Regex::new(s(p)?)?;
    }
    if let Some(p) = v.get("policy_default") {
        ensure!(p.is_boolean(), "policy_default must be boolean");
    }
    if let Some(e) = v.get("enum") {
        ensure!(!arr(e)?.is_empty(), "Empty enum");
    }
    Ok(())
}
fn limits(v: &J) -> Result<()> {
    match v {
        J::Null | J::Bool(_) => {}
        J::Number(n) => ensure!(
            n.as_f64().is_some_and(|n| n >= 0. && n.is_finite()),
            "Invalid service limit"
        ),
        J::String(s) => ensure!(!s.is_empty(), "Empty service limit"),
        J::Array(a) => {
            ensure!(!a.is_empty(), "Empty limit array");
            for v in a {
                limits(v)?;
            }
        }
        J::Object(m) => {
            ensure!(!m.is_empty(), "Empty limit map");
            for (k, v) in m {
                ensure!(!k.is_empty(), "Empty limit name");
                limits(v)?;
            }
        }
    };
    Ok(())
}
pub fn policy(c: &J, files: &J) -> Result<J> {
    let p = &c["policy"];
    exact(
        p,
        &[
            "schema_version",
            "version",
            "defaults",
            "limits",
            "configuration",
            "operations",
            "billing",
            "sources",
        ],
    )?;
    ensure!(p["schema_version"] == 1, "Policy schema version");
    version(&p["version"])?;
    ensure!(
        keys(&p["limits"])?.is_disjoint(&keys(&c["limits"])?),
        "Policy cannot override semantic limits"
    );
    for v in obj(&p["limits"])?.values() {
        limits(v)?;
    }
    let sources = strings(&p["sources"])?;
    ensure!(
        !sources.is_empty() && sources.iter().all(|s| files.get(s).is_some()),
        "Policy sources must be pinned"
    );
    let b = &p["billing"];
    exact(b, &["status", "unit", "basis", "amount"])?;
    match s(&b["status"])? {
        "unknown" => ensure!(
            ["unit", "basis", "amount"].iter().all(|k| b[k].is_null()),
            "Unknown billing must remain unknown"
        ),
        "declared" => {
            ensure!(
                !s(&b["unit"])?.is_empty() && !s(&b["basis"])?.is_empty(),
                "Billing identity required"
            );
            ensure!(
                b["amount"].is_null()
                    || b["amount"]
                        .as_f64()
                        .is_some_and(|n| n >= 0. && n.is_finite()),
                "Invalid billing amount"
            );
        }
        _ => anyhow::bail!("Invalid billing status"),
    }
    let config = crate::skills::policy(&c["configuration"], &p["configuration"])?;
    schema(&config)?;
    for (k, v) in obj(&p["defaults"])? {
        ensure!(
            c["configuration"]["properties"][k]["policy_default"] == true,
            "Default requires explicit permission"
        );
        crate::skills::payload(&config["properties"][k], v)?;
    }
    ensure!(
        keys(&p["operations"])? == keys(&c["operations"])?,
        "Policy operation coverage mismatch"
    );
    let mut operations = object();
    for (name, op) in obj(&c["operations"])? {
        exact(&p["operations"][name], &["input", "output"])?;
        for kind in ["input", "output"] {
            let e = crate::skills::policy(&op[kind], &p["operations"][name][kind])?;
            schema(&e)?;
            if let Some(en) = e.get("enum") {
                for v in arr(en)? {
                    crate::skills::payload(&e, v)?;
                }
            }
            operations[name][kind] = e;
        }
    }
    Ok(json!({"configuration":config,"operations":operations}))
}
pub fn validate(c: &J, files: &J) -> Result<J> {
    let name_pattern = regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")?;
    let flag_pattern = regex::Regex::new(r"^--[a-z][a-z-]*$")?;
    let env_pattern = regex::Regex::new(r"^[A-Z][A-Z0-9_]*$")?;
    exact(
        c,
        &[
            "schema_version",
            "id",
            "version",
            "consumer",
            "description",
            "configuration",
            "operations",
            "resources",
            "estimates",
            "errors",
            "risk",
            "limits",
            "limitations",
            "execution",
            "client",
            "api",
            "policy",
            "prerequisites",
        ],
    )?;
    ensure!(
        c["schema_version"] == 2 && c["consumer"] == "cloud-skill",
        "Invalid contract schema/consumer"
    );
    version(&c["version"])?;
    ensure!(
        name_pattern.is_match(s(&c["id"])?),
        "Invalid Skill identity"
    );
    ensure!(
        !s(&c["description"]["en"])?.is_empty() && !s(&c["description"]["zh_CN"])?.is_empty(),
        "Localized description required"
    );
    ensure!(!obj(&c["operations"])?.is_empty(), "Operations required");
    for op in obj(&c["operations"])?.values() {
        schema(&op["input"])?;
        schema(&op["output"])?;
    }
    schema(&c["configuration"])?;
    exact(&c["resources"], &["memory_bytes", "storage_bytes"])?;
    exact(&c["estimates"], &["memory_bytes", "storage_bytes"])?;
    for v in obj(&c["estimates"])?.values() {
        ensure!(v.is_null() || v.is_u64(), "Invalid estimate");
    }
    for k in [
        "unsupported",
        "unknown",
        "invalid_input",
        "provider_failure",
    ] {
        ensure!(!s(&c["errors"][k])?.is_empty(), "Error semantics required");
    }
    ensure!(
        !c["risk"].is_null()
            && !arr(&c["limitations"])?.is_empty()
            && c["limits"]["automatic_retry"] == false,
        "Risk/limitations/no-retry required"
    );
    ensure!(
        c["execution"] == json!({"execution":"agent-host","device_integration":"unknown"}),
        "Host execution is not device integration"
    );
    let cl = &c["client"];
    exact(cl, &["python_minimum", "operations"])?;
    ensure!(
        !s(&cl["python_minimum"])?.is_empty()
            && files.get("SKILL.md").is_some()
            && files.get("contract.json").is_some(),
        "Pinned entry and contract required"
    );
    ensure!(
        keys(&cl["operations"])? == keys(&c["operations"])?,
        "Client operation coverage mismatch"
    );
    for (name, m) in obj(&cl["operations"])? {
        exact(
            m,
            &[
                "entrypoint",
                "command",
                "arguments",
                "options",
                "fixed_arguments",
            ],
        )?;
        let entry = s(&m["entrypoint"])?;
        ensure!(
            entry.ends_with(".py") && files.get(entry).is_some(),
            "Client entry must be pinned"
        );
        for part in strings(&m["command"])? {
            ensure!(name_pattern.is_match(&part), "Invalid command");
        }
        ensure!(
            keys(&m["arguments"])? == keys(&c["operations"][name]["input"]["properties"])?
                && keys(&m["options"])? == keys(&c["configuration"]["properties"])?,
            "CLI mapping/schema mismatch"
        );
        let mut flags = BTreeSet::new();
        for v in obj(&m["arguments"])?
            .values()
            .chain(obj(&m["options"])?.values())
            .chain(arr(&m["fixed_arguments"])?.iter())
        {
            let f = s(v)?;
            ensure!(
                flag_pattern.is_match(f) && flags.insert(f),
                "Invalid/duplicate flag"
            );
        }
    }
    let api = &c["api"];
    exact(api, &["version", "authorization", "routing", "operations"])?;
    ensure!(
        !s(&api["version"])?.is_empty()
            && api["authorization"] == "Device"
            && api["routing"] == "service-fixed",
        "Invalid API binding"
    );
    ensure!(
        keys(&api["operations"])? == keys(&c["operations"])?,
        "API coverage mismatch"
    );
    for b in obj(&api["operations"])?.values() {
        exact(b, &["method", "path"])?;
        ensure!(
            ["GET", "POST"].contains(&s(&b["method"])?)
                && s(&b["path"])?.starts_with("/deviceapi/"),
            "Invalid API route"
        );
    }
    let pre = &c["prerequisites"];
    for k in ["network", "tls", "loopback_http"] {
        ensure!(pre[k].is_boolean(), "Prerequisite boolean required");
    }
    ensure!(!s(&pre["privacy"])?.is_empty(), "Privacy required");
    for k in ["environment_refs", "secret_refs"] {
        for v in strings(&pre[k])? {
            ensure!(env_pattern.is_match(&v), "Credential values forbidden");
        }
    }
    policy(c, files)
}
