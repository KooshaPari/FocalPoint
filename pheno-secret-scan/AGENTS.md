# pheno-secret-scan — Agent Instructions

## Project

Canonical TruffleHog-based secret scanning for the **pheno-\*** fleet.
Ships a GitHub Actions workflow, a pre-commit hook manifest, and a baseline
allowlist so every pheno-* repo has the same secret-scanning posture.

## Stack

- Language: YAML (GitHub Actions workflow + pre-commit hooks)
- Build: N/A (integration is a Docker image, not a compiled crate)
- License: MIT

## Validation

```bash
python3 -c "import yaml; [yaml.safe_load(open(f)) for f in [
    'pheno-secret-scan/.github/workflows/secret-scan.yml',
    'pheno-secret-scan/.pre-commit-hooks.yaml'
]]"
```

## Test

The workflow is validated by the GitHub Actions YAML linter when pushed.
The pre-commit hook is validated by pre-commit.com schema.
No local integration test exists (runtime is a Docker image).

## Style

- Workflow triggers must cover `push`, `pull_request`, `schedule`, and `workflow_dispatch`.
- The `TRUFFLEHOG_IMAGE` digest is a security pin; do not change without review.
- The allowlist suppresses only *verified* findings; unverified hits still fail CI.
- Pre-commit hook uses `--since-commit HEAD` and `--no-verification` for speed.
- `pass_filenames: false` because TruffleHog reads git history directly.

## PR Conventions

- Branch naming: `feat/`, `fix/`, `chore/`, `docs/`, `refactor/`.
- One logical change per PR.
- All PRs must pass YAML validation and lint.
- Update `CHANGELOG.md` under `[Unreleased]`.
- Squash-merge with a conventional commit message.

## Do-Not-Touch

- `TRUFFLEHOG_IMAGE` digest in `env:` — security pin.
- `--fail` flag — removing it makes verified secrets non-blocking.
- `--no-verification` in pre-commit — adding verification there makes the hook unusably slow.
- Empty default allowlist — adding broad suppressions is a security regression.

## Scope

This repo holds only integration wrappers (workflow + hook + allowlist).
Do not add a re-implementation of TruffleHog here; the canonical runtime
is `trufflesecurity/trufflehog`.
