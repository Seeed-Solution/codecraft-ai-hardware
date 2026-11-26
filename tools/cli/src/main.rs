use anyhow::{ensure, Context, Result};
use clap::{Parser, Subcommand};
use codecraft::{catalog::Catalog, project, util::*};
use serde_json::json;
use std::path::PathBuf;
#[derive(Parser)]
#[command(
    version,
    about = "Create hardware applications without a Python or Rust installation"
)]
struct Args {
    #[arg(long, global = true, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Catalog,
    /// Inspect the selected native environment without building or changing locks.
    Doctor {
        #[arg(long)]
        target: String,
    },
    Firmware {
        #[command(subcommand)]
        command: ImageCommand,
    },
    Discover {
        #[arg(long)]
        interface: Vec<String>,
        #[arg(long)]
        platform: Option<String>,
        #[arg(long)]
        constraints: Option<PathBuf>,
    },
    Context {
        #[arg(long)]
        target: String,
        #[arg(long)]
        example: Option<String>,
        #[arg(long)]
        variant: Option<String>,
        #[arg(long, default_value = "develop")]
        task: String,
        #[arg(long)]
        skills: Option<PathBuf>,
    },
    New {
        #[arg(long)]
        target: String,
        #[arg(long)]
        example: String,
        #[arg(long)]
        variant: Option<String>,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        brief: Option<PathBuf>,
        #[arg(long)]
        skills: Option<PathBuf>,
    },
    VerifyProject {
        path: PathBuf,
    },
    LockProject {
        path: PathBuf,
        #[arg(long)]
        toolchain: bool,
    },
    BuildProject {
        path: PathBuf,
    },
    Package {
        path: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Validate {
        #[arg(long)]
        r#static: bool,
        #[arg(long)]
        target: Option<String>,
        #[arg(long)]
        example: Option<String>,
        #[arg(long)]
        variant: Option<String>,
        #[arg(long)]
        projects: bool,
        #[arg(long)]
        report: Option<PathBuf>,
    },
    /// Repository checks and package maintenance; client tests use their own runtimes.
    Maintenance {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
#[derive(Subcommand)]
enum ImageCommand {
    Plan {
        #[arg(long)]
        project: Option<PathBuf>,
        #[arg(long)]
        target: Option<String>,
        #[arg(long)]
        example: Option<String>,
        #[arg(long)]
        variant: Option<String>,
        #[arg(long, default_value = "bin")]
        format: String,
        #[arg(long)]
        output: PathBuf,
    },
    Merge {
        #[arg(long)]
        plan: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Verify {
        bundle: PathBuf,
    },
}
fn run(mut a: Args) -> Result<(J, i32)> {
    a.root = a
        .root
        .canonicalize()
        .context("Repository/project root is unavailable")?;
    Ok(match a.command {
        Commands::Doctor { target } => {
            let data = codecraft::environment::inspect(&a.root, &target)?;
            let exit = if data["status"] == "passed" { 0 } else { 2 };
            (data, exit)
        }
        Commands::Firmware { command } => {
            let value = match command {
                ImageCommand::Plan {
                    project,
                    target,
                    example,
                    variant,
                    format,
                    output,
                } => {
                    if format != "bin" {
                        return Err(unsupported("Only native factory BIN is supported"));
                    }
                    codecraft::firmware::plan(
                        project.as_deref().unwrap_or(&a.root),
                        target.as_deref(),
                        example.as_deref(),
                        variant.as_deref(),
                        &output,
                    )?
                }
                ImageCommand::Merge { plan, output } => codecraft::firmware::merge(&plan, &output)?,
                ImageCommand::Verify { bundle } => codecraft::firmware::verify(&a.root, &bundle)?,
            };
            (value, 0)
        }
        Commands::Catalog => {
            let c = Catalog::load(&a.root)?;
            (
                json!({"schema_version":4,"devices":c.devices,"targets":c.targets,"platforms":c.platforms}),
                0,
            )
        }
        Commands::Discover {
            interface,
            platform,
            constraints,
        } => (
            Catalog::load(&a.root)?.discover(
                &interface,
                platform.as_deref(),
                &constraints
                    .map(|p| read(&p))
                    .transpose()?
                    .unwrap_or(object()),
            )?,
            0,
        ),
        Commands::Context {
            target,
            example,
            variant,
            task,
            skills,
        } => {
            let c = Catalog::load(&a.root)?;
            let mut value = c.context(&target, example.as_deref(), variant.as_deref(), &task)?;
            if let Some(path) = skills {
                let selections = read(&path)?;
                let t = c.selected(&target, example.as_deref(), variant.as_deref())?;
                let mut graph = c.graph(&t)?;
                codecraft::skills::add_selections(&mut graph, &selections)?;
                value["files"] = json!(graph.files.keys().collect::<Vec<_>>());
                value["dependencies"] = json!(graph.edges);
                value["source_digest"] = json!(graph.fingerprint());
                value["skills"] = selections;
            }
            (value, 0)
        }
        Commands::New {
            target,
            example,
            variant,
            output,
            brief,
            skills,
        } => (
            project::create(
                &Catalog::load(&a.root)?,
                &target,
                &example,
                variant.as_deref(),
                &output,
                brief.map(|p| read(&p)).transpose()?,
                skills.map(|p| read(&p)).transpose()?.unwrap_or(json!([])),
            )?,
            0,
        ),
        Commands::VerifyProject { path } => {
            let v = project::verify(&path)?;
            let code = if v["status"] == "passed" { 0 } else { 1 };
            (v, code)
        }
        Commands::LockProject { path, toolchain } => (project::relock(&path, None, toolchain)?, 0),
        Commands::BuildProject { path } => {
            let v = project::build(&path)?;
            let code = code(&v["results"])?;
            (v, code)
        }
        Commands::Package { path, output } => (project::package(&path, &output)?, 0),
        Commands::Validate {
            r#static,
            target,
            example,
            variant,
            projects,
            report,
        } => {
            ensure!(
                !(r#static
                    && (target.is_some() || projects || example.is_some() || variant.is_some())),
                "--static cannot select native checks"
            );
            ensure!(
                target.is_some() || (example.is_none() && variant.is_none()),
                "--example/--variant require --target"
            );
            ensure!(!a.root.join("project.yaml").exists(), "Application builds require codecraft build-project; reference validation cannot bypass its lock");
            let mut rows = vec![];
            if target.is_none() {
                let checks = codecraft::maintenance_checks::static_checks(&a.root)?;
                rows.extend(arr(&checks["results"])?.iter().cloned());
            }
            if !r#static && rows.iter().all(|r| r["status"] == "passed") {
                let c = Catalog::load(&a.root)?;
                let ids = target
                    .clone()
                    .map(|t| vec![t])
                    .unwrap_or_else(|| c.targets.keys().cloned().collect());
                for tid in ids {
                    let t = c.selected(&tid, example.as_deref(), variant.as_deref())?;
                    rows.extend(codecraft::firmware::build(&c, &t)?);
                    if projects || target.is_none() {
                        for (ex, selected) in codecraft::firmware::selection(&t)? {
                            let path = std::env::temp_dir().join(format!(
                                "codecraft-export-{}-{}",
                                std::process::id(),
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)?
                                    .as_nanos()
                            ));
                            let outcome = (|| -> Result<J> {
                                project::create(
                                    &c,
                                    &tid,
                                    &ex,
                                    selected.as_deref(),
                                    &path,
                                    None,
                                    json!([]),
                                )?;
                                project::relock(&path, None, true)?;
                                project::build(&path)
                            })();
                            if path.exists() {
                                std::fs::remove_dir_all(&path)?;
                            }
                            let row = match outcome {
                                Ok(build) => {
                                    json!({"status":match code(&build["results"])? {0=>"passed",1=>"failed",_=>"not_run"},"build_results":build["results"]})
                                }
                                Err(e) => json!({"status":"failed","reason":format!("{e:#}")}),
                            };
                            let mut row = row;
                            row["name"] = json!(format!(
                                "{tid}/{ex}/{}/generated-project",
                                selected.as_deref().unwrap_or("default")
                            ));
                            rows.push(row);
                        }
                    }
                }
            }
            let status = code(&json!(rows))?;
            let data = json!({"scope":if r#static {"static+host"} else {target.as_deref().unwrap_or("complete-software-gate")},"results":rows,"device_verification":"not_run"});
            if let Some(path) = report {
                write(
                    &path,
                    &json!({"schema_version":2,"ok":status==0,"data":data}),
                )?;
            }
            (data, status)
        }
        Commands::Maintenance { args } => return maintenance(&a.root, &args),
    })
}
fn code(rows: &J) -> Result<i32> {
    let r = arr(rows)?;
    Ok(if r.iter().any(|r| r["status"] == "failed") {
        1
    } else if r.iter().all(|r| r["status"] == "passed") {
        0
    } else {
        2
    })
}
fn maintenance(root: &std::path::Path, args: &[String]) -> Result<(J, i32)> {
    if root.join("project.yaml").exists() || !root.join("tools/cli/Cargo.toml").is_file() {
        return Err(unsupported(
            "Repository maintenance is unavailable in application exports",
        ));
    }
    let (name, tail) = args
        .split_first()
        .context("Choose a repository maintenance command")?;
    let data = match name.as_str() {
        "generate" => codecraft::maintenance_generate::command(root, tail)?,
        "test" => codecraft::maintenance_checks::command(root, tail)?,
        "validate" => {
            ensure!(
                tail == ["--static"],
                "Use validate --static for repository checks"
            );
            codecraft::maintenance_checks::static_checks(root)?
        }
        "skill-reference" => codecraft::maintenance_reference::command(root, tail)?,
        "record-evidence" => codecraft::maintenance_evidence::command(root, tail)?,
        "ci" => codecraft::maintenance_ci::command(root, tail)?,
        "skill-release" | "check-cloud-skills" | "skill-contract" => {
            codecraft::maintenance_skills::command(root, args)?
        }
        _ => anyhow::bail!("Unknown repository maintenance command: {name}"),
    };
    let status = if name == "ci" && tail.first().is_some_and(|v| v == "check") {
        // Unselected native jobs are intentionally skipped; the CI gate validates
        // that distinction instead of treating every skip as an incomplete check.
        if data["ok"] == true {
            0
        } else {
            1
        }
    } else if data.get("results").is_some() {
        code(&data["results"])?
    } else if (data["check"] == true && data["changed"].as_array().is_some_and(|v| !v.is_empty()))
        || data["ok"] == false
        || data["status"] == "failed"
    {
        1
    } else if data.get("status").is_some_and(|v| v != "passed") {
        2
    } else {
        0
    };
    Ok((data, status))
}
fn main() {
    let a = Args::parse();
    match run(a) {
        Ok((data, code)) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({"schema_version":2,"ok":code==0,"data":data}))
                    .unwrap()
            );
            std::process::exit(code)
        }
        Err(e) => {
            if let Some(Unavailable(status, reason)) = e.downcast_ref::<Unavailable>() {
                println!(
                    "{}",
                    json!({"schema_version":2,"ok":false,"data":{"status":status,"reason":reason}})
                );
                std::process::exit(2);
            }
            println!(
                "{}",
                json!({"schema_version":2,"ok":false,"error":{"type":"contract","message":format!("{e:#}")}})
            );
            std::process::exit(1)
        }
    }
}
