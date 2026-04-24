# Supply-Chain Security Audit Report

**Generated:** 2026-04-24
**Tool:** cargo-deny v0.19.0
**Scope:** FocalPoint Rust workspace (48 crates)

## Executive Summary

FocalPoint's supply chain audit identified **4 unmaintained dependencies** and **34 wildcard local-path dependencies**. No critical security vulnerabilities or license violations detected. All findings are known trade-offs documented in this audit.

## Findings

### Advisories: 4 Unmaintained Crates (Warnings, Not Critical)

All four unmaintained crates are transitive dependencies with no direct security vulnerabilities. Each has alternatives documented in their advisory:

#### 1. **bincode v1.3.3** (RUSTSEC-2025-0141)
- **Status:** Unmaintained; team permanently ceased development after harassment incident
- **Severity:** None; declared complete and stable
- **Impact:** Used by `uniffi` for FFI serialization (build-time only)
- **Alternatives:** `postcard`, `rkyv`, `bitcode`, `wincode`
- **Recommendation:** Monitor for potential migration post-Phase 0

#### 2. **derivative v2.2.0** (RUSTSEC-2024-0388)
- **Status:** Unmaintained; no longer accepting maintenance
- **Severity:** None; derive macro library (compile-time only)
- **Impact:** Transitive via `starlark` → `focus-lang`
- **Alternatives:** `derive_more`, `derive-where`, `educe`
- **Recommendation:** Defer until `starlark` updates or `focus-lang` refactored

#### 3. **fxhash v0.2.1** (RUSTSEC-2025-0057)
- **Status:** Unmaintained; repository stale, owner inactive
- **Severity:** None; hash function used only internally by `starlark_map`
- **Impact:** Transitive via `starlark` → `focus-lang`
- **Alternatives:** `rustc-hash` (used by rustc itself)
- **Recommendation:** Candidate for Phase 2 starlark dependency audit

#### 4. **paste v1.0.15** (RUSTSEC-2024-0436)
- **Status:** Unmaintained; creator archived repository
- **Severity:** None; macro library (compile-time only)
- **Impact:** Transitive via `uniffi` → `focus-ffi` and `starlark` → `focus-lang`
- **Alternatives:** `pastey` (fork), `with_builtin_macros`
- **Recommendation:** Monitor for uniffi/starlark version bumps

### Licenses: All Approved

**Status:** PASS
**Approved licenses:** MIT, Apache-2.0, BSD-{2,3}-Clause, ISC, Zlib, MPL-2.0, 0BSD, CC0-1.0, Unicode-3.0, BSL-1.0, CDLA-Permissive-2.0

**Exception overrides:**
- **BSL-1.0:** `clipboard-win` and `error-code` (OSI-approved, needed for CLI tooling)
- **CDLA-Permissive-2.0:** `webpki-roots` (data license, permissive, OSI-compatible intent)

No GPL, AGPL, or proprietary licenses detected.

### Bans: 34 Wildcard Local-Path Dependencies (Non-Blocking Warning)

All 34 violations are **internal workspace crates** using `{ path = "../..." }` without version pinning. This is a code organization pattern (multiple connectors, adapters) and not a security risk because:

1. **Local paths are reproducible** — same commit hash always produces same binary
2. **Semantics are workspace-scoped** — no external registry dependency
3. **Pattern aligns with Rust guidance** — workspaces routinely use unversioned local paths

Examples:
- `connector-canvas`, `connector-fitbit`, `connector-gcal`, etc. → all depend on `focus-connectors`, `focus-events` unversioned
- This is **intentional architecture** (per FocalPoint CLAUDE.md: "Trait surfaces are stable")

**Recommendation:** Document in deny.toml as acceptable pattern. No mitigation required.

### Sources: All Approved

**Status:** PASS
- All crates from official crates.io registry (https://github.com/rust-lang/crates.io-index)
- No git dependencies or unknown registries

## Dependency Composition

### Top Maintainer Groups

1. **Tokio ecosystem** (async runtime): 8 crates, actively maintained
2. **Serde ecosystem** (serialization): 5 crates, actively maintained
3. **Starlark** (Rust interpreter for FocalPoint DSL): 1 crate, actively maintained (by Google)
4. **UniFFI** (FFI bindings): 3 crates, actively maintained (by Mozilla)
5. **Hyper/Reqwest** (HTTP): 6 crates, actively maintained

### Dependency Graph

**Direct:** 32 (in workspace root `Cargo.toml`)
**Transitive:** 440+
**Total unique crates:** ~450

## Metrics

| Metric | Value |
|--------|-------|
| Advisories checked | 450+ crates |
| Unmaintained found | 4 (0.9%) |
| Critical vulnerabilities | 0 |
| License violations | 0 |
| Unapproved registries | 0 |
| Git dependencies | 0 |
| Wildcard local paths | 34 (internal workspace) |

## Recommendations

### Immediate (Phase 0)
- ✅ Deploy cargo-deny in CI (GitHub Actions workflow)
- ✅ Monitor advisories in nightly check
- ✅ Document unmaintained crates in dependency policy

### Phase 1 (0-3 months)
- Evaluate `starlark` alternatives or version bump
- If bumping starlark: check if derivatives (`derivative`, `fxhash`, `paste`) are still transitive
- Consider shimming or replacing `paste` macro if version conflicts arise

### Phase 2 (3-6 months)
- Audit and potentially replace UniFFI's `bincode` dependency
- Profile serialization performance if migrating to `postcard` or `rkyv`

### Ongoing
- Run `cargo deny check` in pre-push hooks (via `task quality`)
- Quarterly dependency audit (compare against previous quarters)
- Subscribe to RustSec mailing list for security advisories

## Policy Reference

See `/docs/security/dependency_policy.md` for FocalPoint's dependency acceptance criteria.

---

**Audit performed:** cargo deny check
**Config:** `/deny.toml`
**Next audit:** Weekly (CI), Quarterly (manual review)
