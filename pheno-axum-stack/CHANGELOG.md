# Changelog — pheno-axum-stack

All notable changes to this crate are documented here. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-11

### Added
- Initial release as part of L3-54 `pheno-tower-stack` wave (V4 §83.6).
- `hello_router()` returns a `Router` that responds 200 OK with body `"hello from pheno-axum-stack\n"` to any request.
- 3 unit tests covering: router function existence, response body, response status code.
- Workspace member of the monorepo per the L3 L1 Stabilize contract.

### Notes
- This is the **axum-specific layer** of the tower stack. Use `pheno-tokio-base` for the runtime, `pheno-tower` for the service abstraction, and `pheno-axum-stack` for HTTP routing.
- See `AGENTS.md` for full do-not-touch zones and `WORKLOG.md` for task history.

[0.1.0]: #0.1.0
