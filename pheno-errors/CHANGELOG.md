# Changelog

All notable changes to `pheno-errors` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- L3 #46 finalization: minimal `Error(pub String)` newtype + `Result<T>` alias
- 3 unit tests + 2 integration tests = 5/5 passing
- `Display`, `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash` derives on `Error`
- `std::error::Error` impl on `Error`
- `From<&str>` and `From<String>` impls for ergonomic construction
- Convenience constructors: `Error::new`, `Error::as_str`, `Error::into_string`
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Changed (L3 #46 finalization)
- **BREAKING:** Replaced 5-variant `AppError` enum (`Domain`/`NotFound`/`Conflict`/`Validation`/`Storage`) with minimal `Error(pub String)` newtype
- **BREAKING:** Replaced `AppResult<T>` type alias with `Result<T>`
- **BREAKING:** Removed dependencies on `thiserror`, `anyhow`, and `tracing` (crate is now zero-dep)
- **BREAKING:** Renamed `tests/smoke.rs` → `tests/integration.rs`

### Notes
- Standalone Rust crate (no `[workspace]` table, no `Cargo.toml` `[dependencies]`)
- Zero external dependencies — the entire surface is a `String` wrapper + `Result` alias
- V20 §96.1: L3-46 pheno-errors finalization landed on `chore/l3-57-pheno-plugin-registry-2026-06-11`
- Unblocks 6+ L3 deps that were waiting for a minimal, dependency-free error type
