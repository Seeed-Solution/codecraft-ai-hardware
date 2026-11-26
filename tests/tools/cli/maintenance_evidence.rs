use codecraft::{
    catalog::Catalog, maintenance_device, maintenance_evidence as evidence, maintenance_generate,
    project, util::*,
};
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn fixture() -> (tempfile::TempDir, Catalog, J) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().canonicalize().unwrap();
    for path in [
        "AGENTS.md",
        "devices/test/AGENTS.md",
        "devices/test/targets/target.json",
        "platforms/linux/platform.json",
        "platforms/linux/AGENTS.md",
        "toolchain.txt",
        "devices/test/targets/overlays/board.dts",
    ] {
        let p = root.join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, "fixture\n").unwrap();
    }
    let t = json!({"id":"linux/test","device":"test","platform":"linux","path":"devices/test/targets","examples":["hello"],"variants":{},"inputs":[],"dependencies":[],"example_inputs":{"hello":[]},"example_dependencies":{"hello":[]},"interfaces":[{"id":"gpio","implementation":["devices/test/AGENTS.md"],"operations":{}}],"toolchain":{"version":"3","source":"toolchain.txt"}});
    let d = json!({"id":"test","revision":"rev1","references":[],"assets":[],"acceptance":{"cases":[{"id":"gpio-observe","source":"devices/test/AGENTS.md","interfaces":["gpio"]}]},"variants":{}});
    let c = Catalog {
        root,
        devices: BTreeMap::from([("test".into(), d)]),
        targets: BTreeMap::from([("linux/test".into(), t.clone())]),
        platforms: BTreeMap::from([("linux".into(), json!({"application_inputs":[]}))]),
    };
    let log = "tests/devices/test/logs/run.log";
    fs::create_dir_all(c.root.join(log).parent().unwrap()).unwrap();
    fs::write(c.root.join(log), "synthetic offline test\n").unwrap();
    let tc = json!({"status":"passed","python":"3.14.6","dtc":"Version: DTC 1.8.1","scope":"host checks only"});
    let rows = json!([{"name":"linux/test/hello","status":"passed","toolchain":tc,"log":log,"log_sha256":digest(&c.root.join(log)).unwrap()},{"name":"linux/test/overlay/board","status":"passed","toolchain":tc,"log":log,"log_sha256":digest(&c.root.join(log)).unwrap()}]);
    let source = c.graph(&t).unwrap().fingerprint();
    let cli = project::cli_identity().unwrap();
    let record = json!({"schema_version":3,"graph_schema_version":4,"target":"linux/test","kind":"build","repository_commit":"a".repeat(40),"repository_dirty":false,"source_digest":source,"revision":"unknown","toolchain":tc,"command":["synthetic-offline-fixture"],"environment":{"host":"fixture"},"result":"passed","limitations":["Synthetic test only"],"operator":"test","log":log,"log_sha256":digest(&c.root.join(log)).unwrap(),"executed_at":"2026-11-26T00:00:00Z","cases":rows,"cli":cli,"generated_project":{"status":"passed","provenance":"passed","origin_source_digest":source,"source_digest":"b".repeat(64),"cli":cli,"example":"hello","variant":null,"build_results":rows,"toolchain":tc,"log":log,"log_sha256":digest(&c.root.join(log)).unwrap()}});
    (tmp, c, record)
}
#[test]
fn evidence_rejects_identity_cases_toolchain_and_log_tampering() {
    let (_tmp, c, r) = fixture();
    evidence::validate(&c, &r, true).unwrap();
    for key in ["source_digest", "cli"] {
        let mut bad = r.clone();
        bad[key] = if key == "cli" {
            json!({"contract":4,"version":"0.1.0","sha256":"c".repeat(64)})
        } else {
            json!("c".repeat(64))
        };
        assert!(evidence::validate(&c, &bad, true).is_err());
    }
    let mut bad = r.clone();
    bad["cases"].as_array_mut().unwrap().pop();
    assert!(evidence::validate(&c, &bad, true).is_err());
    let mut bad = r.clone();
    bad["cases"][0]["toolchain"]["python"] = json!("3.9.0");
    assert!(evidence::validate(&c, &bad, true).is_err());
    let mut bad = r.clone();
    bad["generated_project"]["variant"] = json!("wrong");
    assert!(evidence::validate(&c, &bad, true).is_err());
    fs::write(c.root.join(s(&r["log"]).unwrap()), "altered\n").unwrap();
    assert!(evidence::validate(&c, &r, true).is_err());
}
#[test]
fn failed_observations_do_not_promote_and_old_bytes_are_retained() {
    let (_tmp, c, r) = fixture();
    let dir = c.root.join("tests/devices/test/records");
    fs::create_dir_all(&dir).unwrap();
    let old = b"{\"schema_version\":2,\"target\":\"linux/test\",\"result\":\"passed\"}\n";
    fs::write(dir.join("old.json"), old).unwrap();
    write(&dir.join("new.json"), &r).unwrap();
    let state = evidence::support(&c, "linux/test").unwrap();
    assert_eq!(state["level"], "build-verified");
    assert_eq!(state["device"], "not_run");
    assert_eq!(state["archived_records"].as_array().unwrap().len(), 1);
    assert_eq!(fs::read(dir.join("old.json")).unwrap(), old);
    let mut fail = r.clone();
    fail["executed_at"] = json!("2026-11-26T01:00:00Z");
    fail["result"] = json!("failed");
    fail["cases"][0]["status"] = json!("failed");
    fail["cases"][0]["reason"] = json!("Synthetic failure");
    write(&dir.join("failed.json"), &fail).unwrap();
    assert_eq!(
        evidence::support(&c, "linux/test").unwrap()["level"],
        "unknown"
    );
}
#[test]
fn invalid_dates_and_secret_logs_are_rejected() {
    let (_tmp, c, r) = fixture();
    for date in [
        "2026-02-30T00:00:00Z",
        "2026-11-26T24:00:00Z",
        "2026-11-26T00:00:00",
    ] {
        let mut bad = r.clone();
        bad["executed_at"] = json!(date);
        assert!(evidence::validate(&c, &bad, true).is_err());
    }
    let mut bad = r;
    fs::write(
        c.root.join(s(&bad["log"]).unwrap()),
        "password=synthetic-fixture\n",
    )
    .unwrap();
    bad["log_sha256"] = json!(digest(&c.root.join(s(&bad["log"]).unwrap())).unwrap());
    assert!(evidence::validate(&c, &bad, true).is_err());
}
#[test]
fn authored_devices_and_generated_views_preserve_sources() {
    let root = repo();
    let c = Catalog::load(&root).unwrap();
    let before: BTreeMap<_, _> = c
        .devices
        .keys()
        .map(|id| {
            let path = format!("devices/{id}/AGENTS.md");
            (path.clone(), digest(&root.join(path)).unwrap())
        })
        .collect();
    for d in c.devices.values() {
        maintenance_device::validate(&root, d).unwrap();
    }
    let historical: BTreeMap<_, _> = walkdir::WalkDir::new(root.join("tests/devices"))
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry
                .path()
                .components()
                .any(|part| matches!(part.as_os_str().to_str(), Some("records" | "logs")))
                || entry.file_name() == "observations.md"
        })
        .map(|entry| (entry.path().to_owned(), digest(entry.path()).unwrap()))
        .collect();
    let views = maintenance_generate::views(&c).unwrap();
    for (path, hash) in historical {
        assert_eq!(
            digest(&path).unwrap(),
            hash,
            "Historical evidence changed: {}",
            path.display()
        );
    }

    assert_eq!(views.len(), 6);
    for name in [
        "docs/interface-matrix.json",
        "docs/skill-inventory.json",
        ".github/target-matrix.json",
    ] {
        assert_eq!(
            parse(&views[name]).unwrap(),
            read(&root.join(name)).unwrap()
        );
    }
    for (path, hash) in before {
        assert_eq!(digest(&root.join(path)).unwrap(), hash);
    }
}
#[test]
fn fact_assertions_cannot_cite_own_specifications() {
    let tmp = tempfile::tempdir().unwrap();
    let p = tmp.path().join("devices/test/AGENTS.md");
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(
        &p,
        "# Test\n## Specifications\nThe count is 42.\n## Sources\nIndependent notes.\n",
    )
    .unwrap();
    let f = json!({"label":"Count","section":"Controller","kind":"integer","status":"documented","value":42,"unit":"items","source":{"path":"devices/test/AGENTS.md","template":"The count is {value}."}});
    assert!(maintenance_device::validate_fact(tmp.path(), &f).is_err());
    fs::write(&p, "# Test\n## Hardware notes\nThe count is 42.\n").unwrap();
    maintenance_device::validate_fact(tmp.path(), &f).unwrap();
}

