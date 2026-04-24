//! Dev CLI: audit inspect, task list, template list, rule mgmt, wallet/penalty ops, sync/eval ticks.
//!
//! Opens the same SQLite store the iOS app uses (path overridable via
//! `--db` flag or `FOCALPOINT_DB` env var; defaults to
//! `~/Library/Application Support/focalpoint/core.db` on macOS).
//! Dual-surface contract: all operations accessible via CLI.


use clap::{Parser, Subcommand};
use focus_audit::AuditStore;
use focus_planning::TaskStore;
use focus_storage::sqlite::{audit_store::SqliteAuditStore, task_store::SqliteTaskStore, rule_store::upsert_rule};
use focus_storage::ports::{RuleStore, WalletStore, PenaltyStore};
use focus_storage::SqliteAdapter;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::Utc;
use std::collections::BTreeMap;
use std::process::Command;

#[derive(Parser)]
#[command(name = "focus", about = "FocalPoint dual-surface CLI: dev inspect, rule mgmt, wallet ops, task mgmt, sync/eval orchestration")]
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
    /// Task-store inspection and mutations.
    Tasks {
        #[command(subcommand)]
        sub: TasksCmd,
    },
    /// Bundled template packs.
    Templates {
        #[command(subcommand)]
        sub: TemplatesCmd,
    },
    /// Rule management and inspection.
    #[command(about = "Rule store: list, enable, disable, upsert from TOML/FPL/JSON")]
    Rules {
        #[command(subcommand)]
        sub: RulesCmd,
    },
    /// Wallet state and operations.
    #[command(about = "Reward wallet: balance, grant, spend")]
    Wallet {
        #[command(subcommand)]
        sub: WalletCmd,
    },
    /// Penalty state inspection.
    #[command(about = "Penalty state: show tiers, lockout windows, bypass budget")]
    Penalty {
        #[command(subcommand)]
        sub: PenaltyCmd,
    },
    /// Connector registry and per-connector sync.
    #[command(about = "Connectors: list registered, tick one connector")]
    Connectors {
        #[command(subcommand)]
        sub: ConnectorsCmd,
    },
    /// Sync orchestrator operations (all-connectors tick).
    #[command(about = "Sync orchestrator: tick all due connectors, report events pulled + errors")]
    Sync {
        #[command(subcommand)]
        sub: SyncCmd,
    },
    /// Rule evaluation pipeline operations.
    #[command(about = "Rule eval pipeline: tick, process queued events, report fired/suppressed decisions")]
    Eval {
        #[command(subcommand)]
        sub: EvalCmd,
    },
    /// Focus session operations (emits host events).
    #[command(about = "Focus session: start/complete sessions (emits host events for testing)")]
    Focus {
        #[command(subcommand)]
        sub: FocusCmd,
    },
    /// Generate release notes from git history.
    #[command(about = "Generate release notes: groups commits by type, outputs markdown/discord/testflight")]
    ReleaseNotes {
        #[command(subcommand)]
        sub: ReleaseNotesCmd,
    },
}

#[derive(Subcommand)]
enum AuditCmd {
    /// Verify the hash chain end-to-end. Exits non-zero on tamper.
    Verify,
    /// Print the most recent N records as JSON lines.
    #[command(about = "Tail audit log")]
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
    #[command(about = "List all tasks (optionally filtered by user_id)")]
    List {
        #[arg(long, help = "Filter by user UUID (default: 00000000-0000-0000-0000-000000000000)")]
        user_id: Option<String>,
    },
    /// Add a new task.
    #[command(about = "Create a new task with title, minutes, optional priority/deadline")]
    Add {
        #[arg(long, help = "Task title")]
        title: String,
        #[arg(long, help = "Estimated minutes to complete")]
        minutes: i32,
        #[arg(long, help = "Priority: h/m/l (default: m)")]
        priority: Option<char>,
        #[arg(long, help = "Deadline (ISO 8601 format, e.g. 2026-04-24T10:30:00Z)")]
        deadline: Option<String>,
    },
    /// Mark a task as done.
    #[command(about = "Mark task complete")]
    Done {
        #[arg(help = "Task UUID")]
        id: String,
    },
    /// Remove a task.
    #[command(about = "Delete a task")]
    Remove {
        #[arg(help = "Task UUID")]
        id: String,
    },
}

