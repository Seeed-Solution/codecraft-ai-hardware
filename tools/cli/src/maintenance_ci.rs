//! Committed-tree CI planning. Historical source is read as data, never executed.
use crate::util::*;
use anyhow::{bail, ensure, Context, Result};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    ensure!(
        o.status.success(),
        "Git CI input failed: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    Ok(o.stdout)
}
fn revision(root: &Path, v: &str) -> Result<String> {
    ensure!(!v.is_empty() && !v.starts_with('-'), "Invalid Git revision");
    Ok(String::from_utf8(git(
        root,
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{v}^{{commit}}"),
        ],
    )?)?
    .trim()
    .into())
}
struct Tree(PathBuf);
impl Drop for Tree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn relative(v: &str) -> Result<()> {
    ensure!(
        !v.is_empty()
            && !Path::new(v).is_absolute()
            && Path::new(v)
                .components()
                .all(|x| matches!(x, std::path::Component::Normal(_))),
        "Unsafe repository path: {v}"
    );
    Ok(())
}
fn snapshot(root: &Path, commit: &str) -> Result<Tree> {
    let tree = Tree(std::env::temp_dir().join(format!(
        "codecraft-ci-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    )));
    fs::create_dir(&tree.0)?;
    let listing = git(root, &["ls-tree", "-rz", "--full-tree", commit])?;
    let mut entries = vec![];
    for row in listing.split(|b| *b == 0).filter(|x| !x.is_empty()) {
        let t = std::str::from_utf8(row)?;
        let (meta, name) = t.split_once('\t').context("Invalid Git tree")?;
        relative(name)?;
        let fields: Vec<_> = meta.split_whitespace().collect();
        ensure!(
            fields.len() == 3 && fields[1] == "blob",
            "Unsupported Git tree entry: {name}"
        );
        entries.push((fields[0].to_owned(), fields[2].to_owned(), name.to_owned()));
    }
    let mut cmd = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let requests = entries
        .iter()
        .map(|x| format!("{}\n", x.1))
        .collect::<String>();
    let mut stdin = cmd.stdin.take().unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(requests.as_bytes()));
    let output = cmd.wait_with_output()?;
    writer
        .join()
        .map_err(|_| anyhow::anyhow!("Git input writer failed"))??;
    ensure!(output.status.success(), "Git blob read failed");
    let mut bytes = output.stdout.as_slice();
    let mut links = vec![];
    for (mode, _, name) in entries {
        let e = bytes
            .iter()
            .position(|b| *b == b'\n')
            .context("Truncated Git header")?;
        let header = std::str::from_utf8(&bytes[..e])?;
        let n: usize = header
            .split_whitespace()
            .nth(2)
            .context("Missing blob size")?
            .parse()?;
        bytes = &bytes[e + 1..];
        ensure!(bytes.len() > n, "Truncated Git blob");
        let data = &bytes[..n];
        let path = tree.0.join(&name);
        fs::create_dir_all(path.parent().unwrap())?;
        if mode == "120000" {
            links.push((path, String::from_utf8(data.to_vec())?));
        } else {
            ensure!(
                mode == "100644" || mode == "100755",
                "Unsupported file mode"
            );
            fs::write(&path, data)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(
                    path,
                    fs::Permissions::from_mode(if mode == "100755" { 0o755 } else { 0o644 }),
                )?;
            }
        }
        bytes = &bytes[n + 1..];
    }
    for (p, target) in &links {
        ensure!(!Path::new(target).is_absolute(), "External Git link");
        #[cfg(unix)]
        std::os::unix::fs::symlink(target, p)?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(target, p)?;
    }
    let canonical = tree.0.canonicalize()?;
    for (p, _) in links {
        ensure!(
            p.canonicalize()?.starts_with(&canonical),
            "External Git link"
        );
    }
    Ok(tree)
}
pub struct Inputs {
    pub targets: BTreeMap<String, J>,
    pub platforms: BTreeMap<String, J>,
    files: BTreeMap<String, BTreeSet<String>>,
    directories: BTreeMap<String, BTreeSet<String>>,
}
pub fn native_roots(t: &J, p: &J) -> Result<Vec<String>> {
    let mut roots = vec![
        format!("{}/target.json", s(&t["path"])?),
        format!("platforms/{}/platform.json", s(&p["id"])?),
    ];
    // Historical manifests retain adapter/runtime inputs; current manifests declare their actual pipeline.
    if let Some(adapter) = p.get("adapter") {
        roots.push(s(adapter)?.into());
    }
    if let Some(runtime) = p.get("runtime") {
        roots.extend(strings(runtime)?);
    }
    for key in ["inputs", "dependencies"] {
        roots.extend(strings(&t[key])?);
    }
    roots.push(s(&t["toolchain"]["source"])?.into());
    for key in ["example_inputs", "example_dependencies"] {
        for values in obj(&t[key])?.values() {
            roots.extend(strings(values)?);
        }
    }
    for v in obj(&t["variants"])?.values() {
        for key in ["inputs", "dependencies"] {
            roots.extend(strings(&v[key])?);
        }
    }
    for i in arr(&t["interfaces"])? {
        roots.extend(strings(&i["implementation"])?);
        if let Some(e) = i.get("example_implementation") {
            for paths in obj(e)?.values() {
                roots.extend(strings(paths)?);
            }
        }
    }
    Ok(roots)
}
impl Inputs {
    pub fn load(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        let mut this = Self {
            targets: BTreeMap::new(),
            platforms: BTreeMap::new(),
            files: BTreeMap::new(),
            directories: BTreeMap::new(),
        };
        for entry in fs::read_dir(root.join("platforms"))? {
            let p = entry?.path().join("platform.json");
            if p.is_file() {
                let v = read(&p)?;
                // CI compares historical data-only manifests with current manifests;
                // it never imports their adapter or schema implementation.
                ensure!(
                    v["schema_version"] == 2 || v["schema_version"] == 3,
                    "Unsupported Platform snapshot schema"
                );
                let id = s(&v["id"])?.to_owned();
                ensure!(this.platforms.insert(id, v).is_none(), "Duplicate Platform");
            }
        }
        for entry in fs::read_dir(root.join("devices"))? {
            let p = entry?.path().join("targets/target.json");
            if !p.is_file() {
                continue;
            }
            let t = read(&p)?;
            let id = s(&t["id"])?.to_owned();
            let device = s(&t["device"])?;
            let stack = s(&t["platform"])?;
            ensure!(
                t["schema_version"] == 3
                    && id == format!("{stack}/{device}")
                    && t["path"] == format!("devices/{device}/targets"),
                "Target identity/schema mismatch"
            );
            let platform = this.platforms.get(stack).context("Unknown Platform")?;
            let mut graph = Graph::new(&root);
            graph.resolve(native_roots(&t, platform)?)?;
            let mut files: BTreeSet<_> = graph.files.keys().cloned().collect();
            let mut dirs = BTreeSet::new();
            for name in graph.edges.keys() {
                let p = root.join(name);
                let resolved = p
                    .canonicalize()?
                    .strip_prefix(&root)?
                    .to_string_lossy()
                    .replace('\\', "/");
                if p.is_dir() {
                    dirs.insert(name.clone());
                    dirs.insert(resolved);
                } else {
                    files.insert(resolved);
                }
            }
            this.files.insert(id.clone(), files);
            this.directories.insert(id.clone(), dirs);
            ensure!(this.targets.insert(id, t).is_none(), "Duplicate Target");
        }
        ensure!(
            !this.targets.is_empty() && !this.platforms.is_empty(),
            "Empty Target/Platform inventory"
        );
        Ok(this)
    }
    fn owners(&self, path: &str) -> BTreeSet<String> {
        if matches!(path, "Cargo.toml" | "Cargo.lock" | "tools/cli/Cargo.toml")
            || path.starts_with(".cargo/")
            || path.starts_with("tools/cli/src/")
        {
            return self.targets.keys().cloned().collect();
        }
        if path.starts_with("platforms/") && path.ends_with(".rs") {
            let stack = path.split('/').nth(1).unwrap();
            return self
                .targets
                .iter()
                .filter(|(_, t)| t["platform"] == stack)
                .map(|(id, _)| id.clone())
                .collect();
        }
        self.targets
            .keys()
            .filter(|id| {
                self.files[*id].contains(path)
                    || self.directories[*id]
                        .iter()
                        .any(|d| path.starts_with(&format!("{d}/")))
            })
            .cloned()
            .collect()
    }
    fn row(&self, id: &str) -> J {
        let t = &self.targets[id];
        let stack = t["platform"].as_str().unwrap();
        json!({"id":id,"device":t["device"],"platform":stack,"toolchain":t["toolchain"].get("version").cloned().unwrap_or(json!("unknown")),"chip":t["toolchain"].get("target").cloned().unwrap_or(json!("")),"runner":self.platforms[stack]["ci"]["runner"]})
    }
}
fn host_only(path: &str, declared: bool) -> Option<&'static str> {
    let p = Path::new(path);
    let name = p.file_name()?.to_str()?;
    let doc = matches!(
        name,
        "README.md" | "README.zh_CN.md" | "README.rst" | "AGENTS.md" | "CLAUDE.md" | "SKILL.md"
    ) || path.starts_with("docs/");
    if ((path.ends_with(".md") || path.ends_with(".rst")) && (doc || !declared))
        || matches!(name, "LICENSE" | "NOTICE")
    {
        Some("documentation or attribution; static checks remain required")
    } else if path.starts_with("tests/") {
        Some("host tests or retained evidence")
    } else if path.starts_with("tools/cli/src/maintenance_") {
        Some("repository-only maintenance implementation")
    } else {
        None
    }
}
pub fn selection(
    before: Option<&Inputs>,
    after: &Inputs,
    changed: &[String],
    full: Option<&str>,
) -> Result<J> {
    let mut selected: BTreeMap<String, Vec<J>> =
        after.targets.keys().map(|x| (x.clone(), vec![])).collect();
    let mut changes = vec![];
    if let Some(reason) = full {
        for reasons in selected.values_mut() {
            reasons.push(json!({"path":null,"reason":reason}));
        }
    } else {
        for path in changed.iter().collect::<BTreeSet<_>>() {
            relative(path)?;
            let mut dependencies = after.owners(path);
            if let Some(b) = before {
                dependencies.extend(b.owners(path));
            }
            let mut owners = BTreeSet::new();
            let mut scope = "static";
            let reason = if let Some(reason) = host_only(path, !dependencies.is_empty()) {
                reason
            } else if !dependencies.is_empty() {
                owners = dependencies;
                scope = "native";
                "declared native input or pipeline dependency"
            } else if ["skills/", "tools/skill_packages/", "docs/", "tests/"]
                .iter()
                .any(|p| path.starts_with(p))
                || path == ".github/target-matrix.json"
                || path.starts_with(".github/ISSUE_TEMPLATE/")
            {
                "independent Skill, maintenance, documentation or host-test source"
            } else {
                for inv in before.into_iter().chain(std::iter::once(after)) {
                    for (id, t) in &inv.targets {
                        if path.starts_with(&format!("{}/", s(&t["path"])?))
                            || path.starts_with(&format!("platforms/{}/", s(&t["platform"])?))
                        {
                            owners.insert(id.clone());
                        }
                    }
                }
                scope = "conservative";
                if owners.is_empty() {
                    owners = after.targets.keys().cloned().collect();
                    "unclassified infrastructure change; conservatively select all Targets"
                } else {
                    "undeclared native-area change; conservatively select its owner"
                }
            };
            let current: Vec<_> = owners
                .iter()
                .filter(|id| after.targets.contains_key(*id))
                .cloned()
                .collect();
            changes.push(json!({"path":path,"scope":scope,"reason":reason,"targets":current}));
            for id in current {
                selected
                    .get_mut(&id)
                    .unwrap()
                    .push(json!({"path":path,"reason":reason}));
            }
        }
        if let Some(b) = before {
            for (id, t) in &after.targets {
                if !b.targets.contains_key(id) {
                    selected.get_mut(id).unwrap().push(json!({"path":format!("{}/target.json",s(&t["path"])?),"reason":"new Target"}));
                }
            }
        }
    }
    let mut matrices: BTreeMap<String, J> = after
        .platforms
        .keys()
        .map(|p| (p.clone(), json!({"include":[]})))
        .collect();
    let mut targets = BTreeMap::new();
    for (id, reasons) in selected {
        let yes = !reasons.is_empty();
        if yes {
            matrices
                .get_mut(s(&after.targets[&id]["platform"])?)
                .unwrap()["include"]
                .as_array_mut()
                .unwrap()
                .push(after.row(&id));
        }
        targets.insert(id,json!({"selected":yes,"reasons":if yes{reasons}else{vec![json!({"path":null,"reason":"no changed native input or build behavior identified"})]}}));
    }
    let counts: BTreeMap<_, _> = matrices
        .iter()
        .map(|(id, m)| (id.clone(), m["include"].as_array().unwrap().len()))
        .collect();
    let removed: Vec<_> = before
        .map(|b| {
            b.targets
                .keys()
                .filter(|id| !after.targets.contains_key(*id))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    Ok(
        json!({"schema_version":1,"static":"required","targets":targets,"matrices":matrices,"counts":counts,"removed_targets":removed,"changes":changes}),
    )
}
fn ancestor(root: &Path, parent: &str, head: &str) -> Result<bool> {
    Ok(Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["merge-base", "--is-ancestor", parent, head])
        .status()?
        .success())
}
pub fn plan(
    root: &Path,
    base: Option<&str>,
    head: &str,
    full: bool,
    event: Option<&J>,
    event_name: Option<&str>,
) -> Result<J> {
    ensure!(
        !root.join("project.yaml").exists(),
        "CI planning requires reference repository"
    );
    ensure!(
        Path::new(String::from_utf8(git(root, &["rev-parse", "--show-toplevel"])?)?.trim())
            .canonicalize()?
            == root.canonicalize()?,
        "CI root must be Git root"
    );
    let head = revision(root, head)?;
    let mut base = base.map(str::to_owned);
    let mut reason = full.then(|| "explicit full validation".to_owned());
    if let Some(e) = event {
        ensure!(
            base.is_none() && !full,
            "Choose event or explicit base/full"
        );
        ensure!(
            head == revision(root, "HEAD")?,
            "Event planning must use checked-out revision"
        );
        match event_name {
            Some("pull_request") => {
                let b = revision(root, s(&e["pull_request"]["base"]["sha"])?)?;
                let source = revision(root, s(&e["pull_request"]["head"]["sha"])?)?;
                ensure!(
                    ancestor(root, &b, &head)? && ancestor(root, &source, &head)?,
                    "PR planning requires tested merge tree with both parents"
                );
                base = Some(b)
            }
            Some("push") => {
                ensure!(
                    revision(root, s(&e["after"])?)? == head,
                    "Push event revision mismatch"
                );
                let previous = s(&e["before"])?;
                ensure!(
                    (previous.len() == 40 || previous.len() == 64)
                        && previous
                            .bytes()
                            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
                    "Invalid push baseline"
                );
                base = if previous.bytes().all(|b| b == b'0') {
                    None
                } else {
                    revision(root, previous).ok()
                };
                if base.is_none() {
                    reason = Some(
                        "push baseline unavailable; conservatively validate all Targets".into(),
                    )
                }
            }
            Some("workflow_dispatch" | "schedule") => {
                reason = Some("manual or scheduled full validation".into())
            }
            _ => bail!("Unsupported CI event"),
        }
    }
    ensure!(
        base.is_some() || reason.is_some(),
        "Provide --base, --full or event"
    );
    let base = base.map(|b| revision(root, &b)).transpose()?;
    let after_tree = snapshot(root, &head)?;
    let after = Inputs::load(&after_tree.0)?;
    let mut result = if let Some(reason) = reason.as_deref() {
        selection(None, &after, &[], Some(reason))?
    } else {
        let b = base.as_deref().unwrap();
        let before_tree = snapshot(root, b)?;
        let before = Inputs::load(&before_tree.0)?;
        let changed = git(
            root,
            &["diff", "--name-only", "--no-renames", "-z", b, &head, "--"],
        )?;
        let changed = changed
            .split(|b| *b == 0)
            .filter(|s| !s.is_empty())
            .map(|b| String::from_utf8(b.to_vec()))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        selection(Some(&before), &after, &changed, None)?
    };
    result["base"] = json!(base);
    result["head"] = json!(head);
    result["mode"] = json!(if reason.is_some() { "full" } else { "affected" });
    result["reason"] =
        json!(reason
            .unwrap_or_else(|| "compare native dependencies at both committed revisions".into()));
    Ok(result)
}
fn row(name: &str, status: &str, reason: String) -> J {
    json!({"name":name,"status":status,"reason":reason})
}
pub fn check(needs: &J, expected_head: Option<&str>) -> Result<J> {
    let needs = obj(needs)?;
    let mut results = vec![];
    for job in ["cli", "contracts"] {
        let v = needs.get(job).context("Missing required CI job")?;
        let passed = v["result"] == "success";
        results.push(row(
            &format!("ci/{job}"),
            if passed { "passed" } else { "failed" },
            if passed {
                String::new()
            } else {
                format!("Required job: {}", v["result"])
            },
        ));
    }
    if needs["contracts"]["result"] != "success" {
        return Ok(json!({"results":results,"ok":false,"device_verification":"not_run"}));
    }
    let outputs = &needs["contracts"]["outputs"];
    let data = parse(s(&outputs["plan"])?)?;
    ensure!(data["schema_version"] == 1, "Invalid CI plan schema");
    let counts = obj(&data["counts"])?;
    let matrices = obj(&data["matrices"])?;
    ensure!(
        !counts.is_empty() && counts.keys().eq(matrices.keys()),
        "CI matrix/count mismatch"
    );
    let head = s(&data["head"])?;
    ensure!(
        (head.len() == 40 || head.len() == 64)
            && head
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
        "Invalid CI head"
    );
    ensure!(expected_head.is_none_or(|h| h == head), "Stale CI plan");
    let required: BTreeSet<_> = counts
        .keys()
        .map(String::as_str)
        .chain(["cli", "contracts"])
        .collect();
    ensure!(
        needs.keys().map(String::as_str).collect::<BTreeSet<_>>() == required,
        "Missing or unexpected CI platform job"
    );
    let mut ids = BTreeSet::new();
    for (pid, count) in counts {
        let n = count.as_u64().context("Invalid CI count")?;
        let rows = arr(&matrices[pid]["include"])?;
        ensure!(n as usize == rows.len(), "CI matrix/count mismatch");
        let key = pid.replace('-', "_");
        let count_text = n.to_string();
        ensure!(
            outputs[format!("{key}_count")].as_str() == Some(count_text.as_str())
                && parse(s(&outputs[&key])?)? == matrices[pid],
            "CI job output/matrix mismatch"
        );
        for r in rows {
            ensure!(
                r["platform"] == *pid && ids.insert(s(&r["id"])?.to_owned()),
                "Invalid/duplicate selected Target"
            );
        }
        let expected = if n == 0 { "skipped" } else { "success" };
        let actual = &needs[pid]["result"];
        let status = if actual != expected {
            "failed"
        } else if n == 0 {
            "skipped"
        } else {
            "passed"
        };
        results.push(row(
            &format!("ci/{pid}"),
            status,
            format!("{n} selected Targets require {expected}; got {actual}"),
        ));
    }
    Ok(
        json!({"ok":results.iter().all(|r|r["status"]!="failed"),"results":results,"device_verification":"not_run"}),
    )
}
fn append(path: &str, text: &str) -> Result<()> {
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?
        .write_all(text.as_bytes())?;
    Ok(())
}
pub fn test_binaries(root: &Path, input: &Path, output: &Path) -> Result<J> {
    let cache = root.join(".codecraft/cargo").canonicalize()?;
    let mut binaries = BTreeSet::new();
    for line in fs::read_to_string(input)?
        .lines()
        .filter(|line| !line.trim().is_empty())
    {
        let artifact = parse(line)?;
        ensure!(artifact.is_object(), "Invalid Cargo artifact record");
        if artifact["reason"] != "compiler-artifact" || artifact["profile"]["test"] != true {
            continue;
        }
        if artifact["executable"].is_null() {
            continue;
        }
        let executable = s(&artifact["executable"])?;
        ensure!(
            !executable.contains(['\n', '\r']),
            "Invalid executable path"
        );
        let path = Path::new(executable);
        ensure!(
            path.is_absolute() && path.is_file() && path.canonicalize()?.starts_with(&cache),
            "Cargo test executable must exist inside the build cache"
        );
        ensure!(
            binaries.insert(executable.to_owned()),
            "Duplicate Cargo test executable"
        );
    }
    ensure!(!binaries.is_empty(), "Cargo emitted no executable tests");
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        output,
        binaries
            .iter()
            .map(|p| format!("{p}\n"))
            .collect::<String>(),
    )?;
    Ok(json!({"status":"passed","executables":binaries,"output":output}))
}
pub fn command(root: &Path, args: &[String]) -> Result<J> {
    let action = args.first().context("ci requires plan/check")?;
    let mut options = BTreeMap::new();
    let mut full = false;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--full" {
            ensure!(!full, "Duplicate --full");
            full = true;
            i += 1;
            continue;
        }
        let key = &args[i];
        ensure!(key.starts_with("--"), "Unexpected CI argument");
        let value = args.get(i + 1).context("Missing CI option value")?;
        ensure!(!value.starts_with("--"), "Missing CI option value");
        ensure!(
            options.insert(key.as_str(), value.as_str()).is_none(),
            "Duplicate CI option"
        );
        i += 2;
    }
    let get = |k| options.get(k).copied();
    let allowed = if action == "plan" {
        vec![
            "--base",
            "--head",
            "--event",
            "--event-name",
            "--github-output",
            "--summary",
            "--report",
        ]
    } else if action == "check" {
        vec!["--needs-env", "--summary"]
    } else if action == "test-binaries" {
        vec!["--input", "--output"]
    } else {
        bail!("Unknown CI command")
    };
    ensure!(
        options.keys().all(|k| allowed.contains(k)) && (action == "plan" || !full),
        "Unsupported CI option"
    );
    if action == "test-binaries" {
        return test_binaries(
            root,
            &root.join(get("--input").context("--input required")?),
            &root.join(get("--output").context("--output required")?),
        );
    }
    let data = if action == "plan" {
        ensure!(
            get("--event").is_some() == get("--event-name").is_some(),
            "--event and --event-name required together"
        );
        let event = get("--event").map(|p| read(Path::new(p))).transpose()?;
        let data = plan(
            root,
            get("--base"),
            get("--head").unwrap_or("HEAD"),
            full,
            event.as_ref(),
            get("--event-name"),
        )?;
        if let Some(path) = get("--github-output") {
            let compact = json!({"schema_version":1,"head":data["head"],"counts":data["counts"],"matrices":data["matrices"]});
            let mut output = format!("plan={}\n", serde_json::to_string(&compact)?);
            for (pid, m) in obj(&data["matrices"])? {
                let key = pid.replace('-', "_");
                ensure!(
                    key.bytes().enumerate().all(|(i, b)| b.is_ascii_lowercase()
                        || (i > 0 && (b.is_ascii_digit() || b == b'_'))),
                    "Invalid platform output name"
                );
                output += &format!(
                    "{key}={}\n{key}_count={}\n",
                    serde_json::to_string(m)?,
                    data["counts"][pid]
                );
            }
            append(path, &output)?;
        }
        if let Some(path) = get("--report") {
            write(
                Path::new(path),
                &json!({"schema_version":2,"ok":true,"data":data}),
            )?;
        }
        data
    } else {
        let var = get("--needs-env").context("--needs-env required")?;
        check(
            &parse(&std::env::var(var).context("Missing job-results environment")?)?,
            Some(&revision(root, "HEAD")?),
        )?
    };
    if let Some(path) = get("--summary") {
        append(
            path,
            &format!(
                "## CI {action}\n\n```json\n{}\n```\n",
                serde_json::to_string_pretty(&data)?
            ),
        )?;
    }
    Ok(data)
}
