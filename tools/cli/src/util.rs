use anyhow::{bail, ensure, Context, Result};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt, fs,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;
pub type J = Value;
pub fn object() -> J {
    json!({})
}
pub fn s(v: &J) -> Result<&str> {
    v.as_str().context("Expected text")
}
pub fn arr(v: &J) -> Result<&Vec<J>> {
    v.as_array().context("Expected list")
}
pub fn obj(v: &J) -> Result<&Map<String, J>> {
    v.as_object().context("Expected object")
}
pub fn strings(v: &J) -> Result<Vec<String>> {
    arr(v)?.iter().map(|x| s(x).map(str::to_owned)).collect()
}
pub fn hash(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
pub fn digest(path: &Path) -> Result<String> {
    Ok(hash(
        &fs::read(path).with_context(|| format!("Read {}", path.display()))?,
    ))
}
pub fn identity(v: &J) -> String {
    hash(serde_json::to_string(v).expect("JSON value").as_bytes())
}
// serde_json's default map representation is sorted; reject duplicate keys at every depth.
struct Unique(J);
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "JSON with unique keys")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut m = Map::new();
                while let Some((k, v)) = a.next_entry::<String, Unique>()? {
                    if m.insert(k.clone(), v.0).is_some() {
                        return Err(serde::de::Error::custom(format!("Duplicate key: {k}")));
                    }
                }
                Ok(Unique(J::Object(m)))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut v = vec![];
                while let Some(x) = a.next_element::<Unique>()? {
                    v.push(x.0)
                }
                Ok(Unique(J::Array(v)))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_string<E: serde::de::Error>(
                self,
                v: String,
            ) -> std::result::Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> std::result::Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> std::result::Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(J::Null))
            }
        }
        d.deserialize_any(V)
    }
}
pub fn parse(text: &str) -> Result<J> {
    Ok(serde_json::from_str::<Unique>(text)?.0)
}
pub fn read(path: &Path) -> Result<J> {
    parse(&fs::read_to_string(path).with_context(|| format!("Read {}", path.display()))?)
}
pub fn write(path: &Path, value: &J) -> Result<()> {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p)?;
    }
    fs::write(path, serde_json::to_string_pretty(value)? + "\n")?;
    Ok(())
}
pub fn safe(root: &Path, relative: &str) -> Result<PathBuf> {
    let rel = Path::new(relative);
    ensure!(
        !relative.is_empty()
            && !rel.is_absolute()
            && rel.components().all(|x| matches!(x, Component::Normal(_))),
        "Noncanonical relative path: {relative}"
    );
    let path = root.join(rel);
    let resolved = path
        .canonicalize()
        .with_context(|| format!("Missing reference: {relative}"))?;
    ensure!(
        resolved.starts_with(root.canonicalize()?),
        "Path escapes root: {relative}"
    );
    Ok(path)
}
pub fn ignored(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | ".codecraft"
            | ".codegraph"
            | ".venv"
            | "secrets"
            | ".secrets"
            | "__pycache__"
            | ".pytest_cache"
            | "node_modules"
            | "managed_components"
            | "build"
            | "built"
    ) || name.starts_with("build-")
}
pub fn files(root: &Path) -> Result<Vec<String>> {
    let mut out = vec![];
    for e in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !ignored(&e.file_name().to_string_lossy()))
    {
        let e = e?;
        let name = e.file_name().to_string_lossy();
        if name == ".env"
            || (name.starts_with(".env.") && name != ".env.example")
            || matches!(name.as_ref(), "sdkconfig" | "sdkconfig.old" | ".DS_Store")
        {
            continue;
        }
        if e.path().is_dir() {
            ensure!(
                !e.path_is_symlink(),
                "Directory symlink must be declared explicitly"
            );
            continue;
        }
        if matches!(
            e.path().extension().and_then(|s| s.to_str()),
            Some("pyc" | "uf2" | "hex" | "bin" | "elf")
        ) {
            continue;
        }
        let rel = e
            .path()
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        ensure!(e.path().is_file(), "Broken source link: {rel}");
        out.push(rel);
    }
    out.sort();
    Ok(out)
}
#[derive(Clone)]
pub struct Graph {
    pub root: PathBuf,
    pub files: BTreeMap<String, String>,
    pub edges: BTreeMap<String, Vec<String>>,
    active: Vec<String>,
}
impl Graph {
    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_owned(),
            files: BTreeMap::new(),
            edges: BTreeMap::new(),
            active: vec![],
        }
    }
    pub fn add(&mut self, r: &str) -> Result<()> {
        ensure!(
            !r.starts_with("projects/")
                && !Path::new(r).components().any(|c| {
                    let n = c.as_os_str().to_string_lossy();
                    ignored(&n) || n == ".env" || (n.starts_with(".env.") && n != ".env.example")
                }),
            "Private/cache dependency: {r}"
        );
        ensure!(
            !self.active.iter().any(|x| x == r),
            "Dependency cycle: {:?} -> {r}",
            self.active
        );
        if self.edges.contains_key(r) {
            return Ok(());
        }
        let p = safe(&self.root, r)?;
        ensure!(
            !p.canonicalize()?.starts_with(self.root.join("projects")),
            "Private dependency: {r}"
        );
        self.active.push(r.into());
        let mut children = vec![];
        if p.is_dir() {
            if p.join("resource.json").is_file() {
                children.push(format!("{r}/resource.json"))
            } else {
                children = files(&p)?.into_iter().map(|x| format!("{r}/{x}")).collect();
            }
            ensure!(!children.is_empty(), "Empty dependency {r}");
        } else {
            self.files.insert(r.into(), digest(&p)?);
            if p.file_name().and_then(|s| s.to_str()) == Some("resource.json") {
                let v = read(&p)?;
                ensure!(
                    v["schema_version"] == 1 && v["kind"] == "resource",
                    "Invalid resource {r}"
                );
                children.extend(strings(&v["inputs"])?);
                children.extend(strings(&v["dependencies"])?);
                ensure!(!children.is_empty(), "Empty resource {r}");
            }
        }
        children.sort();
        children.dedup();
        for c in &children {
            self.add(c)?;
        }
        self.edges.insert(r.into(), children);
        self.active.pop();
        Ok(())
    }
    pub fn resolve(&mut self, roots: Vec<String>) -> Result<()> {
        for r in roots {
            self.add(&r)?;
        }
        Ok(())
    }
    pub fn snapshot(&self) -> J {
        json!({"schema_version":4,"files":self.files,"edges":self.edges})
    }
    pub fn fingerprint(&self) -> String {
        identity(&self.snapshot())
    }
}
pub fn output(cmd: &mut std::process::Command) -> Result<String> {
    let r = cmd.output()?;
    ensure!(
        r.status.success(),
        "Command failed: {}",
        String::from_utf8_lossy(&r.stderr)
    );
    Ok(String::from_utf8(r.stdout)?.trim().into())
}
pub fn fail(message: impl fmt::Display) -> Result<J> {
    bail!("{message}")
}

#[derive(Debug)]
pub struct Unavailable(pub &'static str, pub String);
impl std::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.1)
    }
}
impl std::error::Error for Unavailable {}
pub fn unsupported(reason: &str) -> anyhow::Error {
    Unavailable("not_supported", reason.into()).into()
}