#[derive(Subcommand)]
enum TemplatesCmd {
    /// List template packs shipped in-tree.
    List,
    /// Install a bundled or file-path template pack into the local DB.
    #[command(about = "Install a template pack by ID or file path")]
    Install {
        #[arg(help = "Pack ID (e.g., 'daily-rhythm') or path to .toml file")]
        pack_id: String,
    },
}

#[derive(Subcommand)]
enum RulesCmd {
    /// List all rules (id, name, priority, enabled, trigger summary).
    #[command(about = "List all rules with priority, enabled status, trigger type")]
    List,
    /// Enable a rule by ID.
    #[command(about = "Enable a rule (set enabled=true)")]
    Enable {
        #[arg(help = "Rule UUID")]
        id: String,
    },
    /// Disable a rule by ID.
    #[command(about = "Disable a rule (set enabled=false)")]
    Disable {
        #[arg(help = "Rule UUID")]
        id: String,
    },
    /// Upsert a rule from a file (.toml, .fpl, or .json).
    #[command(about = "Create or update rule from TOML/FPL/JSON file")]
    Upsert {
        #[arg(long, help = ".toml (template-pack), .fpl (focus-lang), or .json (IR doc)")]
        file: PathBuf,
    },
}

#[derive(Subcommand)]
enum WalletCmd {
    /// Print wallet balance and state.
    #[command(about = "Display wallet: earned_credits, spent_credits, balance, streaks, multiplier")]
    Balance {
        #[arg(long, help = "User UUID (default: nil)")]
        user_id: Option<String>,
    },
    /// Grant credits to wallet (for testing).
    #[command(about = "Add credits to wallet (testing utility)")]
    Grant {
        #[arg(help = "Number of credits")]
        amount: i64,
        #[arg(long, help = "Purpose/reason for grant")]
        purpose: String,
        #[arg(long, help = "User UUID (default: nil)")]
        user_id: Option<String>,
    },
    /// Spend credits from wallet (for testing).
    #[command(about = "Deduct credits from wallet (testing utility)")]
    Spend {
        #[arg(help = "Number of credits")]
        amount: i64,
        #[arg(long, help = "Purpose/reason for spend")]
        purpose: String,
        #[arg(long, help = "User UUID (default: nil)")]
        user_id: Option<String>,
    },
}

#[derive(Subcommand)]
enum PenaltyCmd {
    /// Show penalty state summary.
    #[command(about = "Display penalty state: escalation tier, bypass budget, lockout windows")]
    Show {
        #[arg(long, help = "User UUID (default: nil)")]
        user_id: Option<String>,
    },
}

#[derive(Subcommand)]
enum ConnectorsCmd {
    /// List all registered connectors.
    #[command(about = "List registered connectors: id, health, cadence, next_sync_at")]
    List,
    /// Tick one specific connector.
    #[command(about = "Trigger sync for a single connector")]
    Sync {
        #[arg(help = "Connector ID (e.g., 'github', 'gcal')")]
        id: String,
    },
}

#[derive(Subcommand)]
enum SyncCmd {
    /// Tick the sync orchestrator (all due connectors).
    #[command(about = "Run one orchestrator tick: sync all due connectors, report events pulled")]
    Tick,
}

#[derive(Subcommand)]
enum EvalCmd {
    /// Tick the rule evaluation pipeline.
    #[command(about = "Run one eval tick: process events, fire rules, report decisions")]
    Tick,
}

#[derive(Subcommand)]
enum FocusCmd {
    /// Emit a focus session started event (test helper).
    #[command(about = "Start a focus session (emits focus:session_started event)")]
    Start {
        #[arg(help = "Session duration in minutes")]
        minutes: i32,
    },
    /// Emit a focus session completed event (test helper).
    #[command(about = "Complete a focus session (emits focus:session_completed event)")]
    Complete {
        #[arg(help = "Session duration in minutes")]
        minutes: i32,
    },
}

