//! Registration only. Native behavior belongs in the stack's source directory.
use crate::util::*;
use anyhow::{bail, Result};
use std::path::Path;
#[path = "../../../platforms/arduino/native.rs"]
pub mod arduino;
#[path = "../../../platforms/esp-idf/native.rs"]
pub mod esp_idf;
#[path = "../../../platforms/linux/native.rs"]
pub mod linux;
pub trait Adapter {
    fn probe(&self, root: &Path, target: &J) -> Result<J>;
    fn build(&self, root: &Path, target: &J) -> Result<Vec<J>>;
    fn image_layout(
        &self,
        _root: &Path,
        _target: &J,
        _device: &J,
        _example: &str,
        _variant: Option<&str>,
    ) -> Result<J> {
        Err(unsupported(
            "Firmware images are not supported by this platform",
        ))
    }
    fn image_merge(&self, _root: &Path, _layout: &J, _stage: &Path) -> Result<J> {
        Err(unsupported(
            "Firmware images are not supported by this platform",
        ))
    }
    fn image_verify(&self, _bundle: &Path, _manifest: &J) -> Result<J> {
        Err(unsupported(
            "Firmware images are not supported by this platform",
        ))
    }
}
pub fn adapter(stack: &str) -> Result<Box<dyn Adapter>> {
    Ok(match stack {
        "esp-idf" => Box::new(esp_idf::EspIdf),
        "arduino" => Box::new(arduino::Arduino),
        "linux" => Box::new(linux::Linux),
        _ => bail!("Unknown native adapter: {stack}"),
    })
}
pub fn command(root: &Path, name: &str, argv: &[String], toolchain: &J) -> Result<J> {
    command_timeout(
        root,
        name,
        argv,
        toolchain,
        std::time::Duration::from_secs(1800),
    )
}
pub fn command_timeout(
    root: &Path,
    name: &str,
    argv: &[String],
    toolchain: &J,
    timeout: std::time::Duration,
) -> Result<J> {
    use serde_json::json;
    use std::{
        fs,
        process::{Command, Stdio},
    };
    let log = root
        .join(".codecraft/logs")
        .join(format!("{}.log", name.replace(['/', ':'], "_")));
    fs::create_dir_all(log.parent().unwrap())?;
    let file = fs::File::create(&log)?;
    let mut cmd = Command::new(&argv[0]);
    cmd.args(&argv[1..])
        .current_dir(root)
        .stdout(Stdio::from(file.try_clone()?))
        .stderr(Stdio::from(file));
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let started = std::time::Instant::now();
    let mut timed_out = false;
    let run = cmd.spawn().and_then(|mut child| {
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(status);
            }
            if started.elapsed() >= timeout {
                timed_out = true;
                #[cfg(unix)]
                {
                    // The child created its own group; terminate its compiler descendants too.
                    unsafe {
                        libc::kill(-(child.id() as i32), libc::SIGKILL);
                    }
                }
                #[cfg(windows)]
                {
                    let _ = Command::new("taskkill")
                        .args(["/PID", &child.id().to_string(), "/T", "/F"])
                        .output();
                }
                let _ = child.kill();
                return child.wait();
            }
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
    });
    let (status, reason, code) = match run {
        Ok(s) => (
            if s.success() && !timed_out {
                "passed"
            } else {
                "failed"
            },
            if timed_out {
                format!("Native command exceeded {} seconds", timeout.as_secs_f64())
            } else if s.success() {
                String::new()
            } else {
                format!("Command exited {s}")
            },
            s.code(),
        ),
        Err(e) => ("not_run", e.to_string(), None),
    };
    Ok(
        json!({"name":name,"status":status,"reason":reason,"exit_code":code,"command":argv,"toolchain":toolchain,"log":log.strip_prefix(root)?.to_string_lossy(),"log_sha256":digest(&log)?}),
    )
}

#[path = "../../../platforms/arduino/evidence.rs"]
mod arduino_evidence;
#[path = "../../../platforms/esp-idf/evidence.rs"]
mod esp_idf_evidence;
#[path = "../../../platforms/linux/evidence.rs"]
mod linux_evidence;

pub fn evidence_cases(root: &Path, target: &J) -> Result<Vec<String>> {
    match s(&target["platform"])? {
        "esp-idf" => esp_idf_evidence::cases(root, target),
        "arduino" => arduino_evidence::cases(root, target),
        "linux" => linux_evidence::cases(root, target),
        stack => bail!("Unknown evidence adapter: {stack}"),
    }
}
pub fn evidence_toolchain(target: &J, actual: &J) -> Result<()> {
    match s(&target["platform"])? {
        "esp-idf" => esp_idf_evidence::toolchain(target, actual),
        "arduino" => arduino_evidence::toolchain(target, actual),
        "linux" => linux_evidence::toolchain(target, actual),
        stack => bail!("Unknown evidence adapter: {stack}"),
    }
}

pub fn validate_target(root: &Path, target: &J) -> Result<()> {
    match s(&target["platform"])? {
        "esp-idf" => esp_idf::composition::validate(root, target),
        "arduino" => arduino_evidence::validate_target(root, target),
        "linux" => linux_evidence::validate_target(root, target),
        stack => bail!("Unknown native adapter: {stack}"),
    }
}
