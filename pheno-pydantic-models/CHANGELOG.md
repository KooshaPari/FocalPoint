# Changelog

All notable changes to `pheno-pydantic-models` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial adoption from `chore/l3-53-pheno-zod-pydantic-2026-06-11` (L3-53)
- 5 pytest tests, all passing
- `User` — RFC 4122 UUID, email, display name, created_at
- `WorklogEntry` — task_id, 6-status enum, agent_id, commit_sha, timestamps, files_changed
- `Project` — slug id, name, owner_email, members array with owner-invariant
- `WorklogStatus` enum with 6 canonical statuses
- Wire-codes aligned with sibling `pheno-zod-schemas` TypeScript package
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Notes
- Standalone Python package (monorepo member; no workspace linkage required)
- Built on `pydantic` >=2.6 + `email-validator` >=2.0 + `pytest` >=8.0
- V20 §96.1: L3-53 pheno-pydantic-models adopted into `chore/l3-57-pheno-plugin-registry-2026-06-11`