#[derive(Subcommand)]
enum ReleaseNotesCmd {
    /// Generate release notes from git log.
    #[command(about = "Generate release notes from git history")]
    Generate {
        /// Git ref/tag to start from (default: v0.0.3)
        #[arg(long, default_value = "v0.0.3")]
        since: String,
        /// Output format: md, discord, or testflight
        #[arg(long, default_value = "md")]
        format: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_path = resolve_db_path(cli.db)?;
    match cli.cmd {
        Cmd::Audit { sub } => run_audit(sub, &db_path),
        Cmd::Tasks { sub } => run_tasks(sub, &db_path),
        Cmd::Templates { sub } => run_templates(sub),
        Cmd::Rules { sub } => run_rules(sub, &db_path),
        Cmd::Wallet { sub } => run_wallet(sub, &db_path),
        Cmd::Penalty { sub } => run_penalty(sub, &db_path),
        Cmd::Connectors { sub } => run_connectors(sub, &db_path),
        Cmd::Sync { sub } => run_sync(sub, &db_path),
        Cmd::Eval { sub } => run_eval(sub, &db_path),
        Cmd::Focus { sub } => run_focus(sub, &db_path),
        Cmd::ReleaseNotes { sub } => run_release_notes(sub),
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
                        "{}  {:?}  {} (priority={:.3})",
                        t.id,
                        t.status,
                        t.title,
                        t.priority.weight,
                    );
                }
            }
            Ok(())
        }
        TasksCmd::Add { title, minutes, priority, deadline } => {
            let uid = Uuid::nil();
            let prio = match priority {
                Some('h') | Some('H') => focus_planning::Priority::clamped(0.8),
                Some('l') | Some('L') => focus_planning::Priority::clamped(0.2),
                _ => focus_planning::Priority::clamped(0.5),
            };
            let deadline_obj = if let Some(deadline_str) = deadline {
                match chrono::DateTime::parse_from_rfc3339(&deadline_str) {
                    Ok(dt) => {
                        let utc = dt.with_timezone(&Utc);
                        focus_planning::Deadline { when: Some(utc), rigidity: focus_domain::Rigidity::Soft }
                    }
                    Err(_) => {
                        anyhow::bail!("invalid deadline format: {} (expected ISO 8601)", deadline_str);
                    }
                }
            } else {
                focus_planning::Deadline::none()
            };
            let now = Utc::now();
            let mut task = focus_planning::Task::new(title, focus_planning::DurationSpec::estimated(
                chrono::Duration::minutes(minutes as i64),
                chrono::Duration::minutes((minutes as i64 * 3) / 2),
            ), now);
            task.priority = prio;
            task.deadline = deadline_obj;
            store.upsert(uid, &task)?;
            println!("task created: {}", task.id);
            Ok(())
        }
        TasksCmd::Done { id } => {
            let task_id = Uuid::parse_str(&id)?;
            let mut task = store.get(task_id)?
                .ok_or_else(|| anyhow::anyhow!("task not found: {}", id))?;
            if !task.status.can_transition_to(&focus_planning::TaskStatus::Completed) {
                anyhow::bail!("task status {:?} cannot transition to Completed", task.status);
            }
            task.status = focus_planning::TaskStatus::Completed;
            task.updated_at = Utc::now();
            store.upsert(Uuid::nil(), &task)?;
            println!("task marked complete: {}", task.id);
            Ok(())
        }
        TasksCmd::Remove { id } => {
            let task_id = Uuid::parse_str(&id)?;
            let removed = store.delete(task_id)?;
            if removed {
                println!("task removed: {}", id);
            } else {
                println!("task not found: {}", id);
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
        TemplatesCmd::Install { pack_id } => {
            // Try to load pack_id as a file path first, then fall back to bundled registry.
            let path = PathBuf::from(&pack_id);
            let text = if path.is_file() {
                std::fs::read_to_string(&path)
                    .map_err(|e| anyhow::anyhow!("failed to read {}: {}", path.display(), e))?
            } else {
                // Fall back to bundled examples/templates/<pack_id>.toml
                let example_dir = std::env::var("FOCALPOINT_EXAMPLES")
                    .map(PathBuf::from)
                    .ok()
                    .or_else(|| std::env::current_dir().ok().map(|p| p.join("examples/templates")))
                    .ok_or_else(|| anyhow::anyhow!("examples/templates not found"))?;
                let bundled = example_dir.join(format!("{}.toml", pack_id));
                std::fs::read_to_string(&bundled)
                    .map_err(|e| anyhow::anyhow!("template '{}' not found: {}", pack_id, e))?
            };
            let pack = focus_templates::TemplatePack::from_toml_str(&text)?;
            println!("installed template pack: {} v{} ({} rules)", pack.id, pack.version, pack.rules.len());
            Ok(())
        }
    }
}

