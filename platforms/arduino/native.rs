use crate::{
    platform::{command, Adapter},
    util::*,
};
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{path::Path, process::Command};
pub struct Arduino;
fn inspect(args: &[&str]) -> Result<J> {
    parse(&output(
        Command::new("arduino-cli")
            .args(args)
            .args(["--format", "json"]),
    )?)
}
fn tree(path: &Path) -> Result<String> {
    let mut inventory = object();
    for entry in walkdir::WalkDir::new(path).follow_links(true) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let name = entry
                .path()
                .strip_prefix(path)?
                .to_string_lossy()
                .replace('\\', "/");
            inventory[&name] = json!(digest(entry.path())?);
        }
    }
    Ok(identity(&inventory))
}
fn probe(t: &J) -> Result<J> {
    let tc = &t["toolchain"];
    let version = inspect(&["version"])?;
    ensure!(
        version["VersionString"] == tc["version"],
        "Arduino CLI version differs from Target"
    );
    let fqbn = s(&tc["fqbn"])?;
    let details = inspect(&["board", "details", "--fqbn", fqbn])?;
    ensure!(
        details["fqbn"] == tc["fqbn"] && details["version"] == tc["core_version"],
        "Arduino board/core mismatch"
    );
    let properties: std::collections::BTreeMap<_, _> = strings(&details["build_properties"])?
        .iter()
        .map(|v| {
            let (k, v) = v.split_once('=').context("Invalid build property")?;
            Ok((k.to_owned(), v.to_owned()))
        })
        .collect::<Result<_>>()?;
    let core = Path::new(
        properties
            .get("runtime.platform.path")
            .context("Missing core path")?,
    );
    let installed = inspect(&["lib", "list"])?;
    let mut libraries = object();
    for (name, version) in obj(&tc["libraries"])? {
        let library = arr(&installed["installed_libraries"])?
            .iter()
            .find(|l| l["library"]["name"] == *name)
            .context("Required Arduino library missing")?;
        let lib = &library["library"];
        ensure!(
            lib["version"] == *version,
            "Arduino library version mismatch: {name}"
        );
        let path = Path::new(s(&lib["install_dir"])?).canonicalize()?;
        libraries[name] = json!({"version":version,"path":path,"source_digest":tree(&path)?});
    }
    let mut bundled = object();
    for name in strings(&tc["bundled_libraries"])? {
        let path = core.join("libraries").join(&name);
        ensure!(path.is_dir(), "Bundled library missing");
        bundled[&name] = json!(path);
    }
    let mut tools = object();
    for item in arr(&details["tools_dependencies"])? {
        let name = s(&item["name"])?;
        let version = s(&item["version"])?;
        let path = Path::new(
            properties
                .get(&format!("runtime.tools.{name}-{version}.path"))
                .context("Missing native tool path")?,
        );
        tools[format!("{}/{name}", s(&item["packager"])?)] =
            json!({"version":version,"source_digest":tree(path)?});
    }
    Ok(
        json!({"status":"passed","cli":tc["version"],"fqbn":fqbn,"core":{"id":fqbn.split(':').take(2).collect::<Vec<_>>().join(":"),"version":tc["core_version"],"path":core,"source_digest":tree(core)?},"libraries":libraries,"bundled_libraries":bundled,"tools":tools}),
    )
}
impl Adapter for Arduino {
    fn probe(&self, _root: &Path, t: &J) -> Result<J> {
        Ok(match probe(t) {
            Ok(v) => v,
            Err(e) => json!({"status":"not_run","reason":e.to_string()}),
        })
    }
    fn build(&self, root: &Path, t: &J) -> Result<Vec<J>> {
        let actual = self.probe(root, t)?;
        let mut rows = vec![];
        for ex in strings(&t["examples"])? {
            let name = format!("{}/{ex}", s(&t["id"])?);
            if actual["status"] != "passed" {
                rows.push(json!({"name":name,"status":"not_run","reason":actual["reason"],"toolchain":actual}));
                continue;
            }
            let build = root
                .join(".codecraft/builds/arduino")
                .join(s(&t["device"])?)
                .join(&ex);
            std::fs::create_dir_all(&build)?;
            let mut args = vec![
                "arduino-cli".into(),
                "compile".into(),
                "--fqbn".into(),
                s(&t["toolchain"]["fqbn"])?.into(),
                "--build-path".into(),
                build.to_string_lossy().into(),
                "--jobs".into(),
                "4".into(),
            ];
            for lib in obj(&actual["libraries"])?.values() {
                args.extend(["--library".into(), s(&lib["path"])?.into()]);
            }
            for lib in obj(&actual["bundled_libraries"])?.values() {
                args.extend(["--library".into(), s(lib)?.into()]);
            }
            args.push(
                safe(root, s(&t["path"])?)?
                    .join("examples")
                    .join(ex)
                    .to_string_lossy()
                    .into(),
            );
            rows.push(command(root, &name, &args, &actual)?);
        }
        Ok(rows)
    }
}
