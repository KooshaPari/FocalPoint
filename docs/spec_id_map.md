# FocalPoint — Canonical Spec ID Mapping

**Last Updated:** 2026-04-25  
**Audit:** W-56 Spec-to-Reality Alignment

## Spec Numbering Scheme

FocalPoint adopts the AgilePlus canonical numbering: `FocalPoint-NNN` (zero-padded, 3-digit IDs).

---

## Spec Matrix

| Spec ID | Name | Type | Status | File | Evidence |
|---------|------|------|--------|------|----------|
| **FocalPoint-001** | Platform MVP | ROOT | **DONE** | `PRD.md` | v0.0.9 released (commit 1b586d9); asset wave shipped; mascot, icons, motion, audio |
| **FocalPoint-002** | Connector Trait System | CRATE | **DONE** | `crates/focus-core/src/connector.rs` | Trait defined; SQLite adapter live; test coverage present |
| **FocalPoint-003** | Rule Engine + Policy Store | CRATE | **DONE** | `crates/focus-core/src/rule_engine.rs` | TOML-based rule config active; policy evaluation implemented |
| **FocalPoint-004** | Audit + Tamper Evidence Chain | CRATE | **DONE** | `crates/focus-core/src/audit.rs` | Append-only ledger; SHA-256 chains; AuditRecord trait live |
| **FocalPoint-005** | iOS UniFFI Bindings (Phase 0) | FFI | **IN_PROGRESS** | `crates/focus-ffi/` | Scaffold present; FamilyControls entitlement blocking (awaiting Apple review) |
| **FocalPoint-006** | Android JNI Bindings (Phase 0) | FFI | **IN_PROGRESS** | `crates/focus-ffi/` | Scaffold present; PACKAGE_USAGE_STATS + Accessibility permissions required |
| **FocalPoint-007** | LocalStore (SQLite) Adapter | ADAPTER | **DONE** | `crates/focus-adapters/src/store/sqlite.rs` | Persistent storage; migrations active; zero warnings |
| **FocalPoint-008** | v0.0.10 Roadmap | PLAN | **DEFERRED** | `PLAN.md` | Mentioned in commit 22174cc; current work tracked in roadmap, not yet shipped |
| **FocalPoint-009** | CI Hardening + SBOM | GOVERNANCE | **DONE** | `.github/workflows/` | Monthly SBOM refresh (commit aed0aae); final clippy + notion fixes (commit aed0aae) |
| **FocalPoint-010** | Family Controls POC Adapter | ADAPTER | **DONE** | `crates/focus-adapters/src/platform/ios.rs` | Test coverage POC present (commit ffa8f98); proof-of-concept validated |

---

## Status Legend

| Status | Definition | Next Action |
|--------|-----------|-------------|
| **DONE** | Feature shipped, tests passing, in v0.0.9+ release or active crate | Archive if obsolete; maintain in CHANGELOG |
| **IN_PROGRESS** | Scaffold live, blocking on entitlements/external dependencies | Link to blocking issue; add target release version |
| **DEFERRED** | Planned; no active commits in past 30 days | Document reason; target release date |
| **OBSOLETE** | No longer needed; repo decomposed or feature cancelled | Archive in `docs/specs/archive/` |

---

## Root-Level Docs with Status Markers

### PRD.md
```yaml
---
title: FocalPoint — Product Requirements
spec_id: FocalPoint-001
status: DONE
version: v0.0.9
last_updated: 2026-04-25
evidence:
  - commit: 1b586d9
    message: "release: FocalPoint v0.0.9 (asset wave)"
  - commit: aed0aae
    message: "ci(sbom): monthly refresh + clippy fixes"
---
```

### PLAN.md
```yaml
---
title: FocalPoint — Roadmap
spec_id: FocalPoint-008
status: DEFERRED
current_version: v0.0.9
target_version: v0.0.10
last_updated: 2026-04-25
notes: |
  v0.0.10 roadmap scaffolded in commit 22174cc.
  Current work tracked in this file; awaiting release coordination.
---
```

### ADR.md
```yaml
---
title: FocalPoint — Architectural Decisions
spec_id: FocalPoint-ADR
status: CURRENT
last_updated: 2026-04-25
---
```

---

## Integration with AgilePlus

When creating new FocalPoint specs in AgilePlus, use the prefix `FocalPoint-NNN`:

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus specify --title "FocalPoint-011: <feature>" --description "..."
```

---

## Cross-Repo Reference

- **AgilePlus:** eco-series (eco-001 through eco-012) + numbered series (001–022)
- **thegent:** thegent-001 through thegent-012 (see `docs/spec_id_map.md`)
- **FocalPoint:** FocalPoint-001 through FocalPoint-010 (this file)

For multi-repo features, use hyphenated IDs: `FocalPoint-005:iOS-Entitlements-Gate` or `thegent-007:Cross-Repo-TDD-Framework`.

---

**Updated by:** W-56 Spec-to-Reality Alignment Agent  
**Audit Link:** `/repos/docs/org-audit-2026-04/spec_reality_reconciliation_2026_04_25.md`