#[test]
fn physical_evidence_is_revision_and_variant_scoped() {
    let (_tmp, mut c, mut r) = fixture();
    c.targets.get_mut("linux/test").unwrap()["variants"] = json!({"a":{"device_variant":"a","inputs":[],"dependencies":[]},"b":{"device_variant":"b","inputs":[],"dependencies":[]}});
    c.targets.get_mut("linux/test").unwrap()["default_variant"] = json!("a");
    c.devices.get_mut("test").unwrap()["variants"] = json!({"a":{"facts":{"compatible_revisions":{"status":"documented","value":["rev1"]}}},"b":{"facts":{"compatible_revisions":{"status":"unknown","value":null}}}});
    r["kind"] = json!("device");
    r["revision"] = json!("rev1");
    r["variant"] = json!("a");
    r["source_digest"] = json!(c
        .graph(c.target("linux/test").unwrap())
        .unwrap()
        .fingerprint());
    r["cases"] = json!([{"name":"gpio-observe","status":"passed","observation":"Synthetic fixture","procedure":"Synthetic offline fixture"}]);
    evidence::validate(&c, &r, true).unwrap();
    let dir = c.root.join("tests/devices/test/records");
    fs::create_dir_all(&dir).unwrap();
    write(&dir.join("a.json"), &r).unwrap();
    let state = evidence::support(&c, "linux/test").unwrap();
    assert_eq!(state["variants"]["a"]["device"], "passed");
    assert_eq!(state["variants"]["b"]["device"], "not_run");
    assert_eq!(state["device"], "not_run");
    let mut bad = r.clone();
    bad["variant"] = json!("missing");
    assert!(evidence::validate(&c, &bad, true).is_err());
    bad = r.clone();
    bad["revision"] = json!("rev2");
    assert!(evidence::validate(&c, &bad, true).is_err());
    r["variant"] = json!("b");
    write(&dir.join("b.json"), &r).unwrap();
    assert_eq!(
        evidence::support(&c, "linux/test").unwrap()["device"],
        "not_run"
    );
}

