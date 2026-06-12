# pheno-pydantic-models AGENTS.md

This is the **agent constitution** for the `pheno-pydantic-models` crate. Read this before editing.

## Build & Test

```bash
pip install -e ".[dev]"   # install package + pytest
pytest -q                   # 5 pytest tests
```

## Code Style

- **Language:** Python 3.11+ (type-annotated, `from __future__ import annotations`)
- **Lints:** `ruff` / `pyright` if available; at minimum `mypy --strict` clean
- **Naming:** `PascalCase` classes, `snake_case` fns/vars, `SCREAMING_SNAKE` module-level constants
- **Exports:** `__all__` declared explicitly in `__init__.py`
- **No `Any`** without a `# TODO: tighten` comment
- **Pydantic v2 primitives:** reuse `EmailStr`, `UUID4`, `AwareDatetime`, `Field` metadata

## PR Conventions

- Title: `feat(pydantic):` / `fix(pydantic):` / `docs(pydantic):`
- Body: 1-3 bullets, link to task ID (e.g. `V20-pydantic.1`)
- Rebase onto `main`; no merge commits
- Run `pytest -q` before pushing

## Do Not Touch

- `pyproject.toml` `version` field — bumped by release-drafter only
- `LICENSE-MIT` — fixed text
- The 6-variant `WorklogStatus` enum — adding or removing a status is a breaking wire change
- `pyproject.toml` `[build-system]` — change only with monorepo-wide Python alignment

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `V20_STRATEGIC_PLAN_2026_06_12.md` §96.1
- **PyPI package:** `pheno-pydantic-models` (when published)

## Layer

- L3 Consolidate: canonical Pydantic models adopted from `chore/l3-53-pheno-zod-pydantic-2026-06-11`
- L5 Consume: used by L5 #81-85 across the pheno-* fleet
- Design: 3 entities (`User`, `WorklogEntry`, `Project`) + 6-status enum on Pydantic v2
