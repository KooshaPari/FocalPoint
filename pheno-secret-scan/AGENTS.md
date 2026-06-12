# pheno-secret-scan — AGENTS.md

## Build & Test
- This is a **policy repo** — no build, no test. The contents are consumed by `trufflehog`, `gitleaks`, and pre-commit hooks.
- Verify the policy: `trufflehog filesystem --config .trufflehog-allowlist.txt /tmp/test-repo`.

## Code Style
- Allowlist entries are regex patterns (one per line).
- `.pre-commit-hooks.yaml` follows the standard `pre-commit` schema.

## PR Conventions
- Branch: `chore/<scope>-secret-scan-YYYY-MM-DD`
- Adding a new allowlist entry needs a justification in the PR body.
- Removing an entry is a breaking change for consumers.

## Do Not Touch
- The 12 default denylist patterns (private keys, AWS keys, etc.) — these are security-critical.
- The pre-commit hook IDs (`pheno-secret-scan-trufflehog`, etc.) — downstream `.pre-commit-config.yaml` depends on these.

## Reference
- See `.trufflehog-allowlist.txt` for the full allowlist.
- See `.pre-commit-hooks.yaml` for the pre-commit integration.
- See `llms.txt` for the LLM-friendly API.
- V19 EXTENSION: FLEET_DAG_v3.md §96
