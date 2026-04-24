# FocalPoint CLI Demo

**Quick Start:** Copy and paste these commands into your terminal to walk through the full FocalPoint CLI.

```bash
# 1. Initialize a temporary database with demo data
focus demo seed --db=/tmp/focus-demo.db

# 2. List all tasks
focus tasks list --db=/tmp/focus-demo.db --json

# 3. List all rules
focus rules list --db=/tmp/focus-demo.db --json

# 4. Check your reward wallet
focus wallet balance --db=/tmp/focus-demo.db --json

# 5. Verify the audit chain (tamper-detection)
focus audit verify --db=/tmp/focus-demo.db

# 6. View recent audit records
focus audit tail --limit=5 --db=/tmp/focus-demo.db --json

# 7. Run a sync tick (pull events from connectors)
focus sync tick --db=/tmp/focus-demo.db

# 8. Run an eval tick (process events through rules)
focus eval tick --db=/tmp/focus-demo.db

# 9. Check your wallet again (should reflect eval changes)
focus wallet balance --db=/tmp/focus-demo.db --json

# 10. Start a focus session
focus focus start "Deep work" --minutes=1 --db=/tmp/focus-demo.db

# 11. Reset demo data
focus demo reset --db=/tmp/focus-demo.db
```

## Automated Walkthrough

For a reproducible end-to-end walkthrough with transcript output:

```bash
# Build the demo runner
cargo run -p demo-walkthrough --release > /tmp/transcript.md

# View the generated transcript
cat /tmp/transcript.md
```

This generates a markdown report with:
- Each command executed
- Output captured
- Exit codes verified
- Missing/unimplemented subcommands clearly marked

## What Each Command Does

| Command | Purpose |
|---------|---------|
| `focus demo seed` | Populate the database with sample tasks, rules, audit records |
| `focus tasks list` | Show all tasks (filterable by user) |
| `focus rules list` | Show all active rules |
| `focus wallet balance` | Display current reward wallet state |
| `focus audit verify` | Verify the hash chain (detects tampering) |
| `focus audit tail` | Show recent audit records |
| `focus sync tick` | Pull events from all registered connectors |
| `focus eval tick` | Process queued events through rules engine |
| `focus focus start` | Emit a focus-session-started host event |
| `focus focus complete` | Emit a focus-session-completed host event |
| `focus demo reset` | Clear demo data and reset markers |

## Expected Behavior

- All JSON output is pretty-printed
- Exit code 0 on success, non-zero on error
- Audit chain verification prints the root hash or "(empty)" if no records exist
- Sync/Eval ticks report event count and any errors

## Troubleshooting

**"focus: command not found"**

Ensure you've built the CLI:
```bash
cargo build -p focus-cli --release
export PATH="$PATH:$(pwd)/target/release"
```

**"Error: db file locked"**

Two processes are accessing the database simultaneously. Use separate temporary files:
```bash
focus demo seed --db=/tmp/focus-$(date +%s).db
```

**"(no output)" for a command**

Some subcommands may not yet be fully implemented. Check [honest_coverage.md](../reference/honest_coverage.md) for known gaps.
