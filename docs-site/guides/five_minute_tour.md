---
title: Five-Minute Tour — Visual Overview
description: Skimmable visual guide to what FocalPoint is and how it works.
---

# Five-Minute Tour

A visual skimmer's guide to FocalPoint in screenshots and captions. No words you don't need.

---

## What is FocalPoint?

**Screen-time management that rewards you for staying focused.**

<!-- tutorial asset pending: [](/assets/tutorial/tour-01-hero.png) -->
*Three tabs: Track events (Activity), write automation (Rules), manage rewards (Wallet).*

---

## How It Works (The Loop)

<!-- tutorial asset pending: [](/assets/tutorial/tour-02-loop.png) -->

1. **Connect a tool** (GitHub, Strava, Canvas, etc.)
2. **FocalPoint watches** for events (commits, workouts, assignments)
3. **Your rules react** (grant credits, lock apps, send notifications)
4. **You earn rewards** (unlock a distraction, get screen time)

---

## The Activity Tab (Real-Time Event Feed)

<!-- tutorial asset pending: [](/assets/tutorial/tour-03-activity.png) -->

Every event from every connector shows up here in chronological order:

- **GitHub**: Commit pushed, PR opened, PR merged, issue created
- **Strava**: Activity completed, PR earned
- **Canvas**: Assignment due, assignment submitted
- **Apple Health**: Workout completed, step goal reached
- Custom events from any connector you build

**Each event is immutable** — cryptographically signed and audit-chained. Proof of what happened, when.

---

## The Rules Tab (Write Automation)

<!-- tutorial asset pending: [](/assets/tutorial/tour-04-rules.png) -->

Three ways to express the same rule:

**Via CLI:**
```bash
focus rules add --when "github:pr_merged" --then "wallet:grant 10"
```

**Via Rule Builder (UI):**
- When: GitHub → PR merged
- Then: Grant 10 credits

**Via FPL (Code):**
```python
rule(
    trigger=on_event("github:pull_request_merged"),
    actions=[grant_credits(10)],
    ...
)
```

All three compile to the same canonical IR. Same rule, three surfaces.

---

## Rule Builder in Action

<!-- tutorial asset pending: [](/assets/tutorial/tour-05-builder-when.png) -->
*Step 1: Choose the trigger event*

<!-- tutorial asset pending: [](/assets/tutorial/tour-06-builder-then.png) -->
*Step 2: Choose the action (grant credits, send notification, lock app, etc.)*

<!-- tutorial asset pending: [](/assets/tutorial/tour-07-builder-conditions.png) -->
*Step 3 (Optional): Add conditions (only weekdays, only work hours, etc.)*

**Result:** A visual rule that you can debug and version-control.

---

## The Wallet Tab (Track Credits)

<!-- tutorial asset pending: [](/assets/tutorial/tour-08-wallet.png) -->

**Left side:**
- Your current credit balance
- Total earned this week
- Streak indicator (if active)

**Right side:**
- Recent transactions
- Each transaction is audit-chained
  - Timestamp
  - Event that triggered it (or reward redeemed)
  - Amount (±credits)
  - Rule name
  - Immutable hash

---

## The Rewards Shop

<!-- tutorial asset pending: [](/assets/tutorial/tour-09-rewards.png) -->

Spend your credits on unlocks:

- **"Screen time boost"** — +1 hour of allowed usage (25 credits)
- **"Skip focus session"** — Dismiss a focus mode (15 credits)
- **"Unlock one distraction"** — 5-min grace period on a blocked app (10 credits)
- **"Focus mode grace period"** — Defer locking by 10 minutes (20 credits)

Each redemption is recorded in the audit chain. Non-fungible credits.

---

## Dual Ledger (Rewards + Penalties)

<!-- tutorial asset pending: [](/assets/tutorial/tour-10-dual-ledger.png) -->

FocalPoint tracks both **what you earned** and **what you lost**:

**Credit ledger:**
- PR merged → +10 credits
- Focus session completed → +5 credits
- Focused work hour → +2 credits

**Penalty ledger:**
- Skipped exercise → -5 credits
- App overuse → -2 credits
- Missed deadline → -10 credits

**Net ledger:** Credits - Penalties = Your balance.

All three ledgers are immutable and audit-chained.

---

## Connectors (Pluggable Integrations)

<!-- tutorial asset pending: [](/assets/tutorial/tour-11-connectors.png) -->

FocalPoint ships with:

