# pheno-fastapi-base AGENTS

This is the **agent constitution** for the `pheno-fastapi-base` Python package. Read this before editing.

## Build & Test

```bash
python -m pip install -e ".[dev]"
python -m pytest tests/ -v
python -m ruff check src/ tests/
python -m mypy src/
```

## Code Style

- **Python:** 3.10+ (uses `match` statements, `|` union types, structural pattern matching)
- **Type hints:** Required on all public functions; `from __future__ import annotations`
- **Style:** `ruff` (rules: E, F, I, UP, B, SIM, RUF)
- **Imports:** `isort`-compatible; first-party = `pheno_fastapi_base`
- **Naming:** `snake_case` fns, `PascalCase` classes, `SCREAMING_SNAKE` consts
- **Async:** `async def` for I/O; sync wrappers only at module boundary
- **Pydantic:** v2 (use `model_config = ConfigDict(...)`)

## PR Conventions

- Title: `feat(fastapi):` / `fix(fastapi):` / `docs(fastapi):`
- Body: 1-3 bullets, link to task ID (e.g. `V18-fastapi.2`)
- Rebase onto `main`; no merge commits
- Run `pytest && ruff check && mypy` before pushing

## Do Not Touch

- `pheno_fastapi_base/__init__.py` re-exports — stable public API
- `pyproject.toml` `version` field — bumped by release-drafter only
- `LICENSE-MIT` — fixed text
- `pheno_fastapi_base/errors.py` error codes — HTTP-stable contract

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `FLEET_DAG_v3.md` §92 (V18 EXT)
- **PyPI:** `pheno-fastapi-base` (when published)

## Layer

- L1 Stabilize: `App` factory, `Router` builder, error → HTTP conversion
- L4 Hexagonal: composes `pheno-tower` (Rust) via `pyo3` if needed
- L1: pre-wired middleware (CORS, request-ID, OTEL)
