use codecraft::{maintenance_checks as checks, util::*};
use serde_json::json;
use std::{fs, path::Path};
fn put(root: &Path, path: &str, text: &str) {
    let p = root.join(path);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, text).unwrap();
}
fn fixture(root: &Path) {
    put(
        root,
        "tests/conftest.py",
        &fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conftest.py"))
            .unwrap(),
    );
    put(
        root,
        "tests/tools/unit/test_one.py",
        "def test_one(): pass\n",
    );
    put(
        root,
        "tests/platforms/linux/test_linux.py",
        "def test_linux(): pass\n",
    );
    put(
        root,
        "tests/platforms/arduino/test_avr.py",
        "def test_avr(): pass\n",
    );
    put(
        root,
        "tests/tools/unit/fixtures/test_hidden.py",
        "raise Exception()\n",
    );
    put(
        root,
        "tests/devices/a/targets/test_board.py",
        "def test_board(): pass\n",
    );
    put(
        root,
        "tests/devices/a/records/test_observation.py",
        "raise Exception()\n",
    );
    write(
        &root.join("devices/a/targets/target.json"),
        &json!({"device":"a","path":"devices/a/targets","id":"linux/a","platform":"linux"}),
    )
    .unwrap();
}
#[test]
fn discovery_respects_owners_platforms_and_exclusions() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    let all = checks::test_files(tmp.path(), None, None).unwrap();
    assert_eq!(all.len(), 4);
    let linux = checks::test_files(tmp.path(), Some("linux"), None).unwrap();
    assert_eq!(linux.len(), 3);
    assert!(linux.contains(&"tests/devices/a/targets/test_board.py".into()));
    assert!(checks::test_files(tmp.path(), None, Some("tests/devices/a/records")).is_err());
    assert!(checks::test_files(tmp.path(), None, Some("../tests")).is_err());
}
#[test]
fn mismatched_device_test_ownership_is_rejected() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    write(
        &tmp.path().join("devices/a/targets/target.json"),
        &json!({"device":"other","path":"devices/a/targets","id":"linux/a","platform":"linux"}),
    )
    .unwrap();
    assert!(checks::test_files(tmp.path(), None, None).is_err());
}
#[cfg(unix)]
#[test]
fn symlink_test_paths_are_rejected() {
    let tmp = tempfile::tempdir().unwrap();
    fixture(tmp.path());
    std::os::unix::fs::symlink(
        "test_one.py",
        tmp.path().join("tests/tools/unit/test_link.py"),
    )
    .unwrap();
    assert!(checks::test_files(tmp.path(), None, None).is_err());
}
#[test]
fn junit_preserves_failed_skipped_and_fixture_errors() {
    let xml = r#"<testsuites><testsuite><testcase classname="tests.tools.unit.test_one" name="test_ok"/><testcase classname="tests.tools.unit.test_one.Group" name="test_bad"><failure message="bad &amp; worse">trace</failure></testcase><testcase classname="tests.tools.unit.test_one" name="test_skip"><skipped message="missing dependency"/></testcase><testcase classname="tests.tools.unit.test_one" name="setup"><error message="failed on setup with fixture failed"/></testcase></testsuite></testsuites>"#;
    let rows = checks::junit_results(xml, "tests/tools/unit/test_one.py").unwrap();
    assert_eq!(rows.len(), 5);
    assert_eq!(rows[0]["status"], "passed");
    assert_eq!(
        rows[1]["name"],
        "tests/tools/unit/test_one.py::Group::test_bad"
    );
    assert_eq!(rows[1]["reason"], "bad & worse");
    assert_eq!(rows[2]["status"], "skipped");
    assert_eq!(rows[3]["status"], "failed");
    assert_eq!(rows[4]["status"], "not_run");
    assert!(checks::junit_results("<testsuite/>", "x").is_err());
}
#[test]
fn duplicate_junit_ids_are_rejected() {
    assert!(checks::junit_results(
        r#"<testsuite><testcase name="same"/><testcase name="same"/></testsuite>"#,
        "x"
    )
    .is_err());
}
#[test]
fn documentation_checks_named_links_and_ignores_code_examples() {
    let tmp = tempfile::tempdir().unwrap();
    put(
        tmp.path(),
        "guide.md",
        "[other][ref]\n[ref]: other.md\n```md\n[example](does-not-exist.md)\n```\n",
    );
    put(tmp.path(), "other.md", "# Other\n");
    checks::documentation(tmp.path()).unwrap();
    put(tmp.path(), "guide.md", "[other][missing]\n");
    assert!(checks::documentation(tmp.path()).is_err());
    put(tmp.path(), "guide.md", "[other](missing.md)\n");
    assert!(checks::documentation(tmp.path()).is_err());
}
#[test]
fn documentation_requires_readme_parity_and_english_engineering_files() {
    let tmp = tempfile::tempdir().unwrap();
    put(tmp.path(), "README.md", "[中文](README.zh_CN.md)\n");
    assert!(checks::documentation(tmp.path()).is_err());
    put(tmp.path(), "README.zh_CN.md", "[English](README.md)\n");
    checks::documentation(tmp.path()).unwrap();
    put(tmp.path(), "guide.zh_CN.md", "bad duplicate\n");
    assert!(checks::documentation(tmp.path()).is_err());
}
#[test]
fn pytest_lifecycle_and_node_selection_are_reported() {
    let tmp = tempfile::tempdir().unwrap();
    put(
        tmp.path(),
        "tests/conftest.py",
        &fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conftest.py"))
            .unwrap(),
    );
    put(tmp.path(),"tests/tools/unit/test_case.py","import unittest\nclass TestFixture(unittest.TestCase):\n    @classmethod\n    def setUpClass(cls): cls.value=42\n    def test_ok(self): self.assertEqual(self.value,42)\n    def test_sub(self):\n        with self.subTest(case=1): self.assertEqual(1,2)\ndef test_skip(): raise unittest.SkipTest('explicit skip')\n");
    let output = checks::run_tests(tmp.path(), &["tests/tools/unit/test_case.py".into()]).unwrap();
    assert_eq!(output["ok"], false);
    let rows = arr(&output["results"]).unwrap();
    assert_eq!(rows.len(), 3);
    assert!(rows.iter().any(|x| x["status"] == "failed"));
    assert!(rows.iter().any(|x| x["status"] == "skipped"));
    let output = checks::run_tests(
        tmp.path(),
        &["tests/tools/unit/test_case.py::TestFixture::test_ok".into()],
    )
    .unwrap();
    assert_eq!(output["ok"], true);
    assert_eq!(arr(&output["results"]).unwrap().len(), 1);
}
#[test]
fn collection_never_executes_bodies_and_rejects_alias_async_import_failures() {
    let tmp = tempfile::tempdir().unwrap();
    put(
        tmp.path(),
        "tests/conftest.py",
        &fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conftest.py"))
            .unwrap(),
    );
    put(tmp.path(), "tests/tools/unit/test_valid.py", "from shutil import copy as test_imported\nimport unittest\ndef test_body(): raise AssertionError('body executed')\nclass Cases(unittest.TestCase):\n    def test_case(self): raise AssertionError('body executed')\n");
    let collected = checks::inventory(tmp.path(), None, None).unwrap();
    assert_eq!(arr(&collected["tests"]).unwrap().len(), 2);
    assert!(arr(&collected["collection_errors"]).unwrap().is_empty());
    for (name, source) in [
        ("alias", "def test_a(): pass\ntest_alias=test_a\n"),
        ("async", "async def test_async(): pass\n"),
        ("syntax", "def invalid(\n"),
        ("import", "raise ImportError('fixture')\n"),
        ("empty", "value=1\n"),
        ("exit", "raise SystemExit(0)\n"),
    ] {
        put(
            tmp.path(),
            &format!("tests/tools/unit/test_{name}.py"),
            source,
        );
    }
    let collected = checks::inventory(tmp.path(), None, None).unwrap();
    assert_eq!(arr(&collected["collection_errors"]).unwrap().len(), 6);
}
#[test]
fn fixture_failures_expected_failures_and_unexpected_successes_are_distinct() {
    let tmp = tempfile::tempdir().unwrap();
    put(
        tmp.path(),
        "tests/conftest.py",
        &fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conftest.py"))
            .unwrap(),
    );
    put(tmp.path(), "tests/tools/unit/test_states.py", "import unittest\nclass Broken(unittest.TestCase):\n    @classmethod\n    def setUpClass(cls): raise RuntimeError('setup failed')\n    def test_never(self): raise AssertionError('never reached')\nclass Cases(unittest.TestCase):\n    @unittest.expectedFailure\n    def test_expected(self): self.fail()\n    @unittest.expectedFailure\n    def test_unexpected(self): pass\n");
    let output =
        checks::run_tests(tmp.path(), &["tests/tools/unit/test_states.py".into()]).unwrap();
    let rows = arr(&output["results"]).unwrap();
    assert!(rows
        .iter()
        .any(|r| r["name"].as_str().unwrap().ends_with("test_never") && r["status"] == "not_run"));
    assert!(rows.iter().any(
        |r| r["name"].as_str().unwrap().ends_with("test_expected") && r["status"] == "skipped"
    ));
    assert!(rows.iter().any(
        |r| r["name"].as_str().unwrap().ends_with("test_unexpected") && r["status"] == "failed"
    ));
    assert!(rows
        .iter()
        .any(|r| r["name"].as_str().unwrap().ends_with("::fixture") && r["status"] == "failed"));
}

#[test]
fn teardown_failure_is_failed_rather_than_an_unperformed_case() {
    let rows=checks::junit_results(r#"<testsuite><testcase name="test_body"><error message="failed on teardown with cleanup failure"/></testcase></testsuite>"#,"test_fixture.py").unwrap();
    assert!(rows.iter().all(|r| r["status"] == "failed"));
}
