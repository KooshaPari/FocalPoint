# pheno-go-ctxkit — Agent Instructions

## Project

Canonical Go context utilities for the `pheno-*` fleet.
Provides per-request UUID generation, structured logger injection/extraction,
and a small `net/http` middleware that wires both together and emits a
single "request.complete" log line.

## Stack

- Language: Go (1.22)
- Build: Go modules
- License: MIT
- Dependencies: none (standard library only)

## Build

```bash
go build ./...
go vet ./...
go test -race -count=1 ./...
```

## Test

```bash
go test -race -count=1 ./...
go test -cover ./...
```

## Style

- `gofmt` is authoritative.
- Every exported function must have a doc comment.
- Use unexported empty struct keys for context values to prevent collisions.
- Prefer `slog` over `log` or `fmt` for all output.
- Keep the middleware panic-free on malformed input.

## PR Conventions

- Branch naming: `feat/`, `fix/`, `chore/`, `docs/`, `refactor/`.
- One logical change per PR.
- All PRs must pass `go vet`, `gofmt`, and `go test -race`.
- Update `CHANGELOG.md` under `[Unreleased]`.
- Squash-merge with a conventional commit message.

## Do-Not-Touch

- `go.mod` module path — changing it breaks every importer.
- `HeaderRequestID` constant — changing it breaks cross-service correlation.
- `NewRequestID` UUID v4 implementation — must remain RFC 4122 compliant.
- `Logger` fallback to `slog.Default` — removing it breaks unconditional logging.

## Scope

This package holds only universal context utilities. Do not add
application-specific types (e.g. `UserContext`, `AuthContext`) here;
create a downstream package that depends on `ctxkit` and defines those.