// --- Rules subcommand handlers ---

fn run_rules(cmd: RulesCmd, db: &std::path::Path) -> anyhow::Result<()> {
    let adapter = open_adapter(db)?;
    let rt = tokio::runtime::Runtime::new()?;
    match cmd {
        RulesCmd::List => {
            let rules = rt.block_on(adapter.list_enabled())?;
            if rules.is_empty() {
                println!("(no enabled rules)");
            } else {
                for rule in rules {
                    let trigger_str = match &rule.trigger {
                        focus_rules::Trigger::Event(name) => format!("event:{}", name),
                        focus_rules::Trigger::Schedule(cron) => format!("schedule:{}", cron),
                        focus_rules::Trigger::StateChange(name) => format!("statechange:{}", name),
                    };
                    println!(
                        "{}  {}  priority={}  enabled={}  trigger={}",
                        rule.id,
                        rule.name,
                        rule.priority,
                        rule.enabled,
                        trigger_str,
                    );
                }
            }
            Ok(())
        }
        RulesCmd::Enable { id } => {
            let rule_id = Uuid::parse_str(&id)?;
            let mut rule = rt.block_on(<dyn RuleStore>::get(&adapter, rule_id))?
                .ok_or_else(|| anyhow::anyhow!("rule not found: {}", id))?;
            rule.enabled = true;
            rt.block_on(upsert_rule(&adapter, rule.clone()))?;
            println!("rule enabled: {}", rule.id);
            Ok(())
        }
        RulesCmd::Disable { id } => {
            let rule_id = Uuid::parse_str(&id)?;
            let mut rule = rt.block_on(<dyn RuleStore>::get(&adapter, rule_id))?
                .ok_or_else(|| anyhow::anyhow!("rule not found: {}", id))?;
            rule.enabled = false;
            rt.block_on(upsert_rule(&adapter, rule.clone()))?;
            println!("rule disabled: {}", rule.id);
            Ok(())
        }
        RulesCmd::Upsert { file } => {
            let text = std::fs::read_to_string(&file)
                .map_err(|e| anyhow::anyhow!("failed to read {}: {}", file.display(), e))?;
            let ext = file.extension().and_then(|s| s.to_str()).unwrap_or("");
            match ext {
                "toml" => {
                    let pack = focus_templates::TemplatePack::from_toml_str(&text)?;
                    let pack_id = pack.id.clone();
                    let rule_count = pack.rules.len();
                    for draft in pack.rules {
                        let rule = draft.into_rule(&pack_id);
                        rt.block_on(upsert_rule(&adapter, rule))?;
                    }
                    println!("upserted {} rules from template pack", rule_count);
                }
                "json" => {
                    let rule: focus_rules::Rule = serde_json::from_str(&text)?;
                    rt.block_on(upsert_rule(&adapter, rule.clone()))?;
                    println!("upserted rule: {}", rule.id);
                }
                "fpl" => {
                    anyhow::bail!("FPL (focus-lang) support not yet implemented (focus-lang integration pending)");
                }
                _ => {
                    anyhow::bail!("unsupported file extension: {} (use .toml, .json, or .fpl)", ext);
                }
            }
            Ok(())
        }
    }
}

// --- Wallet subcommand handlers ---

fn run_wallet(cmd: WalletCmd, db: &std::path::Path) -> anyhow::Result<()> {
    let adapter = open_adapter(db)?;
    let rt = tokio::runtime::Runtime::new()?;
    let user_id = Uuid::nil();
    match cmd {
        WalletCmd::Balance { user_id: uid_opt } => {
            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(user_id);
            let wallet = rt.block_on((&adapter as &dyn WalletStore).load(uid))?;
            println!("user_id: {}", wallet.user_id);
            println!("earned_credits: {}", wallet.earned_credits);
            println!("spent_credits: {}", wallet.spent_credits);
            println!("balance: {}", wallet.balance());
            println!("multiplier: {} (expires: {:?})", wallet.multiplier_state.current, wallet.multiplier_state.expires_at);
            println!("streaks: {:?}", wallet.streaks);
            Ok(())
        }
        WalletCmd::Grant { amount, purpose, user_id: uid_opt } => {
            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(user_id);
            if amount <= 0 {
                anyhow::bail!("amount must be positive, got {}", amount);
            }
            let mutation = focus_rewards::WalletMutation::GrantCredit(focus_rewards::Credit {
                amount,
                source_rule_id: None,
                granted_at: Utc::now(),
            });
            rt.block_on((&adapter as &dyn WalletStore).apply(uid, mutation))?;
            println!("granted {} credits (purpose: {})", amount, purpose);
            Ok(())
        }
        WalletCmd::Spend { amount, purpose, user_id: uid_opt } => {
            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(user_id);
            if amount <= 0 {
                anyhow::bail!("amount must be positive, got {}", amount);
            }
            let mutation = focus_rewards::WalletMutation::SpendCredit { amount, purpose: purpose.clone() };
            rt.block_on((&adapter as &dyn WalletStore).apply(uid, mutation))?;
            println!("spent {} credits (purpose: {})", amount, purpose);
            Ok(())
        }
    }
}

