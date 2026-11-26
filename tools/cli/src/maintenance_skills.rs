//! Reviewed standalone package maintenance. Never authenticates or publishes.
use crate::{maintenance_contracts as contracts, util::*};
use anyhow::{bail, ensure, Context, Result};
use serde_json::json;
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};
const SDK: &str = "codecraft-development-recamera-pysdk";
pub const HELPER: &str = "skills/codecraft-capabilities-llm/scripts/device_api.py";
const SDK_RECORD: &str = "tools/skill_packages/recamera/release.json";

pub(crate) fn captured(
    command: &mut Command,
    timeout: std::time::Duration,
) -> Result<std::process::Output> {
    use std::{
        io::{Read, Seek, SeekFrom},
        process::Stdio,
        time::Instant,
    };
    let mut stdout = tempfile::tempfile()?;
    let mut stderr = tempfile::tempfile()?;
    command
        .stdout(Stdio::from(stdout.try_clone()?))
        .stderr(Stdio::from(stderr.try_clone()?));
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command.spawn()?;
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= timeout {
            #[cfg(unix)]
            unsafe {
                libc::kill(-(child.id() as i32), libc::SIGKILL);
            }
            #[cfg(windows)]
            {
                let _ = Command::new("taskkill")
                    .args(["/PID", &child.id().to_string(), "/T", "/F"])
                    .output();
            }
            let _ = child.kill();
            let _ = child.wait();
            bail!(
                "Maintenance subprocess exceeded {} seconds",
                timeout.as_secs_f64()
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    };
    stdout.seek(SeekFrom::Start(0))?;
    stderr.seek(SeekFrom::Start(0))?;
    let mut out = vec![];
    let mut err = vec![];
    stdout.read_to_end(&mut out)?;
    stderr.read_to_end(&mut err)?;
    Ok(std::process::Output {
        status,
        stdout: out,
        stderr: err,
    })
}

pub fn inventory(root: &Path) -> Result<J> {
    ensure!(
        root.is_dir() && !root.is_symlink(),
        "Package directory missing or symlinked"
    );
    let mut out = object();
    for item in walkdir::WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| e.file_name() != "__pycache__")
    {
        let e = item?;
        ensure!(!e.path_is_symlink(), "Package symlink forbidden");
        if !e.file_type().is_file() || e.file_name() == ".DS_Store" {
            continue;
        }
        let rel = e
            .path()
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        ensure!(
            !Path::new(&rel).components().any(|c| {
                let n = c.as_os_str().to_string_lossy();
                (n.starts_with(".env") && n != ".env.example") || n == "secrets" || n == ".secrets"
            }),
            "Private package file"
        );
        if e.path().extension().is_some_and(|s| s == "pyc") {
            continue;
        }
        out[rel] = json!(digest(e.path())?);
    }
    Ok(out)
}
pub fn records(root: &Path, sdk: bool) -> Result<J> {
    let mut out = object();
    if sdk {
        let record = read(&safe(root, SDK_RECORD)?)?;
        ensure!(record["id"] == SDK, "Wrong SDK record");
        out[SDK] = record;
    } else {
        for e in fs::read_dir(safe(root, "tools/skill_packages/packages")?)? {
            let e = e?;
            if e.path().extension().is_some_and(|s| s == "json") {
                let record = read(&e.path())?;
                let id = s(&record["id"])?.to_owned();
                ensure!(
                    record_path(&id, false)?
                        == e.path()
                            .strip_prefix(root)?
                            .to_string_lossy()
                            .replace('\\', "/"),
                    "Record identity/path mismatch"
                );
                ensure!(out.get(&id).is_none(), "Duplicate Skill");
                out[id] = record;
            }
        }
        ensure!(!obj(&out)?.is_empty(), "No Skill packages");
        let owners = fs::read_dir(root.join("skills"))?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().join("contract.json").is_file())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect::<BTreeSet<_>>();
        ensure!(
            owners == obj(&out)?.keys().cloned().collect(),
            "Contract and release owners differ"
        );
    }
    let commit_pattern = regex::Regex::new(r"^[a-f0-9]{40}$")?;
    for (id, r) in obj(&out)? {
        let expected: BTreeSet<&str> = if sdk {
            ["schema_version", "id", "version", "source", "files"]
                .into_iter()
                .collect()
        } else {
            [
                "schema_version",
                "id",
                "version",
                "api_version",
                "contract_version",
                "policy_version",
                "source",
                "files",
            ]
            .into_iter()
            .collect()
        };
        ensure!(
            obj(r)?.keys().map(String::as_str).collect::<BTreeSet<_>>() == expected,
            "Invalid release record fields"
        );
        if !sdk {
            contracts::version(&r["contract_version"])?;
            contracts::version(&r["policy_version"])?;
        }
        ensure!(r["schema_version"] == 1, "Unknown release schema");
        contracts::version(&r["version"])?;
        ensure!(
            obj(&r["files"])?.contains_key("SKILL.md"),
            "Incomplete inventory"
        );
        let src = &r["source"];
        let import = safe(root, s(&src["import_manifest"])?)?;
        ensure!(
            commit_pattern.is_match(s(&src["commit"])?),
            "Invalid import commit"
        );
        if sdk {
            ensure!(
                digest(&import)? == s(&src["import_sha256"])?,
                "Immutable SDK import changed"
            );
            let imported = read(&import)?;
            ensure!(
                imported["installed_skill"] == id.as_str(),
                "SDK import identity mismatch"
            );
            for k in ["repository", "commit", "path"] {
                ensure!(
                    src[k] == imported[format!("source_{k}")],
                    "SDK import provenance mismatch"
                );
            }
        } else {
            ensure!(src["dirty"].is_boolean(), "Original source state missing");
        }
    }
    Ok(out)
}
fn record_path(id: &str, sdk: bool) -> Result<String> {
    ensure!(
        regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")?.is_match(id),
        "Invalid Skill identity"
    );
    Ok(if sdk {
        SDK_RECORD.into()
    } else {
        format!("tools/skill_packages/packages/{id}.json")
    })
}
fn copy(root: &Path, dest: &Path, files: &J) -> Result<()> {
    fs::create_dir_all(dest)?;
    for (name, sha) in obj(files)? {
        let src = safe(root, name)?;
        ensure!(!src.is_symlink(), "Symlink forbidden");
        let rel = Path::new(name);
        ensure!(
            rel.components().all(|x| matches!(x, Component::Normal(_))),
            "Unsafe copy path"
        );
        let target = dest.join(rel);
        fs::create_dir_all(target.parent().unwrap())?;
        fs::write(&target, fs::read(src)?)?;
        ensure!(
            digest(&target)? == s(sha)?,
            "Reviewed file changed during copy"
        );
    }
    Ok(())
}
fn sources(root: &Path, sdk: bool, records: &J) -> Result<J> {
    let mut files = object();
    let mut roots = vec![
        "Cargo.toml",
        "Cargo.lock",
        "tools/cli",
        "tools/skill_packages/release-checks.json",
        "tests/requirements.txt",
        "tests/conftest.py",
        "tests/tools/skill_packages/skill_harness.py",
        "docs/architecture/skill-contracts.md",
    ];
    if sdk {
        roots.extend([
            "tools/skill_packages/recamera/api-notes.json",
            "tools/skill_packages/recamera/api-http-notes.json",
            "tests/skills/codecraft-development-recamera-pysdk",
        ]);
    } else {
        roots.extend([
            "tools/skill_packages/import.json",
            "tools/skill_packages/upstream.json",
            "tests/tools/skill_packages/test_protocol.py",
            "tests/tools/skill_packages/test_onboarding.py",
        ]);
    }
    for path in roots {
        let p = safe(root, path)?;
        if p.is_dir() {
            for (name, h) in obj(&inventory(&p)?)? {
                files[format!("{path}/{name}")] = h.clone();
            }
        } else {
            files[path] = json!(digest(&p)?);
        }
    }
    for (id, r) in obj(records)? {
        let dir = format!("skills/{id}");
        for (name, h) in obj(&inventory(&safe(root, &dir)?)?)? {
            files[format!("{dir}/{name}")] = h.clone();
        }
        let rp = record_path(id, sdk)?;
        files[&rp] = json!(digest(&safe(root, &rp)?)?);
        let import = s(&r["source"]["import_manifest"])?;
        files[import] = json!(digest(&safe(root, import)?)?);
        if !sdk {
            let dir = format!("tests/skills/{id}");
            for (name, h) in obj(&inventory(&safe(root, &dir)?)?)? {
                files[format!("{dir}/{name}")] = h.clone();
            }
        }
    }
    Ok(json!({"files":files,"cli_sha256":digest(&std::env::current_exe()?)?}))
}
fn changes(before: &J, after: &J) -> Result<J> {
    let mut names = BTreeSet::new();
    names.extend(obj(before)?.keys());
    names.extend(obj(after)?.keys());
    Ok(json!(names
        .into_iter()
        .filter(|n| before[*n] != after[*n])
        .map(|n| json!({"path":n,"before":before[n],"after":after[n]}))
        .collect::<Vec<_>>()))
}
fn proposal(previous: &J, proposed: &J, contract: Option<&J>) -> Result<J> {
    let mut expected = previous.clone();
    for k in ["version", "files"] {
        expected[k] = proposed[k].clone();
    }
    if contract.is_some() {
        for k in ["api_version", "contract_version", "policy_version"] {
            expected[k] = proposed[k].clone();
        }
    }
    ensure!(
        expected == *proposed,
        "Release cannot rewrite identity or provenance"
    );
    let diff = changes(&previous["files"], &proposed["files"])?;
    let old = contracts::version(&previous["version"])?;
    let new = contracts::version(&proposed["version"])?;
    ensure!(new >= old, "Package version cannot decrease");
    ensure!(
        arr(&diff)?.is_empty() && previous["api_version"] == proposed["api_version"] || new > old,
        "Changed package/API requires explicit higher version"
    );
    if let Some(c) = contract {
        contracts::validate(c, &proposed["files"])?;
        ensure!(
            contracts::version(&proposed["policy_version"])?
                >= contracts::version(&previous["policy_version"])?,
            "Policy version cannot decrease"
        );
        ensure!(
            proposed["api_version"] == c["api"]["version"]
                && proposed["contract_version"] == c["version"]
                && proposed["policy_version"] == c["policy"]["version"],
            "Independent versions differ"
        );
    }
    Ok(diff)
}
fn external(root: &Path, path: &Path, exists: bool) -> Result<PathBuf> {
    let root = root.canonicalize()?;
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut existing = absolute.as_path();
    let mut suffix = vec![];
    while !existing.exists() {
        suffix.push(existing.file_name().context("Invalid output")?.to_owned());
        existing = existing.parent().context("Invalid output")?;
    }
    let mut canonical = existing.canonicalize()?;
    for name in suffix.iter().rev() {
        ensure!(name != ".." && name != ".", "Noncanonical output");
        canonical.push(name);
    }
    ensure!(
        !canonical.starts_with(root),
        "Candidate must be outside repository"
    );
    ensure!(
        if exists {
            canonical.is_dir()
        } else {
            !canonical.exists()
        },
        "Candidate must be {} directory",
        if exists { "an existing" } else { "a new" }
    );
    Ok(canonical)
}
pub fn prepare(root: &Path, output: &Path, versions: &J, sdk: bool) -> Result<J> {
    let output = external(root, output, false)?;
    let records = records(root, sdk)?;
    ensure!(
        obj(versions)?.keys().all(|id| records.get(id).is_some()),
        "Unknown Skill in versions"
    );
    let source = sources(root, sdk, &records)?;
    let mut packages = object();
    let mut all = object();
    let mut baseline = object();
    fs::create_dir_all(&output)?;
    let result = (|| -> Result<J> {
        for (id, previous) in obj(&records)? {
            let path = format!("skills/{id}");
            let source_dir = safe(root, &path)?;
            let inv = inventory(&source_dir)?;
            baseline[id] = inv.clone();
            let target = output.join(&path);
            copy(&source_dir, &target, &inv)?;
            if !sdk {
                let helper = "scripts/device_api.py";
                let maintained = digest(&safe(root, HELPER)?)?;
                ensure!(
                    inv[helper] == previous["files"][helper] || inv[helper] == maintained,
                    "Conflicting distribution helper edit"
                );
                fs::copy(safe(root, HELPER)?, target.join(helper))?;
            }
            let mut proposed = previous.clone();
            proposed["files"] = inventory(&target)?;
            if let Some(overrides) = versions.get(id) {
                for (k, v) in obj(overrides)? {
                    let dest = match k.as_str() {
                        "skill_version" => "version",
                        "api_version" | "contract_version" | "policy_version" if !sdk => k,
                        _ => bail!("Unknown version override"),
                    };
                    proposed[dest] = v.clone();
                }
            }
            let contract = if sdk {
                None
            } else {
                Some(read(&target.join("contract.json"))?)
            };
            let diff = proposal(previous, &proposed, contract.as_ref())?;
            let classification = if arr(&diff)?.is_empty() {
                "none"
            } else if arr(&diff)?
                .iter()
                .all(|v| s(&v["path"]).is_ok_and(|s| s.ends_with(".md")))
            {
                "documentation"
            } else {
                "runtime"
            };
            let record = record_path(id, sdk)?;
            write(&output.join(&record), &proposed)?;
            all[&record] = json!(digest(&output.join(&record))?);
            for (name, h) in obj(&proposed["files"])? {
                all[format!("{path}/{name}")] = h.clone();
            }
            packages[id] = json!({"record_path":record,"skill_path":path,"previous":previous,"proposed":proposed,"changes":diff,"classification":classification});
        }
        let manifest = json!({"schema_version":3,"kind":"skill-release-candidate","selection":if sdk{SDK}else{"cloud"},"sources":source,"baseline_packages":baseline,"packages":packages,"files":all});
        write(&output.join("manifest.json"), &manifest)?;
        Ok(
            json!({"path":output,"review_sha256":identity(&manifest),"packages":packages,"repository_changed":false,"api_execution":"not_run","published":false}),
        )
    })();
    if result.is_err() {
        fs::remove_dir_all(&output)?;
    }
    result
}
pub fn inspect(
    root: &Path,
    candidate: &Path,
    reviewed: Option<&str>,
    sdk: bool,
) -> Result<(J, String)> {
    let candidate = external(root, candidate, true)?;
    let m = read(&candidate.join("manifest.json"))?;
    ensure!(
        obj(&m)?.keys().map(String::as_str).collect::<BTreeSet<_>>()
            == BTreeSet::from([
                "schema_version",
                "kind",
                "selection",
                "sources",
                "baseline_packages",
                "packages",
                "files"
            ]),
        "Invalid candidate fields"
    );
    ensure!(
        m["schema_version"] == 3
            && m["kind"] == "skill-release-candidate"
            && m["selection"] == if sdk { SDK } else { "cloud" },
        "Invalid candidate schema/selection"
    );
    let sha = identity(&m);
    if let Some(r) = reviewed {
        ensure!(sha == r, "Candidate differs from reviewed digest");
    }
    let mut expected = m["files"].clone();
    expected["manifest.json"] = json!(digest(&candidate.join("manifest.json"))?);
    ensure!(
        inventory(&candidate)? == expected,
        "Candidate files drifted"
    );
    let records = records(root, sdk)?;
    ensure!(
        sources(root, sdk, &records)? == m["sources"],
        "Maintained sources changed after preparation"
    );
    ensure!(
        obj(&records)?.keys().eq(obj(&m["packages"])?.keys()),
        "Candidate package set changed"
    );
    let mut all = object();
    for (id, old) in obj(&records)? {
        let e = &m["packages"][id];
        let path = format!("skills/{id}");
        let rp = record_path(id, sdk)?;
        ensure!(
            e["skill_path"] == path && e["record_path"] == rp && e["previous"] == *old,
            "Candidate identity changed"
        );
        ensure!(
            inventory(&safe(root, &path)?)? == m["baseline_packages"][id],
            "Source inventory changed"
        );
        let dir = safe(&candidate, &path)?;
        let inv = inventory(&dir)?;
        let proposed = &e["proposed"];
        ensure!(
            inv == proposed["files"] && read(&safe(&candidate, &rp)?)? == *proposed,
            "Candidate fingerprints differ"
        );
        let contract = if sdk {
            None
        } else {
            Some(read(&dir.join("contract.json"))?)
        };
        let diff = proposal(old, proposed, contract.as_ref())?;
        ensure!(diff == e["changes"], "Candidate change summary differs");
        let kind = if arr(&diff)?.is_empty() {
            "none"
        } else if arr(&diff)?
            .iter()
            .all(|v| s(&v["path"]).is_ok_and(|s| s.ends_with(".md")))
        {
            "documentation"
        } else {
            "runtime"
        };
        ensure!(
            e["classification"] == kind,
            "Candidate classification differs"
        );
        all[&rp] = json!(digest(&candidate.join(&rp))?);
        for (name, h) in obj(&inv)? {
            all[format!("{path}/{name}")] = h.clone();
        }
    }
    ensure!(m["files"] == all, "Unexpected candidate file mapping");
    Ok((m, sha))
}
pub fn check_packages(root: &Path, installed: Option<&Path>) -> Result<J> {
    let records = records(root, false)?;
    let mut results = vec![];
    for (id, r) in obj(&records)? {
        let (_, c) = crate::skills::package(root, id)?;
        let dir = installed.unwrap_or(&root.join("skills")).join(id);
        ensure!(
            inventory(&dir)? == r["files"],
            "Installed package differs: {id}"
        );
        let tmp = tempfile::tempdir()?;
        copy(&dir, tmp.path(), &r["files"])?;
        for mapping in obj(&c["client"]["operations"])?.values() {
            let mut cmd = Command::new(
                std::env::var("CODECRAFT_TEST_PYTHON")
                    .or_else(|_| std::env::var("PYTHON"))
                    .unwrap_or("python3".into()),
            );
            cmd.args(["-E", "-s", "-B"])
                .arg(tmp.path().join(s(&mapping["entrypoint"])?))
                .args(strings(&mapping["command"])?)
                .arg("--help")
                .current_dir(tmp.path())
                .env_remove("PYTHONPATH")
                .env_remove("CODECRAFT_API_ORIGIN")
                .env_remove("CODECRAFT_DEVICE_FILE");
            let output = captured(&mut cmd, std::time::Duration::from_secs(30))?;
            ensure!(output.status.success(), "Isolated CLI help failed: {id}");
            let stdout = String::from_utf8_lossy(&output.stdout);
            for flag in obj(&mapping["arguments"])?
                .values()
                .chain(obj(&mapping["options"])?.values())
                .chain(arr(&mapping["fixed_arguments"])?.iter())
            {
                ensure!(stdout.contains(s(flag)?), "CLI flag missing");
            }
        }
        results.push(json!({"name":id,"status":"passed","source_digest":identity(&r["files"]),"api_execution":"not_run"}));
    }
    Ok(json!({"status":"passed","results":results}))
}
pub fn validate(root: &Path, candidate: &Path, reviewed: Option<&str>, sdk: bool) -> Result<J> {
    let (m, sha) = inspect(root, candidate, reviewed, sdk)?;
    let staged = tempfile::tempdir()?;
    copy(root, staged.path(), &m["sources"]["files"])?;
    for e in obj(&m["packages"])?.values() {
        let dest = staged.path().join(s(&e["skill_path"])?);
        if dest.exists() {
            fs::remove_dir_all(dest)?;
        }
    }
    copy(candidate, staged.path(), &m["files"])?;
    let mut paths = vec![];
    if sdk {
        ensure!(
            inventory(&staged.path().join(format!("skills/{SDK}")))?
                == m["packages"][SDK]["proposed"]["files"],
            "SDK inventory differs"
        );
        paths.push(format!("tests/skills/{SDK}"));
    } else {
        check_packages(staged.path(), None)?;
        paths.extend([
            "tests/tools/skill_packages/test_protocol.py".into(),
            "tests/tools/skill_packages/test_onboarding.py".into(),
        ]);
        let coverage = read(
            &staged
                .path()
                .join("tools/skill_packages/release-checks.json"),
        )?;
        ensure!(
            obj(&coverage)?.keys().eq(obj(&m["packages"])?.keys()),
            "Release coverage missing Skills"
        );
        for (id, c) in obj(&coverage)? {
            let (_, contract) = crate::skills::package(staged.path(), id)?;
            ensure!(
                strings(&c["operations"])?
                    .into_iter()
                    .collect::<BTreeSet<_>>()
                    == obj(&contract["operations"])?.keys().cloned().collect(),
                "Release operation coverage mismatch"
            );
            let tests = strings(&c["tests"])?;
            ensure!(
                !tests.is_empty() && tests.iter().collect::<BTreeSet<_>>().len() == tests.len(),
                "Missing/duplicate registered tests"
            );
            for t in tests {
                paths.push(format!("{}::{t}", s(&c["path"])?));
            }
            for p in c
                .get("suites")
                .map(strings)
                .transpose()?
                .unwrap_or_default()
            {
                ensure!(
                    p.starts_with(&format!("tests/skills/{id}/")),
                    "Wrong suite owner"
                );
                paths.push(p);
            }
        }
    }
    let checked = crate::maintenance_checks::run_tests(staged.path(), &paths)?;
    let rows = arr(&checked["results"])?;
    ensure!(
        !rows.is_empty() && rows.iter().all(|r| r["status"] == "passed"),
        "Candidate conformance incomplete or failed: {checked}"
    );
    inspect(root, candidate, Some(&sha), sdk)?;
    Ok(
        json!({"status":"passed","review_sha256":sha,"checks":rows,"repository_changed":false,"api_execution":"not_run","published":false}),
    )
}
fn install_transaction(
    root: &Path,
    candidate: &Path,
    m: &J,
    sdk: bool,
    rename: &mut impl FnMut(&Path, &Path) -> std::io::Result<()>,
) -> Result<Vec<String>> {
    let stage = tempfile::Builder::new()
        .prefix(".release-")
        .tempdir_in(root.join("skills"))?
        .keep();
    let mut moved = vec![];
    let mut written: Vec<(PathBuf, Vec<u8>)> = vec![];
    let mut changed = vec![];
    let transaction = (|| -> Result<()> {
        for (id, e) in obj(&m["packages"])? {
            let path = s(&e["skill_path"])?;
            let dest = root.join(path);
            if m["baseline_packages"][id] != e["proposed"]["files"] {
                let next = stage.join(format!("new-{id}"));
                copy(&candidate.join(path), &next, &e["proposed"]["files"])?;
                let backup = stage.join(format!("old-{id}"));
                rename(&dest, &backup)?;
                moved.push((dest.clone(), backup));
                rename(&next, &dest)?;
                changed.push(path.to_string());
            }
            if e["previous"] != e["proposed"] {
                let rp = s(&e["record_path"])?;
                let path = root.join(rp);
                written.push((path.clone(), fs::read(&path)?));
                let next = stage.join(format!("{id}.json"));
                fs::copy(safe(candidate, rp)?, &next)?;
                ensure!(
                    digest(&next)? == s(&m["files"][rp])?,
                    "Candidate record drift"
                );
                rename(&next, &path)?;
                changed.push(rp.to_owned());
            }
        }
        for (id, e) in obj(&m["packages"])? {
            ensure!(
                inventory(&root.join(format!("skills/{id}")))? == e["proposed"]["files"],
                "Applied package drift"
            );
        }
        records(root, sdk)?;
        Ok(())
    })();
    if let Err(error) = transaction {
        let rollback = (|| -> Result<()> {
            for (path, bytes) in written.iter().rev() {
                fs::write(path, bytes)?;
            }
            for (dest, backup) in moved.iter().rev() {
                if dest.exists() {
                    fs::remove_dir_all(dest)?;
                }
                rename(backup, dest)?;
            }
            Ok(())
        })();
        if rollback.is_err() {
            bail!(
                "Rollback incomplete; recovery files retained at {}: {error}",
                stage.display()
            );
        }
        fs::remove_dir_all(&stage)?;
        return Err(error);
    }
    fs::remove_dir_all(stage)?;
    Ok(changed)
}
pub fn apply(root: &Path, candidate: &Path, reviewed: &str, sdk: bool) -> Result<J> {
    ensure!(
        regex::Regex::new(r"^[a-f0-9]{64}$")?.is_match(reviewed),
        "Explicit reviewed digest required"
    );
    fs::create_dir_all(root.join(".codecraft"))?;
    let lock = root.join(".codecraft/skill-release.lock");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock)
        .context("Another release owns skill-release.lock")?;
    use std::io::Write;
    writeln!(file, "{reviewed}")?;
    let result = (|| -> Result<J> {
        let mut checked = validate(root, candidate, Some(reviewed), sdk)?;
        let (m, _) = inspect(root, candidate, Some(reviewed), sdk)?;
        let changed = install_transaction(root, candidate, &m, sdk, &mut |a, b| fs::rename(a, b))?;
        checked["repository_changed"] = json!(!changed.is_empty());
        checked["changed"] = json!(changed);
        Ok(checked)
    })();
    let unlock = fs::remove_file(lock);
    match result {
        Err(error) => Err(error),
        Ok(value) => {
            unlock?;
            Ok(value)
        }
    }
}
fn option(args: &[String], name: &str) -> Result<Option<String>> {
    let mut found = None;
    for (i, a) in args.iter().enumerate() {
        if a == name {
            ensure!(found.is_none(), "Duplicate option {name}");
            found = Some(
                args.get(i + 1)
                    .filter(|s| !s.starts_with("--"))
                    .context("Missing option value")?
                    .clone(),
            );
        }
    }
    Ok(found)
}
fn validate_args(args: &[String]) -> Result<()> {
    let command = args.first().context("Command required")?.as_str();
    let (start, allowed): (usize, &[&str]) = match command {
        "check-cloud-skills" => (1, &["--skills-root"]),
        "skill-release" => match args.get(1).map(String::as_str) {
            Some("prepare") => (2, &["--output", "--versions", "--skill"]),
            Some("check") => (2, &["--candidate", "--skill"]),
            Some("apply") => (2, &["--candidate", "--review-sha256", "--skill"]),
            _ => bail!("Unknown release action"),
        },
        "skill-contract" => match args.get(1).map(String::as_str) {
            Some("package") => (2, &["--skill"]),
            Some("payload") => (2, &["--input"]),
            _ => bail!("Unknown contract action"),
        },
        _ => bail!("Unknown Skill command"),
    };
    let mut seen = BTreeSet::new();
    let mut i = start;
    while i < args.len() {
        let key = args[i].as_str();
        ensure!(
            allowed.contains(&key) && seen.insert(key),
            "Unknown or duplicate option: {key}"
        );
        ensure!(
            args.get(i + 1).is_some_and(|v| !v.starts_with("--")),
            "Missing value for {key}"
        );
        i += 2;
    }
    Ok(())
}
pub fn command(root: &Path, args: &[String]) -> Result<J> {
    validate_args(args)?;
    let name = args.first().context("Skill maintenance command required")?;
    match name.as_str() {
        "skill-release" => {
            let action = args.get(1).context("Release action required")?;
            let skill = option(args, "--skill")?;
            ensure!(
                skill.is_none() || skill.as_deref() == Some(SDK),
                "Unknown Skill selection"
            );
            let sdk = skill.is_some();
            match action.as_str() {
                "prepare" => prepare(
                    root,
                    Path::new(&option(args, "--output")?.context("--output required")?),
                    &option(args, "--versions")?
                        .map(|p| read(Path::new(&p)))
                        .transpose()?
                        .unwrap_or(json!({})),
                    sdk,
                ),
                "check" => validate(
                    root,
                    Path::new(&option(args, "--candidate")?.context("--candidate required")?),
                    None,
                    sdk,
                ),
                "apply" => apply(
                    root,
                    Path::new(&option(args, "--candidate")?.context("--candidate required")?),
                    &option(args, "--review-sha256")?.context("--review-sha256 required")?,
                    sdk,
                ),
                _ => bail!("Unknown release action"),
            }
        }
        "check-cloud-skills" => check_packages(
            root,
            option(args, "--skills-root")?.as_deref().map(Path::new),
        ),
        "skill-contract" => match args.get(1).map(String::as_str) {
            Some("package") => {
                let id = option(args, "--skill")?.context("--skill required")?;
                let (r, c) = crate::skills::package(root, &id)?;
                Ok(json!({"record":r,"contract":c,"effective":contracts::policy(&c,&r["files"])?}))
            }
            Some("payload") => {
                let file = option(args, "--input")?.context("--input required")?;
                let input = read(Path::new(&file))?;
                crate::skills::payload(&input["schema"], &input["value"])?;
                Ok(json!({"status":"passed"}))
            }
            _ => bail!("Unknown contract command"),
        },
        _ => bail!("Unknown Skill maintenance command"),
    }
}

#[cfg(test)]
#[path = "../../../tests/tools/cli/skill_transaction.rs"]
mod transaction_tests;
