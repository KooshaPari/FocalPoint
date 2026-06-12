# Changelog

All notable changes to `pheno-plugin` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial scaffold (L3-57 cherry-pick)
- 4 unit tests + 1 doctest = 5/5 passing
- `Plugin` trait with `name()`, `version()`, `config_schema()` methods
- `PluginRegistry` for global lookups (uses `inventory::collect!`)
- `PluginError` enum: `NotFound`, `AlreadyRegistered`, `VersionMismatch`
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Notes
- Standalone Rust crate (monorepo member; see `Cargo.toml`)
- Depends on `inventory` for global registration pattern
- L4 hexagonal: `Plugin` wraps the `tower::Service` contract from `pheno-tower`