#[test]
fn offline_platform_identity_checks_reject_versions_and_dependency_drift() {
    let t = json!({"platform":"esp-idf","toolchain":{"version":"6.0.2"}});
    let mut a = json!({"status":"passed","version":"ESP-IDF v6.0.2-dirty","compiler":"xtensa GCC 15","python":"3.13.2","sdk_commit":"a".repeat(40),"sdk_changes_sha256":"b".repeat(64),"sdk_submodules":""});
    codecraft::platform::evidence_toolchain(&t, &a).unwrap();
    for v in [
        "ESP-IDF v6.0.20",
        "ESP-IDF v6.0.2-rc1",
        "ESP-IDF v6.0.2-4-gabcdef",
    ] {
        a["version"] = json!(v);
        assert!(codecraft::platform::evidence_toolchain(&t, &a).is_err());
    }
    let t = json!({"platform":"arduino","toolchain":{"version":"1.5.0","fqbn":"arduino:avr:uno","core_version":"1.8.8","libraries":{"Example":"1.0"},"bundled_libraries":[]}});
    let a = json!({"status":"passed","cli":"1.5.0","fqbn":"arduino:avr:uno","core":{"id":"arduino:avr","version":"1.8.8","source_digest":"a".repeat(64)},"libraries":{"Example":{"version":"1.0","source_digest":"b".repeat(64)}},"bundled_libraries":{},"tools":{"compiler":{"version":"7.3.0","source_digest":"c".repeat(64)}}});
    codecraft::platform::evidence_toolchain(&t, &a).unwrap();
    let mut bad = a.clone();
    bad["libraries"]["Example"]["version"] = json!("2.0");
    assert!(codecraft::platform::evidence_toolchain(&t, &bad).is_err());
    let mut bad = a;
    bad["core"]["source_digest"] = json!("unknown");
    assert!(codecraft::platform::evidence_toolchain(&t, &bad).is_err());
}

