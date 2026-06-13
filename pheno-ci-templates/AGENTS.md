# pheno-ci-templates — Agent Instructions

## Project

Canonical reusable GitHub Actions CI + release workflows for the **pheno-\***
fleet. One source of truth for four languages (Rust, Python, Go, Node);
each consumer pulls the workflows via `uses:` and only sees the language
jobs that match the manifests present in the caller repo.

## Stack

- Language: YAML (GitHub Actions reusable workflows)
- Build: N/A (workflows are validated by GitHub Actions YAML linter)
- License: MIT

## Validation

```bash
python3 -c "import yaml; [yaml.safe_load(open(f)) for f in [
    'pheno-ci-templates/.github/workflows/ci.yml',
    'pheno-ci-templates/.github/workflows/release.yml'
]]"
```

## Test

Workflows are smoke-tested on merge to `main` via `workflow_dispatch`.
No local test runner exists.

## Style

- Inputs must have sensible defaults so a one-line `uses:` works.
- Every job-level step can be toggled independently.
- Language auto-detect via `hashFiles()` against manifest files.
- Pin all third-party actions to SHA or stable tag.
- Pass-through secrets so callers do not redeclare them.

## PR Conventions

- Branch naming: `feat/`, `fix/`, `chore/`, `docs/`, `refactor/`.
- One logical change per PR.
- All PRs must pass YAML validation and lint.
- Update `CHANGELOG.md` under `[Unreleased]`.
- Squash-merge with a conventional commit message.

## Do-Not-Touch

- Image digests / action SHAs in the `env:` block — these are security pins.
- Default input values — changing them is a breaking change for every caller.
- The `permissions:` block — least-privilege is intentional.

## Scope

This repo holds only reusable workflow definitions. Do not add application
code, crates, or SDKs here.
