//! Derived views, generated from authored owners without modifying their sources.
use crate::{catalog::Catalog, maintenance_evidence, skills, util::*};
use anyhow::{ensure, Result};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn pretty(v: &J) -> Result<String> {
    Ok(serde_json::to_string_pretty(v)? + "\n")
}
pub fn views(c: &Catalog) -> Result<BTreeMap<String, String>> {
    let mut outputs = BTreeMap::new();
    let mut rows = vec![
        "| Device | Target | Implementations | Build + project | Device |".to_owned(),
        "| --- | --- | --- | --- | --- |".into(),
    ];
    let mut overview=vec!["# Device directory".to_owned(),String::new(),"This directory index is generated from the authored Device Markdown entries.".into(),String::new(),"Each device has its own directory and AGENTS.md knowledge entry. Keep hardware knowledge, references and assets there; targets/ owns its native BSP, examples and board configuration. Use reference/ for supporting material and assets/ for actual registered media when needed. Shared stack/runtime adapters, build helpers and reusable components belong in platforms/<stack>/. Board-specific tests live in tests/devices/<device>/targets/; observations, records and logs live alongside that test subtree.".into(),String::new()];
    let mut sources = vec![
        "# Vendor source index".to_owned(),
        String::new(),
        "Generated references. Read primary sources before relying on facts.".into(),
        String::new(),
    ];
    let mut bindings = vec![];
    for (did, d) in &c.devices {
        let targets: Vec<_> = c.targets.values().filter(|t| t["device"] == *did).collect();
        let tids: Vec<_> = targets.iter().map(|t| s(&t["id"])).collect::<Result<_>>()?;
        let mut caps = BTreeSet::new();
        let mut levels = vec![];
        let mut physical = vec![];
        for t in &targets {
            let state = maintenance_evidence::support(c, s(&t["id"])?)?;
            levels.push(s(&state["level"])?.to_owned());
            physical.push(s(&state["device"])?.to_owned());
            for b in arr(&t["interfaces"])? {
                let id = s(&b["id"])?;
                caps.insert(id.to_owned());
                bindings.push(json!({"target":t["id"],"interface":id,"operations":b["operations"],"configuration":b["configuration"],"resources":b["resources"],"verification":state["interfaces"][id]}));
            }
        }
        rows.push(format!(
            "| [{}](../devices/{did}/AGENTS.md) | {} | {} | {} | {} |",
            s(&d["name"])?,
            tids.join(", "),
            caps.into_iter().collect::<Vec<_>>().join(", "),
            levels.join(", "),
            physical.join(", ")
        ));
        overview.extend([
            format!("## [{}]({did}/AGENTS.md)", s(&d["name"])?),
            String::new(),
            s(&d["overview"])?.into(),
            String::new(),
            s(&d["scenarios"])?.into(),
            String::new(),
        ]);
        sources.extend([
            format!("## {}", s(&d["name"])?),
            String::new(),
            format!("[Authoritative sources and notes](../devices/{did}/AGENTS.md#sources)"),
            String::new(),
        ]);
    }
    outputs.insert(
        "devices/AGENTS.md".into(),
        overview.join("\n").trim_end().to_owned() + "\n",
    );
    outputs.insert(
        "docs/vendor-sources.md".into(),
        sources.join("\n").trim_end().to_owned() + "\n",
    );
    outputs.insert("docs/inventory.md".into(),"# Native interface matrix\n\nGenerated implementations and scoped evidence. See the [JSON bindings](interface-matrix.json) for operation, configuration and resource details.\n\n".to_owned()+&rows.join("\n")+"\n");
    outputs.insert(
        "docs/interface-matrix.json".into(),
        pretty(&json!({"schema_version":2,"bindings":bindings}))?,
    );
    let mut packages = vec![];
    let package_dir = c.root.join("tools/skill_packages/packages");
    let mut paths: Vec<_> = fs::read_dir(package_dir)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<_>>()?;
    paths.sort();
    for path in paths {
        if path.extension().and_then(|x| x.to_str()) != Some("json") {
            continue;
        }
        let id = path.file_stem().unwrap().to_string_lossy();
        let (record, contract) = skills::package(&c.root, &id)?;
        packages.push(json!({"id":id,"version":record["version"],"contract_version":contract["version"],"policy_version":contract["policy"]["version"],"api_version":contract["api"]["version"],"operations":obj(&contract["operations"])?.keys().collect::<Vec<_>>(),"route":format!("skills/{id}/SKILL.md")}));
    }
    outputs.insert(
        "docs/skill-inventory.json".into(),
        pretty(&json!({"schema_version":1,"skills":packages}))?,
    );
    let mut matrix = vec![];
    let mut targets: Vec<_> = c.targets.values().collect();
    targets.sort_by_key(|t| {
        (
            t["device"].as_str().unwrap_or(""),
            t["id"].as_str().unwrap_or(""),
        )
    });
    for t in targets {
        let mut row = json!({"id":t["id"],"device":t["device"],"platform":t["platform"],"toolchain":t["toolchain"]["version"],"chip":t["toolchain"].get("target").cloned().unwrap_or(json!(""))});
        for (k, v) in obj(&c.platforms[s(&t["platform"])?]["ci"])? {
            row[k] = v.clone();
        }
        matrix.push(row);
    }
    outputs.insert(
        ".github/target-matrix.json".into(),
        pretty(&json!({"include":matrix}))?,
    );
    Ok(outputs)
}
pub fn generate(c: &Catalog, check: bool) -> Result<J> {
    let mut changed = vec![];
    for (relative, text) in views(c)? {
        let path = c.root.join(&relative);
        safe(
            &c.root,
            Path::new(&relative).parent().unwrap().to_str().unwrap(),
        )?;
        if path.exists() {
            safe(&c.root, &relative)?;
        }
        if fs::read_to_string(&path)
            .map(|s| s.replace("\r\n", "\n"))
            .ok()
            .as_deref()
            != Some(&text)
        {
            changed.push(relative);
            if !check {
                fs::create_dir_all(path.parent().unwrap())?;
                fs::write(path, text)?;
            }
        }
    }
    Ok(json!({"changed":changed,"check":check}))
}
pub fn command(root: &Path, args: &[String]) -> Result<J> {
    ensure!(
        args.is_empty() || args == ["--check"],
        "Usage: generate [--check]"
    );
    generate(&Catalog::load(root)?, !args.is_empty())
}
