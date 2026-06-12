# pheno-go-ctxkit AGENTS

This is the **agent constitution** for the `pheno-go-ctxkit` Go module. Read this before editing.

## Build & Test

```bash
go test ./...                  # unit + race
go test -race ./...            # race detector
go vet ./...
golangci-lint run              # full lint
go mod tidy
```

## Code Style

- **Go:** 1.22+ (uses `slices`, `maps`, `cmp` stdlib packages)
- **Style:** `gofmt` + `goimports`; no exceptions
- **Lints:** `golangci-lint` with `default` + `gocritic`, `govet`, `staticcheck`, `revive`
- **Naming:** `PascalCase` exported, `camelCase` unexported, `SCREAMING_SNAKE` consts
- **Errors:** Wrap with `fmt.Errorf("ctxkit: %w", err)`; never `panic` in library code
- **Context:** First parameter on every I/O function (`func Foo(ctx context.Context, ...)`)
- **Doc comments:** Every exported identifier has a `// Foo does X` line

## PR Conventions

- Title: `feat(ctxkit):` / `fix(ctxkit):` / `docs(ctxkit):`
- Body: 1-3 bullets, link to task ID (e.g. `V18-ctxkit.2`)
- Rebase onto `main`; no merge commits
- Run `go test -race ./... && golangci-lint run` before pushing

## Do Not Touch

- `ctxkit.go` `Context` type alias — stable contract
- `go.mod` module path — `github.com/kooshapari/ctxkit` (canonical)
- `LICENSE-MIT` — fixed text
- `internal/` packages — go-private, no public API

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `FLEET_DAG_v3.md` §92 (V18 EXT)
- **GoDoc:** `pkg.go.dev/github.com/kooshapari/ctxkit` (when published)

## Layer

- L1 Stabilize: `Context`, `WithTimeout`, `WithDeadline` helpers
- L4 Hexagonal: `Context` is the standard `context.Context` re-exported with trace IDs
- L1: `Done()` channel pattern for cancellation
