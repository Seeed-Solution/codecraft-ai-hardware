use codecraft::{maintenance_contracts as contracts, maintenance_skills as release, util::*};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn copy(source: &Path, dest: &Path) {
    fs::create_dir_all(dest).unwrap();
    for e in walkdir::WalkDir::new(source)
        .into_iter()
        .filter_entry(|e| e.file_name() != "__pycache__")
    {
        let e = e.unwrap();
        if e.file_type().is_file() {
            let p = dest.join(e.path().strip_prefix(source).unwrap());
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::copy(e.path(), p).unwrap();
        }
    }
}
fn fixture() -> (tempfile::TempDir, PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("repo");
    fs::create_dir(&dest).unwrap();
    for p in [
        "Cargo.toml",
        "Cargo.lock",
        "tests/requirements.txt",
        "tests/conftest.py",
    ] {
        let t = dest.join(p);
        fs::create_dir_all(t.parent().unwrap()).unwrap();
        fs::copy(root().join(p), t).unwrap();
    }
    for p in [
        "tools/cli",
        "tools/skill_packages",
        "skills",
        "tests/tools/skill_packages",
        "tests/skills",
        "docs",
    ] {
        copy(&root().join(p), &dest.join(p));
    }
    (tmp, dest)
}
#[test]
fn unchanged_candidates_are_reproducible_and_preserve_provenance() {
    let (tmp, repo) = fixture();
    let a = release::prepare(&repo, &tmp.path().join("a"), &json!({}), false).unwrap();
    let b = release::prepare(&repo, &tmp.path().join("b"), &json!({}), false).unwrap();
    assert_eq!(a["review_sha256"], b["review_sha256"]);
    assert_eq!(
        release::inventory(&tmp.path().join("a")).unwrap(),
        release::inventory(&tmp.path().join("b")).unwrap()
    );
    release::inspect(
        &repo,
        &tmp.path().join("a"),
        Some(s(&a["review_sha256"]).unwrap()),
        false,
    )
    .unwrap();
    assert_eq!(
        fs::read(root().join("tools/skill_packages/import.json")).unwrap(),
        fs::read(repo.join("tools/skill_packages/import.json")).unwrap()
    );
}
#[test]
fn version_review_candidate_tamper_and_source_drift_fail() {
    let (tmp, repo) = fixture();
    let file = repo.join("skills/codecraft-capabilities-asr/references/api.md");
    fs::write(
        &file,
        fs::read_to_string(&file).unwrap() + "\nReviewed fixture note.\n",
    )
    .unwrap();
    assert!(release::prepare(&repo, &tmp.path().join("bad"), &json!({}), false).is_err());
    assert!(!tmp.path().join("bad").exists());
    let v = json!({"codecraft-capabilities-asr":{"skill_version":"99.0.0"}});
    let out = tmp.path().join("good");
    let made = release::prepare(&repo, &out, &v, false).unwrap();
    assert_eq!(
        made["packages"]["codecraft-capabilities-asr"]["classification"],
        "documentation"
    );
    assert!(release::inspect(&repo, &out, Some(&"0".repeat(64)), false).is_err());
    fs::write(out.join("rogue.txt"), "extra").unwrap();
    assert!(release::inspect(&repo, &out, None, false).is_err());
    fs::remove_file(out.join("rogue.txt")).unwrap();
    fs::write(file, "drift").unwrap();
    assert!(release::inspect(&repo, &out, None, false).is_err());
}
#[test]
fn helper_is_owned_by_an_installed_package_and_conflicts_are_not_overwritten() {
    let (tmp, repo) = fixture();
    let helper = repo.join(release::HELPER);
    let bytes = fs::read(&helper).unwrap();
    for id in obj(&release::records(&repo, false).unwrap())
        .unwrap()
        .keys()
    {
        assert_eq!(
            fs::read(repo.join(format!("skills/{id}/scripts/device_api.py"))).unwrap(),
            bytes
        );
    }
    fs::write(
        &helper,
        [bytes, b"\n# fixture source change\n".to_vec()].concat(),
    )
    .unwrap();
    let mut versions = object();
    for id in obj(&release::records(&repo, false).unwrap())
        .unwrap()
        .keys()
    {
        versions[id] = json!({"skill_version":"99.0.0"});
    }
    let result = release::prepare(&repo, &tmp.path().join("runtime"), &versions, false).unwrap();
    assert!(obj(&result["packages"])
        .unwrap()
        .values()
        .all(|v| v["classification"] == "runtime"));
    let other = repo.join("skills/codecraft-capabilities-asr/scripts/device_api.py");
    fs::write(other, "unreviewed conflicting bytes").unwrap();
    assert!(release::prepare(&repo, &tmp.path().join("conflict"), &versions, false).is_err());
}
#[test]
fn sdk_candidate_pins_original_import_and_rejects_private_files() {
    let (tmp, repo) = fixture();
    let out = tmp.path().join("sdk");
    let made = release::prepare(&repo, &out, &json!({}), true).unwrap();
    release::inspect(&repo, &out, Some(s(&made["review_sha256"]).unwrap()), true).unwrap();
    let path = repo.join("skills/codecraft-development-recamera-pysdk/.env");
    fs::write(path, "fixture=true").unwrap();
    assert!(release::prepare(&repo, &tmp.path().join("private"), &json!({}), true).is_err());
}
#[test]
fn package_contracts_reject_weakening_defaults_and_version_confusion() {
    let root = root();
    for id in obj(&release::records(&root, false).unwrap())
        .unwrap()
        .keys()
    {
        let (r, c) = codecraft::skills::package(&root, id).unwrap();
        contracts::validate(&c, &r["files"]).unwrap();
        let mut bad = c.clone();
        bad["policy"]["configuration"]["additionalProperties"] = json!(true);
        assert!(contracts::validate(&bad, &r["files"]).is_err());
        let mut bad = c.clone();
        bad["policy"]["billing"] =
            json!({"status":"unknown","unit":"invented","basis":null,"amount":null});
        assert!(contracts::validate(&bad, &r["files"]).is_err());
    }
    let (r, mut c) = codecraft::skills::package(&root, "codecraft-capabilities-llm").unwrap();
    c["configuration"]["properties"]["max_tokens"]["policy_default"] = json!(false);
    assert!(contracts::validate(&c, &r["files"]).is_err());
}
#[test]
fn unknown_missing_and_duplicate_release_options_fail() {
    let root = root();
    for args in [
        vec!["skill-release", "prepare", "--wat", "x"],
        vec!["skill-release", "prepare", "--output"],
        vec![
            "check-cloud-skills",
            "--skills-root",
            "a",
            "--skills-root",
            "b",
        ],
        vec!["skill-release", "check", "--candidate", "x", "junk"],
    ] {
        assert!(release::command(
            &root,
            &args.into_iter().map(str::to_owned).collect::<Vec<_>>()
        )
        .is_err());
    }
}
#[test]
fn package_inventory_rejects_symlinks_and_preserves_binary_assets() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("asset.bin"), [0, 255, 1, 2]).unwrap();
    assert!(release::inventory(tmp.path())
        .unwrap()
        .get("asset.bin")
        .is_some());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("asset.bin", tmp.path().join("link")).unwrap();
        assert!(release::inventory(tmp.path()).is_err());
    }
}
#[test]
fn reviewed_candidate_runs_real_offline_clients_and_applies_only_reviewed_bytes() {
    let (tmp, repo) = fixture();
    let sdk = repo.join("skills/codecraft-development-recamera-pysdk/SKILL.md");
    fs::write(
        &sdk,
        fs::read_to_string(&sdk).unwrap() + "\nReviewed release fixture.\n",
    )
    .unwrap();
    let out = tmp.path().join("candidate");
    let made = release::prepare(
        &repo,
        &out,
        &json!({"codecraft-development-recamera-pysdk":{"skill_version":"99.0.0"}}),
        true,
    )
    .unwrap();
    std::env::set_var("CODECRAFT_TEST_CLI", env!("CARGO_BIN_EXE_codecraft"));
    let applied = release::apply(&repo, &out, s(&made["review_sha256"]).unwrap(), true).unwrap();
    assert_eq!(applied["status"], "passed");
    assert_eq!(applied["repository_changed"], true);
    assert!(!repo.join(".codecraft/skill-release.lock").exists());
    assert_eq!(
        fs::read(root().join("docs/recamera-skill-import.json")).unwrap(),
        fs::read(repo.join("docs/recamera-skill-import.json")).unwrap()
    );
}
