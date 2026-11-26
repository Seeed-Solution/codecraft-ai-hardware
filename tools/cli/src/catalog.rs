use crate::{device, util::*};
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
pub struct Catalog {
    pub root: PathBuf,
    pub devices: BTreeMap<String, J>,
    pub targets: BTreeMap<String, J>,
    pub platforms: BTreeMap<String, J>,
}
impl Catalog {
    pub fn load(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        let mut c = Self {
            root,
            devices: BTreeMap::new(),
            targets: BTreeMap::new(),
            platforms: BTreeMap::new(),
        };
        for e in fs::read_dir(c.root.join("platforms"))? {
            let p = e?.path();
            if p.is_dir()
                && !p
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(['.', '_'])
            {
                let v = read(&p.join("platform.json"))?;
                let id = s(&v["id"])?.to_owned();
                ensure!(v["schema_version"] == 3, "Platform schema 3 required; recreate exports made with the removed adapter/runtime contract");
                ensure!(c.platforms.insert(id, v).is_none(), "Duplicate platform");
            }
        }
        for e in fs::read_dir(c.root.join("devices"))? {
            let p = e?.path();
            if !p.is_dir()
                || p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(['.', '_'])
            {
                continue;
            }
            let id = p
                .file_name()
                .unwrap()
                .to_str()
                .context("Invalid device path")?;
            let d = device::read_device(&c.root, id)?;
            let t = read(&p.join("targets/target.json"))?;
            let tid = s(&t["id"])?.to_owned();
            ensure!(
                t["schema_version"] == 3
                    && t["device"] == id
                    && tid == format!("{}/{id}", s(&t["platform"])?),
                "Target identity/schema mismatch"
            );
            ensure!(
                t["path"] == format!("devices/{id}/targets")
                    && c.platforms.contains_key(s(&t["platform"])?),
                "Unknown platform or Target root"
            );
            ensure!(
                !strings(&t["examples"])?.is_empty() && t["variants"].is_object(),
                "Missing Target examples/variants"
            );
            c.devices.insert(id.to_owned(), d);
            ensure!(c.targets.insert(tid, t).is_none(), "Duplicate Target");
        }
        ensure!(!c.targets.is_empty(), "Empty catalog");
        let application = c.root.join("project.yaml").is_file();
        for (id, p) in &c.platforms {
            ensure!(
                c.root
                    .join(format!("platforms/{id}/platform.json"))
                    .is_file(),
                "Platform identity mismatch"
            );
            ensure!(
                !c.root.join(format!("platforms/{id}/targets")).exists(),
                "Targets belong under devices"
            );
            crate::platform_schema::platform(&c.root, p, application)?;
        }
        for t in c.targets.values() {
            let base = c.root.join(s(&t["path"])?);
            ensure!(
                !base.join("tests").exists()
                    && !c
                        .root
                        .join("devices")
                        .join(s(&t["device"])?)
                        .join("tests")
                        .exists(),
                "Tests belong under root tests"
            );
            ensure!(
                c.platforms.keys().all(|stack| !base.join(stack).exists()),
                "Target may not contain stack subdirectories"
            );
            crate::platform_schema::target(&c, t)?;
            c.graph(t)?;
        }
        Ok(c)
    }
    pub fn target(&self, id: &str) -> Result<&J> {
        self.targets
            .get(id)
            .with_context(|| format!("Unknown Target: {id}"))
    }
    pub fn selected(&self, id: &str, example: Option<&str>, variant: Option<&str>) -> Result<J> {
        let mut t = self.target(id)?.clone();
        let variants = obj(&t["variants"])?;
        if variant.is_some() || (example.is_some() && !variants.is_empty()) {
            let name = variant.unwrap_or(s(&t["default_variant"])?).to_owned();
            let value = variants
                .get(&name)
                .with_context(|| format!("Unknown variant {name}"))?
                .clone();
            ensure!(
                self.devices[s(&t["device"])?]["variants"]
                    .get(s(&value["device_variant"])?)
                    .is_some(),
                "Unknown hardware variant"
            );
            t["variants"] = json!({name.clone():value});
            t["selected_variant"] = json!(name);
            t["default_variant"] = t["selected_variant"].clone();
        }
        if let Some(ex) = example {
            ensure!(
                strings(&t["examples"])?.contains(&ex.to_owned()),
                "Unknown example {ex}"
            );
            t["examples"] = json!([ex]);
            for key in ["example_inputs", "example_dependencies"] {
                let value = t[key].get(ex).context("Missing example scope")?.clone();
                t[key] = json!({ex:value});
            }
            let mut interfaces = vec![];
            for original in arr(&t["interfaces"])? {
                let mut i = original.clone();
                let scope = i["example_implementation"].get(ex).cloned();
                i["example_implementation"] = scope.map_or(object(), |v| json!({ex:v}));
                if arr(&i["implementation"])?.is_empty()
                    && obj(&i["example_implementation"])?.is_empty()
                {
                    continue;
                }
                for op in i["operations"]
                    .as_object_mut()
                    .context("Missing operations")?
                    .values_mut()
                {
                    if op.get("example_sources").is_some() {
                        let v = op["example_sources"]
                            .get(ex)
                            .context("Missing operation source")?
                            .clone();
                        op["example_sources"] = json!({ex:v});
                    }
                }
                interfaces.push(i);
            }
            t["interfaces"] = json!(interfaces);
        }
        Ok(t)
    }
    pub fn graph(&self, t: &J) -> Result<Graph> {
        let d = &self.devices[s(&t["device"])?];
        let stack = s(&t["platform"])?;
        let platform = &self.platforms[stack];
        let mut roots = vec![
            "AGENTS.md".into(),
            format!("devices/{}/AGENTS.md", s(&t["device"])?),
            format!("{}/target.json", s(&t["path"])?),
            format!("platforms/{stack}/platform.json"),
            format!("platforms/{stack}/AGENTS.md"),
            s(&t["toolchain"]["source"])?.into(),
        ];
        // Application dependencies are authored separately from repository validation tools.
        roots.extend(
            strings(&platform["application_inputs"])
                .context("Platform application_inputs required")?,
        );
        roots.extend(strings(&d["references"])?);
        for a in arr(&d["assets"])? {
            roots.push(s(&a["path"])?.into());
        }
        for a in arr(&d["acceptance"]["cases"])? {
            roots.push(s(&a["source"])?.into());
        }
        for key in ["inputs", "dependencies"] {
            roots.extend(strings(&t[key])?);
        }
        for key in ["example_inputs", "example_dependencies"] {
            for v in obj(&t[key])?.values() {
                roots.extend(strings(v)?);
            }
        }
        for v in obj(&t["variants"])?.values() {
            roots.extend(strings(&v["inputs"])?);
            roots.extend(strings(&v["dependencies"])?);
        }
        for i in arr(&t["interfaces"])? {
            roots.extend(strings(&i["implementation"])?);
            if let Some(v) = i.get("example_implementation") {
                for p in obj(v)?.values() {
                    roots.extend(strings(p)?);
                }
            }
        }
        let mut graph = Graph::new(&self.root);
        graph.resolve(roots)?;
        if self.root.join("project.yaml").is_file() {
            let profile = read(&self.root.join("project.yaml"))?;
            crate::skills::add_selections(&mut graph, &profile["skills"])?;
        }
        Ok(graph)
    }
    pub fn context(
        &self,
        id: &str,
        example: Option<&str>,
        variant: Option<&str>,
        task: &str,
    ) -> Result<J> {
        ensure!(
            [
                "ideate",
                "develop",
                "setup",
                "build",
                "debug",
                "device-test",
                "discover",
                "validate"
            ]
            .contains(&task),
            "Unknown task"
        );
        let t = self.selected(id, example, variant)?;
        let g = self.graph(&t)?;
        let d = &self.devices[s(&t["device"])?];
        let mut guidance = vec![
            "AGENTS.md".to_owned(),
            format!("devices/{}/AGENTS.md", s(&t["device"])?),
            format!("platforms/{}/AGENTS.md", s(&t["platform"])?),
            format!("skills/codecraft-development-{task}/SKILL.md"),
        ];
        if task == "ideate" {
            guidance.push("skills/codecraft-development-discover/SKILL.md".into());
        }
        ensure!(
            guidance.iter().all(|v| g.files.contains_key(v)),
            "Missing portable guidance"
        );
        Ok(
            json!({"schema_version":4,"device":d,"target":t,"task":task,"guidance":guidance,"files":g.files.keys().collect::<Vec<_>>(),"dependencies":g.edges,"source_digest":g.fingerprint(),"unknowns":d["unknowns"],"support":crate::maintenance_evidence::support(self,id)?}),
        )
    }
    pub fn discover(
        &self,
        interfaces: &[String],
        platform: Option<&str>,
        constraints: &J,
    ) -> Result<J> {
        let mut requested: BTreeSet<_> = interfaces.iter().cloned().collect();
        requested.extend(obj(constraints)?.keys().cloned());
        let available: BTreeSet<String> = self
            .targets
            .values()
            .flat_map(|t| {
                t["interfaces"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|i| i["id"].as_str().map(str::to_owned))
            })
            .collect();
        ensure!(
            requested.is_subset(&available),
            "Unknown requested native interface"
        );
        if let Some(p) = platform {
            ensure!(self.platforms.contains_key(p), "Unknown platform");
        }
        let mut candidates = vec![];
        let mut rejected = vec![];
        for (id, t) in &self.targets {
            if platform.is_some_and(|p| t["platform"] != p) {
                continue;
            }
            let bindings: BTreeMap<_, _> = arr(&t["interfaces"])?
                .iter()
                .map(|i| Ok((s(&i["id"])?.to_owned(), i)))
                .collect::<Result<_>>()?;
            let missing: Vec<_> = requested
                .iter()
                .filter(|r| !bindings.contains_key(*r))
                .collect();
            if !missing.is_empty() {
                rejected.push(
                    json!({"target":id,"missing":missing,"reason":"Missing implementations"}),
                );
                continue;
            }
            let mut matches = object();
            let mut conflicts = vec![];
            let mut unknowns = vec![];
            for cid in &requested {
                let request = constraints.get(cid).cloned().unwrap_or(object());
                ensure!(obj(&request)?.keys().all(|k|["operations","configuration","resources"].contains(&k.as_str())),"Unknown interface constraint");
                let binding = bindings[cid];
                let mut no = vec![];
                let mut unknown = vec![];
                if let Some(ops) = request.get("operations") {
                    for op in strings(ops)? {
                        let operation = binding["operations"]
                            .get(&op)
                            .context("Unknown operation")?;
                        if operation["supported"] != true {
                            no.push(op);
                        }
                    }
                }
                for group in ["configuration", "resources"] {
                    if let Some(parameters) = request.get(group) {
                        for (name, value) in obj(parameters)? {
                            let spec = binding[group]
                                .get(name)
                                .context("Unknown native parameter")?;
                            crate::skills::payload(spec, value)?;
                            if spec["status"] == "unknown" {
                                unknown.push(format!("{group}/{name}"));
                            } else if (group == "resources"
                                && spec["value"]
                                    .as_f64()
                                    .zip(value.as_f64())
                                    .is_some_and(|(a, b)| a > b))
                                || (group == "configuration" && spec["value"] != *value)
                            {
                                no.push(format!("{group}/{name}"));
                            }
                        }
                    }
                }
                matches[cid] = json!({"status":if !no.is_empty(){"unsupported"}else if !unknown.is_empty(){"unknown"}else{"match"},"conflicts":no,"unknowns":unknown,"physical_verification":"unknown"});
                conflicts.extend(no);
                unknowns.extend(unknown);
            }
            if !conflicts.is_empty() {
                rejected.push(json!({"target":id,"constraints":matches}));
            } else {
                candidates.push(json!({"target":id,"compatibility":if unknowns.is_empty(){"match"}else{"unknown"},"constraints":matches,"verification":"unknown","unknowns":self.devices[s(&t["device"])?]["unknowns"]}));
            }
        }
        Ok(
            json!({"status":if candidates.is_empty(){"unsupported"}else{"candidates"},"needs_user_input":candidates.len()!=1,"candidates":candidates,"rejected":rejected,"physical_compatibility":"unknown"}),
        )
    }
}
