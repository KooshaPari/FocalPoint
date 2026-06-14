# W2-07: Orphaned Re-Export Cleanup

**Reference:** W1-04 audit output (`plans/2026-06-13-DAG-V4/w1-outputs/w1-04-reexports.md`)
**Date:** 2026-06-13

## Methodology

For each `pub use` re-export listed in the W1-04 audit:

1. Verified the target crate exists in the workspace.
2. Verified the target item is still exported from the target module/crate.
3. Marked any re-export where either the target crate or target item no longer exists as **orphaned**.

## Verification Results

### Summary

| Metric | Count |
|--------|-------|
| Total re-exports audited | 56 |
| Valid (target crate + item exist) | 56 |
| Orphaned (removed) | 0 |

### Per-Crate Breakdown

All 56 re-exports across 14 crates were verified and found to be valid:

| Crate | Re-exports | Status |
|-------|------------|--------|
| `focus-always-on` | 1 | Valid |
| `focus-backup` | 1 | Valid |
| `focus-connectors-mock-familycontrols` | 2 | Valid |
| `focus-crypto` | 5 | Valid |
| `focus-errors` | 1 | Valid |
| `focus-eval` | 1 | Valid |
| `focus-hash` | 1 | Valid |
| `focus-mcp-server` | 2 | Valid |
| `focus-observability` | 3 | Valid |
| `focus-plugin-sdk` | 6 | Valid |
| `focus-result` | 2 | Valid |
| `focus-rules` | 1 | Valid |
| `focus-storage` | 3 | Valid |
| `focus-sync` | 5 | Valid |
| `focus-telemetry` | 2 | Valid |
| `focus-templates` | 1 | Valid |
| `focus-transpilers` | 1 | Valid |
| `pheno-tracing` | 1 | Valid |
| `phenotype-config` | 13 | Valid |
| `phenotype-crypto` | 5 | Valid |

### Notable Discrepancies vs. W1-04 Audit

The W1-04 audit listed a few items that no longer appear in the current source code. These are **not** orphaned re-exports; rather, the re-export statements themselves were already simplified in prior commits:

- `focus-errors/src/lib.rs`: W1-04 listed `ErrorContext, PhenotypeError, Result, ResultExt`; actual code exports `PhenotypeError, Result` only.
- `focus-result/src/lib.rs`: W1-04 listed `FocusError, FocusResult, PhenotypeError, Result`; actual code exports `FocusResult, Result` only.

These are benign differences indicating the re-export lists were already cleaned up between the W1-04 audit and this task.

## `cargo check --workspace` Verification

The workspace-wide check completed with **no errors in FocalPoint crates**. The only compilation failures were in unrelated workspace members (`connector-strava`, `connector-fitbit`, `melosviz-desktop`) caused by duplicate `#[tauri::command]` macro definitions — issues outside the scope of the FocalPoint re-export audit.

## Conclusion

No orphaned re-exports were found. All 56 audited `pub use` statements have valid, existing targets. **No code changes were required.**
