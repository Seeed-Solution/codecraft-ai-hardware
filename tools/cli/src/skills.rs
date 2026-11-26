//! Explicit package selections, verified offline. Never execute clients or read credentials.
use crate::util::*;
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{collections::BTreeSet, path::Path};
pub fn payload(spec: &J, value: &J) -> Result<()> {
    if value.is_null() && spec["nullable"] == true {
        return Ok(());
    }
    let kind = s(&spec["type"])?;
    ensure!(
        match kind {
            "object" => value.is_object(),
            "array" => value.is_array(),
            "string" => value.is_string(),
            "integer" => value.is_i64() || value.is_u64(),
            "number" => value.is_number(),
            "boolean" => value.is_boolean(),
            "null" => value.is_null(),
            _ => false,
        },
        "Payload type mismatch"
    );
    if kind == "object" {
        for k in strings(&spec["required"])? {
            ensure!(
                value.get(&k).is_some(),
                "Missing required configuration: {k}"
            );
        }
        for (k, v) in obj(value)? {
            if let Some(child) = spec["properties"].get(k) {
                payload(child, v)?;
            } else {
                ensure!(
                    spec["additionalProperties"] == true,
                    "Unknown configuration: {k}"
                );
            }
        }
    }
    if kind == "array" {
        for v in arr(value)? {
            payload(&spec["items"], v)?;
        }
    }
    if let Some(v) = spec.get("enum") {
        ensure!(arr(v)?.contains(value), "Value outside enum");
    }
    for (key, lower) in [("minimum", true), ("maximum", false)] {
        if let Some(limit) = spec.get(key) {
            let n = value.as_f64().context("Numeric constraint on nonnumber")?;
            let bound = limit.as_f64().context("Invalid numeric bound")?;
            ensure!(
                if lower { n >= bound } else { n <= bound },
                "Value outside {key}"
            );
        }
    }
    for (key, lower) in [("minLength", true), ("maxLength", false)] {
        if let Some(limit) = spec.get(key) {
            let n = s(value)?.chars().count() as u64;
            let bound = limit.as_u64().context("Invalid length")?;
            ensure!(
                if lower { n >= bound } else { n <= bound },
                "Value outside {key}"
            );
        }
    }
    if let Some(pattern) = spec.get("pattern") {
        ensure!(
            regex::Regex::new(&format!("^(?:{})$", s(pattern)?))?.is_match(s(value)?),
            "Pattern mismatch"
        );
    }
    if let Some(keys) = spec.get("oneRequired") {
        ensure!(
            strings(keys)?
                .iter()
                .filter(|k| value.get(k.as_str()).is_some())
                .count()
                == 1,
            "Exactly one input required"
        );
    }
    Ok(())
}
pub(crate) fn policy(base: &J, limits: &J) -> Result<J> {
    let mut out = base.clone();
    for (k, v) in obj(limits)? {
        match k.as_str() {
            "properties" => {
                ensure!(
                    base["type"] == "object",
                    "Properties require an object schema"
                );
                for (name, child) in obj(v)? {
                    out["properties"][name] = policy(
                        base["properties"]
                            .get(name)
                            .context("Policy added a field")?,
                        child,
                    )?;
                }
            }
            "items" => {
                ensure!(base["type"] == "array", "Items require an array schema");
                out[k] = policy(&base[k], v)?;
            }
            "required" => {
                let mut keys: BTreeSet<_> = strings(&base[k])?.into_iter().collect();
                for name in strings(v)? {
                    ensure!(
                        base["properties"].get(&name).is_some(),
                        "Policy requires unknown field"
                    );
                    keys.insert(name);
                }
                out[k] = json!(keys);
            }
            "additionalProperties" => {
                ensure!(
                    base["type"] == "object",
                    "Properties policy requires object"
                );
                ensure!(*v == false, "Policy may only prohibit extra fields");
                out[k] = v.clone();
            }
            "enum" => {
                ensure!(!arr(v)?.is_empty(), "Empty policy enum");
                for item in arr(v)? {
                    payload(base, item)?;
                }
                out[k] = v.clone();
            }
            "minimum" | "maximum" | "minLength" | "maxLength" => {
                if k.ends_with("Length") {
                    ensure!(
                        base["type"] == "string" && v.is_u64(),
                        "Length bound requires string and nonnegative integer"
                    );
                } else {
                    ensure!(
                        (base["type"] == "number" || base["type"] == "integer") && v.is_number(),
                        "Numeric bound requires numeric schema"
                    );
                }
                if let Some(old) = base.get(k) {
                    let new = v.as_f64().context("Invalid policy bound")?;
                    let old = old.as_f64().context("Invalid contract bound")?;
                    ensure!(
                        if k.starts_with("min") {
                            new >= old
                        } else {
                            new <= old
                        },
                        "Policy weakens {k}"
                    );
                }
                out[k] = v.clone();
            }
            "pattern" => {
                ensure!(base["type"] == "string", "Pattern requires string");
                ensure!(
                    base.get(k).is_none() || base[k] == *v,
                    "Policy replaces contract pattern"
                );
                regex::Regex::new(s(v)?)?;
                out[k] = v.clone();
            }
            _ => anyhow::bail!("Unknown policy constraint {k}"),
        }
    }
    crate::maintenance_contracts::schema(&out)?;
    if let Some(values) = out.get("enum") {
        for value in arr(values)? {
            payload(&out, value)?;
        }
    }
    Ok(out)
}
pub fn package(root: &Path, id: &str) -> Result<(J, J)> {
    ensure!(
        regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")?.is_match(id),
        "Invalid Skill ID"
    );
    let record = read(&safe(
        root,
        &format!("tools/skill_packages/packages/{id}.json"),
    )?)?;
    ensure!(
        record["schema_version"] == 1 && record["id"] == id,
        "Invalid Skill release"
    );
    let dir = safe(root, &format!("skills/{id}"))?;
    let mut actual = object();
    for entry in walkdir::WalkDir::new(&dir)
        .into_iter()
        .filter_entry(|e| e.file_name() != "__pycache__")
    {
        let entry = entry?;
        ensure!(
            !entry.path_is_symlink(),
            "Skill package cannot contain symlinks"
        );
        if !entry.file_type().is_file() || entry.file_name() == ".DS_Store" {
            continue;
        }
        let name = entry
            .path()
            .strip_prefix(&dir)?
            .to_string_lossy()
            .replace('\\', "/");
        ensure!(
            !name.starts_with(".env") || name == ".env.example",
            "Private package file"
        );
        actual[&name] = json!(digest(entry.path())?);
    }
    ensure!(
        actual == record["files"],
        "Skill differs from reviewed release: {id}"
    );
    let c = read(&dir.join("contract.json"))?;
    ensure!(
        c["schema_version"] == 2
            && c["id"] == id
            && c["version"] == record["contract_version"]
            && c["api"]["version"] == record["api_version"]
            && c["policy"]["version"] == record["policy_version"],
        "Independent Skill versions differ"
    );
    crate::maintenance_contracts::validate(&c, &record["files"])?;
    safe(root, s(&record["source"]["import_manifest"])?)?;
    Ok((record, c))
}
pub fn add_selections(graph: &mut Graph, items: &J) -> Result<()> {
    let mut seen = BTreeSet::new();
    let environment_name = regex::Regex::new(r"^[A-Z][A-Z0-9_]*$")?;
    for item in arr(items)? {
        ensure!(
            obj(item)?
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
                == BTreeSet::from(["skill", "configuration", "environment", "limits"]),
            "Selection requires skill, configuration, environment and limits"
        );
        let id = s(&item["skill"])?;
        ensure!(seen.insert(id), "Duplicate Skill");
        let (record, c) = package(&graph.root, id)?;
        let effective = policy(&c["configuration"], &c["policy"]["configuration"])?;
        let mut config = c["policy"]["defaults"].clone();
        for (k, v) in obj(&config)? {
            ensure!(
                c["configuration"]["properties"][k]["policy_default"] == true,
                "Default requires explicit permission"
            );
            payload(&effective["properties"][k], v)?;
        }
        for (k, v) in obj(&item["configuration"])? {
            config[k] = v.clone();
        }
        payload(&effective, &config)?;
        let env = &item["environment"];
        ensure!(
            obj(env)?.keys().all(|k| [
                "network",
                "tls",
                "loopback",
                "secret_refs",
                "environment_refs",
                "privacy_approved"
            ]
            .contains(&k.as_str())),
            "Unknown environment field; credential values forbidden"
        );
        for k in ["network", "tls", "loopback", "privacy_approved"] {
            ensure!(
                env.get(k).is_none() || env[k].is_boolean(),
                "Environment decisions must be booleans"
            );
        }
        let pre = &c["prerequisites"];
        for k in ["network", "tls"] {
            if k == "tls" && pre["loopback_http"] == true && env["loopback"] == true {
                continue;
            }
            ensure!(
                pre[k] != true || env[k] == true,
                "Skill prerequisite unknown or unsupported: {k}"
            );
        }
        ensure!(
            pre["privacy"] == "no external data" || env["privacy_approved"] == true,
            "Privacy decision required"
        );
        for k in ["secret_refs", "environment_refs"] {
            let names = strings(env.get(k).unwrap_or(&json!([])))?;
            ensure!(
                names.iter().all(|n| environment_name.is_match(n)),
                "Environment references must be names"
            );
            ensure!(
                strings(&pre[k])?.iter().all(|n| names.contains(n)),
                "Missing prerequisite {k}"
            );
        }
        for (k, v) in obj(&item["limits"])? {
            let maximum = v.as_u64().context("Invalid resource limit")?;
            ensure!(c["resources"].get(k).is_some(), "Unknown resource");
            ensure!(
                c["estimates"][k].as_u64().is_some_and(|x| x <= maximum),
                "Resource bound unknown or exceeds limit"
            );
        }
        let mut roots = vec![
            format!("tools/skill_packages/packages/{id}.json"),
            s(&record["source"]["import_manifest"])?.into(),
            "tools/skill_packages/upstream.json".into(),
            "docs/architecture/skill-contracts.md".into(),
        ];
        roots.extend(
            obj(&record["files"])?
                .keys()
                .map(|p| format!("skills/{id}/{p}")),
        );
        graph.resolve(roots)?;
    }
    Ok(())
}
