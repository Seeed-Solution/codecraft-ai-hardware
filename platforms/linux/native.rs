use crate::{
    platform::{command, Adapter},
    util::*,
};
use anyhow::Result;
use serde_json::json;
use std::{fs, path::Path, process::Command};
pub struct Linux;
impl Adapter for Linux {
    fn probe(&self, _root: &Path, _t: &J) -> Result<J> {
        match (
            output(
                Command::new("python3").args(["-c", "import sys;print(sys.version.split()[0])"]),
            ),
            output(Command::new("dtc").arg("--version")),
        ) {
            (Ok(python), Ok(dtc)) => Ok(
                json!({"status":"passed","python":python,"dtc":dtc,"scope":"host syntax and overlay compilation; device OS/kernel not verified"}),
            ),
            _ => Ok(
                json!({"status":"not_run","reason":"Linux Python examples need python3 and dtc for host validation."}),
            ),
        }
    }
    fn build(&self, root: &Path, t: &J) -> Result<Vec<J>> {
        let tc = self.probe(root, t)?;
        let base = safe(root, s(&t["path"])?)?;
        let mut rows = vec![];
        for ex in strings(&t["examples"])? {
            rows.push(command(
                root,
                &format!("{}/{ex}", s(&t["id"])?),
                &[
                    "python3".into(),
                    "-c".into(),
                    "import ast,pathlib,sys;ast.parse(pathlib.Path(sys.argv[1]).read_text())"
                        .into(),
                    base.join("demos")
                        .join(format!("{ex}.py"))
                        .to_string_lossy()
                        .into(),
                ],
                &tc,
            )?);
        }
        let out = root.join(".codecraft/builds").join(s(&t["device"])?);
        fs::create_dir_all(&out)?;
        for e in fs::read_dir(base.join("overlays"))? {
            let path = e?.path();
            if path.extension().and_then(|x| x.to_str()) != Some("dts") {
                continue;
            }
            let stem = path.file_stem().unwrap().to_string_lossy();
            rows.push(command(
                root,
                &format!("{}/overlay/{stem}", s(&t["id"])?),
                &[
                    "dtc".into(),
                    "-@".into(),
                    "-I".into(),
                    "dts".into(),
                    "-O".into(),
                    "dtb".into(),
                    "-o".into(),
                    out.join(format!("{stem}.dtbo")).to_string_lossy().into(),
                    path.to_string_lossy().into(),
                ],
                &tc,
            )?);
        }
        Ok(rows)
    }
}