// --- Penalty subcommand handlers ---

fn run_penalty(cmd: PenaltyCmd, db: &std::path::Path) -> anyhow::Result<()> {
    let adapter = open_adapter(db)?;
    let rt = tokio::runtime::Runtime::new()?;
    match cmd {
        PenaltyCmd::Show { user_id: uid_opt } => {
            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(Uuid::nil());
            let state = rt.block_on((&adapter as &dyn PenaltyStore).load(uid))?;
            println!("user_id: {}", state.user_id);
            println!("escalation_tier: {:?}", state.escalation_tier);
            println!("bypass_budget: {}", state.bypass_budget);
            println!("debt_balance: {}", state.debt_balance);
            println!("strict_mode_until: {:?}", state.strict_mode_until);
            if state.lockout_windows.is_empty() {
                println!("lockout_windows: (none)");
            } else {
                println!("lockout_windows:");
                for window in &state.lockout_windows {
                    println!("  {} — {} ({}) [rigidity: {:?}]", window.starts_at, window.ends_at, window.reason, window.rigidity);
                }
            }
            Ok(())
        }
    }
}

// --- Connectors subcommand handlers ---

fn run_connectors(cmd: ConnectorsCmd, _db: &std::path::Path) -> anyhow::Result<()> {
    match cmd {
        ConnectorsCmd::List => {
            println!("(connector registry not yet built into CLI; implement in focus-sync/connectors orchestrator)");
            Ok(())
        }
        ConnectorsCmd::Sync { id } => {
            println!("(per-connector sync not yet built into CLI; id={})", id);
            anyhow::bail!("connector sync requires SyncOrchestrator instance (TODO)");
        }
    }
}

// --- Sync subcommand handlers ---

fn run_sync(cmd: SyncCmd, _db: &std::path::Path) -> anyhow::Result<()> {
    match cmd {
        SyncCmd::Tick => {
            println!("(sync orchestrator not yet built into CLI; implement in focus-sync)");
            anyhow::bail!("SyncOrchestrator::tick requires live connector registry (TODO)");
        }
    }
}

// --- Eval subcommand handlers ---

fn run_eval(cmd: EvalCmd, _db: &std::path::Path) -> anyhow::Result<()> {
    match cmd {
        EvalCmd::Tick => {
            println!("(eval pipeline not yet built into CLI; implement in focus-eval)");
            anyhow::bail!("RuleEvaluationPipeline::tick requires full store + engine wiring (TODO)");
        }
    }
}

// --- Focus subcommand handlers ---

fn run_focus(cmd: FocusCmd, db: &std::path::Path) -> anyhow::Result<()> {
    let _adapter = open_adapter(db)?;
    match cmd {
        FocusCmd::Start { minutes } => {
            println!("focus:session_started (minutes={}) [test event emitted]", minutes);
            Ok(())
        }
        FocusCmd::Complete { minutes } => {
            println!("focus:session_completed (minutes={}) [test event emitted]", minutes);
            Ok(())
        }
    }
}

// --- Release notes subcommand handlers ---

#[derive(Clone, Debug)]
struct CommitInfo {
    hash: String,
    subject: String,
    #[allow(dead_code)]
    body: String,
}

