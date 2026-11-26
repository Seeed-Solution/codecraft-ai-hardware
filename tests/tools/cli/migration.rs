//! Public behavior retained when removing the Python reference implementation.
use codecraft::{catalog::Catalog, project, util::*};
use serde_json::json;
use std::{fs, path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}
fn export(tid: &str, example: &str) -> (tempfile::TempDir, PathBuf) {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("app");
    project::create(
        &Catalog::load(&root()).unwrap(),
        tid,
        example,
        None,
        &p,
        None,
        json!([]),
    )
    .unwrap();
    (d, p)
}
#[test]
fn every_example_exports_only_selected_sources_and_all_acceptance_references() {
    let c = Catalog::load(&root()).unwrap();
    for (id, t) in &c.targets {
        for ex in strings(&t["examples"]).unwrap() {
            let d = tempfile::tempdir().unwrap();
            let p = d.path().join("app");
            project::create(&c, id, &ex, None, &p, None, json!([])).unwrap();
            let exported = Catalog::load(&p).unwrap();
            assert_eq!(exported.targets[id]["examples"], json!([ex]));
            project::verify(&p).unwrap();
            for case in
                arr(&exported.devices[s(&t["device"]).unwrap()]["acceptance"]["cases"]).unwrap()
            {
                assert!(p.join(s(&case["source"]).unwrap()).is_file());
            }
            assert!(!files(&p)
                .unwrap()
                .iter()
                .any(|p| p.starts_with("tests/")
                    && p.rsplit('/').next().unwrap().starts_with("test_")));
            assert!(!p.join("tools/cli").exists());
            assert!(!p.join("tools/codecraft").exists());
            if t["platform"] == "arduino" {
                let sketches: Vec<_> = files(&p)
                    .unwrap()
                    .into_iter()
                    .filter(|x| x.ends_with(".ino"))
                    .collect();
                assert_eq!(sketches.len(), 1);
                assert!(sketches[0].ends_with(&format!("/{ex}.ino")));
            }
        }
    }
}
#[test]
fn brief_rules_are_literal_and_malformed_markers_preserve_lock_and_custom_rules() {
    let (_d, p) = export("linux/reterminal", "buttons_evdev");
    let mut profile = read(&p.join("project.yaml")).unwrap();
    profile["brief"]["purpose"] = json!("[fake](missing.md) <script>text</script>");
    write(&p.join("project.yaml"), &profile).unwrap();
    project::relock(&p, None, false).unwrap();
    let rules = fs::read_to_string(p.join("AGENTS.md")).unwrap();
    assert!(rules.contains("&lt;script&gt;"));
    assert!(!rules.contains("[fake](missing.md)"));
    let bad = format!("{rules}\n<!-- CODECRAFT_APPLICATION_START -->\ncustom\n");
    fs::write(p.join("AGENTS.md"), &bad).unwrap();
    let lock = fs::read(p.join("codecraft.lock")).unwrap();
    assert!(project::relock(&p, None, false).is_err());
    assert_eq!(fs::read(p.join("codecraft.lock")).unwrap(), lock);
    assert_eq!(fs::read_to_string(p.join("AGENTS.md")).unwrap(), bad);
}
#[test]
fn invalid_briefs_and_unknown_fields_do_not_leave_partial_projects() {
    let c = Catalog::load(&root()).unwrap();
    let d = tempfile::tempdir().unwrap();
    for (i, b) in [
        json!({"name":""}),
        json!({"surprise":true}),
        json!({"bom":[{"item":"x","quantity":0,"source":null}]}),
        json!({"target_users":"everyone"}),
    ]
    .into_iter()
    .enumerate()
    {
        let p = d.path().join(i.to_string());
        assert!(project::create(
            &c,
            "linux/reterminal",
            "buttons_evdev",
            None,
            &p,
            Some(b),
            json!([])
        )
        .is_err());
        assert!(!p.exists());
    }
}
#[test]
fn metadata_rejects_unknown_fields_duplicate_examples_and_self_certified_interfaces() {
    let (_d, p) = export("linux/reterminal", "buttons_evdev");
    let path = p.join("devices/reterminal/targets/target.json");
    let original = read(&path).unwrap();
    let mut mutations = vec![];
    let mut t = original.clone();
    t["unowned_cloud"] = json!({});
    mutations.push(t);
    let mut t = original.clone();
    t["examples"] = json!(["buttons_evdev", "buttons_evdev"]);
    mutations.push(t);
    let mut t = original.clone();
    t["interfaces"][0]["verification"] = json!("device-verified");
    mutations.push(t);
    let mut t = original.clone();
    t["example_dependencies"] = json!({});
    mutations.push(t);
    for bad in mutations {
        write(&path, &bad).unwrap();
        assert!(Catalog::load(&p).is_err());
    }
    write(&path, &original).unwrap();
    assert!(Catalog::load(&p).is_ok());
}
#[test]
fn graph_missing_and_cross_layer_resources_are_not_filtered() {
    let d = tempfile::tempdir().unwrap();
    let root = d.path();
    fs::write(root.join("leaf"), "a").unwrap();
    write(
        &root.join("resource.json"),
        &json!({"schema_version":1,"kind":"resource","inputs":["leaf"],"dependencies":[]}),
    )
    .unwrap();
    let mut g = Graph::new(root);
    g.add("resource.json").unwrap();
    assert!(g.files.contains_key("leaf"));
    fs::remove_file(root.join("leaf")).unwrap();
    assert!(Graph::new(root).add("resource.json").is_err());
}
#[test]
fn project_build_routes_never_bypass_source_lock() {
    let (_d, p) = export("linux/reterminal", "buttons_evdev");
    fs::write(p.join("new.py"), "pass").unwrap();
    assert!(project::build(&p).is_err());
    let out = Command::new(env!("CARGO_BIN_EXE_codecraft"))
        .arg("--root")
        .arg(&p)
        .args(["validate", "--target", "linux/reterminal"])
        .output()
        .unwrap();
    assert!(!out.status.success());
}
#[cfg(unix)]
#[test]
fn links_cannot_redirect_rules_or_project_output_and_secrets_are_not_packaged() {
    use std::os::unix::fs::symlink;
    let (d, p) = export("linux/reterminal", "buttons_evdev");
    let outside = d.path().join("outside");
    fs::write(&outside, "preserve").unwrap();
    fs::remove_file(p.join("AGENTS.md")).unwrap();
    symlink(&outside, p.join("AGENTS.md")).unwrap();
    assert!(project::relock(&p, None, false).is_err());
    assert_eq!(fs::read_to_string(&outside).unwrap(), "preserve");
    let c = Catalog::load(&root()).unwrap();
    let link = d.path().join("source-link");
    symlink(&c.root, &link).unwrap();
    assert!(project::create(
        &c,
        "linux/reterminal",
        "buttons_evdev",
        None,
        &link.join("devices/new-app"),
        None,
        json!([])
    )
    .is_err());
}
#[test]
fn firmware_layout_rejects_overlap_capacity_alignment_and_role_drift() {
    let good = json!({"schema_version":1,"address_space":"flash","fill":255,"format":"bin","purpose":"factory","base_address":0,"flash_capacity":1048576,"partition_table_offset":32768,"segments":[{"offset":0,"size":64,"role":"bootloader"},{"offset":32768,"size":64,"role":"partition-table"},{"offset":65536,"size":64,"role":"app"},{"offset":131072,"size":64,"role":"data"}],"partitions":[{"offset":65536,"size":65536,"type":0},{"offset":131072,"size":65536,"type":1}],"write_range":{"start":0,"end":131136}});
    codecraft::platform::esp_idf::image::validate(&good).unwrap();
    for (pointer, value) in [
        ("/segments/2/offset", json!(32768)),
        ("/segments/0/size", json!(32769)),
        ("/segments/2/size", json!(65537)),
        ("/segments/3/offset", json!(131073)),
        ("/flash_capacity", json!(131072)),
        ("/base_address", json!(-1)),
        ("/segments/3/role", json!("app")),
        ("/write_range/end", json!(1)),
    ] {
        let mut bad = good.clone();
        *bad.pointer_mut(pointer).unwrap() = value;
        assert!(
            codecraft::platform::esp_idf::image::validate(&bad).is_err(),
            "{pointer}"
        );
    }
}
#[test]
fn current_device_sources_are_preserved_byte_for_byte_in_exports() {
    let (_d, p) = export("esp-idf/reterminal-e1001", "epd_hello");
    let relative = "devices/reterminal-e1001/AGENTS.md";
    assert_eq!(
        fs::read(root().join(relative)).unwrap(),
        fs::read(p.join(relative)).unwrap()
    );
    let old = read(&p.join("codecraft.lock")).unwrap();
    let mut bytes = fs::read(p.join(relative)).unwrap();
    bytes.extend_from_slice(b"\n## Application notes\n\nAuthored note.\n");
    fs::write(p.join(relative), bytes).unwrap();
    assert_eq!(project::verify(&p).unwrap()["status"], "failed");
    project::relock(&p, None, false).unwrap();
    let new = read(&p.join("codecraft.lock")).unwrap();
    assert_eq!(old["origin"], new["origin"]);
    assert_ne!(old["files"][relative], new["files"][relative]);
}
#[test]
fn bsp_profiles_reject_unmodeled_commands_duplicates_and_path_escape() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("bsp.cmake");
    let valid="set(CODECRAFT_BSP_SOURCES \"src/bsp_button.c\")\nset(CODECRAFT_BSP_REQUIRES freertos log)\n";
    fs::write(&p, valid).unwrap();
    codecraft::platform::esp_idf::composition::settings(&p).unwrap();
    for bad in [
        valid.replace("src/bsp_button.c", "../external.c"),
        valid.replace("src/bsp_button.c", "${OTHER_SOURCE}"),
        valid.replace("freertos log", "freertos log log"),
        valid.replace("freertos log", ""),
        format!("{valid}{valid}"),
        format!("{valid}include(other.cmake)\n"),
        valid.replace("CODECRAFT_BSP_REQUIRES", "UNKNOWN"),
    ] {
        fs::write(&p, bad).unwrap();
        assert!(codecraft::platform::esp_idf::composition::settings(&p).is_err());
    }
}
#[test]
fn every_bsp_manifest_and_example_preserves_native_composition_and_license() {
    let c = Catalog::load(&root()).unwrap();
    let regex = regex::Regex::new(r"(?m)^#define\s+(BSP_\w+)\b").unwrap();
    for t in c.targets.values().filter(|t| t["platform"] == "esp-idf") {
        codecraft::platform::esp_idf::composition::validate(&c.root, t).unwrap();
        let base = c.root.join(s(&t["path"]).unwrap());
        let pins = fs::read_to_string(base.join("bsp/include/bsp_pins.h")).unwrap();
        assert!(pins.contains("SPDX-License-Identifier: Apache-2.0"));
        let names: Vec<_> = regex
            .captures_iter(&pins)
            .map(|c| c[1].to_owned())
            .collect();
        assert_eq!(
            names.len(),
            names
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        );
        for ex in strings(&t["examples"]).unwrap() {
            assert!(base
                .join("examples")
                .join(&ex)
                .join("sdkconfig.defaults")
                .is_file());
            assert!(strings(&t["example_dependencies"][&ex])
                .unwrap()
                .contains(&format!(
                    "{}/examples/{ex}/dependencies.lock",
                    s(&t["path"]).unwrap()
                )));
        }
    }
}
#[test]
fn l2_scan_retains_its_native_source_and_wireless_interface() {
    let c = Catalog::load(&root()).unwrap();
    let t = c
        .selected("esp-idf/wio-tracker-l2", Some("wifi_scan"), None)
        .unwrap();
    let selected =
        codecraft::platform::esp_idf::composition::describe(&c.root, &t, "wifi_scan").unwrap();
    assert_eq!(selected.sources.len(), 1);
    assert!(selected
        .sources
        .iter()
        .next()
        .unwrap()
        .ends_with("bsp/src/bsp_wifi.c"));
    let interfaces = arr(&t["interfaces"]).unwrap();
    assert_eq!(interfaces.len(), 1);
    assert_eq!(interfaces[0]["id"], "wireless");
}

