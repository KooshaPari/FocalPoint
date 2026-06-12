# pheno-ci-templates — AGENTS.md

## Build & Test
- This is a **template repo** — there is no build or test. The contents are consumed by `cookiecutter` / `cruft` / `cargo generate` / `just` in downstream repos.
- Verify a template renders: `cookiecutter pheno-ci-templates --no-input` and inspect the output.

## Code Style
- YAML only (no JSON). All workflow files use 2-space indentation.
- Template variables use `cookiecutter.json` placeholders (e.g. `{{ cookiecutter.project_name }}`).
- No shell scripts in workflows (use `pheno-otel` action or `pheno-go-ctxkit` wrapper).

## PR Conventions
- Branch: `chore/<scope>-ci-templates-YYYY-MM-DD`
- One workflow family per PR (e.g. `release.yml` is a separate concern from `ci.yml`).
- Run `actionlint` on every `.github/workflows/*.yml` change.

## Do Not Touch
- The 3-stage pipeline order (lint → test → release) — changes need a separate ADR.
- The `pheno-*` integration points — these are the contract with downstream consumers.

## Reference
- See `README.md` for the cookiecutter schema.
- See `llms.txt` for the LLM-friendly API.
- V19 EXTENSION: FLEET_DAG_v3.md §96
