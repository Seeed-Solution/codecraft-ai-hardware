//! Source assertions and retained media validation without Python dependencies.
use crate::util::*;
use anyhow::{ensure, Context, Result};
use std::{collections::BTreeSet, fs, path::Path};
const SECTIONS: [&str; 6] = [
    "Controller",
    "Connectivity",
    "Inputs",
    "Outputs",
    "Power",
    "Accessories",
];
fn text(v: &J) -> Result<()> {
    ensure!(!s(v)?.trim().is_empty(), "Nonempty text required");
    Ok(())
}
fn normalize(s: &str) -> String {
    s.replace("**", "")
        .replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn source_notes(text: &str) -> String {
    // Authored specifications/acceptance cannot prove their own facts.
    let excluded = [
        "Profile",
        "Overview",
        "Usage",
        "Physical layout",
        "Limitations",
        "Source conflicts",
        "Sources",
        "Specifications",
        "Hardware variants",
        "Physical acceptance",
        "Assets",
    ];
    let mut keep = true;
    let mut out = String::new();
    let mut fence: Option<char> = None;
    for line in text.lines() {
        let trim = line.trim();
        if trim.starts_with("```") || trim.starts_with("~~~") {
            let c = trim.chars().next().unwrap();
            if fence == Some(c) {
                fence = None;
            } else if fence.is_none() {
                fence = Some(c);
            }
        }
        if fence.is_none() {
            if let Some(h) = line.strip_prefix("## ") {
                keep = !excluded.contains(&h.trim());
            }
        }
        if keep {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}
pub fn validate_fact(root: &Path, f: &J) -> Result<()> {
    text(&f["label"])?;
    ensure!(
        SECTIONS.contains(&s(&f["section"])?),
        "Invalid fact section"
    );
    let kind = s(&f["kind"])?;
    ensure!(
        ["integer", "number", "string", "boolean", "strings"].contains(&kind),
        "Invalid fact type"
    );
    ensure!(
        ["documented", "measured", "unknown"].contains(&s(&f["status"])?),
        "Invalid fact status"
    );
    if f["status"] == "unknown" {
        ensure!(f["value"].is_null(), "Unknown fact asserts value");
        return text(&f["reason"]);
    }
    let v = &f["value"];
    match kind {
        "integer" => ensure!(v.is_u64(), "Invalid unsigned integer fact"),
        "number" => ensure!(v.as_f64().is_some_and(|n| n >= 0.), "Invalid numeric fact"),
        "string" => text(v)?,
        "boolean" => ensure!(v.is_boolean(), "Invalid boolean fact"),
        "strings" => {
            let values = strings(v)?;
            ensure!(
                !values.is_empty() && values.iter().all(|s| !s.is_empty()),
                "Empty string list"
            );
        }
        _ => unreachable!(),
    }
    if ["integer", "number"].contains(&kind) {
        text(&f["unit"])?;
    }
    let template = s(&f["source"]["template"])?;
    ensure!(
        template.matches("{value}").count() == 1
            && !template.replace("{value}", "").contains(['{', '}']),
        "Assertion must contain only one value placeholder"
    );
    let path = safe(root, s(&f["source"]["path"])?)?;
    let raw = fs::read_to_string(&path)?;
    let raw = if path.file_name().and_then(|x| x.to_str()) == Some("AGENTS.md")
        && path
            .parent()
            .and_then(Path::parent)
            .and_then(Path::file_name)
            .is_some_and(|x| x == "devices")
    {
        source_notes(&raw)
    } else {
        raw
    };
    let source = normalize(&raw);
    let values = if kind == "strings" {
        arr(v)?.clone()
    } else {
        vec![v.clone()]
    };
    for item in values {
        let value = match item {
            J::String(s) => s,
            J::Bool(b) => {
                if b {
                    "True".into()
                } else {
                    "False".into()
                }
            }
            v => v.to_string(),
        };
        let assertion = normalize(&template.replace("{value}", &value));
        ensure!(
            source.contains(&assertion),
            "Fact contradicts cited independent source: {assertion}"
        );
    }
    Ok(())
}
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for b in bytes {
        crc ^= *b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ if crc & 1 != 0 { 0xedb88320 } else { 0 };
        }
    }
    !crc
}
pub fn validate_asset(root: &Path, d: &J, a: &J) -> Result<()> {
    let image = safe(root, s(&a["path"])?)?;
    text(&a["source"])?;
    text(&a["license"])?;
    ensure!(
        a["license"] != "unknown" && a["revision"] == d["revision"] && d["revision"] != "unknown",
        "Asset provenance/revision required"
    );
    ensure!(digest(&image)? == s(&a["sha256"])?, "Asset hash mismatch");
    ensure!(
        image
            .extension()
            .and_then(|x| x.to_str())
            .is_some_and(|v| v.eq_ignore_ascii_case("png")),
        "PNG required"
    );
    let raw = fs::read(image)?;
    ensure!(
        raw.len() >= 33 && raw[..8] == *b"\x89PNG\r\n\x1a\n" && raw[12..16] == *b"IHDR",
        "Invalid PNG header"
    );
    let u32_at = |i: usize| u32::from_be_bytes(raw[i..i + 4].try_into().unwrap());
    let mut offset = 8usize;
    let mut idat = false;
    let mut ended = false;
    let mut count = 0;
    while offset < raw.len() {
        ensure!(
            !ended && offset + 12 <= raw.len(),
            "Truncated or trailing PNG"
        );
        let len = u32_at(offset) as usize;
        let end = offset
            .checked_add(8)
            .and_then(|v| v.checked_add(len))
            .context("PNG size overflow")?;
        ensure!(
            end.checked_add(4).is_some_and(|v| v <= raw.len()),
            "PNG chunk truncated"
        );
        ensure!(
            crc32(&raw[offset + 4..end]) == u32_at(end),
            "PNG checksum mismatch"
        );
        let kind = &raw[offset + 4..offset + 8];
        if count == 0 {
            ensure!(kind == b"IHDR" && len == 13, "Invalid PNG IHDR");
        } else {
            ensure!(kind != b"IHDR", "Duplicate PNG IHDR");
        }
        if kind == b"IDAT" {
            idat = true;
        }
        if kind == b"IEND" {
            ensure!(len == 0, "Invalid IEND");
            ended = true;
        }
        count += 1;
        offset = end + 4;
    }
    ensure!(idat && ended, "Incomplete PNG");
    let w = u32_at(16);
    let h = u32_at(20);
    ensure!(
        w > 0 && h > 0 && a["width"] == w && a["height"] == h,
        "PNG dimensions mismatch"
    );
    ensure!(
        a["alpha"].is_boolean() && a["alpha"] == matches!(raw[25], 4 | 6),
        "PNG alpha mismatch"
    );
    Ok(())
}
pub fn validate(root: &Path, d: &J) -> Result<()> {
    ensure!(
        d["schema_version"] == 5
            && !obj(d)?.contains_key("targets")
            && !obj(d)?.contains_key("specs"),
        "Device schema or duplicated facts"
    );
    for key in [
        "name",
        "maintenance",
        "revision",
        "overview",
        "scenarios",
        "physical_description",
    ] {
        text(&d[key])?;
    }
    ensure!(
        ["experimental", "stable", "deprecated"].contains(&s(&d["maturity"])?),
        "Invalid maturity"
    );
    let facts = obj(&d["facts"])?;
    ensure!(!facts.is_empty(), "Missing typed facts");
    let valid = regex::Regex::new(r"^[a-z][a-z0-9_]*$")?;
    let references = strings(&d["references"])?;
    ensure!(!references.is_empty(), "Device references required");
    for r in &references {
        safe(root, r)?;
    }
    let validate_facts = |facts: &serde_json::Map<String, J>| -> Result<()> {
        for (key, f) in facts {
            ensure!(valid.is_match(key), "Invalid fact ID");
            validate_fact(root, f)?;
            ensure!(
                f["status"] == "unknown"
                    || references.contains(&s(&f["source"]["path"])?.to_owned()),
                "Fact source outside closure"
            );
        }
        Ok(())
    };
    validate_facts(facts)?;
    ensure!(
        facts
            .values()
            .map(|f| s(&f["section"]))
            .collect::<Result<BTreeSet<_>>>()?
            == SECTIONS.into_iter().collect(),
        "Missing specification category"
    );
    let variant_id = regex::Regex::new(r"^[a-z][a-z0-9_-]*$")?;
    for (name, v) in obj(&d["variants"])? {
        ensure!(variant_id.is_match(name), "Invalid variant ID");
        text(&v["label"])?;
        ensure!(
            references.contains(&s(&v["source"])?.to_owned()),
            "Variant source outside closure"
        );
        ensure!(
            v["facts"]["compatible_revisions"]["kind"] == "strings",
            "Variant compatibility must be a string list"
        );
        validate_facts(obj(&v["facts"])?)?;
    }
    for item in arr(&d["unknowns"])? {
        text(item)?;
    }
    for conflict in arr(&d["conflicts"])? {
        text(&conflict["resolution"])?;
        safe(root, s(&conflict["source"])?)?;
    }
    ensure!(
        strings(&d["vendor_sources"])?
            .iter()
            .all(|v| v.starts_with("https://")),
        "Invalid vendor URL"
    );
    let acceptance = &d["acceptance"];
    ensure!(
        obj(acceptance)?
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            == ["preconditions", "cases"].into_iter().collect(),
        "Acceptance fields mismatch"
    );
    s(&acceptance["preconditions"])?;
    let cases = arr(&acceptance["cases"])?;
    ensure!(!cases.is_empty(), "Physical acceptance cases required");
    let mut ids = BTreeSet::new();
    for c in cases {
        for key in ["id", "procedure", "expected", "source"] {
            text(&c[key])?;
        }
        ensure!(ids.insert(s(&c["id"])?), "Duplicate acceptance ID");
        ensure!(
            strings(&c["interfaces"])?.iter().all(|v| !v.is_empty()),
            "Invalid interface list"
        );
        safe(root, s(&c["source"])?)?;
    }
    for a in arr(&d["assets"])? {
        validate_asset(root, d, a)?;
    }
    Ok(())
}
