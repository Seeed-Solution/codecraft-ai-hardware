use codecraft::{maintenance_reference as reference, util::*};
use serde_json::json;
use std::{fs, path::PathBuf, process::Command};
fn fixture() -> (tempfile::TempDir, PathBuf, PathBuf, String) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    let sdk = tmp.path().join("sdk");
    fs::create_dir_all(&sdk).unwrap();
    for path in reference::EXTRA_SOURCES {
        let p = sdk.join(path);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(
            &p,
            if path.ends_with(".py") {
                "# fixture\n"
            } else {
                "fixture source\n"
            },
        )
        .unwrap();
    }
    for module in ["kit", "sdk/python/recamera_ext"] {
        let p = sdk.join(module).join("__init__.py");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p,"\"\"\"Synthetic module.\"\"\"\nraise RuntimeError('Do not execute SDK')\ndef ping(value: str = 'ok') -> str:\n    \"\"\"Return supplied value.\"\"\"\n    return value\n").unwrap();
    }
    for args in [
        vec!["init", "--quiet"],
        vec!["add", "."],
        vec!["commit", "--quiet", "-m", "test: synthetic SDK"],
    ] {
        assert!(Command::new("git")
            .args([
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "-C"
            ])
            .arg(&sdk)
            .args(args)
            .status()
            .unwrap()
            .success());
    }
    let rev = String::from_utf8(
        Command::new("git")
            .arg("-C")
            .arg(&sdk)
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    write(&root.join("tools/skill_packages/recamera/api-notes.json"),&json!({"modules":{"kit":"Synthetic Kit.","recamera_ext":"Synthetic binding."},"symbols":{},"common":{}})).unwrap();
    write(
        &root.join("tools/skill_packages/recamera/api-http-notes.json"),
        &json!({}),
    )
    .unwrap();
    write(
        &root.join(format!(
            "skills/{}/scripts/api-reference-lock.json",
            reference::SKILL
        )),
        &json!({"source_revision":rev,"generated_files":{}}),
    )
    .unwrap();
    (tmp, root, sdk, rev)
}
#[test]
fn ast_reference_round_trip_never_executes_sdk_and_detects_page_and_source_drift() {
    let (_tmp, root, sdk, rev) = fixture();
    assert_eq!(
        reference::reference(&root, &sdk, true, Some(&rev)).unwrap()["status"],
        "passed"
    );
    assert_eq!(
        reference::reference(&root, &sdk, false, None).unwrap()["status"],
        "passed"
    );
    let page = root.join(format!(
        "skills/{}/references/api/python/kit.md",
        reference::SKILL
    ));
    fs::write(
        &page,
        fs::read_to_string(&page).unwrap() + "\nmanual edit\n",
    )
    .unwrap();
    assert_eq!(
        reference::reference(&root, &sdk, false, None).unwrap()["changed_pages"],
        json!(["python/kit.md"])
    );
    let source = sdk.join("kit/__init__.py");
    fs::write(
        &source,
        fs::read_to_string(&source)
            .unwrap()
            .replace("value: str", "value: int"),
    )
    .unwrap();
    let result = reference::reference(&root, &sdk, false, None).unwrap();
    assert_eq!(result["status"], "failed");
    assert_eq!(result["source_changes"], json!(["kit/__init__.py"]));
}
#[test]
fn reference_write_requires_explicit_matching_clean_source_and_rejects_invalid_syntax() {
    let (_tmp, root, sdk, rev) = fixture();
    assert!(reference::reference(&root, &sdk, true, None).is_err());
    assert!(reference::reference(&root, &sdk, true, Some(&"0".repeat(40))).is_err());
    fs::write(sdk.join("unreviewed"), "source").unwrap();
    assert!(reference::reference(&root, &sdk, true, Some(&rev)).is_err());
    fs::write(sdk.join("kit/__init__.py"), "this is not Python!\n").unwrap();
    assert!(reference::reference(&root, &sdk, false, None)
        .unwrap_err()
        .to_string()
        .contains("SDK source parsing failed"));
}
#[test]
fn http_routes_cover_literals_constants_regex_and_reject_dynamic_paths() {
    let source="ROUTE = '/health'\nclass Handler:\n    def do_GET(self):\n        if path == ROUTE: pass\n        if re.fullmatch(r'/app/[a-z]+', path): pass\n    def do_POST(self):\n        if path == '/upload': pass\n";
    let routes = reference::http_routes(source).unwrap();
    assert_eq!(routes.len(), 3);
    assert!(routes.contains("GET /health"));
    assert!(
        reference::http_routes("def do_GET():\n    if path.startswith('/api'): pass\n").is_err()
    );
    assert!(reference::http_routes("def do_GET():\n    if path == UNKNOWN: pass\n").is_err());
}
#[test]
fn public_definitions_preserve_properties_fields_aliases_inheritance_and_async_signatures() {
    let (_tmp, root, sdk, rev) = fixture();
    fs::write(sdk.join("kit/__init__.py"),"from recamera_ext import ping as exported\n__all__ = ['Base', 'Child', 'exported', 'run']\nclass Base:\n    \"\"\"Base contract.\"\"\"\n    value: int = 1\n    @property\n    def status(self) -> bool:\n        \"\"\"State.\"\"\"\n        return True\nclass Child(Base):\n    \"\"\"Child contract.\"\"\"\n    pass\nasync def run(x, /, *, flag: bool=False):\n    \"\"\"Run asynchronously.\"\"\"\n    pass\n").unwrap();
    let notes = read(&root.join("tools/skill_packages/recamera/api-notes.json")).unwrap();
    let (_, snapshot) = reference::build(&sdk, &rev, &notes, &json!({})).unwrap();
    assert_eq!(
        snapshot["symbols"]["kit.exported"]["target"],
        "recamera_ext.ping"
    );
    assert_eq!(snapshot["symbols"]["kit.Child.status"]["kind"], "callable");
    assert_eq!(snapshot["symbols"]["kit.Base.value"]["kind"], "field");
    assert!(s(&snapshot["symbols"]["kit.run"]["declaration"])
        .unwrap()
        .contains("async def run(x, /, *, flag: bool=False)"));
}
