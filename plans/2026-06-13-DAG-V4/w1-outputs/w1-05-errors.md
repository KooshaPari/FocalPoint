# W1-05: Error Enums and Result Aliases Mapping

## Search Scope
- **Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/crates/`
- **Patterns:** `pub enum Error` and `type Result` in all `.rs` files
- **Excluded dirs:** `target/`, `.git/` (plus workspace-excluded: `tooling/fr-coverage`, `tooling/target-pruner`, `examples/rule-library/tests`, `fuzz`, `assets/motion`, `phenotype-otel`, `kmobile`, `phenotype-voxel`, `Pine`)

---

## `pub enum Error` — 0 found

No `pub enum Error` declarations exist in the crates directory.

---

## `type Result` — 7 found

### focus-connectors
- `focus-connectors/src/lib.rs:41` — `pub type Result<T> = std::result::Result<T, ConnectorError>;`

### focus-domain
- `focus-domain/src/lib.rs:13` — `pub type Result<T, E = DomainError> = std::result::Result<T, E>;`

### focus-penalties
- `focus-penalties/src/lib.rs:32` — `pub type Result<T> = std::result::Result<T, PenaltyError>;`

### focus-rewards
- `focus-rewards/src/lib.rs:24` — `pub type Result<T> = std::result::Result<T, WalletError>;`

### focus-templates
- `focus-templates/src/lib.rs:48` — `pub type Result<T> = std::result::Result<T, TemplateError>;`

### phenotype-crypto
- `phenotype-crypto/src/lib.rs:43` — `pub type Result<T> = std::result::Result<T, CryptoError>;`

### phenotype-error-core
- `phenotype-error-core/src/lib.rs:254` — `pub type Result<T> = std::result::Result<T, PhenotypeError>;`

---

## Summary

| Metric | Count |
|--------|-------|
| `pub enum Error` | 0 |
| `type Result` aliases | 7 |
| **Affected crates** | 7 |

---

*Generated: 2026-06-13*
