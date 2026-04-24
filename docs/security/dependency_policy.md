# FocalPoint Dependency Acceptance Policy

**Effective:** 2026-04-24
**Version:** 1.0
**Scope:** Rust crates (direct and transitive)

## Rationale

FocalPoint's supply chain must balance innovation (using latest ecosystem tools) with stability (avoiding unmaintained or risky dependencies). This policy defines acceptance criteria for new dependencies.

## Acceptance Criteria

All new dependencies **MUST** satisfy at least one category, with preference for multiple criteria:

### Category A: Active Maintenance (Preferred)
- **Last commit:** Within 3 months of today's date
- **Issue response:** Maintainer responds to bugs/feature requests within 2 weeks
- **Ecosystem prominence:** >100 GitHub stars **AND** >1,000 weekly downloads on crates.io
- **Examples:** tokio, serde, hyper, reqwest, clap

### Category B: Strategic Ecosystem Crate (Preferred)
- Part of a well-known Rust ecosystem (Tokio, Serde, Hyper, UniFFI, Starlark)
- Actively maintained by the ecosystem's primary organization
- Even if niche, provides critical functionality no alternative addresses
- **Examples:** tokio-util, serde_json, uniffi, starlark

### Category C: Stable & Mature (Acceptable)
- **Last commit:** Within 6 months
- **Maturity:** v1.0+ with semantic versioning
- **No critical issues:** All reported CVEs have fixes or mitigations
- **Offline compilation:** No network access required at build-time
- **Examples:** thiserror, anyhow, dirs, uuid

### Category D: Minimal Transitive (Acceptable if Transitive Only)
- Unmaintained but:
  - Compile-time only (macros, build scripts)
  - Read-only functionality (no mutation)
  - No network or unsafe code
- Small crate (<500 LOC)
- Alternative exists and can be substituted
- **Examples:** paste (compile-time macro), fxhash (internal hash table)
- **Action:** Document in `deny.toml` `ignore` list with rationale

## Rejection Criteria

New dependencies **MUST** be rejected if any apply:

1. **GPL, AGPL, or proprietary licenses** — incompatible with MIT/Apache-2.0 dual license
2. **Unmaintained >12 months** — with no stable v1.0 release and no clear alternatives
3. **Network at build-time** — calls external APIs or services during `cargo build`
4. **Unsafe code without justification** — no documented reasons or external audit
5. **Cryptography without audit** — custom crypto code (use well-known libraries like ring, ed25519-dalek)
6. **Unknown provenance** — git or registry not officially audited (use github.com or crates.io only)

## Exceptional Cases

### Pre-Existing Transitive Dependencies
If a well-maintained direct dependency introduces an unmaintained transitive (e.g., starlark → paste), this is acceptable **if**:
- The transitive is compile-time only, AND
- An issue/PR exists in the upstream repo to replace it

**Action:** Document in `deny.toml`, monitor for upstream updates.

### Platform-Specific Dependencies
macOS, Windows, Linux-specific crates (rustyline, clipboard-win, zbus) may be maintained at lower velocity because they're niche. Acceptance criteria relaxed to "last update within 12 months + zero critical CVEs."

## Audit Process

### For Direct Dependencies
1. Check crates.io metadata: downloads/mo, last version date, license
2. Visit GitHub: stars, forks, last commit, open issues, PR response time
3. Run: `cargo audit` (local), `cargo-deny` (CI)
4. If not Category A, require justification in PR (top-of-file comment or CHANGELOG entry)

### For Transitive Dependencies
1. Monitor `cargo deny check` output (run in CI)
2. No action required if no new CVEs or unmaintained advisories
3. If unmaintained advisory appears, file an issue in the direct-dependent repo
4. If critical CVE, escalate to dependency upgrade urgency (Phase 1 blocker)

## Allowed Licenses

Explicitly allowed for redistribution:

- **Permissive:** MIT, Apache-2.0, BSD-{2,3}-Clause, ISC, Zlib, MPL-2.0, 0BSD, CC0-1.0
- **Data/Content:** Unicode-3.0, CC0-1.0
- **Special Ecosystem:** BSL-1.0 (Boost; used in starlark ecosystem)
- **SPDX Compound:** `(MIT OR Apache-2.0)` and equivalent

**Explicitly denied:**
- GPL, GPL-2.0, GPL-3.0, AGPL, AGPL-1.0, AGPL-3.0
- Proprietary, Commercial, custom licenses without explicit review

## Tools & Automation

### cargo-deny

Runs in CI on every PR + nightly. Configuration:

```toml
[advisories]
vulnerability = "all"           # Deny all CVEs
unmaintained = "all"            # Warn on unmaintained (not reject)

[licenses]
allow = [MIT, Apache-2.0, ...]  # See above
deny = [GPL, AGPL, ...]         # See above
confidence-threshold = 0.9

[bans]
multiple-versions = "warn"      # Alert on duplicate versions
wildcards = "deny"              # Local paths only
```

See `/deny.toml` for current configuration.

### SBOM Generation

Run on each release:

```bash
cargo run -p sbom-gen  # generates docs/security/sbom.json
```

CycloneDX-JSON format suitable for:
- Supply-chain risk analysis (SLSA, CISA SBOM)
- Vulnerability scanning tools (Grype, Trivy)
- Compliance reporting

## Enforcement

1. **Pre-commit hook:** `cargo deny check` must pass before commit (via `task quality`)
2. **CI:** `cargo deny check` runs on all PRs; failures block merge unless approved by maintainer
3. **Quarterly audit:** Manual review of all transitive dependencies (compare against previous quarter)

## Review & Updates

This policy is reviewed quarterly (January, April, July, October). Changes require:
- Pull request with rationale
- Approval from 1+ project maintainers
- Update to this document and `deny.toml` in same PR

## References

- **SLSA Supply Chain Security:** https://slsa.dev/
- **CISA SBOM requirements:** https://www.cisa.gov/sbom
- **cargo-deny documentation:** https://embarkstudios.github.io/cargo-deny/
- **RustSec Advisory Database:** https://rustsec.org/

---

**Last reviewed:** 2026-04-24
**Next review:** 2026-07-24
