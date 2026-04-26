# FocalPoint v0.0.11 Release

**Release Date:** 2026-04-25

## Summary

FocalPoint v0.0.11 is a quality and maintainability release. FR annotations added across all test modules, spec numbering system formalized, and dead code eliminated from core crates.

## What's New

### FR Annotations & Spec Traceability

- **Test FR tagging**: All 41 connector tests annotated with FR IDs via `@trace` markers
- **Spec numbering**: Introduced `spec_id_map.md` (19KB) mapping feature/requirement → test artifacts
- **Coverage**: 100% test→FR bidirectional traceability; zero orphaned tests

### Dead Code Cleanup

- **focus-* crates**: Removed unused imports, dead functions, and unreachable branches
- **Clippy hardening**: All 23 warnings eliminated; `cargo clippy --workspace -- -D warnings` passes
- **Verified**: `cargo check --workspace` zero errors/warnings

### Quality Gates

- **Build**: Clean zero-warning build across all 31 workspace members
- **Tests**: All 41 tests passing; connector coverage now complete for base adapter set
- **Security**: No cargo advisories; `cargo audit --deny warnings` green

## Upgrading

```bash
cargo install --locked --git https://github.com/KooshaPari/FocalPoint focalpoint-cli --tag v0.0.11
```

## Changes

- **6 commits** since v0.0.10 (chore/release, quality hardening, dead code fixes)
- **31 workspace members** verified clean
- **100% spec traceability** (test → FR mapping)

## Known Issues

- Apple FamilyControls entitlement review still pending (blocks iOS enforcement testing)
- Android PACKAGE_USAGE_STATS binding deferred to v0.0.12

## What's Next (v0.0.12)

- Real iOS enforcement integration (post entitlement approval)
- Android UsageStats + Accessibility Service binding
- End-to-end connector sync test suite

## Release Link

[FocalPoint v0.0.11 on GitHub](https://github.com/KooshaPari/FocalPoint/releases/tag/v0.0.11)
