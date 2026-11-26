//! Stack-independent metadata contracts. Native commands stay in stack adapters.
use crate::{catalog::Catalog, util::*};
use anyhow::{ensure, Result};
use std::{collections::BTreeSet, fs, path::Path};
fn unique(v: &J) -> Result<BTreeSet<String>> {
    let xs = strings(v)?;
    let out: BTreeSet<_> = xs.iter().cloned().collect();
    ensure!(out.len() == xs.len(), "Duplicate declaration");
    Ok(out)
}
fn nonempty(v: &J) -> Result<()> {
    ensure!(!s(v)?.trim().is_empty(), "Empty declaration");
    Ok(())
}
pub fn platform(root: &Path, p: &J, application: bool) -> Result<()> {
    let id = s(&p["id"])?;
    ensure!(p["schema_version"] == 3, "Platform schema 3 required; recreate exports made with the removed adapter/runtime contract");
    ensure!(
        p["adapter"] == format!("platforms/{id}/native.rs"),
        "Platform must own native implementation"
    );
    if !application {
        safe(root, s(&p["adapter"])?)?;
    }
    ensure!(
        p.get("runtime").is_none(),
        "Obsolete conformance runtime declaration"
    );
    ensure!(
        !strings(&p["application_inputs"])?.is_empty(),
        "Missing application dependencies"
    );
    for input in strings(&p["application_inputs"])? {
        safe(root, &input)?;
    }
    let expected: BTreeSet<_> = [
        "build",
        "configure",
        "device-test",
        "firmware-merge",
        "firmware-plan",
        "firmware-verify",
        "flash",
        "generate",
        "host-test",
        "inspect",
        "logs",
        "package",
        "simulate",
    ]
    .into_iter()
    .collect();
    ensure!(
        obj(&p["operations"])?
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            == expected,
        "Incomplete platform operations"
    );
    for op in obj(&p["operations"])?.values() {
        ensure!(op["supported"].is_boolean(), "Explicit support required");
        nonempty(&op["reason"])?;
    }
    nonempty(&p["ci"]["runner"])?;
    Ok(())
}
pub fn target(c: &Catalog, t: &J) -> Result<()> {
    let allowed = [
        "schema_version",
        "id",
        "platform",
        "device",
        "path",
        "examples",
        "inputs",
        "dependencies",
        "example_inputs",
        "example_dependencies",
        "toolchain",
        "interfaces",
        "firmware",
        "variants",
        "default_variant",
        "selected_variant",
    ];
    ensure!(
        obj(t)?.keys().all(|k| allowed.contains(&k.as_str())),
        "Unknown Target field"
    );
    let examples = unique(&t["examples"])?;
    ensure!(!examples.is_empty(), "Missing examples");
    let id_pattern = regex::Regex::new(r"^[a-zA-Z0-9_-]+$")?;
    ensure!(
        examples.iter().all(|x| id_pattern.is_match(x)),
        "Invalid example identity"
    );
    for key in ["example_inputs", "example_dependencies"] {
        ensure!(
            obj(&t[key])?.keys().cloned().collect::<BTreeSet<_>>() == examples,
            "Example scope mismatch"
        );
        for value in obj(&t[key])?.values() {
            let refs = unique(value)?;
            ensure!(
                key != "example_inputs" || !refs.is_empty(),
                "Missing native inputs"
            );
            for p in refs {
                safe(&c.root, &p)?;
            }
        }
    }
    for key in ["inputs", "dependencies"] {
        for p in unique(&t[key])? {
            safe(&c.root, &p)?;
        }
    }
    for key in ["name", "version", "source"] {
        nonempty(&t["toolchain"][key])?;
    }
    safe(&c.root, s(&t["toolchain"]["source"])?)?;
    if let Some(f) = t.get("firmware") {
        ensure!(
            obj(f)?.len() == 3
                && f["formats"] == serde_json::json!(["bin"])
                && f["purpose"] == "factory"
                && f["security"] == "unsigned-unencrypted",
            "Invalid firmware profile"
        );
        for op in ["firmware-plan", "firmware-merge", "firmware-verify"] {
            ensure!(
                c.platforms[s(&t["platform"])?]["operations"][op]["supported"] == true,
                "Firmware requires platform operation"
            );
        }
    }
    let variants = obj(&t["variants"])?;
    if variants.is_empty() {
        ensure!(
            t.get("default_variant").is_none() && t.get("selected_variant").is_none(),
            "Variant selection without variants"
        );
    } else {
        ensure!(
            variants.contains_key(s(&t["default_variant"])?),
            "Missing default variant"
        );
        let variant_pattern = regex::Regex::new(r"^[a-z][a-z0-9_-]*$")?;
        for (name, v) in variants {
            ensure!(variant_pattern.is_match(name), "Invalid variant identity");
            ensure!(
                c.devices[s(&t["device"])?]["variants"]
                    .get(s(&v["device_variant"])?)
                    .is_some(),
                "Unknown Device variant"
            );
            ensure!(
                !unique(&v["inputs"])?.is_empty() && !obj(&v["configuration"])?.is_empty(),
                "Missing variant inputs/configuration"
            );
            for key in ["inputs", "dependencies"] {
                for p in unique(&v[key])? {
                    safe(&c.root, &p)?;
                }
            }
        }
        if let Some(selected) = t.get("selected_variant") {
            ensure!(
                variants.len() == 1
                    && variants.contains_key(s(selected)?)
                    && selected == &t["default_variant"],
                "Variant projection mismatch"
            );
        }
    }
    let interfaces = arr(&t["interfaces"])?;
    ensure!(!interfaces.is_empty(), "No native interfaces");
    let mut seen = BTreeSet::new();
    for i in interfaces {
        ensure!(seen.insert(s(&i["id"])?), "Duplicate interface");
        ensure!(
            i.get("verification").is_none(),
            "Verification is derived from evidence"
        );
        let mut refs = unique(&i["implementation"])?;
        let scoped = obj(&i["example_implementation"])?;
        for (ex, paths) in scoped {
            ensure!(
                examples.contains(ex) && !arr(paths)?.is_empty(),
                "Invalid interface example scope"
            );
            refs.extend(unique(paths)?);
        }
        ensure!(!refs.is_empty(), "Missing interface implementation");
        for path in &refs {
            safe(&c.root, path)?;
        }
        ensure!(!obj(&i["operations"])?.is_empty(), "Missing operations");
        for op in obj(&i["operations"])?.values() {
            ensure!(op["supported"].is_boolean(), "Missing support status");
            nonempty(&op["reason"])?;
            if op["supported"] == true {
                ensure!(
                    !(op.get("source").is_some() && op.get("example_sources").is_some()),
                    "Ambiguous operation source declarations"
                );
                let sources = if let Some(p) = op.get("source") {
                    vec![s(p)?.to_owned()]
                } else {
                    let map = obj(&op["example_sources"])?;
                    ensure!(
                        map.keys().collect::<BTreeSet<_>>()
                            == scoped.keys().collect::<BTreeSet<_>>(),
                        "Operation scope mismatch"
                    );
                    map.values()
                        .map(|v| Ok(s(v)?.to_owned()))
                        .collect::<Result<Vec<_>>>()?
                };
                ensure!(!sources.is_empty(), "Missing native source");
                nonempty(&op["symbol"])?;
                for path in sources {
                    ensure!(
                        refs.contains(&path)
                            && fs::read_to_string(safe(&c.root, &path)?)?
                                .contains(s(&op["symbol"])?),
                        "Missing native source/symbol"
                    );
                }
            }
        }
        for group in ["configuration", "resources"] {
            for item in obj(&i[group])?.values() {
                ensure!(
                    ["integer", "number", "string", "boolean"].contains(&s(&item["type"])?),
                    "Invalid native parameter type"
                );
                if item["status"] == "unknown" {
                    ensure!(item["value"].is_null(), "Unknown value must be null");
                    nonempty(&item["reason"])?;
                } else {
                    ensure!(item["status"] == "known", "Invalid parameter status");
                    parameter(item, &item["value"])?;
                    let source = &item["source"];
                    let path = s(&source["path"])?;
                    ensure!(refs.contains(path), "Unowned native assertion");
                    let assertion = s(&source["assertion"])?;
                    nonempty(&source["assertion"])?;
                    let norm = |v: &str| v.split_whitespace().collect::<Vec<_>>().join(" ");
                    let value = item["value"]
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| item["value"].to_string());
                    ensure!(
                        norm(&fs::read_to_string(safe(&c.root, path)?)?).contains(&norm(assertion))
                            && assertion.contains(&value),
                        "Native source/value mismatch"
                    );
                }
            }
        }
    }
    Ok(())
}
pub fn parameter(spec: &J, v: &J) -> Result<()> {
    ensure!(
        match s(&spec["type"])? {
            "integer" => v.is_i64() || v.is_u64(),
            "number" => v.is_number(),
            "string" => v.is_string(),
            "boolean" => v.is_boolean(),
            _ => false,
        },
        "Native parameter type mismatch"
    );
    if let Some(values) = spec.get("enum") {
        ensure!(arr(values)?.contains(v), "Outside enum");
    }
    if let Some(min) = spec.get("minimum") {
        ensure!(
            v.as_f64().zip(min.as_f64()).is_some_and(|(a, b)| a >= b),
            "Below minimum"
        );
    }
    if let Some(max) = spec.get("maximum") {
        ensure!(
            v.as_f64().zip(max.as_f64()).is_some_and(|(a, b)| a <= b),
            "Above maximum"
        );
    }
    Ok(())
}
