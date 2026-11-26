use serde_json::Value;
use std::{path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn run(args: &[&str]) -> (i32, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_codecraft"))
        .arg("--root")
        .arg(root())
        .args(args)
        .env("PATH", "")
        .output()
        .unwrap();
    let body = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&output.stderr)));
    (output.status.code().unwrap(), body)
}
#[test]
fn discovers_without_python_or_cargo() {
    let (code, v) = run(&["discover", "--interface", "display"]);
    assert_eq!(code, 0, "{v}");
    assert!(!v["data"]["candidates"].as_array().unwrap().is_empty());
}
#[test]
fn exports_and_verifies_without_python() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("app");
    let (code, v) = run(&[
        "new",
        "--target",
        "esp-idf/reterminal-e1001",
        "--example",
        "epd_hello",
        "--output",
        app.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{v}");
    for p in walkdir::WalkDir::new(&app) {
        let p = p.unwrap();
        assert_ne!(
            p.path().extension().and_then(|x| x.to_str()),
            Some("py"),
            "{}",
            p.path().display()
        );
        assert_ne!(p.path().extension().and_then(|x| x.to_str()), Some("rs"));
    }
    let (code, v) = run(&["verify-project", app.to_str().unwrap()]);
    assert_eq!(code, 0, "{v}");
}

#[test]
fn all_targets_and_variants_export_without_codecraft_runtime() {
    let c = codecraft::catalog::Catalog::load(&root()).unwrap();
    for (id, target) in &c.targets {
        let variants: Vec<Option<&str>> = if target["variants"].as_object().unwrap().is_empty() {
            vec![None]
        } else {
            target["variants"]
                .as_object()
                .unwrap()
                .keys()
                .map(|v| Some(v.as_str()))
                .collect()
        };
        for variant in variants {
            let temp = tempfile::tempdir().unwrap();
            let app = temp.path().join("app");
            let mut args = vec![
                "new",
                "--target",
                id,
                "--example",
                target["examples"][0].as_str().unwrap(),
                "--output",
                app.to_str().unwrap(),
            ];
            if let Some(v) = variant {
                args.extend(["--variant", v]);
            }
            let (code, result) = run(&args);
            assert_eq!(code, 0, "{id}/{variant:?}: {result}");
            let (code, result) = run(&["verify-project", app.to_str().unwrap()]);
            assert_eq!(code, 0, "{result}");
            assert!(!app.join("codecraft.py").exists());
            assert!(!app.join("tools/codecraft").exists());
            assert!(!app.join("tools/cli").exists());
        }
    }
}

#[test]
fn source_updates_preserve_origin_custom_rules_and_binary() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("app");
    let bin = std::path::Path::new(env!("CARGO_BIN_EXE_codecraft"));
    let before = codecraft::util::digest(bin).unwrap();
    let timestamp = std::fs::metadata(bin).unwrap().modified().unwrap();
    let (code, v) = run(&[
        "new",
        "--target",
        "esp-idf/reterminal-e1001",
        "--example",
        "epd_hello",
        "--output",
        app.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{v}");
    let old = codecraft::util::read(&app.join("codecraft.lock")).unwrap();
    let rules = app.join("AGENTS.md");
    std::fs::write(
        &rules,
        format!(
            "{}\nCustom rule: keep my application settings.\n",
            std::fs::read_to_string(&rules).unwrap()
        ),
    )
    .unwrap();
    let source = app.join("devices/reterminal-e1001/targets/examples/epd_hello/main/main.c");
    let old_source = std::fs::read_to_string(&source).unwrap();
    std::fs::write(&source, format!("{old_source}\n// application edit\n")).unwrap();
    let (code, v) = run(&["verify-project", app.to_str().unwrap()]);
    assert_eq!(code, 1, "{v}");
    let (code, v) = run(&["lock-project", app.to_str().unwrap()]);
    assert_eq!(code, 0, "{v}");
    assert_eq!(v["data"]["origin"], old["origin"]);
    assert!(std::fs::read_to_string(rules)
        .unwrap()
        .contains("Custom rule"));
    let (code, v) = run(&["verify-project", app.to_str().unwrap()]);
    assert_eq!(code, 0, "{v}");
    assert_eq!(before, codecraft::util::digest(bin).unwrap());
    assert_eq!(
        timestamp,
        std::fs::metadata(bin).unwrap().modified().unwrap()
    );
}

#[test]
fn rejects_unsafe_output_and_unknown_selection() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("app");
    std::fs::create_dir(&app).unwrap();
    std::fs::write(app.join("keep"), "user data").unwrap();
    let (code, _) = run(&[
        "new",
        "--target",
        "esp-idf/reterminal-e1001",
        "--example",
        "epd_hello",
        "--output",
        app.to_str().unwrap(),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        std::fs::read_to_string(app.join("keep")).unwrap(),
        "user data"
    );
    let (code, _) = run(&[
        "context",
        "--target",
        "esp-idf/reterminal-e1001",
        "--variant",
        "not-real",
    ]);
    assert_eq!(code, 1);
}

#[test]
fn sdk_release_identity_keeps_dirty_and_rejects_prefixes() {
    for (version, expected) in [
        ("ESP-IDF v6.0.2", true),
        ("ESP-IDF v6.0.2-dirty", true),
        ("ESP-IDF v6.0.2-rc1", false),
        ("ESP-IDF v6.0.2.1", false),
        ("ESP-IDF v6.0.2-99-gabcdef", false),
    ] {
        assert_eq!(
            codecraft::platform::esp_idf::version_matches(version, "6.0.2"),
            expected
        );
    }
}

#[test]
fn explicit_skill_selection_needs_no_python_to_copy_verified_package() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("app");
    let selection = temp.path().join("selection.json");
    codecraft::util::write(&selection,&serde_json::json!([{"skill":"codecraft-capabilities-llm","configuration":{},"limits":{},"environment":{"network":true,"tls":true,"privacy_approved":true,"secret_refs":["CODECRAFT_DEVICE_FILE"],"environment_refs":["CODECRAFT_API_ORIGIN"]}}])).unwrap();
    let (code, v) = run(&[
        "new",
        "--target",
        "esp-idf/reterminal-e1001",
        "--example",
        "epd_hello",
        "--output",
        app.to_str().unwrap(),
        "--skills",
        selection.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{v}");
    assert!(app
        .join("skills/codecraft-capabilities-llm/scripts/llm.py")
        .is_file());
    assert!(!app.join("tools/codecraft").exists());
    let (code, v) = run(&["verify-project", app.to_str().unwrap()]);
    assert_eq!(code, 0, "{v}");
}

#[test]
fn graph_rejects_cycles_secrets_and_escape() {
    use codecraft::util::Graph;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(".env"), "synthetic-secret").unwrap();
    assert!(Graph::new(dir.path()).add(".env").is_err());
    std::fs::write(
        dir.path().join("resource.json"),
        r#"{"schema_version":1,"kind":"resource","inputs":["resource.json"],"dependencies":[]}"#,
    )
    .unwrap();
    assert!(Graph::new(dir.path()).add("resource.json").is_err());
    assert!(Graph::new(dir.path()).add("../outside").is_err());
    assert!(codecraft::util::parse(r#"{"nested":{"x":1,"x":2}}"#).is_err());
}

#[test]
fn package_requires_reviewed_sources_and_keeps_output_outside_project() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("app");
    let (code, value) = run(&[
        "new",
        "--target",
        "linux/reterminal",
        "--example",
        "buttons_evdev",
        "--output",
        app.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{value}");
    let output = temp.path().join("app.zip");
    let (code, value) = run(&[
        "package",
        app.to_str().unwrap(),
        "--output",
        output.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{value}");
    let file = std::fs::File::open(output).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    assert!(archive.by_name("codecraft.lock").is_ok());
    std::fs::write(app.join("unreviewed.txt"), "new source").unwrap();
    let output = temp.path().join("changed.zip");
    assert_ne!(
        run(&[
            "package",
            app.to_str().unwrap(),
            "--output",
            output.to_str().unwrap()
        ])
        .0,
        0
    );
    assert!(!output.exists());
    assert_ne!(
        run(&["validate", "--static", "--target", "linux/reterminal"]).0,
        0
    );
}

#[test]
fn missing_native_tool_is_not_success_or_unsupported() {
    let dir = tempfile::tempdir().unwrap();
    let result = codecraft::platform::command(
        dir.path(),
        "missing",
        &["codecraft-nonexistent-native-tool".into()],
        &serde_json::json!({"status":"not_run"}),
    )
    .unwrap();
    assert_eq!(result["status"], "not_run");
    assert!(result["exit_code"].is_null());
}

#[test]
fn native_timeout_reports_failure() {
    let dir = tempfile::tempdir().unwrap();
    // A real child process validates timeout/exit status handling without SDKs.
    #[cfg(unix)]
    let argv = vec!["/bin/sh".into(), "-c".into(), "sleep 10".into()];
    #[cfg(windows)]
    let argv = vec![
        "cmd.exe".into(),
        "/C".into(),
        "ping -n 10 127.0.0.1 >NUL".into(),
    ];
    let result = codecraft::platform::command_timeout(
        dir.path(),
        "timeout",
        &argv,
        &serde_json::json!({}),
        std::time::Duration::from_millis(100),
    )
    .unwrap();
    assert_eq!(result["status"], "failed");
    assert!(result["reason"].as_str().unwrap().contains("exceeded"));
}

#[test]
fn unsupported_image_format_is_distinct_from_contract_errors() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("plan.json");
    let (code, value) = run(&[
        "firmware",
        "plan",
        "--target",
        "linux/reterminal",
        "--example",
        "buttons_evdev",
        "--output",
        output.to_str().unwrap(),
    ]);
    assert_eq!(code, 2, "{value}");
    assert_eq!(value["data"]["status"], "not_supported");
    assert!(!output.exists());
}

#[test]
fn discovery_rejects_invalid_values_even_when_hardware_bound_is_unknown() {
    let catalog = codecraft::catalog::Catalog::load(&root()).unwrap();
    let invalid = serde_json::json!({"audio":{"configuration":{"sample_rate_hz":"44100"}}});
    assert!(catalog.discover(&[], Some("esp-idf"), &invalid).is_err());
    let valid = serde_json::json!({"audio":{"configuration":{"sample_rate_hz":44100}}});
    let result = catalog.discover(&[], Some("esp-idf"), &valid).unwrap();
    assert!(result["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c["compatibility"] == "unknown"));
}

#[test]
fn firmware_without_build_is_not_run_and_creates_no_plan() {
    let dir = tempfile::tempdir().unwrap();
    let app = dir.path().join("app");
    let (code, value) = run(&[
        "new",
        "--target",
        "esp-idf/reterminal-e1001",
        "--example",
        "epd_hello",
        "--output",
        app.to_str().unwrap(),
    ]);
    assert_eq!(code, 0, "{value}");
    let plan = dir.path().join("plan.json");
    let (code, value) = run(&[
        "firmware",
        "plan",
        "--project",
        app.to_str().unwrap(),
        "--output",
        plan.to_str().unwrap(),
    ]);
    assert_eq!(code, 2, "{value}");
    assert_eq!(value["data"]["status"], "not_run");
    assert!(!plan.exists());
}

#[test]
fn doctor_reports_missing_tools_for_each_stack_without_building() {
    for target in [
        "esp-idf/reterminal-e1001",
        "arduino/wio-terminal",
        "linux/reterminal",
    ] {
        let (exit, report) = run(&["doctor", "--target", target]);
        assert_eq!(exit, 2, "{report}");
        assert_eq!(report["data"]["status"], "not_run");
        assert_eq!(report["data"]["actual"]["status"], "not_run");
        assert!(report["data"]["expected"].is_object());
        assert_eq!(report["data"]["next_steps"][0]["retry"][1], "doctor");
    }
    let (exit, report) = run(&["doctor", "--target", "linux/unknown"]);
    assert_eq!(exit, 1, "{report}");
}

#[test]
fn doctor_in_export_preserves_project_and_lock() {
    let temp = tempfile::tempdir().unwrap();
    let app = temp.path().join("app");
    let (exit, report) = run(&[
        "new",
        "--target",
        "esp-idf/reterminal-e1001",
        "--example",
        "epd_hello",
        "--output",
        app.to_str().unwrap(),
    ]);
    assert_eq!(exit, 0, "{report}");
    let snapshot = || {
        walkdir::WalkDir::new(&app)
            .into_iter()
            .map(|entry| {
                let entry = entry.unwrap();
                (
                    entry.path().strip_prefix(&app).unwrap().to_owned(),
                    if entry.file_type().is_file() {
                        std::fs::read(entry.path()).unwrap()
                    } else {
                        vec![]
                    },
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    let before = snapshot();
    let output = Command::new(env!("CARGO_BIN_EXE_codecraft"))
        .arg("--root")
        .arg(&app)
        .args(["doctor", "--target", "esp-idf/reterminal-e1001"])
        .env("PATH", "")
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert_eq!(snapshot(), before);
}
