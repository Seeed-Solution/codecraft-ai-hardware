use codecraft::{
    maintenance_ci::{self, Inputs},
    util::*,
};
use serde_json::json;
use std::{fs, path::Path, process::Command};
fn put(root: &Path, path: &str, text: &str) {
    let p = root.join(path);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, text).unwrap();
}
fn fixture(root: &Path) {
    for stack in ["esp-idf", "linux", "arduino"] {
        write(&root.join(format!("platforms/{stack}/platform.json")),&json!({"id":stack,"schema_version":2,"adapter":format!("platforms/{stack}/native.rs"),"ci":{"runner":"ubuntu-latest"}})).unwrap();
        put(
            root,
            &format!("platforms/{stack}/native.rs"),
            "not executable historical source\n",
        );
    }
    let shared = "platforms/esp-idf/components/sensor";
    write(&root.join(format!("{shared}/resource.json")),&json!({"schema_version":1,"kind":"resource","inputs":[format!("{shared}/sensor.c")],"dependencies":[]})).unwrap();
    put(root, &format!("{shared}/sensor.c"), "shared\n");
    for (stack, board) in [
        ("esp-idf", "alpha"),
        ("esp-idf", "beta"),
        ("esp-idf", "gamma"),
        ("linux", "delta"),
        ("arduino", "uno"),
    ] {
        let path = format!("devices/{board}/targets");
        write(&root.join(format!("{path}/target.json")),&json!({"id":format!("{stack}/{board}"),"schema_version":3,"device":board,"path":path,"platform":stack,"inputs":[format!("{path}/bsp")],"dependencies":if ["alpha","beta"].contains(&board){vec![shared]}else{vec![]},"examples":["demo"],"example_inputs":{"demo":[format!("{path}/examples/demo")]},"example_dependencies":{"demo":[]},"variants":{},"interfaces":[],"toolchain":{"version":"fixture","source":format!("{path}/README.md")}})).unwrap();
        put(root, &format!("{path}/bsp/board.c"), "native\n");
        put(root, &format!("{path}/examples/demo/main.c"), "app\n");
        put(root, &format!("{path}/README.md"), "docs\n");
    }
}
fn selected(v: &J) -> Vec<String> {
    v["targets"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, v)| v["selected"] == true)
        .map(|(k, _)| k.clone())
        .collect()
}
fn select(paths: &[&str]) -> J {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    let inv = Inputs::load(tmp.path()).unwrap();
    maintenance_ci::selection(
        Some(&inv),
        &inv,
        &paths.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        None,
    )
    .unwrap()
}
#[test]
fn static_docs_tests_and_maintenance_do_not_select_firmware() {
    for p in [
        "docs/guide.md",
        "AGENTS.md",
        "tests/tools/cli/ci.rs",
        "tools/cli/src/maintenance_ci.rs",
        "skills/example/SKILL.md",
    ] {
        assert!(selected(&select(&[p])).is_empty(), "{p}");
    }
}
#[test]
fn selects_each_native_stack_and_shared_consumers() {
    assert_eq!(
        selected(&select(&["devices/uno/targets/bsp/board.c"])),
        vec!["arduino/uno"]
    );
    assert_eq!(
        selected(&select(&["devices/delta/targets/new-native.c"])),
        vec!["linux/delta"]
    );
    assert_eq!(
        selected(&select(&["platforms/esp-idf/components/sensor/sensor.c"])),
        vec!["esp-idf/alpha", "esp-idf/beta"]
    );
    assert_eq!(selected(&select(&["platforms/esp-idf/native.rs"])).len(), 3);
}
#[test]
fn shared_cli_and_unknown_infrastructure_select_all() {
    for p in [
        "tools/cli/src/project.rs",
        "Cargo.lock",
        ".cargo/config.toml",
        ".github/workflows/hardware.yml",
        "unknown.file",
    ] {
        assert_eq!(selected(&select(&[p])).len(), 5, "{p}");
    }
}
#[test]
fn declared_markdown_assets_are_native_but_guides_are_not() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    put(
        tmp.path(),
        "devices/alpha/targets/bsp/page.md",
        "compiled asset",
    );
    let inv = Inputs::load(tmp.path()).unwrap();
    assert_eq!(
        selected(
            &maintenance_ci::selection(
                Some(&inv),
                &inv,
                &["devices/alpha/targets/bsp/page.md".into()],
                None
            )
            .unwrap()
        ),
        vec!["esp-idf/alpha"]
    );
}
#[test]
fn missing_dependencies_and_cycles_are_rejected() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    fs::remove_file(
        tmp.path()
            .join("platforms/esp-idf/components/sensor/sensor.c"),
    )
    .unwrap();
    assert!(Inputs::load(tmp.path()).is_err());
    put(
        tmp.path(),
        "platforms/esp-idf/components/sensor/sensor.c",
        "native",
    );
    write(&tmp.path().join("platforms/esp-idf/components/sensor/resource.json"),&json!({"schema_version":1,"kind":"resource","inputs":[],"dependencies":["platforms/esp-idf/components/sensor"]})).unwrap();
    assert!(Inputs::load(tmp.path()).is_err());
}
#[test]
fn selection_tracks_deleted_dependencies_and_removed_targets() {
    let before = tempfile::tempdir().unwrap();
    fixture(before.path());
    let b = Inputs::load(before.path()).unwrap();
    let after = tempfile::tempdir().unwrap();
    fixture(after.path());
    let path = after.path().join("devices/alpha/targets/target.json");
    let mut t = read(&path).unwrap();
    t["dependencies"] = json!([]);
    write(&path, &t).unwrap();
    fs::remove_dir_all(after.path().join("devices/beta")).unwrap();
    let a = Inputs::load(after.path()).unwrap();
    let plan = maintenance_ci::selection(
        Some(&b),
        &a,
        &["platforms/esp-idf/components/sensor/sensor.c".into()],
        None,
    )
    .unwrap();
    assert_eq!(selected(&plan), vec!["esp-idf/alpha"]);
    assert_eq!(plan["removed_targets"], json!(["esp-idf/beta"]));
}
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap().trim().into()
}
fn repo(root: &Path) -> String {
    fixture(root);
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "fixture@example.invalid"]);
    git(root, &["config", "user.name", "Fixture"]);
    git(root, &["add", "."]);
    git(root, &["commit", "-qm", "fixture"]);
    git(root, &["rev-parse", "HEAD"])
}
#[test]
fn plans_committed_snapshots_and_ignores_dirty_tree() {
    let tmp = tempfile::tempdir().unwrap();
    let base = repo(tmp.path());
    put(tmp.path(), "docs/note.md", "docs\n");
    git(tmp.path(), &["add", "."]);
    git(tmp.path(), &["commit", "-qm", "docs"]);
    put(
        tmp.path(),
        "devices/alpha/targets/bsp/board.c",
        "dirty native change",
    );
    let plan = maintenance_ci::plan(tmp.path(), Some(&base), "HEAD", false, None, None).unwrap();
    assert!(selected(&plan).is_empty());
    assert!(maintenance_ci::plan(
        tmp.path(),
        Some("does-not-exist"),
        "HEAD",
        false,
        None,
        None
    )
    .is_err());
}
#[test]
fn push_and_pr_event_baselines_are_checked() {
    let tmp = tempfile::tempdir().unwrap();
    let base = repo(tmp.path());
    let event = json!({"before":"0000000000000000000000000000000000000000","after":base});
    let plan =
        maintenance_ci::plan(tmp.path(), None, "HEAD", false, Some(&event), Some("push")).unwrap();
    assert_eq!(selected(&plan).len(), 5);
    assert!(maintenance_ci::plan(
        tmp.path(),
        None,
        "HEAD",
        false,
        Some(&json!({"before":"nonsense","after":base})),
        Some("push")
    )
    .is_err());
    let pr = json!({"pull_request":{"base":{"sha":base},"head":{"sha":base}}});
    assert!(maintenance_ci::plan(
        tmp.path(),
        None,
        "HEAD",
        false,
        Some(&pr),
        Some("pull_request")
    )
    .is_ok());
}
fn needs() -> J {
    let mut n = json!({"cli":{"result":"success"},"contracts":{"result":"success","outputs":{}},"esp-idf":{"result":"success"},"linux":{"result":"skipped"},"arduino":{"result":"skipped"}});
    let matrices = json!({"esp-idf":{"include":[{"id":"esp-idf/alpha","platform":"esp-idf"}]},"linux":{"include":[]},"arduino":{"include":[]}});
    let counts = json!({"esp-idf":1,"linux":0,"arduino":0});
    let plan =
        json!({"schema_version":1,"head":"a".repeat(40),"matrices":matrices,"counts":counts});
    n["contracts"]["outputs"]["plan"] = json!(plan.to_string());
    for p in ["esp-idf", "linux", "arduino"] {
        let key = p.replace('-', "_");
        n["contracts"]["outputs"][&key] = json!(matrices[p].to_string());
        n["contracts"]["outputs"][format!("{key}_count")] = json!(counts[p].to_string());
    }
    n
}
#[test]
fn aggregate_requires_all_selected_jobs_and_cli() {
    let n = needs();
    assert_eq!(
        maintenance_ci::check(&n, Some(&"a".repeat(40))).unwrap()["ok"],
        true
    );
    for job in ["cli", "contracts", "esp-idf"] {
        for status in ["failure", "cancelled", "skipped"] {
            let mut bad = n.clone();
            bad[job]["result"] = json!(status);
            assert_eq!(maintenance_ci::check(&bad, None).unwrap()["ok"], false);
        }
    }
    let mut bad = n;
    bad["linux"]["result"] = json!("success");
    assert_eq!(maintenance_ci::check(&bad, None).unwrap()["ok"], false);
}
#[test]
fn aggregate_rejects_missing_jobs_output_drift_and_stale_head() {
    let n = needs();
    assert!(maintenance_ci::check(&n, Some(&"b".repeat(40))).is_err());
    let mut bad = n.clone();
    bad.as_object_mut().unwrap().remove("linux");
    assert!(maintenance_ci::check(&bad, None).is_err());
    let mut bad = n;
    bad["contracts"]["outputs"]["esp_idf_count"] = json!("0");
    assert!(maintenance_ci::check(&bad, None).is_err());
}
#[cfg(unix)]
#[test]
fn symlink_targets_must_remain_inside_snapshot() {
    use std::os::unix::fs::symlink;
    let tmp = tempfile::tempdir().unwrap();
    let base = repo(tmp.path());
    symlink("/etc/passwd", tmp.path().join("outside")).unwrap();
    git(tmp.path(), &["add", "outside"]);
    git(tmp.path(), &["commit", "-qm", "bad-link"]);
    assert!(maintenance_ci::plan(tmp.path(), Some(&base), "HEAD", false, None, None).is_err());
}
#[test]
fn new_targets_are_selected_without_omitting_other_static_changes() {
    let before = tempfile::tempdir().unwrap();
    fixture(before.path());
    fs::remove_dir_all(before.path().join("devices/gamma")).unwrap();
    let after = tempfile::tempdir().unwrap();
    fixture(after.path());
    let plan = maintenance_ci::selection(
        Some(&Inputs::load(before.path()).unwrap()),
        &Inputs::load(after.path()).unwrap(),
        &["docs/new.md".into()],
        None,
    )
    .unwrap();
    assert_eq!(selected(&plan), vec!["esp-idf/gamma"]);
}
#[cfg(unix)]
#[test]
fn internal_configuration_links_preserve_real_file_ownership() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    put(tmp.path(), "shared/config.txt", "config\n");
    std::os::unix::fs::symlink(
        "../../../../shared/config.txt",
        tmp.path().join("devices/alpha/targets/bsp/config.txt"),
    )
    .unwrap();
    let inv = Inputs::load(tmp.path()).unwrap();
    assert_eq!(
        selected(
            &maintenance_ci::selection(Some(&inv), &inv, &["shared/config.txt".into()], None)
                .unwrap()
        ),
        vec!["esp-idf/alpha"]
    );
}
#[test]
fn public_planner_writes_consistent_outputs_and_summary() {
    let tmp = tempfile::tempdir().unwrap();
    let base = repo(tmp.path());
    put(tmp.path(), "devices/alpha/targets/bsp/board.c", "changed\n");
    git(tmp.path(), &["add", "."]);
    git(tmp.path(), &["commit", "-qm", "native"]);
    let outputs = tmp.path().join("outputs");
    let summary = tmp.path().join("summary");
    let report = tmp.path().join("report");
    let data = maintenance_ci::command(
        tmp.path(),
        &[
            "plan".into(),
            "--base".into(),
            base,
            "--github-output".into(),
            outputs.to_string_lossy().into(),
            "--summary".into(),
            summary.to_string_lossy().into(),
            "--report".into(),
            report.to_string_lossy().into(),
        ],
    )
    .unwrap();
    assert_eq!(selected(&data), vec!["esp-idf/alpha"]);
    let text = fs::read_to_string(outputs).unwrap();
    assert!(text.contains("esp_idf_count=1\n"));
    assert!(text.contains("linux_count=0\n"));
    assert!(fs::read_to_string(summary)
        .unwrap()
        .contains("esp-idf/alpha"));
    assert_eq!(read(&report).unwrap()["data"], data);
}
#[test]
fn public_aggregate_accepts_planned_skips_but_rejects_failed_jobs() {
    let tmp = tempfile::tempdir().unwrap();
    let head = repo(tmp.path());
    put(tmp.path(), "tools/cli/Cargo.toml", "# reference marker\n");
    let mut n = needs();
    let mut plan = parse(s(&n["contracts"]["outputs"]["plan"]).unwrap()).unwrap();
    plan["head"] = json!(head);
    for p in ["esp-idf", "linux", "arduino"] {
        plan["counts"][p] = json!(0);
        plan["matrices"][p] = json!({"include":[]});
        n[p]["result"] = json!("skipped");
        let key = p.replace('-', "_");
        n["contracts"]["outputs"][&key] = json!(plan["matrices"][p].to_string());
        n["contracts"]["outputs"][format!("{key}_count")] = json!("0");
    }
    n["contracts"]["outputs"]["plan"] = json!(plan.to_string());
    let call = |n: &J| {
        Command::new(env!("CARGO_BIN_EXE_codecraft"))
            .arg("--root")
            .arg(tmp.path())
            .args([
                "maintenance",
                "ci",
                "check",
                "--needs-env",
                "CODECRAFT_CI_TEST",
            ])
            .env("CODECRAFT_CI_TEST", n.to_string())
            .output()
            .unwrap()
    };
    let output = call(&n);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let body = parse(&String::from_utf8(output.stdout).unwrap()).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["ok"], true);
    n["cli"]["result"] = json!("failure");
    let output = call(&n);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        parse(&String::from_utf8(output.stdout).unwrap()).unwrap()["ok"],
        false
    );
}
#[test]
fn cargo_test_artifacts_require_unique_real_cache_executables() {
    let tmp = tempfile::tempdir().unwrap();
    let exe = tmp
        .path()
        .join(".codecraft/cargo/target/release/deps/tests");
    put(
        tmp.path(),
        ".codecraft/cargo/target/release/deps/tests",
        "test executable",
    );
    let input = tmp.path().join("input.jsonl");
    let output = tmp.path().join("executables.txt");
    let record = json!({"reason":"compiler-artifact","profile":{"test":true},"executable":exe});
    fs::write(
        &input,
        format!(
            "{}\n{}\n",
            json!({"reason":"build-finished","success":true}),
            record
        ),
    )
    .unwrap();
    let result = maintenance_ci::test_binaries(tmp.path(), &input, &output).unwrap();
    assert_eq!(arr(&result["executables"]).unwrap().len(), 1);
    assert_eq!(
        fs::read_to_string(&output).unwrap(),
        format!("{}\n", exe.display())
    );
    fs::write(&input, format!("{record}\n{record}\n")).unwrap();
    assert!(maintenance_ci::test_binaries(tmp.path(), &input, &output).is_err());
    fs::write(&input, "not-json\n").unwrap();
    assert!(maintenance_ci::test_binaries(tmp.path(), &input, &output).is_err());
    let external = tempfile::NamedTempFile::new().unwrap();
    let mut bad = record;
    bad["executable"] = json!(external.path());
    fs::write(&input, bad.to_string()).unwrap();
    assert!(maintenance_ci::test_binaries(tmp.path(), &input, &output).is_err());
    fs::write(
        &input,
        json!({"reason":"build-finished","success":true}).to_string(),
    )
    .unwrap();
    assert!(maintenance_ci::test_binaries(tmp.path(), &input, &output).is_err());
}
#[test]
fn historical_schema_two_and_current_schema_three_are_compared_as_data() {
    let tmp = tempfile::tempdir().unwrap();
    let base = repo(tmp.path());
    let manifest = tmp.path().join("platforms/linux/platform.json");
    let mut p = read(&manifest).unwrap();
    p["schema_version"] = json!(3);
    write(&manifest, &p).unwrap();
    git(tmp.path(), &["add", "."]);
    git(tmp.path(), &["commit", "-qm", "platform-schema"]);
    let plan = maintenance_ci::plan(tmp.path(), Some(&base), "HEAD", false, None, None).unwrap();
    assert_eq!(selected(&plan), vec!["linux/delta"]);
    p["schema_version"] = json!(99);
    write(&manifest, &p).unwrap();
    assert!(Inputs::load(tmp.path()).is_err());
}
#[test]
fn ci_options_reject_unknown_flags_and_flag_shaped_values() {
    let tmp = tempfile::tempdir().unwrap();
    for args in [
        vec!["plan", "--full", "--unknown", "x"],
        vec!["plan", "--full", "--summary", "--typo"],
        vec!["test-binaries", "--input", "--output", "file"],
        vec!["check", "--full", "--needs-env", "X"],
    ] {
        assert!(maintenance_ci::command(
            tmp.path(),
            &args.into_iter().map(str::to_owned).collect::<Vec<_>>()
        )
        .is_err());
    }
}
