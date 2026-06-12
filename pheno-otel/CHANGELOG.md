# Changelog

All notable changes to `pheno-otel` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial scaffold (L3-49 cherry-pick)
- 5 unit tests + 3 doctests = 8/8 passing
- AI-DD crutches: `AGENTS.md` (110 lines), `llms.txt` (80 lines)
- This `WORKLOG.md` (V2 10-col schema)
- `LICENSE-MIT` (2026 Koosha Pari)

### Notes
- Standalone Rust crate (monorepo member; see `Cargo.toml`)
- Depends on `opentelemetry`, `opentelemetry_sdk`, `tracing`, `tracing-opentelemetry`
- L4 hexagonal layer support: implements the pheno-tower `Service` trait
