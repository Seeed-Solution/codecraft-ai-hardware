use std::{fs, path::PathBuf, process::Command};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn release_installer_checks_hash_and_version_before_replacing_binary() {
    check_installer("powershell.exe");
}

#[cfg(windows)]
#[test]
fn powershell7_installer_checks_hash_and_version_before_replacing_binary() {
    check_installer("pwsh.exe");
}

fn check_installer(_shell: &str) {
    let temp = tempfile::tempdir().unwrap();
    let fixtures = temp.path().join("fixtures");
    let bin = temp.path().join("bin");
    let destination = temp.path().join("destination with spaces");
    fs::create_dir_all(&fixtures).unwrap();
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&destination).unwrap();
    let executable = std::path::Path::new(env!("CARGO_BIN_EXE_codecraft"));
    let asset = if cfg!(windows) {
        "codecraft-x86_64-pc-windows-msvc.exe"
    } else if cfg!(target_os = "macos") {
        "codecraft-aarch64-apple-darwin"
    } else {
        "codecraft-x86_64-unknown-linux-musl"
    };
    fs::copy(executable, fixtures.join(asset)).unwrap();
    let hashes = format!(
        "{}  {asset}\n",
        codecraft::util::digest(executable).unwrap()
    );
    let installed = destination.join(if cfg!(windows) {
        "codecraft.exe"
    } else {
        "codecraft"
    });
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // Intercept downloads only; no network or compiler is involved.
        fs::write(bin.join("curl"), r#"#!/bin/sh
while [ $# -gt 0 ]; do case "$1" in https://*) url=$1 ;; -o) shift; output=$1 ;; esac; shift; done
printf '%s\n' "$url" >> "$CODECRAFT_INSTALL_FIXTURES/requests"
case "$url" in
    */releases/latest) printf 'https://github.com/Links17/codecraft-ai-hardware/releases/tag/%s' "$(/bin/cat "$CODECRAFT_INSTALL_FIXTURES/latest-tag")" ;;
    *) /bin/cp "$CODECRAFT_INSTALL_FIXTURES/${url##*/}" "$output" ;;
esac
"#).unwrap();
        fs::set_permissions(bin.join("curl"), fs::Permissions::from_mode(0o755)).unwrap();
    }
    #[cfg(windows)]
    {
        fs::write(bin.join("run.ps1"), r#"param([string]$Installer,[string]$Version)
function Invoke-RestMethod { param([string]$Uri)
    Add-Content (Join-Path $env:CODECRAFT_INSTALL_FIXTURES 'requests') $Uri
    @{ tag_name = (Get-Content (Join-Path $env:CODECRAFT_INSTALL_FIXTURES 'latest-tag')); draft = $false; prerelease = $false }
}
function Invoke-WebRequest { param([switch]$UseBasicParsing,[string]$Uri,[string]$OutFile)
    Add-Content (Join-Path $env:CODECRAFT_INSTALL_FIXTURES 'requests') $Uri
    Copy-Item (Join-Path $env:CODECRAFT_INSTALL_FIXTURES ($Uri.Split('/')[-1])) $OutFile -ErrorAction Stop
}
if ($Version) { & $Installer -Version $Version } else { & $Installer }
"#).unwrap();
    }
    let run = |version: &str| {
        #[cfg(unix)]
        let mut cmd = {
            let mut c = Command::new("sh");
            c.arg(root().join("tools/cli/install.sh"));
            if !version.is_empty() {
                c.args(["--version", version]);
            }
            c
        };
        #[cfg(windows)]
        let mut cmd = {
            let mut c = Command::new(_shell);
            c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"])
                .arg(bin.join("run.ps1"))
                .arg(root().join("tools/cli/install.ps1"));
            if !version.is_empty() {
                c.arg(version);
            }
            c
        };
        let mut paths = vec![bin.clone()];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        cmd.env("PATH", std::env::join_paths(paths).unwrap())
            .env("CODECRAFT_INSTALL_FIXTURES", &fixtures)
            .env("CODECRAFT_BIN_DIR", &destination)
            .output()
            .unwrap()
    };
    fs::write(&installed, b"existing installation").unwrap();
    fs::write(
        fixtures.join("SHA256SUMS"),
        format!("{}  {asset}\n", "0".repeat(64)),
    )
    .unwrap();
    assert!(!run("0.5.0").status.success());
    assert_eq!(fs::read(&installed).unwrap(), b"existing installation");
    fs::write(fixtures.join("SHA256SUMS"), &hashes).unwrap();
    assert!(!run("0.5.1").status.success());
    assert_eq!(fs::read(&installed).unwrap(), b"existing installation");
    fs::write(fixtures.join("latest-tag"), "v0.5.1").unwrap();
    assert!(!run("").status.success());
    assert_eq!(fs::read(&installed).unwrap(), b"existing installation");
    fs::write(fixtures.join("latest-tag"), "v0.5.0-rc1").unwrap();
    assert!(!run("").status.success());
    assert_eq!(fs::read(&installed).unwrap(), b"existing installation");
    fs::write(
        fixtures.join("latest-tag"),
        format!("v{}", codecraft::VERSION),
    )
    .unwrap();
    fs::write(fixtures.join("requests"), "").unwrap();
    let output = run("");
    assert!(
        output.status.success(),
        "{} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        codecraft::util::digest(&installed).unwrap(),
        codecraft::util::digest(executable).unwrap()
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 1);
    let requests = fs::read_to_string(fixtures.join("requests")).unwrap();
    let urls: Vec<_> = requests.lines().collect();
    assert_eq!(urls.len(), 3, "{requests}");
    assert!(urls[0].ends_with("/releases/latest"));
    assert!(urls[1..]
        .iter()
        .all(|url| url.contains(&format!("/releases/download/v{}/", codecraft::VERSION))));
    // Explicit versions bypass latest resolution, including rollback installs.
    fs::remove_file(fixtures.join("latest-tag")).unwrap();
    // Replacement also works for an already installed executable.
    assert!(run(codecraft::VERSION).status.success());
}

#[cfg(unix)]
#[test]
fn doctor_accepts_activated_linux_host_tools_without_writing_files() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().unwrap();
    for (name, text) in [("python3", "3.14.6"), ("dtc", "Version: DTC 1.7.2")] {
        let path = temp.path().join(name);
        fs::write(&path, format!("#!/bin/sh\nprintf '%s\\n' '{text}'\n")).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let result = Command::new(env!("CARGO_BIN_EXE_codecraft"))
        .arg("--root")
        .arg(root())
        .args(["doctor", "--target", "linux/reterminal"])
        .env("PATH", temp.path())
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stdout)
    );
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["data"]["status"], "passed");
    assert_eq!(report["data"]["next_steps"], serde_json::json!([]));
    assert!(report["data"]["scope"]
        .as_str()
        .unwrap()
        .contains("no compilation"));
}
