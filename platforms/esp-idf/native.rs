pub mod composition;
#[path = "image.rs"]
pub mod image;
// ESP-IDF command selection and identity. No SDK is needed until probe/build.
use crate::{
    platform::{command, Adapter},
    util::*,
};
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{env, fs, path::Path, process::Command};
pub struct EspIdf;
pub fn version_matches(actual: &str, required: &str) -> bool {
    actual == format!("ESP-IDF v{required}") || actual == format!("ESP-IDF v{required}-dirty")
}
impl Adapter for EspIdf {
    fn image_layout(
        &self,
        root: &Path,
        target: &J,
        device: &J,
        example: &str,
        variant: Option<&str>,
    ) -> Result<J> {
        image::layout(root, target, device, example, variant)
    }
    fn image_merge(&self, root: &Path, layout: &J, stage: &Path) -> Result<J> {
        image::merge(root, layout, stage)
    }
    fn image_verify(&self, bundle: &Path, manifest: &J) -> Result<J> {
        image::verify(bundle, manifest)
    }
    fn probe(&self, _root: &Path, t: &J) -> Result<J> {
        let actual = match output(Command::new("idf.py").arg("--version")) {
            Ok(v) => v,
            Err(e) => {
                return Ok(
                    json!({"status":"not_run","reason":format!("Activate the declared ESP-IDF SDK: {e}")}),
                )
            }
        };
        if !version_matches(&actual, s(&t["toolchain"]["version"])?) {
            return Ok(
                json!({"status":"not_run","reason":format!("Expected ESP-IDF {}; got {actual}",t["toolchain"]["version"])}),
            );
        }
        let sdk = match env::var("IDF_PATH") {
            Ok(v) => v,
            Err(_) => {
                return Ok(
                    json!({"status":"not_run","reason":"IDF_PATH is required for SDK provenance."}),
                )
            }
        };
        let git = |args: &[&str]| output(Command::new("git").arg("-C").arg(&sdk).args(args));
        let compiler = output(Command::new("xtensa-esp-elf-gcc").arg("--version"))?;
        let python = output(
            Command::new("python").args(["-c", "import sys; print(sys.version.split()[0])"]),
        )?;
        Ok(
            json!({"status":"passed","version":actual,"sdk_commit":git(&["rev-parse","HEAD"] )?,"sdk_submodules":git(&["submodule","status","--recursive"] )?,"sdk_changes_sha256":identity(&json!(git(&["diff","HEAD","--submodule=diff"])?)),"compiler":compiler.lines().next(),"python":python}),
        )
    }
    fn build(&self, root: &Path, t: &J) -> Result<Vec<J>> {
        let tc = self.probe(root, t)?;
        let mut checks = vec![];
        let variants: Vec<Option<String>> = if obj(&t["variants"])?.is_empty() {
            vec![None]
        } else {
            obj(&t["variants"])?.keys().cloned().map(Some).collect()
        };
        for ex in strings(&t["examples"])? {
            for variant in &variants {
                let name = format!(
                    "{}/{}{}",
                    s(&t["id"])?,
                    ex,
                    variant
                        .as_ref()
                        .map(|v| format!("/{v}"))
                        .unwrap_or_default()
                );
                if tc["status"] != "passed" {
                    checks.push(json!({"name":name,"status":"not_run","reason":tc["reason"],"toolchain":tc}));
                    continue;
                }
                let base = safe(root, s(&t["path"])?)?;
                let mut build = root
                    .join(".codecraft/builds")
                    .join(s(&t["device"])?)
                    .join(&ex);
                if let Some(v) = variant {
                    build = build.join(v);
                }
                let mut argv = vec![
                    "idf.py".into(),
                    "-C".into(),
                    base.join("examples").join(&ex).to_string_lossy().into(),
                    "-B".into(),
                    build.to_string_lossy().into(),
                ];
                if let Some(v) = variant {
                    argv.extend(["-D".into(), format!("CODECRAFT_HARDWARE_VARIANT={v}")]);
                }
                argv.push("build".into());
                let mut row = command(root, &name, &argv, &tc)?;
                if row["status"] == "passed" {
                    match verify_composition(root, t, &ex, variant.as_deref(), &build) {
                        Ok(v) => row["composition"] = v,
                        Err(e) => {
                            row["status"] = json!("failed");
                            row["reason"] =
                                json!(format!("Native composition verification failed: {e}"));
                        }
                    }
                }
                checks.push(row);
            }
        }
        Ok(checks)
    }
}
fn settings(text: &str, key: &str) -> Result<Vec<String>> {
    let clean = regex::Regex::new(r"(?m)#[^\n]*")?.replace_all(text, "");
    let pattern = regex::Regex::new(&format!(r"set\({key}\s+([^)]*)\)"))?;
    let body = pattern
        .captures(&clean)
        .context("Missing native composition setting")?;
    shlex::split(&body[1]).context("Invalid native setting")
}
pub fn verify_composition(
    root: &Path,
    t: &J,
    ex: &str,
    variant: Option<&str>,
    build: &Path,
) -> Result<J> {
    let base = safe(root, s(&t["path"])?)?;
    let text = fs::read_to_string(base.join("examples").join(ex).join("bsp.cmake"))?;
    let sources = settings(&text, "CODECRAFT_BSP_SOURCES")?;
    let requirements = settings(&text, "CODECRAFT_BSP_REQUIRES")?;
    let data = read(&build.join("project_description.json"))?;
    let bsp = &data["build_component_info"]["bsp"];
    let components = strings(&data["build_components"])?;
    ensure!(
        components.contains(&"main".into()) && components.contains(&"bsp".into()),
        "Missing native application/BSP component"
    );
    let expected: std::collections::BTreeSet<_> = sources
        .iter()
        .map(|p| base.join("bsp").join(p).canonicalize())
        .collect::<std::io::Result<_>>()?;
    let actual: std::collections::BTreeSet<_> = strings(&bsp["sources"])?
        .iter()
        .map(|p| Path::new(p).canonicalize())
        .collect::<std::io::Result<_>>()?;
    ensure!(expected == actual, "Built BSP source selection differs");
    let normal = |values: Vec<String>| {
        values
            .into_iter()
            .map(|s| s.trim_start_matches("espressif__").to_owned())
            .collect::<std::collections::BTreeSet<_>>()
    };
    ensure!(
        normal(requirements) == normal(strings(&bsp["reqs"])?),
        "Built BSP requirements differ"
    );
    if let Some(v) = variant {
        let cache = fs::read_to_string(build.join("CMakeCache.txt"))?;
        let matches: Vec<_> =
            regex::Regex::new(r"(?m)^CODECRAFT_HARDWARE_VARIANT(?::[^=\n]+)?=([^\n]*)$")?
                .captures_iter(&cache)
                .map(|m| m[1].to_owned())
                .collect();
        ensure!(matches == [v], "Built variant differs from selection");
        let profile = s(&t["variants"][v]["configuration"]["profile"])?;
        let defs = settings(
            &fs::read_to_string(safe(root, profile)?)?,
            "CODECRAFT_VARIANT_DEFINES",
        )?;
        let commands = read(&build.join("compile_commands.json"))?;
        let mut seen = std::collections::BTreeSet::new();
        for cmd in arr(&commands)? {
            let path = Path::new(s(&cmd["file"])?);
            let path = if path.is_absolute() {
                path.to_owned()
            } else {
                Path::new(s(&cmd["directory"])?).join(path)
            }
            .canonicalize()?;
            if !expected.contains(&path) {
                continue;
            }
            let tokens = if cmd["arguments"].is_array() {
                strings(&cmd["arguments"])?
            } else {
                shlex::split(s(&cmd["command"])?).context("Invalid compile command")?
            };
            for def in &defs {
                let name = def.split('=').next().unwrap();
                let actual: Vec<_> = tokens
                    .iter()
                    .filter_map(|x| x.strip_prefix("-D"))
                    .filter(|x| x.split('=').next() == Some(name))
                    .collect();
                ensure!(actual == [def], "Variant compile definition mismatch");
            }
            seen.insert(path);
        }
        ensure!(seen == expected, "Missing BSP compile commands");
    }
    Ok(
        json!({"bsp_sources":sources,"bsp_requirements":bsp["reqs"],"build_components":data["build_components"],"variant":variant}),
    )
}
