use super::*;
fn fixture() -> (tempfile::TempDir, PathBuf, PathBuf, J) {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path().join("repo");
    let candidate = tmp.path().join("candidate");
    let pkg = format!("skills/{SDK}");
    let record = SDK_RECORD;
    fs::create_dir_all(repo.join(&pkg)).unwrap();
    fs::write(repo.join(&pkg).join("SKILL.md"), "original").unwrap();
    write(&repo.join(record), &json!({"original":true})).unwrap();
    fs::create_dir_all(candidate.join(&pkg)).unwrap();
    fs::write(candidate.join(&pkg).join("SKILL.md"), "new").unwrap();
    write(&candidate.join(record), &json!({"new":true})).unwrap();
    let m = json!({"baseline_packages":{SDK:inventory(&repo.join(&pkg)).unwrap()},"packages":{SDK:{"skill_path":pkg,"record_path":record,"previous":{"old":true},"proposed":{"files":inventory(&candidate.join(format!("skills/{SDK}"))).unwrap()}}},"files":{record:digest(&candidate.join(record)).unwrap()}});
    (tmp, repo, candidate, m)
}
#[test]
fn failed_metadata_replacement_rolls_back_package_and_record() {
    let (_tmp, repo, candidate, m) = fixture();
    let before = inventory(&repo).unwrap();
    let result = install_transaction(&repo, &candidate, &m, true, &mut |a, b| {
        if b.ends_with("release.json") {
            Err(std::io::Error::other("injected metadata failure"))
        } else {
            fs::rename(a, b)
        }
    });
    assert!(result.is_err());
    assert_eq!(before, inventory(&repo).unwrap());
    assert!(!fs::read_dir(repo.join("skills")).unwrap().any(|e| e
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with(".release-")));
}
#[test]
fn failed_restoration_retains_original_bytes_for_recovery() {
    let (_tmp, repo, candidate, m) = fixture();
    let result = install_transaction(&repo, &candidate, &m, true, &mut |a, b| {
        if b.ends_with("release.json")
            || a.file_name().unwrap().to_string_lossy().starts_with("old-")
        {
            Err(std::io::Error::other(
                "injected replacement/restoration failure",
            ))
        } else {
            fs::rename(a, b)
        }
    })
    .unwrap_err();
    assert!(result.to_string().contains("recovery files retained"));
    let stage = fs::read_dir(repo.join("skills"))
        .unwrap()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with(".release-"))
        .unwrap()
        .path();
    assert_eq!(
        fs::read_to_string(stage.join(format!("old-{SDK}/SKILL.md"))).unwrap(),
        "original"
    );
}

#[test]
fn captured_subprocess_times_out_without_pipe_deadlock() {
    let mut cmd = Command::new(
        std::env::var("CODECRAFT_TEST_PYTHON")
            .or_else(|_| std::env::var("PYTHON"))
            .unwrap_or_else(|_| "python3".into()),
    );
    cmd.args([
        "-I",
        "-c",
        "import sys,time; sys.stdout.write('x'*200000); sys.stdout.flush(); time.sleep(5)",
    ]);
    let started = std::time::Instant::now();
    let result = captured(&mut cmd, std::time::Duration::from_millis(150));
    assert!(result.unwrap_err().to_string().contains("exceeded"));
    assert!(started.elapsed() < std::time::Duration::from_secs(3));
}
