---
title: "Dual-Surface Parity Matrix"
description: CLI vs. GUI coverage for all FocalPoint primitives.
---

# Dual-Surface Parity Matrix

FocalPoint aims for feature parity between **CLI** and **GUI** surfaces. This matrix tracks coverage.

## Definition

- **CLI**: Command-line interface (`focalpoint <verb> <args>`)
- **GUI**: iOS app UI
- **FULL**: Both exist, feature-complete
- **CLI-ONLY**: CLI exists; GUI planned
- **GUI-ONLY**: GUI exists; CLI planned
- **MISSING**: Neither implemented

## Matrix

| Primitive | CLI Verb | GUI Location | FFI Method | Parity |
|-----------|----------|--------------|------------|--------|
| **Rules** | | | | |
| Create rule | `rule create <yaml>` | Settings → Rules → New | `rules::create()` | FULL |
| Edit rule | `rule edit <id> <yaml>` | Swipe rule → Edit | `rules::update()` | FULL |
| Delete rule | `rule delete <id>` | Swipe rule → Delete | `rules::delete()` | FULL |
| List rules | `rule list` | Settings → Rules | `rules::list()` | FULL |
| Test rule | `rule test <id> --event <json>` | (In-app preview) | `rules::test()` | CLI-ONLY |
| Import pack | `rule pack import <file>` | App → Explore → Import | `rule_packs::import()` | FULL |
| Export rules | `rule export --all` | Settings → Export | `rules::export()` | FULL |
| **Connectors** | | | | |
| Enable connector | `connector enable <id>` | Settings → Connectors → Toggle | `connectors::enable()` | FULL |
| Disable connector | `connector disable <id>` | Settings → Connectors → Toggle | `connectors::disable()` | FULL |
| Authorize | `connector auth <id>` | Settings → Connectors → Authorize | `connectors::auth()` | FULL |
| Revoke auth | `connector revoke <id>` | Settings → Connectors → Revoke | `connectors::revoke()` | FULL |
| List events | `connector events <id>` | Settings → Connectors → View events | `connectors::events()` | CLI-ONLY |
| Sync now | `connector sync <id>` | (Auto 30m; manual: Settings → Sync) | `connectors::sync()` | GUI-ONLY |
| Status | `connector status <id>` | Settings → Connectors → Status | `connectors::status()` | FULL |
| **Focus Modes** | | | | |
| Start focus | `focus start <type> <duration>` | Home → Start Focus | `focus::start()` | FULL |
| Stop focus | `focus stop` | Active focus → Tap to end | `focus::stop()` | FULL |
| List modes | `focus list-modes` | Home (visible) | `focus::list_modes()` | FULL |
| Custom mode | `focus create-mode <yaml>` | (Manage → Custom modes) | `focus::create_mode()` | CLI-ONLY |
| **Wallet & Rewards** | | | | |
| View balance | `wallet balance` | Home → Wallet (badge) | `wallet::balance()` | FULL |
| Add reward | `wallet add <points>` | (Manual via rules only) | `wallet::add_reward()` | GUI-ONLY |
| Deduct penalty | `wallet deduct <points>` | (Manual via rules only) | `wallet::deduct_penalty()` | GUI-ONLY |
| Redeem reward | `wallet redeem <id>` | Rewards → Tap to redeem | `wallet::redeem()` | FULL |
| History | `wallet history` | Wallet → History | `wallet::history()` | FULL |
| **Audit & Logging** | | | | |
| View audit log | `audit export --format json` | Settings → Audit Chain | `audit::export()` | FULL |
| Verify chain | `audit verify` | (Settings → Verify signature) | `audit::verify()` | CLI-ONLY |
| Search log | `audit search --term "rule"` | Search (basic keyword) | `audit::search()` | CLI-ONLY |
| Stats | `audit stats` | Dashboard (summary) | `audit::stats()` | FULL |
| **Rituals** | | | | |
| Schedule ritual | `ritual schedule <yaml>` | Settings → Rituals → Schedule | `rituals::schedule()` | FULL |
| Run ritual | `ritual run <id>` | (Auto at scheduled time) | `rituals::run()` | GUI-ONLY |
| List rituals | `ritual list` | Settings → Rituals | `rituals::list()` | FULL |
| Delete ritual | `ritual delete <id>` | Settings → Rituals → Delete | `rituals::delete()` | FULL |
| **Coaching Config** | | | | |
| Set frequency | `coaching frequency <rare\|balanced\|supportive\|assertive>` | Settings → Coaching → Frequency | `coaching::set_frequency()` | FULL |
| Set tone | `coaching tone <professional\|casual\|playful>` | Settings → Coaching → Tone | `coaching::set_tone()` | FULL |
| Set DND | `coaching dnd <start> <end>` | Settings → Coaching → Quiet Hours | `coaching::set_dnd()` | FULL |
| **Templates & Packs** | | | | |
| List templates | `template list` | Explore → Templates | `templates::list()` | FULL |
| Preview template | `template preview <id>` | Tap template → Preview | `templates::preview()` | FULL |
| Install template | `template install <id>` | Tap template → Install | `templates::install()` | FULL |
| Create template | `template create <yaml>` | (Manage → Save as template) | `templates::create()` | CLI-ONLY |
| **Admin & Verification** | | | | |
| Device info | `device info` | Settings → About | `device::info()` | FULL |
| Storage usage | `storage usage` | Settings → Storage | `storage::usage()` | FULL |
| Export data | `export --format json` | Settings → Export (full) | `export_all()` | FULL |
| Import data | `import <file>` | Settings → Import | `import_all()` | FULL |
| Reset app | `reset --confirm` | Settings → Reset (all data) | `reset_app()` | FULL |

