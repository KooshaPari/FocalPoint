# Changelog

All notable changes to `pheno-errors` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial adoption from `chore/l3-47-pheno-tracing-2026-06-11` (L3-46)
- 8 unit tests + 6 integration tests = 14/14 passing
- `AppError` enum with 5 variants: `Domain`, `NotFound`, `Conflict`, `Validation`, `Storage`
- `AppResult<T>` alias for `Result<T, AppError>`
- `thiserror` derives for `Display` + `std::error::Error`
- `From` impls for `std::io::Error`, `&str`, `String`, `anyhow::Error`
- Structured logging helpers: `log_warn()` and `log_error()` via `tracing`
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Notes
- Standalone Rust crate (monorepo member; see `Cargo.toml` `[workspace]` table)
- Built on `thiserror` + `anyhow` + `tracing`
- V20 §96.1: L3-46 pheno-errors adopted into `chore/l3-57-pheno-plugin-registry-2026-06-11`
