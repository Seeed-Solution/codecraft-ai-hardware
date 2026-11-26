use crate::{catalog::Catalog, platform, util::*, CONTRACT, VERSION};
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
pub fn cli_identity() -> Result<J> {
    Ok(json!({"contract":CONTRACT,"version":VERSION,"sha256":digest(&std::env::current_exe()?)?}))
}
fn profile(root: &Path) -> Result<J> {
    let p = read(&root.join("project.yaml"))?;
    ensure!(p["schema_version"]==4,"Project requires Rust profile schema 4; recreate from its original Target rather than reinterpret a schema 3 lock");
    ensure!(
        p["secrets"] == json!({}),
        "Project must not contain secret values"
    );
    ensure!(p["cli"]["contract"] == CONTRACT, "Unsupported CLI contract");
    Ok(p)
}
fn inventory(root: &Path, explicit: Vec<String>) -> Result<J> {
    let mut result = object();
    for name in files(root)?.into_iter().chain(explicit) {
        if name == "codecraft.lock" {
            continue;
        }
        let p = root.join(&name);
        if p.is_file() {
            safe(root, &name)?;
            result[&name] = json!(digest(&p)?);
        }
    }
    Ok(result)
}
fn brief(value: Option<J>, name: &str) -> Result<J> {
    let mut b = json!({"name":name,"purpose":null,"target_users":[],"scenarios":[],"bom":[]});
    if let Some(v) = value {
        for (k, v) in obj(&v)? {
            ensure!(
                ["name", "purpose", "target_users", "scenarios", "bom"].contains(&k.as_str()),
                "Unknown brief field"
            );
            b[k] = v.clone();
        }
    }
    ensure!(!s(&b["name"])?.trim().is_empty(), "Brief name required");
    ensure!(
        b["purpose"].is_null() || b["purpose"].as_str().is_some_and(|x| !x.trim().is_empty()),
        "Invalid brief purpose"
    );
    strings(&b["target_users"])?;
    strings(&b["scenarios"])?;
    for item in arr(&b["bom"])? {
        ensure!(
            obj(item)?.len() == 3 && !s(&item["item"])?.trim().is_empty(),
            "BOM needs item, quantity and source"
        );
        ensure!(
            item["quantity"].is_null() || item["quantity"].as_u64().is_some_and(|v| v > 0),
            "Invalid BOM quantity"
        );
        ensure!(
            item["source"].is_null() || item["source"].is_string(),
            "Invalid BOM source"
        );
    }
    Ok(b)
}
fn literal(v: &J) -> String {
    let raw = v.as_str().map(str::to_owned).unwrap_or_else(|| {
        if v.is_null() {
            "unknown".into()
        } else {
            v.to_string()
        }
    });
    raw.chars()
        .map(|c| match c {
            '&' => "&amp;".into(),
            '<' => "&lt;".into(),
            '>' => "&gt;".into(),
            '\r' | '\n' => " ".into(),
            '\\' | '`' | '[' | ']' | '*' | '_' | '|' => format!("&#{};", c as u32),
            _ => c.to_string(),
        })
        .collect()
}
fn rules(p: &J, existing: &str) -> Result<String> {
    let start = "<!-- CODECRAFT_APPLICATION_START -->";
    let end = "<!-- CODECRAFT_APPLICATION_END -->";
    let b = brief(Some(p["brief"].clone()), s(&p["device"])?)?;
    let list = |v: &J| -> Result<String> {
        let a = arr(v)?;
        Ok(if a.is_empty() {
            "- unknown".into()
        } else {
            a.iter()
                .map(|x| format!("- {}", literal(x)))
                .collect::<Vec<_>>()
                .join("\n")
        })
    };
    let body=format!("{start}\nGenerated from project.yaml. Run `codecraft lock-project .` after reviewing changes.\n\n## Project\n\nName: {}\n\nPurpose: {}\n\nDevice: {}; Target: {}; example: {}.\n\n## Target users\n\n{}\n\n## Scenarios\n\n{}\n\n## Bill of materials\n\n{}\n\nMissing hardware facts and physical compatibility remain unknown.\n{end}",literal(&b["name"]),literal(&b["purpose"]),literal(&p["device"]),literal(&p["target"]),literal(&p["example"]),list(&b["target_users"] )?,list(&b["scenarios"] )?,list(&b["bom"] )?);
    if existing.contains(start) || existing.contains(end) {
        ensure!(
            existing.matches(start).count() == 1 && existing.matches(end).count() == 1,
            "Invalid application markers"
        );
        let a = existing.find(start).unwrap();
        let z = existing.find(end).unwrap();
        ensure!(a < z, "Reversed application markers");
        Ok(format!(
            "{}{}{}",
            &existing[..a],
            body,
            &existing[z + end.len()..]
        ))
    } else {
        Ok(format!("{}\n\n{body}\n\nRead docs/development/application-workflow.md and the selected Device entry. Builds never flash. Device writes require explicit authorization and an identified device. Preserve sourced hardware facts and custom rules.\n",if existing.trim().is_empty(){"# Application context"}else{existing.trim_end()}))
    }
}
fn destination(root: &Path, out: &Path) -> Result<PathBuf> {
    let out = if out.is_absolute() {
        out.to_owned()
    } else {
        std::env::current_dir()?.join(out)
    };
    ensure!(
        !out.components()
            .any(|c| matches!(c, std::path::Component::ParentDir)),
        "Output must not contain parent traversal"
    );
    ensure!(
        !out.exists() && !out.is_symlink(),
        "Output must be a new directory"
    );
    let mut ancestor = out.parent().context("Missing output parent")?;
    while !ancestor.exists() {
        ancestor = ancestor.parent().context("Missing output ancestor")?;
    }
    let resolved = ancestor.canonicalize()?.join(out.strip_prefix(ancestor)?);
    let root = root.canonicalize()?;
    if resolved.starts_with(&root) {
        ensure!(
            resolved.starts_with(root.join("projects")) && resolved != root.join("projects"),
            "Private applications belong under projects/<name>"
        );
    }
    let mut p = out.as_path();
    let mut linked = false;
    while let Some(parent) = p.parent() {
        if p.canonicalize().ok().as_ref() == Some(&root) {
            ensure!(
                !linked && resolved.starts_with(root.join("projects")),
                "Project output traverses a symlink"
            );
            break;
        }
        linked |= p.is_symlink();
        p = parent;
    }
    Ok(resolved)
}
pub fn create(
    c: &Catalog,
    tid: &str,
    ex: &str,
    variant: Option<&str>,
    out: &Path,
    brief_value: Option<J>,
    skills: J,
) -> Result<J> {
    let target = c.selected(tid, Some(ex), variant)?;
    let mut graph = c.graph(&target)?;
    crate::skills::add_selections(&mut graph, &skills)?;
    let out = destination(&c.root, out)?;
    fs::create_dir_all(out.parent().unwrap())?;
    fs::create_dir(&out)?;
    let result = (|| -> Result<J> {
        for name in graph.files.keys() {
            let dest = out.join(name);
            fs::create_dir_all(dest.parent().unwrap())?;
            fs::copy(safe(&c.root, name)?, dest)?;
        }
        write(&out.join(s(&target["path"])?).join("target.json"), &target)?;
        let mut p = json!({"schema_version":4,"device":target["device"],"target":tid,"example":ex,"toolchain":target["toolchain"],"interfaces":arr(&target["interfaces"])?.iter().map(|v|v["id"].clone()).collect::<Vec<_>>(),"skills":skills,"secrets":{},"brief":brief(brief_value,s(&target["device"])?)?,"cli":{"contract":CONTRACT,"version":VERSION}});
        if target.get("selected_variant").is_some() {
            p["variant"] = target["selected_variant"].clone();
            p["physical_compatibility"] = json!("unknown");
        }
        write(&out.join("project.yaml"), &p)?;
        fs::write(out.join("AGENTS.md"), rules(&p, "")?)?;
        fs::write(out.join("README.md"),format!("[简体中文](README.zh_CN.md)\n\n# {}: {ex}\n\nUse CodeCraft CLI {VERSION} (contract {CONTRACT}). The project contains no CodeCraft Python runtime or Rust sources.\n\n`codecraft verify-project .` checks sources without installing tools. Before the first build, activate the declared platform toolchain and run `codecraft lock-project . --toolchain`, then `codecraft build-project .`. After reviewing application edits, use `codecraft lock-project .`. Builds never flash.\n\nCLI updates are independent of project/source updates. The original import provenance remains in codecraft.lock. Hardware revision and physical compatibility remain unknown unless independently verified.\n",s(&target["device"])?))?;
        fs::write(out.join("README.zh_CN.md"),format!("[English](README.md)\n\n# {}: {ex}\n\n使用 CodeCraft CLI {VERSION}（契约 {CONTRACT}）。项目不包含 CodeCraft Python 运行时或 Rust 源码。\n\n`codecraft verify-project .` 检查源码，不安装工具。首次构建前，激活声明的平台工具链并运行 `codecraft lock-project . --toolchain`，然后运行 `codecraft build-project .`。审查应用修改后运行 `codecraft lock-project .`。构建不会烧录。\n\nCLI 更新与项目及源码更新独立。codecraft.lock 保留最初导入来源。硬件修订和实体兼容性在独立验证前保持未知。\n",s(&target["device"])?))?;
        fs::write(out.join(".gitignore"),".codecraft/\n.codegraph/\n__pycache__/\nmanaged_components/\nbuild/\nsdkconfig\nsdkconfig.old\n")?;
        let origin = json!({"repository_commit":output(Command::new("git").arg("-C").arg(&c.root).args(["rev-parse","HEAD"])).unwrap_or("unknown".into()),"repository_dirty":output(Command::new("git").arg("-C").arg(&c.root).args(["status","--porcelain"])).map(|v|!v.is_empty()).unwrap_or(true),"target":tid,"example":ex,"variant":p.get("variant"),"target_source_digest":c.graph(c.target(tid)?)?.fingerprint(),"import_graph":graph.snapshot(),"import_digest":graph.fingerprint(),"cli":cli_identity()?});
        let lock = relock(&out, Some(origin), false)?;
        Ok(
            json!({"path":out,"target":tid,"example":ex,"variant":p.get("variant"),"source_digest":lock["source_digest"],"origin_source_digest":lock["origin"]["target_source_digest"],"toolchain":lock["resolved_toolchain"],"device_verification":"not_run"}),
        )
    })();
    if result.is_err() {
        fs::remove_dir_all(&out)?;
    }
    result
}
pub fn relock(root: &Path, origin: Option<J>, probe: bool) -> Result<J> {
    let root = root.canonicalize()?;
    let p = profile(&root)?;
    let previous = if root.join("codecraft.lock").exists() {
        read(&root.join("codecraft.lock"))?
    } else {
        J::Null
    };
    let origin = origin.unwrap_or(previous["origin"].clone());
    ensure!(
        identity(&origin["import_graph"]) == s(&origin["import_digest"])?,
        "Original import provenance is corrupt"
    );
    let c = Catalog::load(&root)?;
    let t = c.target(s(&p["target"])?)?;
    ensure!(
        t["device"] == p["device"]
            && t["examples"] == json!([p["example"]])
            && t.get("selected_variant") == p.get("variant"),
        "Profile/Target selection mismatch"
    );
    let g = c.graph(t)?;
    let agents = root.join("AGENTS.md");
    ensure!(
        !agents.is_symlink(),
        "Application rules cannot be a symlink"
    );
    let text = rules(&p, &fs::read_to_string(&agents).unwrap_or_default())?;
    let toolchain = if probe {
        platform::adapter(s(&t["platform"])?)?.probe(&root, t)?
    } else if previous["declared_toolchain"] == t["toolchain"]
        && !previous["resolved_toolchain"].is_null()
    {
        previous["resolved_toolchain"].clone()
    } else {
        json!({"status":"not_run","reason":"Toolchain is not required for authoring; activate it and run lock-project --toolchain before building."})
    };
    fs::write(agents, text)?;
    let files = inventory(&root, g.files.keys().cloned().collect())?;
    let lock = json!({"schema_version":4,"origin":origin,"target":p["target"],"example":p["example"],"variant":p.get("variant"),"cli":cli_identity()?,"declared_toolchain":t["toolchain"],"resolved_toolchain":toolchain,"source_digest":identity(&files),"files":files,"dependency_graph":g.snapshot()});
    write(&root.join("codecraft.lock"), &lock)?;
    Ok(lock)
}
pub fn verify(root: &Path) -> Result<J> {
    let root = root.canonicalize()?;
    let p = profile(&root)?;
    let lock = read(&root.join("codecraft.lock"))?;
    ensure!(
        lock["schema_version"] == 4 && lock["cli"]["contract"] == CONTRACT,
        "Unsupported lock schema/CLI contract"
    );
    ensure!(
        identity(&lock["files"]) == s(&lock["source_digest"])?,
        "Lock digest mismatch"
    );
    ensure!(
        identity(&lock["origin"]["import_graph"]) == s(&lock["origin"]["import_digest"])?,
        "Import provenance mismatch"
    );
    ensure!(
        lock["target"] == p["target"]
            && lock["example"] == p["example"]
            && lock.get("variant").unwrap_or(&J::Null) == p.get("variant").unwrap_or(&J::Null),
        "Profile/lock mismatch"
    );
    let explicit = obj(&lock["dependency_graph"]["files"])?
        .keys()
        .cloned()
        .collect();
    let actual = inventory(&root, explicit)?;
    let names: BTreeSet<_> = obj(&actual)?
        .keys()
        .chain(obj(&lock["files"])?.keys())
        .collect();
    let drift: Vec<_> = names
        .into_iter()
        .filter(|n| actual.get(*n) != lock["files"].get(*n))
        .collect();
    if drift.is_empty() {
        let c = Catalog::load(&root)?;
        ensure!(
            c.graph(c.target(s(&p["target"])?)?)?.snapshot() == lock["dependency_graph"],
            "Dependency graph differs from lock"
        );
    }
    Ok(
        json!({"path":root,"status":if drift.is_empty(){"passed"}else{"failed"},"drift":drift,"source_digest":lock["source_digest"],"cli_changed":cli_identity()?!=lock["cli"]}),
    )
}
pub fn build(root: &Path) -> Result<J> {
    let root = root.canonicalize()?;
    ensure!(
        root.join("project.yaml").is_file(),
        "Not an application root"
    );
    // A failed new attempt must invalidate previous successful image receipts,
    // including failures before native dispatch (source/CLI/toolchain drift).
    write(
        &root.join(".codecraft/build-attempt.json"),
        &json!({"pid":std::process::id(),"time":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos().to_string()}),
    )?;
    write(
        &root.join(".codecraft/build-receipt.json"),
        &json!({"status":"not_run","reason":"New build requested"}),
    )?;
    let report = verify(&root)?;
    ensure!(
        report["status"] == "passed",
        "Source changed; review edits and run codecraft lock-project ."
    );
    let p = profile(&root)?;
    let c = Catalog::load(&root)?;
    let t = c.target(s(&p["target"])?)?;
    let lock = read(&root.join("codecraft.lock"))?;
    ensure!(
        cli_identity()? == lock["cli"],
        "CLI changed; review and relock the project before building"
    );
    let adapter = platform::adapter(s(&t["platform"])?)?;
    let tc = adapter.probe(&root, t)?;
    let rows = if tc["status"] != "passed" || tc != lock["resolved_toolchain"] {
        vec![
            json!({"name":p["target"],"status":"not_run","reason":"Activate the declared toolchain and run codecraft lock-project . --toolchain","actual_toolchain":tc}),
        ]
    } else {
        crate::firmware::build(&c, t)?
    };
    let after = verify(&root)?;
    ensure!(after["status"] == "passed", "Sources changed during build");
    let payload = json!({"schema_version":2,"target":p["target"],"operation":"build","source_digest":c.graph(t)?.fingerprint(),"project_source_digest":report["source_digest"],"resolved_toolchain":tc,"ok":rows.iter().all(|r|r["status"]=="passed"),"results":rows,"device_verification":"not_run"});
    write(&root.join(".codecraft/build-receipt.json"), &payload)?;
    Ok(payload)
}
pub fn package(root: &Path, out: &Path) -> Result<J> {
    use std::io::Write;
    let root = root.canonicalize()?;
    ensure!(
        verify(&root)?["status"] == "passed",
        "Project differs from lock"
    );
    let out = if out.is_absolute() {
        out.to_owned()
    } else {
        std::env::current_dir()?.join(out)
    };
    ensure!(
        !out.exists() && !out.starts_with(&root),
        "Package must be new and outside project"
    );
    fs::create_dir_all(out.parent().unwrap())?;
    ensure!(
        !out.parent().unwrap().canonicalize()?.starts_with(&root),
        "Package must be outside project"
    );
    let file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out)?;
    let mut zip = zip::ZipWriter::new(file);
    let lock = read(&root.join("codecraft.lock"))?;
    for name in obj(&lock["files"])?
        .keys()
        .map(String::as_str)
        .chain(["codecraft.lock"])
    {
        zip.start_file(
            name,
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated),
        )?;
        zip.write_all(&fs::read(safe(&root, name)?)?)?;
    }
    zip.finish()?;
    Ok(
        json!({"path":out,"sha256":digest(&out)?,"kind":"source-package","firmware_included":false,"device_verification":"not_run"}),
    )
}
