# CHANGELOG.md — pheno-ssot-template

## 0.1.0 (2026-06-11)

### Added
- Canonical SSOT starter template for the pheno-* fleet.
- `Cargo.toml.template` and `src/lib.rs.template` with Mustache-style placeholders.
- `template.yaml` — machine-readable template manifest (cookiecutter subset + Phenotype extensions).
- SSOT invariant check scripts (`scripts/check-ssot-invariant-1-errors.sh`, `scripts/check-ssot-invariant-2-logging.sh`).
- `deny.toml` baseline for cargo-deny compliance.
- CI workflow template (`.github/workflows/ci.yml`).
- AI-DD crutch files: `AGENTS.md`, `llms.txt`, `WORKLOG.md`, `CHANGELOG.md`, `LICENSE-MIT`.

### Invariants
1. Every error is `pheno_errors::AppError`.
2. Every log line is structured tracing.
3. Every config is loaded via `pheno_config::load::<MyConfig>()`.
4. Every schema lives in `pheno-zod-schemas` or `pheno-pydantic-models`.
