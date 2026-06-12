# Changelog

All notable changes to `pheno-zod-schemas` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial adoption from `chore/l3-53-pheno-zod-pydantic-2026-06-11` (L3-53)
- 6 vitest tests, all passing
- `UserSchema` / `User` — RFC 4122 UUID, email, display name, created_at
- `WorklogEntrySchema` / `WorklogEntry` — task_id, 6-status enum, agent_id, commit_sha, timestamps, files_changed
- `ProjectSchema` / `Project` — slug id, name, owner_email, members array with owner-invariant
- `WorklogStatus` tuple with 6 canonical statuses
- Wire-codes aligned with sibling `pheno-pydantic-models` Python package
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Notes
- Standalone TypeScript package (monorepo member; no workspace linkage required)
- Built on `zod` ^3.23 + `typescript` ^5.4 + `vitest` ^1.6
- V20 §96.1: L3-53 pheno-zod-schemas adopted into `chore/l3-57-pheno-plugin-registry-2026-06-11`
