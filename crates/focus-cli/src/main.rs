//! Dev CLI: audit inspect, task list, template list.
//!
//! Opens the same SQLite store the iOS app uses (path overridable via
//! `--db` flag or `FOCALPOINT_DB` env var; defaults to
//! `~/Library/Application Support/focalpoint/core.db` on macOS).

use clap::{Parser, Subcommand};
use focus_audit::AuditStore;
use focus_planning::TaskStore;
use focus_storage::sqlite::{audit_store::SqliteAuditStore, task_store::SqliteTaskStore};
use focus_storage::SqliteAdapter;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "focus", about = "FocalPoint dev CLI")]
struct Cli {
    /// Path to core.db. Defaults to FOCALPOINT_DB or the app's default location.
    #[arg(long, global = true)]
    db: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Inspect the audit chain.
    Audit {
        #[command(subcommand)]
        sub: AuditCmd,
    },
    /// Task-store inspection.
    Tasks {
        #[command(subcommand)]
        sub: TasksCmd,
    },
    /// Bundled template packs.
    Templates {
        #[command(subcommand)]
        sub: TemplatesCmd,
    },
}

#[derive(Subcommand)]
enum AuditCmd {
    /// Verify the hash chain end-to-end. Exits non-zero on tamper.
    Verify,
    /// Print the most recent N records as JSON lines.
    Tail {
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// Print the head hash (or "(empty)" if the chain is empty).
    Head,
}

#[derive(Subcommand)]
enum TasksCmd {
    /// List all tasks for the default user.
    List {
        #[arg(long)]
        user_id: Option<String>,
    },
}

#[derive(Subcommand)]
enum TemplatesCmd {
    /// List template packs shipped in-tree.
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_path = resolve_db_path(cli.db)?;
    match cli.cmd {
        Cmd::Audit { sub } => run_audit(sub, &db_path),
        Cmd::Tasks { sub } => run_tasks(sub, &db_path),
        Cmd::Templates { sub } => run_templates(sub),
    }
}

fn resolve_db_path(explicit: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    if let Some(p) = explicit {
        return Ok(p);
    }
    if let Ok(env) = std::env::var("FOCALPOINT_DB") {
        return Ok(PathBuf::from(env));
    }
    // Match iOS default: ~/Library/Application Support/focalpoint/core.db
    let home = std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME unset"))?;
    Ok(PathBuf::from(home)
        .join("Library/Application Support/focalpoint/core.db"))
}

fn open_adapter(db: &std::path::Path) -> anyhow::Result<SqliteAdapter> {
    if !db.exists() {
        anyhow::bail!("db not found at {} — launch the app once first, or pass --db", db.display());
    }
    SqliteAdapter::open(db).map_err(|e| anyhow::anyhow!("open db: {e}"))
}

fn run_audit(cmd: AuditCmd, db: &std::path::Path) -> anyhow::Result<()> {
    let adapter = open_adapter(db)?;
    let store = SqliteAuditStore::from_adapter(&adapter);
    match cmd {
        AuditCmd::Verify => {
            let ok = store.verify_chain()?;
            if ok {
                println!("chain verified");
                Ok(())
            } else {
                anyhow::bail!("chain tamper detected")
            }
        }
        AuditCmd::Tail { limit } => {
            let rt = tokio::runtime::Runtime::new()?;
            let all = rt.block_on(store.load_all())?;
            let start = all.len().saturating_sub(limit);
            for rec in &all[start..] {
                println!("{}", serde_json::to_string(rec)?);
            }
            Ok(())
        }
        AuditCmd::Head => {
            match store.head_hash()? {
                Some(h) => println!("{h}"),
                None => println!("(empty)"),
            }
            Ok(())
        }
    }
}

fn run_tasks(cmd: TasksCmd, db: &std::path::Path) -> anyhow::Result<()> {
    let adapter = open_adapter(db)?;
    let store = SqliteTaskStore::from_adapter(&adapter);
    match cmd {
        TasksCmd::List { user_id } => {
            let uid = user_id.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(Uuid::nil());
            let tasks = store.list(uid)?;
            if tasks.is_empty() {
                println!("(no tasks)");
            } else {
                for t in tasks {
                    println!(
                        "{}  {:?}  {}",
                        t.id,
                        t.status,
                        t.title,
                    );
                }
            }
            Ok(())
        }
    }
}

fn run_templates(cmd: TemplatesCmd) -> anyhow::Result<()> {
    match cmd {
        TemplatesCmd::List => {
            // focus-templates doesn't yet publish a bundled registry, so
            // walk examples/templates/ relative to the workspace root.
            // When run from the workspace root this just works; otherwise
            // callers pass FOCALPOINT_EXAMPLES or invoke from workspace.
            let dir = std::env::var("FOCALPOINT_EXAMPLES")
                .map(PathBuf::from)
                .ok()
                .or_else(|| std::env::current_dir().ok().map(|p| p.join("examples/templates")))
                .ok_or_else(|| anyhow::anyhow!("examples/templates not found"))?;
            if !dir.is_dir() {
                anyhow::bail!("{} is not a directory", dir.display());
            }
            for entry in std::fs::read_dir(&dir)? {
                let path = entry?.path();
                if path.extension().and_then(|s| s.to_str()) != Some("toml") {
                    continue;
                }
                let text = std::fs::read_to_string(&path)?;
                match focus_templates::TemplatePack::from_toml_str(&text) {
                    Ok(pack) => {
                        println!(
                            "{id}  v{ver}  {name}  ({rules} rules)  — {desc}",
                            id = pack.id,
                            ver = pack.version,
                            name = pack.name,
                            rules = pack.rules.len(),
                            desc = pack.description,
                        );
                    }
                    Err(e) => {
                        eprintln!("{}: parse failed: {e:?}", path.display());
                    }
                }
            }
            Ok(())
        }
    }
}
