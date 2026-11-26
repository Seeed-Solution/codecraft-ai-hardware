//! Rust-owned repository validation and discovery; Python is only a selected test runtime.
use crate::{catalog::Catalog, util::*};
use anyhow::{bail, ensure, Context, Result};
use quick_xml::{events::Event, Reader};
use regex::Regex;
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};
fn result(name: &str, status: &str, reason: impl ToString) -> J {
    json!({"name":name,"status":status,"reason":reason.to_string()})
}
fn excluded(name: &str) -> bool {
    ignored(name) || name.starts_with('.') || matches!(name, "fixtures" | "stubs" | "mocks")
}
pub fn test_files(
    root: &Path,
    platform: Option<&str>,
    selection: Option<&str>,
) -> Result<Vec<String>> {
    let root = root.canonicalize()?;
    let mut owners = vec![];
    for area in ["tests/tools", "tests/skills", "tests/platforms"] {
        if !root.join(area).is_dir() {
            continue;
        }
        for e in fs::read_dir(root.join(area))? {
            let p = e?.path();
            if p.is_dir()
                && !excluded(&p.file_name().unwrap().to_string_lossy())
                && (area != "tests/platforms"
                    || platform.is_none_or(|v| p.file_name().unwrap() == v))
            {
                owners.push(p);
            }
        }
    }
    if root.join("tests/devices").is_dir() {
        for e in fs::read_dir(root.join("tests/devices"))? {
            let p = e?.path();
            let path = p.join("targets");
            if !path.is_dir() {
                continue;
            }
            let id = p.file_name().unwrap().to_string_lossy();
            let native = format!("devices/{id}/targets");
            let target = read(&root.join(&native).join("target.json"))?;
            ensure!(
                target["device"] == id.as_ref()
                    && target["path"] == native
                    && target["id"] == format!("{}/{id}", s(&target["platform"])?),
                "Test owner differs from Target"
            );
            if platform.is_none_or(|v| target["platform"] == v) {
                owners.push(path);
            }
        }
    }
    let selected = selection
        .map(|v| {
            ensure!(
                !v.is_empty()
                    && !Path::new(v).is_absolute()
                    && Path::new(v)
                        .components()
                        .all(|c| matches!(c, Component::Normal(_))),
                "Test selection must be repository-relative"
            );
            let p = root.join(v);
            ensure!(p.exists(), "Test selection does not exist");
            for ancestor in p.ancestors().take_while(|p| *p != root) {
                ensure!(!ancestor.is_symlink(), "Symlink test selection");
            }
            Ok(p)
        })
        .transpose()?;
    let mut files = BTreeSet::new();
    for owner in owners {
        for ancestor in owner.ancestors().take_while(|p| *p != root) {
            ensure!(!ancestor.is_symlink(), "Symlink test owner");
        }
        for e in walkdir::WalkDir::new(&owner)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| e.depth() == 0 || !excluded(&e.file_name().to_string_lossy()))
        {
            let e = e?;
            ensure!(!e.path_is_symlink(), "Symlink test path");
            let name = e.file_name().to_string_lossy();
            if !name.starts_with("test_") || !name.ends_with(".py") {
                continue;
            }
            ensure!(
                !e.path_is_symlink() && !owner.is_symlink(),
                "Symlink test path"
            );
            if selected
                .as_ref()
                .is_none_or(|s| e.path() == s || e.path().starts_with(s))
            {
                files.insert(
                    e.path()
                        .strip_prefix(&root)?
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    ensure!(
        !files.is_empty(),
        "No host tests discovered for requested scope"
    );
    Ok(files.into_iter().collect())
}
fn python() -> String {
    std::env::var("CODECRAFT_TEST_PYTHON").unwrap_or_else(|_| "python3".into())
}
fn pytest_command(root: &Path) -> Result<Command> {
    ensure!(
        root.join("tests/conftest.py").is_file(),
        "Missing Python test collection contract: tests/conftest.py"
    );
    let mut c = Command::new(python());
    c.current_dir(root).args([
        "-m",
        "pytest",
        "-q",
        "--strict-config",
        "--import-mode=importlib",
        "-o",
        "python_classes=Test*",
        "-o",
        "junit_family=xunit1",
    ]);
    c.env("PYTEST_DISABLE_PLUGIN_AUTOLOAD", "1")
        .env("PYTHONDONTWRITEBYTECODE", "1");
    let cli = std::env::var_os("CODECRAFT_TEST_CLI")
        .map(PathBuf::from)
        .unwrap_or(std::env::current_exe()?);
    c.env("CODECRAFT_TEST_CLI", cli);
    Ok(c)
}
pub fn inventory(root: &Path, platform: Option<&str>, selection: Option<&str>) -> Result<J> {
    let paths = test_files(root, platform, selection)?;
    let mut tests = vec![];
    let mut errors = vec![];
    let mut seen = BTreeSet::new();
    for p in paths {
        let output = pytest_command(root)?
            .args(["--collect-only", &p])
            .output()?;
        let text = String::from_utf8_lossy(&output.stdout);
        if !output.status.success() {
            errors.push(result(
                &p,
                "failed",
                format!(
                    "Collection: {text}\n{}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            ));
            continue;
        }
        for line in text
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with(&format!("{p}::")))
        {
            ensure!(
                seen.insert(line.to_owned()),
                "Duplicate test identity: {line}"
            );
            tests.push(json!({"name":line,"kind":"python-host"}));
        }
    }
    Ok(json!({"tests":tests,"collection_errors":errors,"ok":errors.is_empty()}))
}
fn attrs(
    e: &quick_xml::events::BytesStart<'_>,
    reader: &Reader<&[u8]>,
) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    for a in e.attributes() {
        let a = a?;
        out.insert(
            String::from_utf8(a.key.0.to_vec())?,
            a.decode_and_unescape_value(reader.decoder())?.into_owned(),
        );
    }
    Ok(out)
}
pub fn junit_results(text: &str, path: &str) -> Result<Vec<J>> {
    let mut reader = Reader::from_str(text);
    let mut out = vec![];
    let mut current: Option<J> = None;
    let mut seen = BTreeSet::new();
    loop {
        match reader.read_event()? {
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == b"testcase" => {
                let a = attrs(&e, &reader)?;
                let name = a.get("name").context("JUnit test name missing")?;
                let class = a.get("classname").map(String::as_str).unwrap_or("");
                let module = path.trim_end_matches(".py").replace('/', ".");
                let class = class
                    .strip_prefix(&module)
                    .unwrap_or(class)
                    .trim_start_matches('.');
                let id = if class.is_empty() {
                    format!("{path}::{name}")
                } else {
                    format!("{path}::{class}::{name}")
                };
                ensure!(seen.insert(id.clone()), "Duplicate JUnit test identity");
                if let Some(previous) = current.take() {
                    out.push(previous)
                }
                current = Some(result(&id, "passed", ""));
            }
            Event::Start(e) | Event::Empty(e)
                if [
                    b"failure".as_slice(),
                    b"error".as_slice(),
                    b"skipped".as_slice(),
                ]
                .contains(&e.name().as_ref()) =>
            {
                let a = attrs(&e, &reader)?;
                let status = if e.name().as_ref() == b"skipped" {
                    "skipped"
                } else {
                    "failed"
                };
                let row =
                    current.get_or_insert_with(|| result(path, "failed", "Suite fixture failure"));
                if e.name().as_ref() == b"error" {
                    let name = format!("{}::fixture", s(&row["name"])?);
                    out.push(result(
                        &name,
                        "failed",
                        a.get("message").cloned().unwrap_or_default(),
                    ));
                    row["status"] = json!(if a
                        .get("message")
                        .is_some_and(|m| m.starts_with("failed on setup"))
                    {
                        "not_run"
                    } else {
                        "failed"
                    });
                } else if row["status"] != "failed" || status == "failed" {
                    row["status"] = json!(status);
                }
                row["reason"] = json!(a.get("message").cloned().unwrap_or_default());
            }
            Event::End(e) if e.name().as_ref() == b"testcase" => {
                if let Some(row) = current.take() {
                    out.push(row)
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    if let Some(row) = current {
        out.push(row)
    }
    ensure!(!out.is_empty(), "Empty JUnit results");
    Ok(out)
}
pub fn run_tests(root: &Path, paths: &[String]) -> Result<J> {
    ensure!(!paths.is_empty(), "Empty host test selection");
    let allowed: BTreeSet<_> = test_files(root, None, None)?.into_iter().collect();
    let mut selected = BTreeSet::new();
    for p in paths {
        let file = p.split("::").next().unwrap();
        if allowed.contains(file) {
            ensure!(!p.contains(['\n', '\r']), "Invalid test node selection");
            selected.insert(p.clone());
        } else {
            ensure!(!p.contains("::"), "Unknown selected test file");
            selected.extend(test_files(root, None, Some(p))?);
        }
    }
    let temp = tempfile::tempdir()?;
    let mut results = vec![];
    for (i, p) in selected.into_iter().enumerate() {
        let report = temp.path().join(format!("{i}.xml"));
        let output = pytest_command(root)?
            .arg(format!("--junitxml={}", report.display()))
            .arg(&p)
            .output()?;
        let transcript = format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if !report.exists() {
            results.push(result(&p, "failed", transcript));
            continue;
        }
        match junit_results(&fs::read_to_string(report)?, p.split("::").next().unwrap()) {
            Ok(mut rows) => {
                if !output.status.success() && rows.iter().all(|r| r["status"] != "failed") {
                    rows.push(result(&format!("{p}::runner"), "failed", transcript));
                }
                results.extend(rows)
            }
            Err(e) => results.push(result(&p, "failed", format!("{e}: {transcript}"))),
        }
    }
    Ok(json!({"ok":results.iter().all(|r|r["status"]=="passed"),"results":results}))
}
fn prose(text: &str) -> String {
    let mut output = String::new();
    let mut fence: Option<(char, usize)> = None;
    for line in text.lines() {
        let t = line.trim_start();
        let c = t.chars().next().unwrap_or(' ');
        let n = t.chars().take_while(|x| *x == c).count();
        if matches!(c, '`' | '~') && n >= 3 {
            if fence.is_none() {
                fence = Some((c, n));
            } else if fence
                .is_some_and(|(f, len)| f == c && n >= len && t.trim().chars().all(|x| x == c))
            {
                fence = None;
            }
            continue;
        }
        if fence.is_none() {
            output.push_str(line);
            output.push('\n');
        }
    }
    Regex::new(r"`+[^`\n]*`+")
        .unwrap()
        .replace_all(&output, "")
        .into_owned()
}
pub fn documentation(root: &Path) -> Result<()> {
    let inline = Regex::new(r#"\]\(([^)]+)\)|href="([^"]+)""#)?;
    let reference = Regex::new(r"(?m)^ {0,3}\[([^\]\n]+)\]:\s*(\S+)\s*$")?;
    let named = Regex::new(r"\[([^\]\n]+)\]\[([^\]\n]*)\]")?;
    for file in files(root)?
        .into_iter()
        .filter(|p| p.ends_with(".md") && !p.starts_with("projects/"))
    {
        let p = root.join(&file);
        let text = fs::read_to_string(&p)?;
        let name = p.file_name().unwrap().to_string_lossy();
        ensure!(
            !name.ends_with(".zh_CN.md") || name == "README.zh_CN.md",
            "Only README may have a Chinese counterpart: {file}"
        );
        if name == "README.md" {
            let counterpart = p.with_file_name("README.zh_CN.md");
            ensure!(
                counterpart.is_file()
                    && text.contains("README.zh_CN.md")
                    && fs::read_to_string(counterpart)?.contains("README.md"),
                "Missing reciprocal README counterpart: {file}"
            );
        } else if name == "README.zh_CN.md" {
            ensure!(
                p.with_file_name("README.md").is_file(),
                "Orphan Chinese README"
            );
        }
        let text = prose(&text);
        let mut refs = BTreeMap::new();
        for cap in reference.captures_iter(&text) {
            let key = cap[1]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();
            ensure!(
                refs.insert(key, cap[2].trim_matches(['<', '>']).to_owned())
                    .is_none(),
                "Duplicate Markdown reference: {file}"
            );
        }
        let mut links: Vec<String> = refs.values().cloned().collect();
        for cap in inline.captures_iter(&text) {
            links.push(
                cap.get(1)
                    .or_else(|| cap.get(2))
                    .unwrap()
                    .as_str()
                    .to_owned(),
            );
        }
        for cap in named.captures_iter(&text) {
            let label = if cap[2].is_empty() { &cap[1] } else { &cap[2] };
            let key = label
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();
            links.push(
                refs.get(&key)
                    .with_context(|| format!("Undefined Markdown reference {key}: {file}"))?
                    .clone(),
            );
        }
        for url in links {
            let link = url.split('#').next().unwrap();
            if link.is_empty()
                || link.contains("://")
                || link.starts_with("mailto:")
                || link.starts_with("data:")
                || link.contains(['<', '>', ' '])
            {
                continue;
            }
            let decoded = percent_encoding::percent_decode_str(link).decode_utf8()?;
            ensure!(
                p.parent().unwrap().join(decoded.as_ref()).exists(),
                "Broken local link {link}: {file}"
            );
        }
    }
    Ok(())
}
pub fn source_checks(root: &Path) -> Result<Vec<J>> {
    let all = files(root)?;
    let key = Regex::new(&format!(
        "{}{}",
        "-----BEGIN ", "(?:RSA |EC |OPENSSH )?PRIVATE KEY-----"
    ))?;
    let mut python_files = vec![];
    for path in &all {
        if path.starts_with("projects/") {
            continue;
        }
        let p = root.join(path);
        let parts: Vec<_> = path.split('/').collect();
        if ["skills", "tools", "platforms", "devices"].contains(&parts[0]) {
            ensure!(
                !parts[..parts.len() - 1]
                    .iter()
                    .any(|x| matches!(*x, "test" | "tests" | "_tests")),
                "Maintained tests must live under tests/: {path}"
            );
        }
        ensure!(
            !(path.starts_with("tools/") && path.ends_with(".py")),
            "Python implementation remains under tools/: {path}"
        );
        if path.ends_with(".py") && ["skills", "platforms", "devices", "tests"].contains(&parts[0])
        {
            let source = fs::read_to_string(&p)?;
            ensure!(
                !source.contains("tools/codecraft"),
                "Python source still depends on removed tools backend: {path}"
            );
            python_files.push(path.clone());
        }
        if matches!(
            p.extension().and_then(|x| x.to_str()),
            Some("md" | "py" | "rs" | "c" | "h" | "json" | "yml" | "yaml" | "sh")
        ) {
            ensure!(
                !key.is_match(&fs::read_to_string(&p).unwrap_or_default()),
                "Private-key material in {path}"
            );
        }
        let canonical = path.ends_with("/platform.json")
            || path.ends_with("/target.json")
            || path.starts_with("skills/") && path.ends_with("/contract.json")
            || path.starts_with("tools/skill_packages/packages/") && path.ends_with(".json");
        if canonical {
            let text = fs::read_to_string(&p)?;
            ensure!(
                text == serde_json::to_string_pretty(&read(&p)?)? + "\n",
                "Noncanonical metadata formatting: {path}"
            );
        }
    }
    ensure!(
        fs::read_to_string(root.join("LICENSE"))?.contains("Apache License")
            && !fs::read_to_string(root.join("NOTICE"))?.trim().is_empty(),
        "Missing license/attribution"
    );
    for path in &all {
        if !path.starts_with("platforms/") || !path.contains("/components/") {
            continue;
        }
        let parts: Vec<_> = path.split('/').collect();
        if parts.len() < 5 {
            continue;
        }
        let owner = parts[..4].join("/");
        let attributed = all
            .iter()
            .filter(|p| p.starts_with(&format!("{owner}/")))
            .any(|p| {
                fs::read_to_string(root.join(p)).is_ok_and(|t| {
                    t.contains("Copyright")
                        || t.contains("SPDX-License-Identifier")
                        || t.contains("Apache License")
                })
            });
        ensure!(attributed, "Component attribution missing: {owner}");
    }
    for path in python_files {
        use rustpython_parser::Parse;
        rustpython_parser::ast::Suite::parse(&fs::read_to_string(root.join(&path))?, &path)
            .map_err(|e| anyhow::anyhow!("Python application/test syntax: {e}"))?;
    }
    for entry in [
        "AGENTS.md",
        "CONTRIBUTION.md",
        "README.md",
        "CLAUDE.md",
        "devices/AGENTS.md",
        "skills/AGENTS.md",
        "tools/AGENTS.md",
        "platforms/AGENTS.md",
        "tests/AGENTS.md",
    ] {
        ensure!(
            root.join(entry).is_file(),
            "Missing documentation entry: {entry}"
        );
    }
    documentation(root)?;
    Ok(vec![
        result("source/format-and-python-syntax", "passed", ""),
        result("source/licenses-and-attribution", "passed", ""),
        result("source/private-keys", "passed", ""),
        result("source/rust-tools-only", "passed", ""),
        result("documentation/language-and-links", "passed", ""),
    ])
}
pub fn static_checks(root: &Path) -> Result<J> {
    let c = Catalog::load(root)?;
    let root = c.root.as_path();
    let mut results = source_checks(root)?;
    let packages = crate::maintenance_skills::check_packages(root, None)?;
    results.extend(arr(&packages["results"])?.iter().cloned());
    for id in c.devices.keys() {
        results.push(result(&format!("{id}/device-contract"), "passed", ""));
    }
    for (id, t) in &c.targets {
        crate::platform::validate_target(root, t)?;
        c.graph(t)?;
        results.push(result(&format!("{id}/closure"), "passed", ""));
    }
    let generated = crate::maintenance_generate::generate(&c, true)?;
    ensure!(
        generated["changed"].as_array().is_some_and(Vec::is_empty),
        "Generated files drifted"
    );
    results.push(result("generated/check", "passed", ""));
    let tests = run_tests(root, &test_files(root, None, None)?)?;
    results.extend(arr(&tests["results"])?.iter().cloned());
    Ok(
        json!({"ok":results.iter().all(|r|r["status"]=="passed"),"scope":"static+host","results":results}),
    )
}
pub fn command(root: &Path, args: &[String]) -> Result<J> {
    let mut list = false;
    let mut selected = None;
    let mut platform = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--list" => {
                ensure!(!list, "Duplicate --list");
                list = true;
                i += 1
            }
            "--path" | "--platform" => {
                let key = &args[i];
                let value = args.get(i + 1).context("Missing test selection value")?;
                if key == "--path" {
                    ensure!(
                        selected.replace(value.as_str()).is_none(),
                        "Duplicate --path"
                    )
                } else {
                    ensure!(
                        platform.replace(value.as_str()).is_none(),
                        "Duplicate --platform"
                    )
                };
                i += 2
            }
            _ => bail!("Unknown test option"),
        }
    }
    if list {
        inventory(root, platform, selected)
    } else {
        run_tests(root, &test_files(root, platform, selected)?)
    }
}
