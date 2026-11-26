//! Parse SDK Python source in Rust. Never imports or executes hardware code.
use crate::util::*;
use anyhow::{ensure, Context, Result};
use rustpython_parser::{
    ast::{self, Ranged, Visitor},
    Parse,
};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
pub const SKILL: &str = "codecraft-development-recamera-pysdk";
const REPOSITORY: &str = "https://github.com/Seeed-Studio/recamera-pro-ext-api";
pub const EXTRA_SOURCES: &[&str] = &[
    "sdk/include/recamera_ext.h",
    "market/appmgr/server.py",
    "market/appmgr/workflow_ui.py",
    "market/appmgr/workflow_models.py",
    "market/appmgr/store_tasks.py",
    "market/appmgr/result_hub.py",
    "market/appmgr/output_tools.py",
    "market/appmgr/visualization.py",
    "market/appmgr/workflow_model_contract.py",
    "market/appmgr/operations.py",
    "sdk/LICENSE",
    "sdk/NOTICE",
];
fn public(n: &str) -> bool {
    !n.starts_with('_')
        || [
            "__init__",
            "__enter__",
            "__exit__",
            "__iter__",
            "__next__",
            "__call__",
            "__or__",
            "__len__",
            "__getitem__",
        ]
        .contains(&n)
}
fn text(e: &ast::Expr) -> Option<String> {
    if let ast::Expr::Constant(c) = e {
        if let ast::Constant::Str(s) = &c.value {
            return Some(s.clone());
        }
    }
    None
}
fn doc(body: &[ast::Stmt]) -> String {
    let Some(ast::Stmt::Expr(e)) = body.first() else {
        return String::new();
    };
    let Some(raw) = text(&e.value) else {
        return String::new();
    };
    let raw = raw.replace('\t', "        ");
    let lines: Vec<_> = raw.lines().collect();
    let indent = lines
        .iter()
        .skip(1)
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let cleaned = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            if i == 0 {
                l.trim().to_owned()
            } else {
                l.get(indent..).unwrap_or("").trim_end().to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    regex::Regex::new(r"\b(?:[A-Za-z][\w.-]*@)?(?:192\.168|10\.\d{1,3}|172\.(?:1[6-9]|2\d|3[01]))\.\d{1,3}\.\d{1,3}\b").unwrap().replace_all(cleaned.trim(),"<device-host>").into_owned()
}
fn arg(a: &ast::Arg) -> String {
    format!(
        "{}{}",
        a.arg,
        a.annotation
            .as_ref()
            .map(|a| format!(": {a}"))
            .unwrap_or_default()
    )
}
fn arg_default(a: &ast::ArgWithDefault) -> String {
    format!(
        "{}{}",
        arg(&a.def),
        a.default
            .as_ref()
            .map(|v| format!("={v}"))
            .unwrap_or_default()
    )
}

fn args(a: &ast::Arguments) -> String {
    let mut out: Vec<_> = a.posonlyargs.iter().map(arg_default).collect();
    if !a.posonlyargs.is_empty() {
        out.push("/".into());
    }
    out.extend(a.args.iter().map(arg_default));
    if let Some(v) = &a.vararg {
        out.push(format!("*{}", arg(v)));
    } else if !a.kwonlyargs.is_empty() {
        out.push("*".into());
    }
    out.extend(a.kwonlyargs.iter().map(arg_default));
    if let Some(v) = &a.kwarg {
        out.push(format!("**{}", arg(v)));
    }
    out.join(", ")
}
fn decorators(ds: &[ast::Expr]) -> String {
    ds.iter().map(|d| format!("@{d}\n")).collect()
}
fn declaration(n: &ast::Stmt) -> String {
    match n {
        ast::Stmt::FunctionDef(f) => format!(
            "{}def {}({}){}",
            decorators(&f.decorator_list),
            f.name,
            args(&f.args),
            f.returns
                .as_ref()
                .map(|v| format!(" -> {v}"))
                .unwrap_or_default()
        ),
        ast::Stmt::AsyncFunctionDef(f) => format!(
            "{}async def {}({}){}",
            decorators(&f.decorator_list),
            f.name,
            args(&f.args),
            f.returns
                .as_ref()
                .map(|v| format!(" -> {v}"))
                .unwrap_or_default()
        ),
        ast::Stmt::ClassDef(c) => format!(
            "{}class {}{}",
            decorators(&c.decorator_list),
            c.name,
            if c.bases.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    c.bases
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        ),
        ast::Stmt::Assign(a) => format!(
            "{} = {}",
            a.targets
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" = "),
            a.value
        ),
        ast::Stmt::AnnAssign(a) => format!(
            "{}: {}{}",
            a.target,
            a.annotation,
            a.value
                .as_ref()
                .map(|v| format!(" = {v}"))
                .unwrap_or_default()
        ),
        _ => String::new(),
    }
}
fn name_body(n: &ast::Stmt) -> Option<(&str, &[ast::Stmt])> {
    match n {
        ast::Stmt::FunctionDef(f) => Some((f.name.as_str(), &f.body)),
        ast::Stmt::AsyncFunctionDef(f) => Some((f.name.as_str(), &f.body)),
        ast::Stmt::ClassDef(c) => Some((c.name.as_str(), &c.body)),
        _ => None,
    }
}
fn assignments(body: &[ast::Stmt]) -> Vec<(String, &ast::Stmt, String)> {
    let mut out = vec![];
    for (i, n) in body.iter().enumerate() {
        let names: Vec<_> = match n {
            ast::Stmt::Assign(a) => a
                .targets
                .iter()
                .filter_map(|t| {
                    if let ast::Expr::Name(n) = t {
                        Some(n.id.to_string())
                    } else {
                        None
                    }
                })
                .collect(),
            ast::Stmt::AnnAssign(a) => {
                if let ast::Expr::Name(n) = a.target.as_ref() {
                    vec![n.id.to_string()]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };
        for name in names {
            if public(&name) {
                let d = match body.get(i + 1) {
                    Some(ast::Stmt::Expr(e)) => text(&e.value).unwrap_or_default(),
                    _ => String::new(),
                };
                out.push((name, n, d));
            }
        }
    }
    out
}
struct Imports {
    module: String,
    package: bool,
    aliases: BTreeMap<String, String>,
}
impl Visitor for Imports {
    fn visit_stmt_function_def(&mut self, _: ast::StmtFunctionDef) {}
    fn visit_stmt_async_function_def(&mut self, _: ast::StmtAsyncFunctionDef) {}
    fn visit_stmt_class_def(&mut self, _: ast::StmtClassDef) {}
    fn visit_stmt_import_from(&mut self, n: ast::StmtImportFrom) {
        let level = n.level.map(|x| x.to_usize()).unwrap_or(0);
        let source = if level > 0 {
            let mut base = self.module.split('.').collect::<Vec<_>>();
            if !self.package {
                base.pop();
            }
            base.truncate(base.len().saturating_sub(level - 1));
            if let Some(m) = &n.module {
                base.push(m.as_str());
            }
            base.join(".")
        } else {
            n.module.map(|n| n.to_string()).unwrap_or_default()
        };
        for a in n.names {
            if (source == "kit"
                || source == "recamera_ext"
                || source.starts_with("kit.")
                || source.starts_with("recamera_ext."))
                && a.name.as_str() != "*"
            {
                self.aliases.insert(
                    a.asname.as_ref().unwrap_or(&a.name).to_string(),
                    format!("{source}.{}", a.name),
                );
            }
        }
    }
    fn visit_stmt_assign(&mut self, n: ast::StmtAssign) {
        if n.targets
            .iter()
            .any(|t| matches!(t,ast::Expr::Name(n) if n.id.as_str()=="_EXPORTS"))
        {
            if let ast::Expr::Dict(d) = n.value.as_ref() {
                for (k, v) in d.keys.iter().zip(&d.values) {
                    if let (Some(k), Some(v)) = (k.as_ref().and_then(text), text(v)) {
                        self.aliases
                            .insert(k.clone(), format!("{}.{v}.{k}", self.module));
                    }
                }
            }
        }
    }
}
fn imports(body: &[ast::Stmt], module: &str, package: bool) -> BTreeMap<String, String> {
    let mut i = Imports {
        module: module.into(),
        package,
        aliases: BTreeMap::new(),
    };
    for n in body {
        i.visit_stmt(n.clone());
    }
    i.aliases
}
fn exports(body: &[ast::Stmt]) -> Result<Option<BTreeSet<String>>> {
    for n in body {
        if let ast::Stmt::Assign(a) = n {
            if a.targets
                .iter()
                .any(|t| matches!(t,ast::Expr::Name(n)if n.id.as_str()=="__all__"))
            {
                let values = match a.value.as_ref() {
                    ast::Expr::List(l) => &l.elts,
                    ast::Expr::Tuple(t) => &t.elts,
                    _ => return Ok(None),
                };
                return Ok(Some(
                    values
                        .iter()
                        .map(|v| text(v).context("Nonliteral export"))
                        .collect::<Result<_>>()?,
                ));
            }
        }
    }
    Ok(None)
}
fn canonical(target: &str, aliases: &BTreeMap<String, BTreeMap<String, String>>) -> Result<String> {
    let mut target = target.to_owned();
    let mut seen = BTreeSet::new();
    loop {
        ensure!(seen.insert(target.clone()), "Circular public re-export");
        let Some((owner, name)) = target.rsplit_once('.') else {
            return Ok(target);
        };
        if let Some(next) = aliases.get(owner).and_then(|a| a.get(name)) {
            target = next.clone();
        } else {
            return Ok(target);
        }
    }
}
fn page_name(m: &str) -> String {
    format!("{}.md", m.replace('.', "-"))
}
fn link(target: &str, modules: &BTreeMap<String, Module>) -> String {
    let mut parts = target.split('.').collect::<Vec<_>>();
    while !parts.is_empty() {
        let module = parts.join(".");
        if modules.contains_key(&module) {
            return format!("[{target}]({})", page_name(&module));
        }
        parts.pop();
    }
    format!("`{target}` (外部/基类接口)")
}
struct Routes {
    method: String,
    constants: BTreeMap<String, String>,
    routes: BTreeSet<String>,
    error: Option<String>,
}
impl Visitor for Routes {
    fn visit_stmt_function_def(&mut self, n: ast::StmtFunctionDef) {
        let old = self.method.clone();
        if [
            "do_GET",
            "do_POST",
            "do_PUT",
            "do_DELETE",
            "do_PATCH",
            "do_HEAD",
        ]
        .contains(&n.name.as_str())
        {
            self.method = n.name.as_str()[3..].to_string();
        }
        for child in n.body {
            self.visit_stmt(child);
        }
        self.method = old;
    }
    fn visit_expr_compare(&mut self, n: ast::ExprCompare) {
        if !self.method.is_empty()
            && matches!(n.left.as_ref(),ast::Expr::Name(v)if v.id.as_str()=="path")
        {
            for value in &n.comparators {
                let value = text(value).or_else(|| {
                    if let ast::Expr::Name(v) = value {
                        self.constants.get(v.id.as_str()).cloned()
                    } else {
                        None
                    }
                });
                match value {
                    Some(v) if v.starts_with('/') => {
                        self.routes.insert(format!("{} {v}", self.method));
                    }
                    Some(_) => {}
                    None => self.error = Some("Review dynamic HTTP route".into()),
                }
            }
        }
        self.generic_visit_expr_compare(n);
    }
    fn visit_expr_call(&mut self, n: ast::ExprCall) {
        if !self.method.is_empty() {
            if let ast::Expr::Attribute(f) = n.func.as_ref() {
                if matches!(f.value.as_ref(),ast::Expr::Name(v) if v.id.as_str()=="path")
                    && ["startswith", "endswith"].contains(&f.attr.as_str())
                {
                    self.error = Some("Review prefix/suffix HTTP route".into());
                }
                if matches!(f.value.as_ref(),ast::Expr::Name(v) if v.id.as_str()=="re")
                    && ["match", "fullmatch"].contains(&f.attr.as_str())
                    && n.args
                        .get(1)
                        .is_some_and(|v| matches!(v,ast::Expr::Name(n)if n.id.as_str()=="path"))
                {
                    if let Some(v) = n.args.first().and_then(text) {
                        if v.starts_with('/') {
                            self.routes.insert(format!("{} {v}", self.method));
                        }
                    } else {
                        self.error = Some("Review dynamic regex HTTP route".into());
                    }
                }
            }
        }
        self.generic_visit_expr_call(n);
    }
}
pub fn http_routes(source: &str) -> Result<BTreeSet<String>> {
    let tree = ast::Suite::parse(source, "<routes>")
        .map_err(|e| anyhow::anyhow!("SDK source parsing failed: {e}"))?;
    let mut r = Routes {
        method: String::new(),
        constants: BTreeMap::new(),
        routes: BTreeSet::new(),
        error: None,
    };
    for n in &tree {
        if let ast::Stmt::Assign(a) = n {
            if let Some(v) = text(&a.value) {
                for t in &a.targets {
                    if let ast::Expr::Name(n) = t {
                        r.constants.insert(n.id.to_string(), v.clone());
                    }
                }
            }
        }
    }
    for n in tree {
        r.visit_stmt(n);
    }
    ensure!(r.error.is_none(), "{}", r.error.unwrap_or_default());
    Ok(r.routes)
}
struct Module {
    relative: String,
    source: String,
    tree: ast::Suite,
    package: bool,
}
fn discover(root: &Path) -> Result<BTreeMap<String, Module>> {
    let mut out = BTreeMap::new();
    for dir in ["kit", "sdk/python/recamera_ext"] {
        let dir = root.join(dir);
        ensure!(dir.is_dir(), "Missing SDK module root");
        for e in walkdir::WalkDir::new(&dir).into_iter().filter_entry(|e| {
            e.depth() == 0
                || !(["tests", "build", "dist", "__pycache__"]
                    .contains(&e.file_name().to_string_lossy().as_ref())
                    || e.file_name().to_string_lossy().starts_with('.'))
        }) {
            let e = e?;
            let n = e.file_name().to_string_lossy();
            if !e.file_type().is_file()
                || e.path().extension().is_none_or(|s| s != "py")
                || ["setup.py", "demo_detect.py", "demo_live.py", "ws_probe.py"]
                    .contains(&n.as_ref())
                || n.starts_with("test_")
                || (n.starts_with('_') && n != "__init__.py")
            {
                continue;
            }
            ensure!(!e.path_is_symlink(), "SDK source symlink");
            let relative = e
                .path()
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            let name = relative
                .strip_prefix("sdk/python/")
                .unwrap_or(&relative)
                .trim_end_matches(".py")
                .replace('/', ".")
                .trim_end_matches(".__init__")
                .to_string();
            let source = fs::read_to_string(e.path())?;
            let tree = ast::Suite::parse(&source, &relative)
                .map_err(|e| anyhow::anyhow!("SDK source parsing failed: {e}"))?;
            ensure!(
                out.insert(
                    name,
                    Module {
                        relative,
                        source,
                        tree,
                        package: n == "__init__.py"
                    }
                )
                .is_none(),
                "Duplicate SDK module"
            );
        }
    }
    Ok(out)
}
fn summary(n: &ast::Stmt, key: &str, notes: &J) -> Result<String> {
    let (name, body) = name_body(n).context("Expected definition")?;
    let original = doc(body);
    if let Some(extra) = notes["symbols"][key].as_str().filter(|s| !s.is_empty()) {
        Ok(format!(
            "{extra}{}",
            if original.is_empty() {
                String::new()
            } else {
                format!("\n\n源码补充：\n\n{original}")
            }
        ))
    } else if !original.is_empty() {
        Ok(original)
    } else {
        Ok(s(&notes["common"][name])
            .with_context(|| format!("Missing API explanation: {key}"))?
            .to_owned())
    }
}
fn methods<'a>(
    class: &'a ast::StmtClassDef,
    classes: &BTreeMap<&str, &'a ast::StmtClassDef>,
    seen: &mut BTreeSet<String>,
) -> Result<Vec<(String, (&'a ast::Stmt, String))>> {
    ensure!(
        seen.insert(class.name.to_string()),
        "Circular local inheritance"
    );
    let mut result: Vec<(String, (&'a ast::Stmt, String))> = vec![];
    for b in &class.bases {
        if let ast::Expr::Name(n) = b {
            if let Some(base) = classes.get(n.id.as_str()) {
                for (key, value) in methods(base, classes, seen)? {
                    if let Some(old) = result.iter_mut().find(|(k, _)| k == &key) {
                        old.1 = value;
                    } else {
                        result.push((key, value));
                    }
                }
            }
        }
    }
    for n in &class.body {
        let ds = match n {
            ast::Stmt::FunctionDef(f) => Some((&f.name, &f.decorator_list)),
            ast::Stmt::AsyncFunctionDef(f) => Some((&f.name, &f.decorator_list)),
            _ => None,
        };
        if let Some((name, ds)) = ds {
            if public(name.as_str()) {
                let suffix = if ds
                    .iter()
                    .any(|d| matches!(d,ast::Expr::Attribute(a)if a.attr.as_str()=="setter"))
                {
                    ".setter"
                } else {
                    ""
                };
                let key = format!("{name}{suffix}");
                let value = (n, class.name.to_string());
                if let Some(old) = result.iter_mut().find(|(k, _)| k == &key) {
                    old.1 = value;
                } else {
                    result.push((key, value));
                }
            }
        }
    }
    seen.remove(class.name.as_str());
    Ok(result)
}
#[allow(clippy::too_many_arguments)] // Rendering shares the page/source context without runtime state.
fn render_function(
    rows: &mut Vec<String>,
    inventory: &mut J,
    n: &ast::Stmt,
    key: &str,
    level: usize,
    inherited: Option<&str>,
    module: &str,
    m: &Module,
    url: &str,
    notes: &J,
) -> Result<()> {
    let declaration = declaration(n);
    rows.extend([
        format!("{} {key}", "#".repeat(level)),
        String::new(),
        "```python".into(),
        declaration.clone(),
        "```".into(),
        String::new(),
    ]);
    let explanation = if let Some(base) = inherited {
        format!("{module}.{base}.{}", name_body(n).unwrap().0)
    } else {
        key.into()
    };
    rows.extend([summary(n, &explanation, notes)?, String::new()]);
    if let Some(base) = inherited {
        rows.extend([format!("此方法定义于基类 `{base}`。"), String::new()]);
    }
    let line = m.source[..usize::from(n.range().start())]
        .bytes()
        .filter(|b| *b == b'\n')
        .count()
        + 1;
    rows.extend([format!("[实现与参数校验]({url}#L{line})"), String::new()]);
    inventory[key] = json!({"kind":"callable","declaration":declaration});
    Ok(())
}
pub fn build(
    root: &Path,
    revision: &str,
    notes: &J,
    http_notes: &J,
) -> Result<(BTreeMap<String, String>, J)> {
    let modules = discover(root)?;
    ensure!(
        modules.keys().cloned().collect::<BTreeSet<_>>()
            == obj(&notes["modules"])?.keys().cloned().collect(),
        "Review module descriptions: new or removed module"
    );
    let aliases = modules
        .iter()
        .map(|(name, m)| (name.clone(), imports(&m.tree, name, m.package)))
        .collect::<BTreeMap<_, _>>();
    let mut pages = BTreeMap::new();
    let mut symbols = object();
    let mut files = object();
    for (module, m) in &modules {
        files[&m.relative] = json!(hash(m.source.as_bytes()));
        let url = format!("{REPOSITORY}/blob/{revision}/{}", m.relative);
        let mut rows=vec![format!("# {module}"),"".into(),"[API 索引](../index.md) · [接口特性与边界](../features.md)".into(),"".into(),format!("源码基线：[{}]({url})；签名由 AST 提取，不导入硬件依赖。",m.relative),"".into(),s(&notes["modules"][module])?.into(),"".into(),"源码说明中的历史验证记录、迁移目标和旧示例不代表当前固件可用；应用入口、资源准入和接口可用性以本页补充及接口特性文档为准。".into(),"".into()];
        let d = doc(&m.tree);
        if !d.is_empty() {
            rows.extend(["## 模块契约（源码）".into(), "".into(), d, "".into()]);
        }
        let exports = exports(&m.tree)?;
        let defined = m
            .tree
            .iter()
            .filter_map(name_body)
            .map(|(n, _)| n)
            .collect::<BTreeSet<_>>();
        if m.package || exports.is_some() {
            let chosen = aliases[module]
                .iter()
                .filter(|(k, _)| {
                    !k.starts_with('_')
                        && !defined.contains(k.as_str())
                        && exports.as_ref().is_none_or(|ex| ex.contains(*k))
                })
                .collect::<Vec<_>>();
            if !chosen.is_empty() {
                rows.extend([
                    "## 公开导出 / 别名".into(),
                    "".into(),
                    "| 导入名称 | 定义与完整说明 |".into(),
                    "| --- | --- |".into(),
                ]);
                for (name, target) in chosen {
                    let target = canonical(target, &aliases)?;
                    rows.push(format!("| `{name}` | {} |", link(&target, &modules)));
                    symbols[format!("{module}.{name}")] = json!({"kind":"export","target":target});
                }
                rows.push("".into());
            }
        }
        let constants = assignments(&m.tree)
            .into_iter()
            .filter(|(n, _, _)| n != "log" && n != "logger")
            .collect::<Vec<_>>();
        if !constants.is_empty() {
            rows.extend([
                "## 常量与类型别名".into(),
                "".into(),
                "以下为该版本源码值；默认地址不等于所有固件都开放该服务。".into(),
                "".into(),
            ]);
            for (name, n, doc) in constants {
                let d = declaration(n);
                rows.extend(["```python".into(), d.clone(), "```".into(), doc, "".into()]);
                symbols[format!("{module}.{name}")] = json!({"kind":"value","declaration":d});
            }
        }
        let classes = m
            .tree
            .iter()
            .filter_map(|n| {
                if let ast::Stmt::ClassDef(c) = n {
                    Some((c.name.as_str(), c))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();
        for n in &m.tree {
            let Some((name, _)) = name_body(n) else {
                continue;
            };
            if !public(name) {
                continue;
            }
            let key = format!("{module}.{name}");
            if let ast::Stmt::ClassDef(c) = n {
                let decl = declaration(n);
                rows.extend([
                    format!("## {key}"),
                    "".into(),
                    "```python".into(),
                    decl.clone(),
                    "```".into(),
                    "".into(),
                    summary(n, &key, notes)?,
                    "".into(),
                ]);
                symbols[&key] = json!({"kind":"class","declaration":decl});
                for n in &c.body {
                    if let ast::Stmt::Assign(a) = n {
                        if a.targets
                            .iter()
                            .any(|t| matches!(t,ast::Expr::Name(n)if n.id.as_str()=="_fields_"))
                        {
                            let value = a.value.to_string();
                            rows.extend([
                                "### C 结构字段".into(),
                                "".into(),
                                "```python".into(),
                                format!("_fields_ = {value}"),
                                "```".into(),
                                "".into(),
                            ]);
                            symbols[&key]["ctypes_fields"] = json!(value);
                        }
                    }
                }
                let fields = assignments(&c.body);
                if !fields.is_empty() {
                    rows.extend([
                        "### 字段 / 默认值".into(),
                        "".into(),
                        "`@dataclass` 未显式定义构造器时，由下列字段生成；无默认值的字段必填。"
                            .into(),
                        "".into(),
                        "```python".into(),
                    ]);
                    rows.extend(fields.iter().map(|(_, n, _)| declaration(n)));
                    rows.extend(["```".into(), "".into()]);
                    for (name, n, d) in fields {
                        symbols[format!("{key}.{name}")] =
                            json!({"kind":"field","declaration":declaration(n)});
                        if !d.is_empty() {
                            rows.extend([format!("`{name}`：{d}"), "".into()]);
                        }
                    }
                }
                for base in &c.bases {
                    if let Some(target) = aliases[module].get(&base.to_string()) {
                        rows.extend([
                            format!(
                                "继承接口：{}。",
                                link(&canonical(target, &aliases)?, &modules)
                            ),
                            "".into(),
                        ]);
                    }
                }
                for (name, (method, owner)) in methods(c, &classes, &mut BTreeSet::new())? {
                    render_function(
                        &mut rows,
                        &mut symbols,
                        method,
                        &format!("{key}.{name}"),
                        3,
                        if owner == c.name.as_str() {
                            None
                        } else {
                            Some(&owner)
                        },
                        module,
                        m,
                        &url,
                        notes,
                    )?;
                }
            } else {
                render_function(
                    &mut rows,
                    &mut symbols,
                    n,
                    &key,
                    2,
                    None,
                    module,
                    m,
                    &url,
                    notes,
                )?;
            }
        }
        if let Some(exports) = exports {
            for name in exports {
                ensure!(
                    symbols.get(format!("{module}.{name}")).is_some(),
                    "Undocumented public export"
                );
            }
        }
        pages.insert(
            format!("python/{}", page_name(module)),
            rows.join("\n").trim_end().to_string() + "\n",
        );
    }
    for (name, spec) in obj(&symbols)? {
        if spec["kind"] == "export" {
            let target = s(&spec["target"])?;
            ensure!(
                symbols.get(target).is_some() || modules.contains_key(target),
                "Unresolved public export {name}"
            );
        }
    }
    for p in EXTRA_SOURCES {
        files[*p] = json!(digest(&safe(root, p)?)?);
    }
    let header = fs::read_to_string(root.join("sdk/include/recamera_ext.h"))?;
    pages.insert("c-abi.md".into(),format!("# Native C ABI\n\n[API 索引](index.md) · [调用边界](features.md#native-and-platform)\n\n本文件保留该版本完整公开头文件及原始参数、错误码、字段、生命周期说明。\nPython 应用优先使用绑定；`osd`/`record` 入口仅限 AppMgr，不能以导出符号的存在推断应用调用权限。\n\n源码基线：`{revision}`，`sdk/include/recamera_ext.h`。\n\n原始授权声明：[SDK NOTICE 与 BSD-3-Clause 许可证](sdk-license.txt)。\n\n```c\n{}\n```\n",header.trim_end()));
    pages.insert(
        "sdk-license.txt".into(),
        format!(
            "{}\n\n{}",
            fs::read_to_string(root.join("sdk/NOTICE"))?.trim_end(),
            fs::read_to_string(root.join("sdk/LICENSE"))?
        ),
    );
    let routes = http_routes(&fs::read_to_string(root.join("market/appmgr/server.py"))?)?;
    ensure!(
        routes == obj(http_notes)?.keys().cloned().collect(),
        "Review HTTP routes: new or removed route"
    );
    let mut rows=vec!["# App Center HTTP routes".into(),"".into(),"[API 索引](index.md) · [鉴权、响应和 WS 契约](http.md)".into(),"".into(),"覆盖 server.py 的所有 HTTP 路由分支；带可选后缀/多个 action 的分支在说明中逐项展开。".into(),"`app_id` 为 `[a-z0-9-]{1,64}`，task/upload id 为 32 位小写十六进制；builtin 仅在明确允许的接口适用。".into(),"".into()];
    for route in &routes {
        let spec = &http_notes[route];
        for k in ["path", "request", "response", "features"] {
            ensure!(
                !s(&spec[k])?.trim().is_empty(),
                "Incomplete HTTP explanation"
            );
        }
        rows.extend([
            format!(
                "## {} {}",
                route.split_once(' ').unwrap().0,
                s(&spec["path"])?
            ),
            "".into(),
            format!("- 请求：{}", s(&spec["request"])?),
            format!("- 返回：{}", s(&spec["response"])?),
            format!("- 特性：{}", s(&spec["features"])?),
            "".into(),
            format!("源码路由：`{route}`"),
            "".into(),
        ]);
    }
    pages.insert("http-routes.md".into(), rows.join("\n"));
    let mut index=vec!["# Public API reference".into(),"".into(),"Read only the modules needed for the task. These references work offline.".into(),"".into(),format!("SDK source revision: `{revision}`. The builder is pinned separately by `sdk-contract-lock.json`."),"".into(),"Start with [interface characteristics and limitations](features.md).".into(),"".into(),"Module descriptions and source docstrings retain their imported upstream language and attribution.".into(),"".into(),"| Python module | Imported description |".into(),"| --- | --- |".into()];
    for module in modules.keys() {
        let short = s(&notes["modules"][module])?
            .lines()
            .next()
            .unwrap_or("")
            .replace('|', "\\|");
        index.push(format!(
            "| [{module}](python/{}) | {short} |",
            page_name(module)
        ));
    }
    index.extend(["", "Other public interfaces:", "", "- [Complete C ABI](c-abi.md)", "- [HTTP, SSE, WebSocket and platform interfaces](http.md)", "", "## Coverage", "", "The snapshot covers public definitions, exports, constructors, properties, fields, enums and local inherited methods in non-private `recamera_ext` and `kit` modules. Cross-module inheritance links to its owner.", "Compatibility and platform-only APIs remain documented with their caller restrictions; tests, private modules, caches and example CLIs are excluded.", "Signatures, annotations, defaults and original docstrings are extracted from AST without importing hardware dependencies. An absent annotation does not establish return semantics.", "Historical examples are context only. Managed entry, resource admission and result routing follow `features.md` and `managed-runtime.md`.", "API presence does not establish deployed firmware support, model accuracy, throughput or physical acceptance.", "", "## Repository maintenance", "", "The generator and editorial notes belong to the hardware repository's `tools/skill_packages/recamera/`, outside this installed Skill.", "", "From that repository, compare a compatible checkout with:", "", "```text", "python3 codecraft.py skill-reference --skill codecraft-development-recamera-pysdk --sdk-root <checkout> --check", "```", "", "This checks module/source inventories, public signatures, HTTP routes and generated pages. Review source changes before `--write --revision <full-commit>`; regeneration never changes the builder pin.", ""].map(str::to_string));
    pages.insert("index.md".into(), index.join("\n"));
    let generated = pages
        .iter()
        .map(|(n, v)| (n.clone(), json!(hash(v.as_bytes()))))
        .collect::<serde_json::Map<_, _>>();
    Ok((
        pages,
        json!({"source_revision":revision,"repository":REPOSITORY,"source_files":files,"symbols":symbols,"http_routes":routes,"generated_files":generated}),
    ))
}
fn safe_new(root: &Path, name: &str) -> Result<PathBuf> {
    ensure!(
        Path::new(name)
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_))),
        "Unsafe generated path"
    );
    let path = root.join(name);
    let mut ancestor = path.as_path();
    while !ancestor.exists() {
        ancestor = ancestor.parent().context("Invalid generated path")?;
    }
    ensure!(
        ancestor.canonicalize()?.starts_with(root.canonicalize()?),
        "Generated path escapes destination"
    );
    Ok(path)
}
pub fn reference(root: &Path, sdk: &Path, write_files: bool, revision: Option<&str>) -> Result<J> {
    let root = root.canonicalize()?;
    let sdk = sdk.canonicalize()?;
    let skill = root.join(format!("skills/{SKILL}"));
    let dest = skill.join("references/api");
    let lock = skill.join("scripts/api-reference-lock.json");
    let old = read(&lock)?;
    let selected = revision.unwrap_or(s(&old["source_revision"])?);
    ensure!(
        regex::Regex::new(r"^[a-f0-9]{40}$")?.is_match(selected),
        "Explicit full commit required"
    );
    if write_files {
        ensure!(
            revision.is_some(),
            "Reference writes require explicit --revision"
        );
        let git = |args: &[&str]| -> Result<String> {
            let r = crate::maintenance_skills::captured(
                Command::new("git").arg("-C").arg(&sdk).args(args),
                std::time::Duration::from_secs(15),
            )?;
            ensure!(r.status.success(), "Cannot verify SDK checkout");
            Ok(String::from_utf8(r.stdout)?.trim().to_owned())
        };
        ensure!(
            Path::new(&git(&["rev-parse", "--show-toplevel"])?).canonicalize()? == sdk,
            "SDK root must be checkout root"
        );
        ensure!(
            git(&["rev-parse", "HEAD"])? == selected,
            "SDK checkout differs from revision"
        );
        ensure!(
            git(&["status", "--porcelain", "--untracked-files=all"])?.is_empty(),
            "Reference writes require a clean SDK checkout"
        );
    }
    let notes = read(&root.join("tools/skill_packages/recamera/api-notes.json"))?;
    let http = read(&root.join("tools/skill_packages/recamera/api-http-notes.json"))?;
    let (pages, snapshot) = build(&sdk, selected, &notes, &http)?;
    let changed = pages
        .iter()
        .filter(|(name, bytes)| fs::read_to_string(dest.join(name)).as_ref().ok() != Some(bytes))
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>();
    let mut source_names = BTreeSet::new();
    if let Some(o) = old["source_files"].as_object() {
        source_names.extend(o.keys().cloned());
    }
    source_names.extend(obj(&snapshot["source_files"])?.keys().cloned());
    let source_changes = source_names
        .into_iter()
        .filter(|k| old["source_files"][k] != snapshot["source_files"][k])
        .collect::<Vec<_>>();
    if write_files {
        fs::create_dir_all(&dest)?;
        if let Some(previous) = old["generated_files"].as_object() {
            for n in previous.keys() {
                if !pages.contains_key(n) {
                    let path = safe_new(&dest, n)?;
                    if path.exists() {
                        fs::remove_file(path)?;
                    }
                }
            }
        }
        for (n, bytes) in &pages {
            let p = safe_new(&dest, n)?;
            fs::create_dir_all(p.parent().unwrap())?;
            fs::write(p, bytes)?;
        }
        crate::util::write(&lock, &snapshot)?;
    }
    Ok(
        json!({"status":if write_files||old==snapshot&&changed.is_empty(){"passed"}else{"failed"},"modules":obj(&notes["modules"] )?.len(),"symbols":obj(&snapshot["symbols"] )?.len(),"pages":pages.len(),"changed_pages":changed,"source_changes":source_changes,"repository_changed":write_files&&(old!=snapshot||!changed.is_empty()),"source_revision":selected,"sdk_execution":"not_run"}),
    )
}
pub fn command(root: &Path, args: &[String]) -> Result<J> {
    let mut seen = BTreeSet::new();
    let mut i = 0;
    while i < args.len() {
        let key = args[i].as_str();
        ensure!(
            ["--skill", "--sdk-root", "--revision", "--write", "--check"].contains(&key)
                && seen.insert(key),
            "Unknown or duplicate reference option: {key}"
        );
        if ["--write", "--check"].contains(&key) {
            i += 1;
        } else {
            ensure!(
                args.get(i + 1).is_some_and(|v| !v.starts_with("--")),
                "Missing value for {key}"
            );
            i += 2;
        }
    }

    let opt = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .map(String::as_str)
    };
    ensure!(
        opt("--skill") == Some(SKILL),
        "Select the reCamera SDK Skill"
    );
    let write = args.iter().any(|a| a == "--write");
    ensure!(
        write != args.iter().any(|a| a == "--check"),
        "Choose --check or --write"
    );
    reference(
        root,
        Path::new(opt("--sdk-root").context("--sdk-root required")?),
        write,
        opt("--revision"),
    )
}