#[test]
fn png_dimensions_alpha_hash_and_crc_are_checked() {
    fn crc(bytes: &[u8]) -> u32 {
        let mut c = !0u32;
        for b in bytes {
            c ^= *b as u32;
            for _ in 0..8 {
                c = (c >> 1) ^ if c & 1 != 0 { 0xedb88320 } else { 0 };
            }
        }
        !c
    }
    fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        let mut body = kind.to_vec();
        body.extend_from_slice(data);
        out.extend_from_slice(&crc(&body).to_be_bytes());
    }
    let tmp = tempfile::tempdir().unwrap();
    let p = tmp.path().join("pixel.png");
    let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut header = vec![];
    header.extend_from_slice(&1u32.to_be_bytes());
    header.extend_from_slice(&1u32.to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    chunk(&mut bytes, b"IHDR", &header);
    chunk(&mut bytes, b"IDAT", &[120, 156, 99, 0, 1, 0, 0, 5, 0, 1]);
    chunk(&mut bytes, b"IEND", &[]);
    fs::write(&p, &bytes).unwrap();
    let d = json!({"revision":"fixture"});
    let a = json!({"path":"pixel.png","source":"Synthetic fixture","license":"CC0","sha256":digest(&p).unwrap(),"revision":"fixture","width":1,"height":1,"alpha":true});
    maintenance_device::validate_asset(tmp.path(), &d, &a).unwrap();
    for key in ["width", "height", "alpha"] {
        let mut bad = a.clone();
        bad[key] = if key == "alpha" {
            json!(false)
        } else {
            json!(2)
        };
        assert!(maintenance_device::validate_asset(tmp.path(), &d, &bad).is_err());
    }
    bytes[29] ^= 1;
    fs::write(&p, &bytes).unwrap();
    let mut bad = a;
    bad["sha256"] = json!(digest(&p).unwrap());
    assert!(maintenance_device::validate_asset(tmp.path(), &d, &bad).is_err());
}

#[test]
fn registration_is_exclusive_and_preserves_original_log_bytes() {
    let (_tmp, c, mut r) = fixture();
    let git = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .current_dir(&c.root)
            .output()
            .unwrap()
    };
    assert!(git(&["init", "-q"]).status.success());
    assert!(git(&[
        "-c",
        "user.name=Offline Fixture",
        "-c",
        "user.email=fixture@example.invalid",
        "-c",
        "commit.gpgsign=false",
        "commit",
        "--allow-empty",
        "-qm",
        "Synthetic test commit"
    ])
    .status
    .success());
    r["repository_commit"] = json!(String::from_utf8(git(&["rev-parse", "HEAD"]).stdout)
        .unwrap()
        .trim());
    let input = c.root.join("input.json");
    write(&input, &r).unwrap();
    let log = c.root.join(s(&r["log"]).unwrap());
    let before = fs::read(&log).unwrap();
    let saved = evidence::record(&c, &input).unwrap();
    let path = c.root.join(s(&saved["path"]).unwrap());
    let record_bytes = fs::read(&path).unwrap();
    assert!(evidence::record(&c, &input).is_err());
    assert_eq!(fs::read(&path).unwrap(), record_bytes);
    assert_eq!(fs::read(log).unwrap(), before);
    assert!(fs::read_dir(path.parent().unwrap()).unwrap().all(|e| !e
        .unwrap()
        .file_name()
        .to_string_lossy()
        .ends_with(".tmp")));
}

#[test]
fn device_markdown_parsing_preserves_crlf_and_rejects_malformed_structure() {
    let c = Catalog::load(&repo()).unwrap();
    let t = c.target("linux/reterminal").unwrap();
    let g = c.graph(t).unwrap();
    let tmp = tempfile::tempdir().unwrap();
    for name in g.files.keys() {
        let dst = tmp.path().join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::copy(c.root.join(name), dst).unwrap();
    }
    let path = tmp.path().join("devices/reterminal/AGENTS.md");
    let original = fs::read_to_string(&path).unwrap();
    let crlf = original.replace('\n', "\r\n");
    fs::write(&path, &crlf).unwrap();
    let d = codecraft::device::read_device(tmp.path(), "reterminal").unwrap();
    assert_eq!(d, c.devices["reterminal"]);
    assert_eq!(fs::read(&path).unwrap(), crlf.as_bytes());
    for bad in [
        original.replace("## Specifications", "## Missing"),
        format!("{original}\n## Profile\nDuplicate\n"),
        original.replace("| ID | reterminal |", "| ID | reterminal |\n| ID | other |"),
        format!("{original}\n[device-notes]: elsewhere.md\n"),
        original.replace("[device-notes][device-notes]", "[device-notes][missing]"),
    ] {
        fs::write(&path, &bad).unwrap();
        assert!(codecraft::device::read_device(tmp.path(), "reterminal").is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), bad);
    }
    fs::write(&path, &original).unwrap();
    let mut graph = Graph::new(tmp.path());
    graph
        .resolve(vec!["devices/reterminal/AGENTS.md".into()])
        .unwrap();
    let before = graph.fingerprint();
    fs::write(
        &path,
        format!("{original}\nAn authored note changes source identity.\n"),
    )
    .unwrap();
    let mut graph = Graph::new(tmp.path());
    graph
        .resolve(vec!["devices/reterminal/AGENTS.md".into()])
        .unwrap();
    assert_ne!(graph.fingerprint(), before);
    assert_eq!(
        graph.files["devices/reterminal/AGENTS.md"],
        digest(&path).unwrap()
    );
}