#[test]
fn compiler_metadata_rejects_unselected_sources_and_wrong_variant_definitions() {
    let c = Catalog::load(&root()).unwrap();
    let t = c.target("esp-idf/sensecap-indicator").unwrap();
    let ex = "display_color_bars";
    let chosen = codecraft::platform::esp_idf::composition::describe(&c.root, t, ex).unwrap();
    let d = tempfile::tempdir().unwrap();
    let description = json!({"build_components":["main","bsp"],"build_component_info":{"bsp":{"sources":chosen.sources,"reqs":chosen.requirements}}});
    write(&d.path().join("project_description.json"), &description).unwrap();
    fs::write(
        d.path().join("CMakeCache.txt"),
        "CODECRAFT_HARDWARE_VARIANT:STRING=dx\n",
    )
    .unwrap();
    let commands:Vec<_>=chosen.sources.iter().map(|p|json!({"file":p,"directory":c.root,"arguments":["cc","-DCODECRAFT_DISPLAY_GX=0","-c",p]})).collect();
    write(&d.path().join("compile_commands.json"), &json!(commands)).unwrap();
    codecraft::platform::esp_idf::verify_composition(&c.root, t, ex, Some("dx"), d.path()).unwrap();
    for flags in [
        json!(["cc", "-c"]),
        json!(["cc", "-DCODECRAFT_DISPLAY_GX=1"]),
        json!(["cc", "-DCODECRAFT_DISPLAY_GX=0", "-DCODECRAFT_DISPLAY_GX=1"]),
    ] {
        let mut bad = commands.clone();
        bad[0]["arguments"] = flags;
        write(&d.path().join("compile_commands.json"), &json!(bad)).unwrap();
        assert!(codecraft::platform::esp_idf::verify_composition(
            &c.root,
            t,
            ex,
            Some("dx"),
            d.path()
        )
        .is_err());
    }
    write(&d.path().join("compile_commands.json"), &json!(commands)).unwrap();
    fs::write(
        d.path().join("CMakeCache.txt"),
        "CODECRAFT_HARDWARE_VARIANT:STRING=gx\n",
    )
    .unwrap();
    assert!(
        codecraft::platform::esp_idf::verify_composition(&c.root, t, ex, Some("dx"), d.path())
            .is_err()
    );
    let mut incomplete = description.clone();
    incomplete["build_components"] = json!(["bsp"]);
    write(&d.path().join("project_description.json"), &incomplete).unwrap();
    assert!(
        codecraft::platform::esp_idf::verify_composition(&c.root, t, ex, None, d.path()).is_err()
    );
    let mut bad = description;
    bad["build_component_info"]["bsp"]["sources"] = json!([]);
    write(&d.path().join("project_description.json"), &bad).unwrap();
    assert!(
        codecraft::platform::esp_idf::verify_composition(&c.root, t, ex, None, d.path()).is_err()
    );
}
