//! Source-bound firmware receipts and immutable release orchestration.
use crate::{catalog::Catalog, platform, project, util::*};
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
};
struct Guard(PathBuf);
impl Drop for Guard {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.0);
    }
}
fn guard(root: &Path, id: &str) -> Result<Guard> {
    let path = root
        .join(".codecraft/firmware/locks")
        .join(identity(&json!(id)));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::create_dir(&path).context("Another firmware operation is active; inspect the lock before recovering an interrupted operation")?;
    Ok(Guard(path))
}
fn receipt_path(root: &Path, tid: &str, example: &str, variant: Option<&str>) -> PathBuf {
    root.join(".codecraft/firmware/receipts").join(format!(
        "{}.json",
        identity(&json!([tid, example, variant]))
    ))
}
pub fn selection(t: &J) -> Result<Vec<(String, Option<String>)>> {
    let variants: Vec<_> = if obj(&t["variants"])?.is_empty() {
        vec![None]
    } else {
        obj(&t["variants"])?.keys().cloned().map(Some).collect()
    };
    Ok(strings(&t["examples"])?
        .into_iter()
        .flat_map(|e| variants.iter().cloned().map(move |v| (e.clone(), v)))
        .collect())
}
fn source(c: &Catalog, t: &J) -> Result<J> {
    let lock = c.root.join("codecraft.lock");
    if lock.exists() {
        ensure!(
            project::verify(&c.root)?["status"] == "passed",
            "Project source differs from lock"
        );
    }
    Ok(
        json!({"source_digest":c.graph(t)?.fingerprint(),"attempt":if c.root.join(".codecraft/build-attempt.json").exists(){Some(digest(&c.root.join(".codecraft/build-attempt.json"))?)}else{None},"lock_sha256":if lock.exists(){Some(digest(&lock)?)}else{None},"cli":project::cli_identity()?}),
    )
}
pub fn build(c: &Catalog, t: &J) -> Result<Vec<J>> {
    let tid = s(&t["id"])?;
    let _guard = guard(&c.root, tid)?;
    let cases = selection(t)?;
    let enabled =
        c.platforms[s(&t["platform"])?]["operations"]["firmware-plan"]["supported"] == true;
    if enabled {
        for (ex, v) in &cases {
            write(
                &receipt_path(&c.root, tid, ex, v.as_deref()),
                &json!({"schema_version":2,"status":"not_run","reason":"A new build was requested; no completed firmware receipt exists"}),
            )?;
        }
    }
    let before = source(c, t)?;
    let case_sources = cases
        .iter()
        .map(|(ex, variant)| source(c, &c.selected(tid, Some(ex), variant.as_deref())?))
        .collect::<Result<Vec<_>>>()?;
    let adapter = platform::adapter(s(&t["platform"])?)?;
    let tools = adapter.probe(&c.root, t)?;
    let mut rows = adapter.build(&c.root, t)?;
    ensure!(
        source(c, t)? == before,
        "Source/lock/CLI changed during build"
    );
    ensure!(
        adapter.probe(&c.root, t)? == tools,
        "Toolchain changed during build"
    );
    if enabled {
        for ((ex, v), case_source) in cases.into_iter().zip(case_sources) {
            let name = format!(
                "{tid}/{ex}{}",
                v.as_ref().map(|v| format!("/{v}")).unwrap_or_default()
            );
            let row = rows
                .iter_mut()
                .find(|r| r["name"] == name)
                .context("Missing build result")?;
            let mut record = json!({"schema_version":2,"kind":"firmware-build","target":tid,"example":ex,"variant":v,"status":row["status"],"reason":row["reason"],"source":case_source,"toolchain":tools});
            if row["status"] == "passed" {
                match adapter.image_layout(
                    &c.root,
                    t,
                    &c.devices[s(&t["device"])?],
                    &ex,
                    v.as_deref(),
                ) {
                    Ok(layout) => record["layout"] = layout,
                    Err(e) => {
                        record["status"] = json!("failed");
                        record["reason"] =
                            json!(format!("Firmware input validation failed: {e:#}"));
                        row["status"] = record["status"].clone();
                        row["reason"] = record["reason"].clone();
                    }
                }
            }
            write(&receipt_path(&c.root, tid, &ex, v.as_deref()), &record)?;
        }
    }
    Ok(rows)
}
fn choose(
    root: &Path,
    tid: Option<&str>,
    example: Option<&str>,
    variant: Option<&str>,
) -> Result<(Catalog, J, String, Option<String>)> {
    let c = Catalog::load(root)?;
    let (tid, example, variant) = if root.join("project.yaml").is_file() {
        let p = read(&root.join("project.yaml"))?;
        ensure!(
            tid.is_none_or(|v| p["target"] == v)
                && example.is_none_or(|v| p["example"] == v)
                && variant.is_none_or(|v| p["variant"] == v),
            "Firmware selection differs from project"
        );
        (
            s(&p["target"])?.to_owned(),
            s(&p["example"])?.to_owned(),
            p["variant"].as_str().map(str::to_owned),
        )
    } else {
        (
            tid.context("Firmware requires --target or --project")?
                .into(),
            example.context("Firmware requires --example")?.into(),
            variant.map(str::to_owned),
        )
    };
    let t = c.selected(&tid, Some(&example), variant.as_deref())?;
    let selected = t["selected_variant"].as_str().map(str::to_owned);
    Ok((c, t, example, selected))
}
fn check(c: &Catalog, t: &J, example: &str, variant: Option<&str>) -> Result<J> {
    if !t["firmware"]["formats"]
        .as_array()
        .is_some_and(|v| v.contains(&json!("bin")))
    {
        return Err(unsupported("Factory BIN is not supported for this Target"));
    }
    let path = receipt_path(&c.root, s(&t["id"])?, example, variant);
    if !path.exists() {
        return Err(Unavailable(
            "not_run",
            "Build the selected example/variant before planning firmware".into(),
        )
        .into());
    }
    let r = read(&path)?;
    if r["status"] == "not_run" {
        return Err(Unavailable(
            "not_run",
            "The latest build attempt has no successful firmware output".into(),
        )
        .into());
    }
    ensure!(
        r["schema_version"] == 2 && r["kind"] == "firmware-build" && r["status"] == "passed",
        "No current passing firmware build receipt"
    );
    ensure!(
        r["target"] == t["id"] && r["example"] == example && r["variant"] == json!(variant),
        "Receipt selection mismatch"
    );
    // Every receipt binds its own example/variant graph, regardless of whether
    // the command built one case, a variant, or the complete Target.
    let identity = source(c, t)?;
    ensure!(
        r["source"] == identity,
        "Firmware source/lock/CLI is stale; rebuild"
    );
    let adapter = platform::adapter(s(&t["platform"])?)?;
    ensure!(
        adapter.probe(&c.root, t)? == r["toolchain"],
        "Firmware toolchain changed"
    );
    ensure!(
        adapter.image_layout(&c.root, t, &c.devices[s(&t["device"])?], example, variant)?
            == r["layout"],
        "Firmware inputs changed; rebuild"
    );
    Ok(r)
}
pub fn plan(
    root: &Path,
    tid: Option<&str>,
    ex: Option<&str>,
    variant: Option<&str>,
    out: &Path,
) -> Result<J> {
    let (c, t, ex, v) = choose(root, tid, ex, variant)?;
    let r = check(&c, &t, &ex, v.as_deref())?;
    let mut p = json!({"schema_version":2,"kind":"firmware-plan","root":c.root,"target":t["id"],"example":ex,"variant":v,"format":"bin","purpose":"factory","build":r});
    p["plan_sha256"] = json!(identity(&p));
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out)?;
    f.write_all((serde_json::to_string_pretty(&p)? + "\n").as_bytes())?;
    Ok(json!({"status":"passed","plan":p,"device_verification":"not_run"}))
}
pub fn merge(plan: &Path, out: &Path) -> Result<J> {
    let p = read(plan)?;
    let mut content = p.clone();
    obj(&p)?;
    content.as_object_mut().unwrap().remove("plan_sha256");
    ensure!(
        p["schema_version"] == 2
            && p["kind"] == "firmware-plan"
            && identity(&content) == s(&p["plan_sha256"])?,
        "Invalid firmware plan"
    );
    let (c, t, ex, v) = choose(
        Path::new(s(&p["root"])?),
        Some(s(&p["target"])?),
        Some(s(&p["example"])?),
        p["variant"].as_str(),
    )?;
    let _guard = guard(&c.root, s(&t["id"])?)?;
    let receipt = check(&c, &t, &ex, v.as_deref())?;
    ensure!(receipt == p["build"], "Build changed after planning");
    let out = if out.is_absolute() {
        out.to_owned()
    } else {
        std::env::current_dir()?.join(out)
    };
    ensure!(!out.exists() && !out.is_symlink(), "Output must be new");
    ensure!(
        !out.starts_with(&c.root) || out.starts_with(c.root.join(".codecraft")),
        "Firmware outputs inside a project belong under .codecraft"
    );
    fs::create_dir_all(out.parent().unwrap())?;
    let parent = out.parent().unwrap().canonicalize()?;
    ensure!(
        !parent.starts_with(&c.root) || parent.starts_with(c.root.join(".codecraft")),
        "Firmware outputs inside a project belong under .codecraft"
    );
    fs::create_dir(&out)?;
    let result = (|| -> Result<J> {
        let adapter = platform::adapter(s(&t["platform"])?)?;
        let mut manifest = adapter.image_merge(&c.root, &receipt["layout"], &out)?;
        manifest["schema_version"] = json!(2);
        manifest["kind"] = json!("firmware-release");
        manifest["layout"] = receipt["layout"].clone();
        manifest["target"] = t["id"].clone();
        manifest["format"] = json!("bin");
        manifest["purpose"] = json!("factory");
        manifest["source"] = receipt["source"].clone();
        manifest["plan_sha256"] = p["plan_sha256"].clone();
        manifest["device_verification"] = json!("not_run");
        ensure!(
            check(&c, &t, &ex, v.as_deref())? == receipt,
            "Build changed during merge"
        );
        write(&out.join("manifest.json"), &manifest)?;
        adapter.image_verify(&out, &manifest)?;
        Ok(
            json!({"status":"passed","path":out,"manifest":manifest,"device_verification":"not_run"}),
        )
    })();
    if result.is_err() {
        fs::remove_dir_all(&out)?;
    }
    result
}
pub fn verify(root: &Path, bundle: &Path) -> Result<J> {
    let bundle = bundle.canonicalize()?;
    let manifest = read(&bundle.join("manifest.json"))?;
    ensure!(
        manifest["schema_version"] == 2 && manifest["kind"] == "firmware-release",
        "Unsupported firmware manifest"
    );
    let c = Catalog::load(root)?;
    let t = c.target(s(&manifest["target"])?)?;
    platform::adapter(s(&t["platform"])?)?.image_verify(&bundle, &manifest)?;
    Ok(
        json!({"status":"passed","path":bundle,"sha256":manifest["image"]["sha256"],"scope":"offline-integrity-and-layout","device_verification":"not_run"}),
    )
}
