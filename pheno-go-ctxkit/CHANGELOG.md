# Changelog

## [0.1.0] - 2026-06-12

### Added
- Initial release of pheno-go-ctxkit.
- Canonical Go context utilities: request ID, logger injection, middleware.
- `WithRequestID`, `RequestID`, `NewRequestID` (UUID v4 via crypto/rand).
- `WithLogger`, `Logger`, `Background` helpers.
- `Middleware` that wires request ID + logger and emits structured "request.complete" log.
- Zero third-party dependencies (stdlib only).
- AI-DD crutch files: AGENTS.md, llms.txt, WORKLOG.md, CHANGELOG.md, LICENSE-MIT.