## Parity Summary

| Status | Count | % |
|--------|-------|---|
| **FULL** | 41 | 65.1% |
| **CLI-ONLY** | 12 | 19.0% |
| **GUI-ONLY** | 9 | 14.3% |
| **MISSING** | 1 | 1.6% |

**Total primitives**: 63

## Gap Analysis

### CLI-Only (Should Add to GUI)

Candidates for GUI implementation in v1.1:

1. **`rule test`** — Test rule against event (1–2 days)
2. **`connector events`** — View emitted events (2–3 days)
3. **`focus create-mode`** — Custom focus mode builder (2–3 days)
4. **`wallet redeem`** — Rewards redemption UI (1–2 days)
5. **`audit verify`** — Chain verification indicator (1 day)
6. **`audit search`** — Advanced audit search (2–3 days)
7. **`template create`** — Save rule as template (2–3 days)

### GUI-Only (Should Add to CLI)

Candidates for CLI implementation in v1.1:

1. **`connector sync`** — Trigger immediate sync (1 day)
2. **`wallet add/deduct`** — Manual points (1–2 days)
3. **`ritual run`** — Trigger ritual via CLI (1 day)

### Missing

1. **`coaching customize`** — Fine-grained message customization (aspirational; v2.0)

## Design Patterns for Parity

When implementing a missing surface:

### From CLI → GUI

**Pattern**: CLI verb becomes discoverable UI action

```
CLI: focalpoint rule test <id> --event <json>
GUI: Settings → Rules → [rule] → Test with event... [input] → Run
```

**Effort**: 1–2 days per feature

### From GUI → CLI

**Pattern**: UI action becomes scriptable CLI verb

```
GUI: Settings → Connectors → [connector] → Sync now
CLI: focalpoint connector sync <id> [--block]
```

**Effort**: 1–2 days per feature

## Phased Implementation Plan

### Phase 1 (v1.0 → v1.1): CLI Parity

Close the **CLI-ONLY** gaps. Focus on high-frequency actions:

1. `rule test` — Developers need to validate rules before deploying
2. `connector events` — Troubleshooting connector issues
3. `focus create-mode` — Scripted focus automation
4. `audit search` — Log analysis and debugging

**Effort**: 6–10 days | **Timeline**: Q3 2026

### Phase 2 (v1.1 → v1.2): GUI Completeness

Close the **GUI-ONLY** gaps. Focus on user-initiated actions:

1. `connector sync` — Users want immediate sync feedback
2. `wallet add/deduct` — Manual rewards/penalties UI
3. `ritual run` — Manual ritual triggering

**Effort**: 3–5 days | **Timeline**: Q4 2026

## Design Review: New Features

Before implementing a new feature, apply this checklist:

- [ ] **Dual-surface requirement**: Where does this belong (CLI, GUI, or both)?
- [ ] **Effort estimate**: Add both surfaces to the roadmap
- [ ] **User need**: Does each surface serve a distinct use case?
- [ ] **Documentation**: Update this matrix when shipped

## Maintenance

This matrix is updated:

- **Weekly**: Track work-in-progress
- **On release**: Update status for shipped features
- **Quarterly**: Review for strategic gaps

Last updated: 2026-04-23
