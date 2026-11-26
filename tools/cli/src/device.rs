//! Authored Markdown remains the hardware source; no generated fact sidecars.
use crate::util::*;
use anyhow::{ensure, Context, Result};
use regex::Regex;
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};
fn re(s: &str) -> Regex {
    Regex::new(s).expect("static regex")
}
fn unescape(s: &str) -> String {
    let mut v = s
        .replace("&#124;", "|")
        .replace("&#10;", "\n")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    v = v.replace("&amp;", "&");
    v
}
fn sections(text: &str, level: usize) -> Result<BTreeMap<String, String>> {
    let marker = format!("{} ", "#".repeat(level));
    let mut out = BTreeMap::new();
    let mut key = None;
    let mut body = String::new();
    let mut fence: Option<char> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let c = trimmed.chars().next().unwrap();
            if fence == Some(c) {
                fence = None
            } else if fence.is_none() {
                fence = Some(c)
            };
        }
        if fence.is_none() && line.starts_with(&marker) {
            if let Some(k) = key.take() {
                ensure!(
                    out.insert(k, body.trim().to_owned()).is_none(),
                    "Duplicate heading"
                );
            }
            key = Some(line[marker.len()..].trim().to_owned());
            body.clear();
        } else if key.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(k) = key {
        ensure!(
            out.insert(k, body.trim().into()).is_none(),
            "Duplicate heading"
        );
    }
    ensure!(fence.is_none(), "Unclosed Markdown fence");
    Ok(out)
}
fn table(text: &str, columns: &[&str]) -> Result<Vec<Vec<String>>> {
    let lines: Vec<_> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    ensure!(lines.len() >= 2, "Missing Markdown table");
    let mut rows = vec![];
    for l in lines {
        let l = l.trim();
        ensure!(l.starts_with('|') && l.ends_with('|'), "Malformed table");
        rows.push(
            l[1..l.len() - 1]
                .split('|')
                .map(|x| x.trim().to_owned())
                .collect::<Vec<_>>(),
        );
    }
    ensure!(
        rows[0] == columns && rows.iter().all(|r| r.len() == columns.len()),
        "Invalid table columns"
    );
    ensure!(
        rows[1].iter().all(|v| re(r"^:?-{3,}:?$").is_match(v)),
        "Invalid table separator"
    );
    Ok(rows.into_iter().skip(2).collect())
}
fn link(cell: &str, defs: &BTreeMap<String, String>) -> Result<String> {
    let m = re(r"^\[([^\]]+)\]\[([^\]]*)\]$")
        .captures(cell)
        .context("Named source reference required")?;
    let key = if m[2].is_empty() { &m[1] } else { &m[2] };
    defs.get(&key.to_lowercase())
        .cloned()
        .context("Undefined source reference")
}
fn local(entry: &str, url: &str) -> Result<String> {
    ensure!(
        !url.contains("://") && !url.starts_with('/') && !url.starts_with("mailto:"),
        "Local source required"
    );
    let raw = percent_encoding::percent_decode_str(url.split('#').next().unwrap()).decode_utf8()?;
    let joined = Path::new(entry).parent().unwrap().join(raw.as_ref());
    let mut p = PathBuf::new();
    for c in joined.components() {
        match c {
            Component::ParentDir => {
                ensure!(p.pop(), "Source escapes root");
            }
            Component::Normal(x) => p.push(x),
            Component::CurDir => {}
            _ => anyhow::bail!("Absolute source"),
        }
    }
    Ok(p.to_string_lossy().replace('\\', "/"))
}
fn scalar(kind: &str, value: &str) -> Result<J> {
    let v = unescape(value);
    Ok(match kind {
        "string" => json!(v),
        "integer" => json!(v.parse::<u64>()?),
        "number" => {
            let j = parse(&v)?;
            ensure!(j.is_number(), "Expected number");
            j
        }
        "boolean" => json!(v.parse::<bool>()?),
        "strings" => {
            let j = parse(&v)?;
            strings(&j)?;
            j
        }
        _ => anyhow::bail!("Unknown fact type {kind}"),
    })
}
fn facts(text: &str, level: usize, defs: &BTreeMap<String, String>, entry: &str) -> Result<J> {
    let mut out = object();
    for (section, body) in sections(text, level)? {
        ensure!(
            [
                "Controller",
                "Connectivity",
                "Inputs",
                "Outputs",
                "Power",
                "Accessories"
            ]
            .contains(&section.as_str()),
            "Unknown specification section"
        );
        for row in table(
            &body,
            &[
                "Fact",
                "Type",
                "Value",
                "Unit",
                "Status",
                "Source",
                "Assertion / reason",
            ],
        )? {
            let m = re(r"^(.+) \(`([a-z][a-z0-9_]*)`\)$")
                .captures(&row[0])
                .context("Missing fact ID")?;
            let id = &m[2];
            ensure!(out.get(id).is_none(), "Duplicate fact {id}");
            let status = &row[4];
            ensure!(
                ["documented", "measured", "unknown"].contains(&status.as_str()),
                "Invalid fact status"
            );
            let mut f =
                json!({"label":unescape(&m[1]),"section":section,"kind":row[1],"status":status});
            if row[3] != "—" {
                f["unit"] = json!(unescape(&row[3]));
            }
            if status == "unknown" {
                ensure!(row[2] == "—" && row[5] == "—", "Unknown fact asserts value");
                f["value"] = J::Null;
                f["reason"] = json!(unescape(&row[6]));
            } else {
                f["value"] = scalar(&row[1], &row[2])?;
                f["source"] =
                    json!({"path":local(entry,&link(&row[5],defs)?)?,"template":unescape(&row[6])});
            }
            out[id] = f;
        }
    }
    Ok(out)
}
pub fn read_device(root: &Path, id: &str) -> Result<J> {
    let entry = format!("devices/{id}/AGENTS.md");
    let text = fs::read_to_string(safe(root, &entry)?)?.replace("\r\n", "\n");
    let mut defs = BTreeMap::new();
    let definition = re(r"(?m)^ {0,3}\[([^\]\n]+)\]:\s*(\S+)\s*$");
    for m in definition.captures_iter(&text) {
        ensure!(
            defs.insert(
                m[1].to_lowercase(),
                m[2].trim_matches(['<', '>']).to_owned()
            )
            .is_none(),
            "Duplicate reference"
        );
    }
    let cleaned = definition.replace_all(&text, "");
    let b = sections(&cleaned, 2)?;
    let get = |k: &str| {
        b.get(k)
            .map(String::as_str)
            .with_context(|| format!("Missing Device section {k}"))
    };
    let titles: Vec<_> = text.lines().filter_map(|x| x.strip_prefix("# ")).collect();
    ensure!(titles.len() == 1, "Exactly one Device title required");
    let mut d = json!({"name":titles[0]});
    for row in table(get("Profile")?, &["Field", "Value"])? {
        let field = match row[0].as_str() {
            "ID" => "id",
            "Format" => "schema_version",
            "Revision" => "revision",
            "Maintenance" => "maintenance",
            "Maturity" => "maturity",
            _ => anyhow::bail!("Unknown profile field"),
        };
        ensure!(d.get(field).is_none(), "Duplicate profile field");
        d[field] = if field == "schema_version" {
            scalar("integer", &row[1])?
        } else {
            json!(unescape(&row[1]))
        };
    }
    ensure!(
        d["id"] == id && d["schema_version"] == 5,
        "Device identity/schema mismatch"
    );
    for (k, h) in [
        ("overview", "Overview"),
        ("scenarios", "Usage"),
        ("physical_description", "Physical layout"),
    ] {
        d[k] = json!(get(h)?);
    }
    d["facts"] = facts(get("Specifications")?, 3, &defs, &entry)?;
    d["variants"] = object();
    if let Some(body) = b.get("Hardware variants") {
        for (h, content) in sections(body, 3)? {
            let (id, label) = h.split_once(": ").context("Invalid variant heading")?;
            let (source, rest) = content
                .split_once("\n\n")
                .context("Missing variant facts")?;
            let source = source
                .strip_prefix("Source: ")
                .context("Missing variant source")?;
            d["variants"][id] = json!({"label":label,"source":local(&entry,&link(source,&defs)?)?,"facts":facts(rest,4,&defs,&entry)?});
        }
    }
    let limits = get("Limitations")?;
    d["unknowns"] = if limits == "None recorded." {
        json!([])
    } else {
        json!(limits
            .lines()
            .map(|l| l.strip_prefix("- ").context("Invalid limitation"))
            .collect::<Result<Vec<_>>>()?)
    };
    let mut refs = vec![];
    let mut vendors = vec![];
    for line in get("Sources")?.lines() {
        let url = link(line.strip_prefix("- ").context("Invalid source")?, &defs)?;
        if url.starts_with("https://") {
            vendors.push(url)
        } else {
            refs.push(local(&entry, &url)?);
        }
    }
    d["references"] = json!(refs);
    d["vendor_sources"] = json!(vendors);
    d["conflicts"] = json!([]);
    if let Some(body) = b.get("Source conflicts") {
        d["conflicts"] = json!(table(body, &["Source", "Resolution"])?
            .iter()
            .map(|r| Ok(
                json!({"source":local(&entry,&link(&r[0],&defs)?)?,"resolution":unescape(&r[1])})
            ))
            .collect::<Result<Vec<J>>>()?);
    }
    let mut acceptance = sections(get("Physical acceptance")?, 3)?;
    let pre = acceptance
        .remove("Preconditions")
        .context("Missing acceptance preconditions")?;
    let mut cases = vec![];
    // Acceptance is an authored procedure: identity/recovery checks retain their
    // documented order instead of the parser map's alphabetical key order.
    let acceptance_text = get("Physical acceptance")?;
    let mut ordered: Vec<_> = acceptance.into_iter().collect();
    ordered.sort_by_key(|(id, _)| {
        acceptance_text
            .find(&format!("### {id}\n"))
            .unwrap_or(usize::MAX)
    });
    for (id, body) in ordered {
        let (meta, procedure) = body
            .split_once("\n\n")
            .context("Invalid acceptance metadata")?;
        let m: Vec<_> = meta.lines().collect();
        ensure!(m.len() == 2, "Invalid acceptance scope");
        let interfaces = m[0]
            .strip_prefix("- Interfaces: ")
            .context("Missing interfaces")?;
        let source = m[1].strip_prefix("- Source: ").context("Missing source")?;
        let (procedure, expected) = procedure
            .rsplit_once("\n\nExpected: ")
            .context("Missing expected result")?;
        cases.push(json!({"id":id,"interfaces":if interfaces=="none"{vec![]}else{interfaces.split(", ").collect::<Vec<_>>()},"source":local(&entry,&link(source,&defs)?)?,"procedure":procedure,"expected":expected}));
    }
    d["acceptance"] = json!({"preconditions":if pre=="None."{""}else{&pre},"cases":cases});
    let mut assets = vec![];
    if let Some(body) = b.get("Assets") {
        for r in table(
            body,
            &[
                "File", "Origin", "License", "SHA256", "Revision", "Width", "Height", "Alpha",
            ],
        )? {
            assets.push(json!({"path":local(&entry,&link(&r[0],&defs)?)?,"source":unescape(&r[1]),"license":unescape(&r[2]),"sha256":r[3],"revision":r[4],"width":scalar("integer",&r[5])?,"height":scalar("integer",&r[6])?,"alpha":scalar("boolean",&r[7])?}));
        }
    }
    d["assets"] = json!(assets);
    crate::maintenance_device::validate(root, &d)?;
    Ok(d)
}