- **GitHub** — Commits, PRs, issues
- **Canvas** — Assignments, submissions
- **Strava** — Workouts, PRs
- **Apple Health** — Workouts, steps, rings
- **Todoist** — Tasks completed
- **YNAB** — Budget tracking
- **More** — Community-built connectors via SDK

Each connector syncs every 60 seconds. Events are timestamped and immutable.

---

## Rule Evaluation Trace (Debugging)

<!-- tutorial asset pending: [](/assets/tutorial/tour-12-rule-trace.png) -->

Click a rule's **History** to see every decision:

- **Trigger:** Did the event match?
- **Conditions:** Did all conditions pass?
- **Actions:** What happened? (credits granted, notification sent, app locked)
- **Audit hash:** Immutable proof of what was recorded

Example trace:
```
rule: github-pr-reward
  t=0s   trigger  github:pull_request_merged (pr_id=42)
  t=0s   eval     conditions: [] (passed)
  t=0s   action   grant_credits(10)
  t=0s   action   notify(PR Merged! You earned 10 credits.)
decision trace: 1 event, 2 actions, audit chain ok
```

---

## The Mascot (Coachy)

<!-- tutorial asset pending: [](/assets/tutorial/tour-13-coachy.png) -->

**Coachy** is your AI coach. She:

- Celebrates when you earn credits
- Cheers when you unlock rewards
- Explains what just happened (rule fired, why, what happens next)
- Suggests new rules based on your events
- Keeps you motivated with progress badges

(Optional: toggle her off in Settings if you prefer silence.)

---

## Foci (Multi-Profile Rules)

<!-- tutorial asset pending: [](/assets/tutorial/tour-14-foci.png) -->

Create different rule profiles for different contexts:

- **"Work"** — Block social apps 9am–5pm, reward GitHub commits, enable focus mode
- **"Gym"** — Reward Strava activities, unlock music streaming, extend screen time
- **"Bedtime"** — Block all apps 10pm–7am, reward sleep tracking, no credits after 8pm

Switch between Foci with one tap. Each Focus has its own rule set, timeline, and wallet.

---

## Settings (Private by Design)

<!-- tutorial asset pending: [](/assets/tutorial/tour-15-settings.png) -->

All data stays on your device:

- **Backup & Restore** — Export encrypted backup (password-protected)
- **Sync policy** — How often to check each connector (default: 60s)
- **Privacy** — Never send rules, events, or credits anywhere
- **Accessibility** — Dark mode, font size, reduce motion
- **Notifications** — Toggle rule alerts, rewards, penalties

---

## The Rule DSL (Rule Description Language)

FocalPoint's rule syntax is inspired by Kubernetes, OpenPilicy, and Terraform:

```yaml
# Declarative rule format (TOML/YAML)
[rule]
id = "github-pr-plus-health"
name = "PR + Workout Combo"

[[when]]
source = "github"
event = "pull_request_merged"

[[when]]
source = "strava"
event = "activity_completed"
match = { sport_type = "Run" }

[[then]]
action = "grant_credits"
amount = 25
reason = "Code + exercise combo"
```

---

## Under the Hood (Trust & Security)

<!-- tutorial asset pending: [](/assets/tutorial/tour-16-trust.png) -->

**Why FocalPoint is tamper-proof:**

1. **Immutable event store** — Events are SHA-256 hashed. Modifying one breaks the chain.
2. **Audit chain** — Every action (credit, penalty, lock) produces a signed record.
3. **Deterministic rules** — Same input (event + rule) always produces same output (decision).
4. **Local-first** — Everything runs on your device. No cloud. No central authority.

You can **verify the audit chain at any time:**

```bash
focus audit verify --output detailed
# Checks every event and action hash
# Confirms nothing was tampered with
```

---

## Where to Go Next

- **[Getting Started](/getting-started/)** — Full 10-step tutorial (install → first credit)
- **[Your First Rule](/guides/your_first_rule)** — Deep dive: CLI, FPL, Builder surfaces
- **[Write a Rule](/rules/)** — Complete DSL reference
- **[Architecture](/architecture/)** — How FocalPoint works inside
- **[Connectors](/connectors/)** — Full list of integrations

---

## Questions?

- [Discord](https://discord.gg/focalpoint) — Community chat
- [GitHub Issues](https://github.com/KooshaPari/FocalPoint/issues) — Bug reports and feature requests
- [Email](mailto:contact@focalpoint.app) — Direct contact

**Welcome to FocalPoint. Stay focused. Earn your rewards. 🎯**
