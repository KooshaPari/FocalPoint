# pheno-zod-schemas AGENTS.md

This is the **agent constitution** for the `pheno-zod-schemas` crate. Read this before editing.

## Build & Test

```bash
npm ci                    # install dependencies
npm run build             # tsc --noEmit
npm test                  # vitest run (6 tests)
```

## Code Style

- **Language:** TypeScript (ES2022, NodeNext module resolution)
- **Lints:** `strict: true`, `noUncheckedIndexedAccess`, `noImplicitOverride`
- **Naming:** `PascalCase` types / schemas, `camelCase` fns, `SCREAMING_SNAKE` consts
- **Exports:** each schema as `<Name>Schema` and inferred type as `<Name>`
- **No `any`** without a `// TODO: tighten` comment
- **Zod primitives:** reuse `EmailSchema`, `IsoDateTimeSchema`, `UuidV4Schema`, `SlugSchema`, `CommitShaSchema`

## PR Conventions

- Title: `feat(zod):` / `fix(zod):` / `docs(zod):`
- Body: 1-3 bullets, link to task ID (e.g. `V20-zod.1`)
- Rebase onto `main`; no merge commits
- Run `npm test` before pushing

## Do Not Touch

- `package.json` `version` field — bumped by release-drafter only
- `LICENSE-MIT` — fixed text
- The 6-variant `WorklogStatus` tuple — adding or removing a status is a breaking wire change
- `tsconfig.json` compilerOptions — change only with monorepo-wide TS alignment

## Reference

- **Public API:** see `llms.txt` (one-page reference)
- **Worklog schema:** `WORKLOG.md` (V2 10-col)
- **Task DAG:** `V20_STRATEGIC_PLAN_2026_06_12.md` §96.1
- **npm package:** `@kooshapari/pheno-zod-schemas` (when published)

## Layer

- L3 Consolidate: canonical Zod schemas adopted from `chore/l3-53-pheno-zod-pydantic-2026-06-11`
- L5 Consume: used by L5 #81-85 across the pheno-* fleet
- Design: 3 entities (`User`, `WorklogEntry`, `Project`) + 6-status enum on Zod 3.23