#[test]
fn disjoint_physical_revisions_cannot_combine_into_a_verified_variant() {
    let (_tmp, mut c, mut r) = fixture();
    c.targets.get_mut("linux/test").unwrap()["variants"] =
        json!({"a":{"device_variant":"a","inputs":[],"dependencies":[]}});
    c.targets.get_mut("linux/test").unwrap()["default_variant"] = json!("a");
    c.devices.get_mut("test").unwrap()["variants"] = json!({"a":{"facts":{"compatible_revisions":{"status":"documented","value":["rev1","rev2"]}}}});
    c.devices.get_mut("test").unwrap()["acceptance"]["cases"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"recovery","source":"devices/test/AGENTS.md","interfaces":["gpio"]}));
    r["kind"] = json!("device");
    r["variant"] = json!("a");
    r["source_digest"] = json!(c
        .graph(c.target("linux/test").unwrap())
        .unwrap()
        .fingerprint());
    let dir = c.root.join("tests/devices/test/records");
    fs::create_dir_all(&dir).unwrap();
    let mut save = |file: &str, revision: &str, case: &str, status: &str, time: &str| {
        r["revision"] = json!(revision);
        r["result"] = json!(status);
        r["executed_at"] = json!(time);
        r["cases"] = json!([{"name":case,"status":status,"reason":if status=="passed"{""}else{"Synthetic failure"},"observation":"Synthetic fixture","procedure":"Offline fixture"}]);
        write(&dir.join(file), &r).unwrap();
    };
    save(
        "a.json",
        "rev1",
        "gpio-observe",
        "passed",
        "2026-11-26T00:00:00Z",
    );
    save(
        "b.json",
        "rev2",
        "recovery",
        "passed",
        "2026-11-26T00:00:01Z",
    );
    let state = evidence::support(&c, "linux/test").unwrap();
    assert_eq!(state["device"], "not_run");
    assert_eq!(state["interfaces"]["gpio"], "unknown");
    save(
        "c.json",
        "rev1",
        "recovery",
        "passed",
        "2026-11-26T00:00:02Z",
    );
    let state = evidence::support(&c, "linux/test").unwrap();
    assert_eq!(state["device"], "passed");
    assert_eq!(
        state["variants"]["a"]["revisions"]["rev1"]["device"],
        "passed"
    );
    assert_eq!(
        state["variants"]["a"]["revisions"]["rev2"]["device"],
        "not_run"
    );
    save(
        "d.json",
        "rev1",
        "gpio-observe",
        "failed",
        "2026-11-26T00:00:03Z",
    );
    assert_eq!(
        evidence::support(&c, "linux/test").unwrap()["device"],
        "failed"
    );
}

#[test]
fn native_metadata_rejects_unowned_sketch_demo_and_overlay_sources() {
    let c = Catalog::load(&repo()).unwrap();
    for tid in ["arduino/grove-beginner-kit", "linux/reterminal"] {
        let mut target = c.target(tid).unwrap().clone();
        let example = target["examples"][0].as_str().unwrap().to_owned();
        codecraft::platform::validate_target(&c.root, &target).unwrap();
        target["example_inputs"][example] = json!(["AGENTS.md"]);
        assert!(codecraft::platform::validate_target(&c.root, &target).is_err());
    }
    let mut target = c.target("linux/reterminal").unwrap().clone();
    target["inputs"] = json!(["AGENTS.md"]);
    assert!(codecraft::platform::validate_target(&c.root, &target).is_err());
}
