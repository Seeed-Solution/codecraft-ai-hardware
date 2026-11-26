//! Native BSP metadata validation, shared by static checks and compiled-output checks.
use crate::util::*;
use anyhow::{ensure, Context, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
pub fn settings(path: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let text = fs::read_to_string(path)?;
    let clean = regex::Regex::new(r"(?m)#[^\n]*")?.replace_all(&text, "");
    let pattern = regex::Regex::new(r"set\((CODECRAFT_BSP_(?:SOURCES|REQUIRES))\s+([^)]*)\)")?;
    ensure!(
        pattern.replace_all(&clean, "").trim().is_empty(),
        "Unsupported composition syntax"
    );
    let mut out = BTreeMap::new();
    let source_pattern = regex::Regex::new(r"^src/bsp_[a-z0-9_]+\.c$")?;
    let require_pattern = regex::Regex::new(r"^[a-z][a-z0-9_]*$")?;
    for c in pattern.captures_iter(&clean) {
        let values = shlex::split(&c[2]).context("Invalid composition quoting")?;
        let allowed = if c[1].ends_with("SOURCES") {
            &source_pattern
        } else {
            &require_pattern
        };
        ensure!(
            !values.is_empty()
                && values.iter().collect::<BTreeSet<_>>().len() == values.len()
                && values.iter().all(|v| allowed.is_match(v)),
            "Invalid/duplicate composition values"
        );
        ensure!(
            out.insert(c[1].into(), values).is_none(),
            "Duplicate composition setting"
        );
    }
    ensure!(out.len() == 2, "Incomplete composition");
    Ok(out)
}
pub struct Selection {
    pub sources: BTreeSet<PathBuf>,
    pub files: BTreeSet<PathBuf>,
    pub requirements: BTreeSet<String>,
    pub components: BTreeSet<PathBuf>,
}
pub fn describe(root: &Path, t: &J, ex: &str) -> Result<Selection> {
    let base = safe(root, s(&t["path"])?)?;
    let bsp = base.join("bsp");
    let folder = base.join("examples").join(ex);
    let data = settings(&folder.join("bsp.cmake"))?;
    let sources: BTreeSet<_> = data["CODECRAFT_BSP_SOURCES"]
        .iter()
        .map(|p| {
            safe(
                root,
                bsp.join(p)
                    .strip_prefix(root)?
                    .to_str()
                    .context("Invalid path")?,
            )
        })
        .collect::<Result<_>>()?;
    let mut files = sources.clone();
    let mut pending: Vec<_> = sources.iter().cloned().collect();
    pending.push(folder.join("main/main.c"));
    let include = regex::Regex::new(r#"(?m)^\s*#include\s+"(bsp_[a-z0-9_]+\.h)""#)?;
    while let Some(path) = pending.pop() {
        for c in include.captures_iter(&fs::read_to_string(path)?) {
            let candidates: Vec<_> = [bsp.join("include").join(&c[1]), bsp.join("src").join(&c[1])]
                .into_iter()
                .filter(|p| p.is_file())
                .collect();
            ensure!(candidates.len() == 1, "Missing/ambiguous BSP header");
            ensure!(
                &c[1] == "bsp_pins.h"
                    || sources.contains(&bsp.join("src").join(c[1].replace(".h", ".c"))),
                "BSP header has no selected implementation"
            );
            let path = candidates[0].clone();
            if files.insert(path.clone()) {
                pending.push(path);
            }
        }
    }
    let requirements: BTreeSet<_> = data["CODECRAFT_BSP_REQUIRES"].iter().cloned().collect();
    ensure!(
        requirements.contains("freertos") && requirements.contains("log"),
        "Missing common BSP runtime requirements"
    );
    let components = requirements
        .iter()
        .map(|n| root.join("platforms/esp-idf/components").join(n))
        .filter(|p| p.join("CMakeLists.txt").is_file())
        .collect();
    Ok(Selection {
        sources,
        files,
        requirements,
        components,
    })
}
pub fn validate(root: &Path, t: &J) -> Result<()> {
    let base = safe(root, s(&t["path"])?)?;
    let bsp = base.join("bsp");
    for e in fs::read_dir(&bsp)? {
        let p = e?.path();
        ensure!(
            !matches!(p.extension().and_then(|s| s.to_str()), Some("c" | "h")),
            "BSP implementation belongs in src/include"
        );
    }
    let manifest = fs::read_to_string(bsp.join("idf_component.yml"))?;
    ensure!(
        regex::Regex::new(r"(?m)#[^\n]*")?
            .replace_all(&manifest, "")
            .trim()
            == "dependencies: {}",
        "Managed dependencies belong to example"
    );
    let component_pattern = regex::Regex::new(r#"\$\{CMAKE_CURRENT_SOURCE_DIR\}/([^"\s]+)"#)?;
    for ex in strings(&t["examples"])? {
        let selected = describe(root, t, &ex)?;
        let folder = base.join("examples").join(&ex);
        let cmake = fs::read_to_string(folder.join("CMakeLists.txt"))?;
        ensure!(
            cmake.contains("idf_build_set_property(MINIMAL_BUILD ON)"),
            "Minimal native component selection required"
        );
        let extra = component_pattern
            .captures_iter(&cmake)
            .map(|c| Ok(folder.join(&c[1]).canonicalize()?))
            .collect::<Result<BTreeSet<_>>>()?;
        let mut expected = selected.components.clone();
        expected.insert(bsp.clone());
        ensure!(extra == expected, "Native component directories differ");
        let mut refs = vec![];
        for key in ["inputs", "dependencies"] {
            refs.extend(strings(&t[key])?);
        }
        for key in ["example_inputs", "example_dependencies"] {
            refs.extend(strings(&t[key][&ex])?);
        }
        for i in arr(&t["interfaces"])? {
            refs.extend(strings(&i["implementation"])?);
            if let Some(v) = i["example_implementation"].get(&ex) {
                refs.extend(strings(v)?);
            }
        }
        let mut graph = Graph::new(root);
        graph.resolve(refs)?;
        let all: BTreeSet<_> = graph.files.keys().map(|p| root.join(p)).collect();
        let actual = all
            .iter()
            .filter(|p| p.starts_with(&bsp))
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut expected = selected.files;
        for f in ["CMakeLists.txt", "idf_component.yml", "include/bsp_pins.h"] {
            expected.insert(bsp.join(f));
        }
        ensure!(actual == expected, "Declared BSP source closure differs");
        let actual_components = graph
            .files
            .keys()
            .filter(|p| p.starts_with("platforms/esp-idf/components/"))
            .map(|p| root.join(p.split('/').take(4).collect::<Vec<_>>().join("/")))
            .collect::<BTreeSet<_>>();
        ensure!(
            actual_components == selected.components,
            "Component dependencies differ"
        );
        let manifest = folder.join("main/idf_component.yml");
        ensure!(
            manifest.is_file()
                && strings(&t["example_dependencies"][&ex])?.contains(
                    &manifest
                        .strip_prefix(root)?
                        .to_string_lossy()
                        .replace('\\', "/")
                ),
            "Example manifest dependency missing"
        );
    }
    Ok(())
}
