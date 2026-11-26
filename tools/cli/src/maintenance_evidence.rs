//! Immutable source-bound evidence. Historical schema 2 records are archival only.
use crate::{catalog::Catalog, platform, project, util::*};
use anyhow::{bail, ensure, Context, Result};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
    process::Command,
};
const STATUSES: [&str; 6] = [
    "passed",
    "failed",
    "skipped",
    "not_run",
    "not_supported",
    "unknown",
];
fn hash_ok(v: &J) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 64
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}
fn text(v: &J) -> Result<()> {
    ensure!(!s(v)?.trim().is_empty(), "Nonempty evidence text required");
    Ok(())
}
fn timestamp(v: &J) -> Result<String> {
    // Require a fully specified UTC timestamp so lexicographic order is chronological.
    let raw = s(v)?;
    let re = regex::Regex::new(
        r"^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(?:Z|\+00:00)$",
    )?;
    let caps = re
        .captures(raw)
        .context("Evidence timestamp must use ISO-8601 UTC")?;
    let n = |i: usize| caps[i].parse::<u32>();
    let y = n(1)?;
    let m = n(2)?;
    let d = n(3)?;
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let days = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        _ => 0,
    };
    ensure!(
        y > 0 && d > 0 && d <= days && n(4)? < 24 && n(5)? < 60 && n(6)? < 60,
        "Invalid evidence timestamp"
    );
    let fraction = raw[19..]
        .strip_prefix('.')
        .map(|tail| tail.split(['Z', '+']).next().unwrap())
        .unwrap_or("");
    ensure!(
        fraction.len() <= 9,
        "Evidence timestamp precision exceeds nanoseconds"
    );
    Ok(format!("{}.{:0<9}Z", &raw[..19], fraction))
}
pub fn validate_log(c: &Catalog, t: &J, path: &J, expected: &J) -> Result<()> {
    ensure!(hash_ok(expected), "Invalid evidence log hash");
    let path = safe(&c.root, s(path)?)?.canonicalize()?;
    let directory = c
        .root
        .join("tests/devices")
        .join(s(&t["device"])?)
        .join("logs");
    ensure!(
        path.starts_with(directory.canonicalize()?),
        "Evidence log outside owning device logs"
    );
    ensure!(digest(&path)? == s(expected)?, "Evidence log hash mismatch");
    let bytes = fs::read(&path)?;
    let content = String::from_utf8_lossy(&bytes);
    ensure!(
        !regex::Regex::new(r"(?i)(access[_-]?token|password|secret|device[_-]?key)\s*[:=]\s*\S+")?
            .is_match(&content),
        "Sanitize evidence log before recording"
    );
    Ok(())
}
fn cases(rows: &J, expected: Option<Vec<String>>, result: &J) -> Result<()> {
    let rows = arr(rows)?;
    ensure!(!rows.is_empty(), "Evidence requires cases");
    let mut names = BTreeSet::new();
    for row in rows {
        ensure!(
            names.insert(s(&row["name"])?.to_owned()),
            "Duplicate evidence case"
        );
        ensure!(
            STATUSES[..5].contains(&s(&row["status"])?),
            "Invalid evidence status"
        );
        if row["status"] != "passed" {
            text(&row["reason"])?;
        }
    }
    if let Some(expected) = expected {
        ensure!(
            names == expected.into_iter().collect(),
            "Evidence must cover complete expected native cases"
        );
    }
    ensure!(
        (result == "passed") == rows.iter().all(|x| x["status"] == "passed"),
        "Evidence result/cases mismatch"
    );
    Ok(())
}
fn toolchains(t: &J, receipt: &J, rows: &J) -> Result<()> {
    platform::evidence_toolchain(t, &receipt["toolchain"])?;
    for row in arr(rows)? {
        if row["status"] == "passed" {
            ensure!(
                row["toolchain"] == receipt["toolchain"],
                "Case toolchain differs from recorded probe"
            );
            platform::evidence_toolchain(t, &row["toolchain"])?;
        }
    }
    Ok(())
}
fn project(c: &Catalog, t: &J, record: &J, p: &J, variant: Option<&str>) -> Result<()> {
    ensure!(
        p["status"] == "passed"
            && p["provenance"] == "passed"
            && p["origin_source_digest"] == record["source_digest"],
        "Generated project provenance mismatch"
    );
    ensure!(
        hash_ok(&p["source_digest"]),
        "Missing project source digest"
    );
    ensure!(
        p["cli"] == record["cli"],
        "Generated project CLI identity mismatch"
    );
    let selected = c.selected(s(&t["id"])?, Some(s(&p["example"])?), variant)?;
    ensure!(
        p.get("variant").and_then(J::as_str)
            == selected.get("selected_variant").and_then(J::as_str),
        "Generated project variant mismatch"
    );
    cases(
        &p["build_results"],
        Some(platform::evidence_cases(&c.root, &selected)?),
        &p["status"],
    )?;
    toolchains(&selected, p, &p["build_results"])?;
    validate_log(c, t, &p["log"], &p["log_sha256"])?;
    for row in arr(&p["build_results"])? {
        validate_log(c, t, &row["log"], &row["log_sha256"])?;
    }
    Ok(())
}
pub fn validate(c: &Catalog, record: &J, current: bool) -> Result<()> {
    ensure!(record["schema_version"]==3 && record["graph_schema_version"]==4,"Evidence requires record schema 3 and creator graph 4; historical records cannot be promoted");
    let t = c.target(s(&record["target"])?)?;
    ensure!(
        ["build", "device", "simulator"].contains(&s(&record["kind"])?),
        "Unsupported evidence kind"
    );
    ensure!(
        STATUSES[..5].contains(&s(&record["result"])?),
        "Invalid evidence result"
    );
    ensure!(
        regex::Regex::new(r"^[0-9a-f]{40}$")?.is_match(s(&record["repository_commit"])?)
            && record["repository_dirty"].is_boolean(),
        "Invalid repository provenance"
    );
    ensure!(hash_ok(&record["source_digest"]), "Invalid source digest");
    timestamp(&record["executed_at"])?;
    arr(&record["limitations"])?;
    for key in ["toolchain", "command", "environment", "operator"] {
        ensure!(
            !record[key].is_null()
                && record[key] != json!("")
                && record[key] != json!({})
                && record[key] != json!([]),
            "Missing evidence {key}"
        );
    }
    text(&record["revision"])?;
    ensure!(
        record["cli"]["contract"] == crate::CONTRACT && hash_ok(&record["cli"]["sha256"]),
        "Missing compiled CLI identity"
    );
    text(&record["cli"]["version"])?;
    let matches = record["source_digest"] == c.graph(t)?.fingerprint()
        && record["cli"] == project::cli_identity()?;
    if current {
        ensure!(matches, "Evidence source or CLI identity is stale");
    }
    validate_log(c, t, &record["log"], &record["log_sha256"])?;
    cases(&record["cases"], None, &record["result"])?;
    match s(&record["kind"])? {
        "build" => {
            ensure!(
                record.get("variant").is_none_or(J::is_null),
                "Reference build evidence must cover all variants"
            );
            if matches {
                cases(
                    &record["cases"],
                    Some(platform::evidence_cases(&c.root, t)?),
                    &record["result"],
                )?;
            }
            for row in arr(&record["cases"])? {
                ensure!(!row["toolchain"].is_null(), "Missing case toolchain");
                if row["status"] == "passed" || row.get("log").is_some() {
                    validate_log(c, t, &row["log"], &row["log_sha256"])?;
                }
            }
            if record["result"] == "passed" {
                let p = &record["generated_project"];
                if matches {
                    toolchains(t, record, &record["cases"])?;
                    project(c, t, record, p, None)?;
                    let variants = obj(&t["variants"])?;
                    if !variants.is_empty() {
                        let children = obj(&p["variant_projects"])?;
                        ensure!(
                            children.keys().collect::<BTreeSet<_>>() == variants.keys().collect(),
                            "Generated projects must cover every variant"
                        );
                        for (name, child) in children {
                            ensure!(
                                child["example"] == p["example"],
                                "Variant project example mismatch"
                            );
                            project(c, t, record, child, Some(name))?;
                        }
                    }
                } else {
                    ensure!(
                        p["status"] == "passed"
                            && p["provenance"] == "passed"
                            && p["origin_source_digest"] == record["source_digest"],
                        "Incomplete historical project receipt"
                    );
                    validate_log(c, t, &p["log"], &p["log_sha256"])?;
                }
            }
        }
        "device" => {
            ensure!(
                record["revision"] != "unknown",
                "Physical evidence requires observed revision"
            );
            let d = &c.devices[s(&t["device"])?];
            let known: BTreeSet<_> = arr(&d["acceptance"]["cases"])?
                .iter()
                .map(|x| s(&x["id"]))
                .collect::<Result<_>>()?;
            for row in arr(&record["cases"])? {
                if matches {
                    ensure!(known.contains(s(&row["name"])?), "Unknown physical case");
                }
                text(&row["observation"])?;
                text(&row["procedure"])?;
            }
            if matches {
                let variants = obj(&t["variants"])?;
                if variants.is_empty() {
                    ensure!(
                        record.get("variant").is_none_or(J::is_null),
                        "Unexpected hardware variant"
                    );
                } else {
                    let name = s(&record["variant"])?;
                    let v = variants
                        .get(name)
                        .context("Physical evidence variant mismatch")?;
                    let facts =
                        &d["variants"][s(&v["device_variant"])?]["facts"]["compatible_revisions"];
                    if facts["status"] != "unknown" {
                        ensure!(
                            strings(&facts["value"])?.contains(&s(&record["revision"])?.to_owned()),
                            "Physical revision incompatible with variant"
                        );
                    }
                }
            }
        }
        "simulator" => bail!("No registered simulator; simulator evidence is not supported"),
        _ => unreachable!(),
    }
    Ok(())
}
pub fn support(c: &Catalog, tid: &str) -> Result<J> {
    let t = c.target(tid)?;
    let d = &c.devices[s(&t["device"])?];
    let current = c.graph(t)?.fingerprint();
    let cli = project::cli_identity()?;
    let mut state = json!({"level":"unknown","build":"not_run","project":"not_run","device":"not_run","records":[],"stale_records":[],"archived_records":[],"interfaces":{}});
    for i in arr(&t["interfaces"])? {
        state["interfaces"][s(&i["id"])?] = json!("unknown");
    }
    let dir = c
        .root
        .join("tests/devices")
        .join(s(&t["device"])?)
        .join("records");
    let mut records = vec![];
    if dir.exists() {
        for entry in fs::read_dir(dir)? {
            let p = entry?.path();
            if p.extension().and_then(|v| v.to_str()) != Some("json") {
                continue;
            }
            let r = read(&p)?;
            if r["target"] != tid {
                continue;
            }
            let relative = p
                .strip_prefix(&c.root)?
                .to_string_lossy()
                .replace('\\', "/");
            if r["schema_version"] == 2 {
                state["archived_records"].as_array_mut().unwrap().push(json!({"path":relative,"reason":"Historical schema 2 / reference graph; never promoted into creator graph 4"}));
                state["stale_records"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(relative));
                continue;
            }
            validate(c, &r, false)?;
            if r["source_digest"] != current || r["cli"] != cli {
                state["stale_records"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(relative));
                continue;
            }
            records.push((timestamp(&r["executed_at"])?, relative, r));
        }
    }
    records.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    let variants = obj(&t["variants"])?;
    let mut physical: BTreeMap<String, BTreeMap<String, BTreeMap<String, String>>> =
        if variants.is_empty() {
            BTreeMap::from([(String::new(), BTreeMap::new())])
        } else {
            variants
                .keys()
                .map(|v| (v.clone(), BTreeMap::new()))
                .collect()
        };
    for (_, path, r) in records {
        state["records"].as_array_mut().unwrap().push(json!(path));
        if r["kind"] == "build" {
            state["build"] = r["result"].clone();
            state["project"] = r["generated_project"]
                .get("status")
                .cloned()
                .unwrap_or(json!("not_run"));
        } else if r["kind"] == "device" {
            let v = r["variant"].as_str().unwrap_or("");
            let compatible = if variants.is_empty() {
                r["revision"] == d["revision"]
            } else {
                let fact = &d["variants"][s(&variants[v]["device_variant"])?]["facts"]
                    ["compatible_revisions"];
                fact["status"] != "unknown"
                    && strings(&fact["value"])?.contains(&s(&r["revision"])?.to_owned())
            };
            if compatible {
                for row in arr(&r["cases"])? {
                    physical
                        .get_mut(v)
                        .context("Unknown evidence variant")?
                        .entry(s(&r["revision"])?.to_owned())
                        .or_default()
                        .insert(s(&row["name"])?.into(), s(&row["status"])?.into());
                }
            }
        }
    }
    let acceptance = arr(&d["acceptance"]["cases"])?;
    let status = |observations: &BTreeMap<String, String>| {
        if acceptance.iter().all(|a| {
            observations
                .get(a["id"].as_str().unwrap_or(""))
                .is_some_and(|v| v == "passed")
        }) {
            "passed"
        } else if observations.values().any(|v| v == "failed") {
            "failed"
        } else {
            "not_run"
        }
    };
    let variant_status = |revisions: &BTreeMap<String, BTreeMap<String, String>>| {
        if revisions
            .values()
            .any(|observations| status(observations) == "passed")
        {
            "passed"
        } else if revisions
            .values()
            .any(|observations| status(observations) == "failed")
        {
            "failed"
        } else {
            "not_run"
        }
    };
    state["device"] = json!(
        if physical.values().all(|o| variant_status(o) == "passed") {
            "passed"
        } else if physical.values().any(|o| variant_status(o) == "failed") {
            "failed"
        } else {
            "not_run"
        }
    );
    state["physical_revisions"] = object();
    for (variant, revisions) in &physical {
        let mut details = object();
        for (revision, observations) in revisions {
            details[revision] = json!({"device":status(observations),"cases":observations});
        }
        state["physical_revisions"][variant] = details.clone();
        if !variants.is_empty() {
            if state.get("variants").is_none() {
                state["variants"] = object();
            }
            state["variants"][variant] =
                json!({"device":variant_status(revisions),"revisions":details});
        }
    }
    for i in arr(&t["interfaces"])? {
        let id = s(&i["id"])?;
        let applicable: Vec<_> = acceptance
            .iter()
            .filter(|a| {
                a["interfaces"]
                    .as_array()
                    .is_some_and(|v| v.is_empty() || v.contains(&json!(id)))
            })
            .collect();
        if !applicable.is_empty()
            && physical.values().all(|revisions| {
                revisions.values().any(|o| {
                    applicable.iter().all(|a| {
                        o.get(a["id"].as_str().unwrap_or(""))
                            .is_some_and(|v| v == "passed")
                    })
                })
            })
        {
            state["interfaces"][id] = json!("device-verified");
        }
    }
    if state["build"] == "passed" && state["project"] == "passed" {
        state["level"] = json!(if state["device"] == "passed" {
            "device-verified"
        } else {
            "build-verified"
        });
    }
    for key in ["stale_records", "archived_records"] {
        state[key].as_array_mut().unwrap().sort_by_key(J::to_string);
    }
    Ok(state)
}
pub fn record(c: &Catalog, input: &Path) -> Result<J> {
    let r = read(input)?;
    validate(c, &r, true)?;
    ensure!(
        Command::new("git")
            .args([
                "cat-file",
                "-e",
                &format!("{}^{{commit}}", s(&r["repository_commit"])?)
            ])
            .current_dir(&c.root)
            .output()?
            .status
            .success(),
        "Evidence commit is unknown"
    );
    let t = c.target(s(&r["target"])?)?;
    let rel = format!(
        "tests/devices/{}/records/{}-{}.json",
        s(&t["device"])?,
        s(&r["kind"])?,
        &identity(&r)[..16]
    );
    let path = c.root.join(&rel);
    fs::create_dir_all(path.parent().unwrap())?;
    let parent = path.parent().unwrap().canonicalize()?;
    ensure!(
        parent.starts_with(c.root.join("tests/devices").join(s(&t["device"])?)),
        "Evidence output escapes owner"
    );
    let bytes = serde_json::to_vec_pretty(&r)?;
    // Write and sync privately, then publish with an exclusive hard link. Readers
    // never see a partially written JSON record and existing observations cannot
    // be overwritten, even by a concurrent registration of the same identity.
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();
    let staging = parent.join(format!(".evidence-{}-{nonce}.tmp", std::process::id()));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&staging)?;
    let written = file
        .write_all(&bytes)
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_all());
    drop(file);
    if let Err(error) = written {
        let _ = fs::remove_file(&staging);
        return Err(error.into());
    }
    let published = fs::hard_link(&staging, &path).context("Cannot publish evidence: destination exists or filesystem does not support atomic hard links");
    let cleanup = fs::remove_file(&staging);
    published?;
    cleanup?;
    Ok(json!({"path":rel,"result":r["result"]}))
}
pub fn command(root: &Path, args: &[String]) -> Result<J> {
    ensure!(
        args.len() == 2 && args[0] == "--file",
        "Usage: record-evidence --file <record.json>"
    );
    record(&Catalog::load(root)?, Path::new(&args[1]))
}