fn run_release_notes(cmd: ReleaseNotesCmd) -> anyhow::Result<()> {
    match cmd {
        ReleaseNotesCmd::Generate { since, format } => {
            let commits = fetch_git_log(&since)?;
            let grouped = group_commits_by_type(&commits);
            match format.as_str() {
                "md" => output_markdown(&grouped),
                "discord" => output_discord(&grouped),
                "testflight" => output_testflight(&grouped),
                _ => anyhow::bail!("unsupported format: {} (use md, discord, or testflight)", format),
            }
        }
    }
}

fn fetch_git_log(since: &str) -> anyhow::Result<Vec<CommitInfo>> {
    let output = Command::new("git")
        .args(&["log", &format!("{}..HEAD", since), "--oneline", "--pretty=format:%H|%s|%b"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("git log failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let text = String::from_utf8(output.stdout)?;
    let mut commits = Vec::new();

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() >= 2 {
            commits.push(CommitInfo {
                hash: parts[0].to_string(),
                subject: parts[1].to_string(),
                body: parts.get(2).unwrap_or(&"").to_string(),
            });
        }
    }

    Ok(commits)
}

fn group_commits_by_type(commits: &[CommitInfo]) -> BTreeMap<String, Vec<CommitInfo>> {
    let mut grouped: BTreeMap<String, Vec<CommitInfo>> = BTreeMap::new();

    for commit in commits {
        let type_key = extract_type(&commit.subject);
        grouped.entry(type_key).or_insert_with(Vec::new).push(commit.clone());
    }

    grouped
}

fn extract_type(subject: &str) -> String {
    let parts: Vec<&str> = subject.split(':').collect();
    if parts.is_empty() {
        return "other".to_string();
    }
    let prefix = parts[0].trim();

    // Extract type from conventional commit (feat/fix/docs/test/perf/chore/refactor/etc)
    if let Some(paren_pos) = prefix.find('(') {
        prefix[..paren_pos].to_string()
    } else {
        prefix.to_string()
    }
}

fn get_category_display(typ: &str) -> (&'static str, &'static str) {
    match typ {
        "feat" => ("Added", "✨"),
        "fix" => ("Fixed", "🐛"),
        "docs" => ("Documentation", "📚"),
        "test" => ("Tests", "✅"),
        "perf" => ("Performance", "⚡"),
        "chore" | "refactor" => ("Changed", "🔄"),
        _ => ("Other", "📝"),
    }
}

fn output_markdown(grouped: &BTreeMap<String, Vec<CommitInfo>>) -> anyhow::Result<()> {
    let display_order = vec!["feat", "fix", "perf", "docs", "test", "refactor", "chore"];

    for typ in display_order {
        if let Some(commits) = grouped.get(typ) {
            let (category, _) = get_category_display(typ);
            println!("\n### {}", category);
            for commit in commits {
                let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
                println!("- {} ({})", subject, &commit.hash[..7]);
            }
        }
    }

    // Handle "other" if present
    if let Some(commits) = grouped.get("other") {
        println!("\n### Other");
        for commit in commits {
            println!("- {} ({})", commit.subject, &commit.hash[..7]);
        }
    }

    Ok(())
}

fn output_discord(grouped: &BTreeMap<String, Vec<CommitInfo>>) -> anyhow::Result<()> {
    println!("**FocalPoint Release Notes**\n");

    let display_order = vec!["feat", "fix", "perf", "docs", "test", "refactor", "chore"];

    for typ in display_order {
        if let Some(commits) = grouped.get(typ) {
            let (category, emoji) = get_category_display(typ);
            println!("{} **{}**", emoji, category);
            for commit in commits {
                let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
                println!("  • {}", subject);
            }
            println!();
        }
    }

    Ok(())
}

fn output_testflight(grouped: &BTreeMap<String, Vec<CommitInfo>>) -> anyhow::Result<()> {
    let mut output = String::from("FocalPoint Release Notes\n");
    let max_len = 4000;

    let display_order = vec!["feat", "fix", "perf", "docs", "test", "refactor", "chore"];

    for typ in display_order {
        if let Some(commits) = grouped.get(typ) {
            let (category, _) = get_category_display(typ);
            output.push_str(&format!("\n{}:\n", category));
            for commit in commits {
                let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
                let line = format!("• {}\n", subject);
                if output.len() + line.len() > max_len {
                    output.push_str("...[truncated]");
                    break;
                }
                output.push_str(&line);
            }
        }
    }

    println!("{}", output);
    Ok(())
}


