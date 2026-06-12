# hwLedger V4 Deep Audit

Audit date: 2026-06-11. Repo: `/Users/kooshapari/CodeProjects/Phenotype/repos/hwLedger`.

## Snapshot

- Recent commits: `be660d8d` merge docs/landing config, `456b8aa8` add `docs/landing.json`, `eb821b20` add `.gitattributes`.
- Worktree: no short-status entries were printed by `git status --short | head -10`.
- README says the repo is `SCAFFOLD`, 25%, pre-alpha Phase 0, with a planned Rust workspace + native apps + sidecars, mostly docs/scaffold so far ([README.md:1](README.md:1), [README.md:12](README.md:12), [README.md:43](README.md:43)).
- Local reality: no root `package.json`, no `Cargo.toml` found to maxdepth 4, no `astro.config*`, no source `*.astro` files outside generated `.astro` residue. The only first-party package found is `docs/package.json`, a VitePress docs package.

## Build Matrix

| Command | Result |
|---|---|
| `npm run build 2>&1 \| tail -10` | Fails: `ENOENT`, no root `package.json` at `hwLedger/package.json`. npm logs also could not be written under `~/.npm/_logs`. |
| `npm test 2>&1 \| tail -5` | Fails same root `package.json` `ENOENT`. |
| Docs build command available | `docs/package.json` exposes `docs:build: vitepress build .`, not root `build` ([docs/package.json:7](docs/package.json:7)). |
| Rust build/test readiness | README documents `cargo install --path crates/hwledger-cli` and `cargo run -p hwledger-devtools -- up` ([README.md:26](README.md:26), [README.md:52](README.md:52)), but no Cargo workspace files were present in this checkout scan. |

## Astro Inventory

- Astro page count: requested `find ... -name '*.astro' | wc -l` returned `3`, but those are generated directories/files under `.astro`/vendored residue. Source-only count excluding `node_modules`, `dist`, and `.astro`: `0`.
- Integration count: `0` first-party Astro integrations; no `astro.config*` found.
- Content collection count: `0`; no `src/content`, no `defineCollection`.
- View transitions: not used; no `ViewTransitions`/view-transition references in first-party source.
- Actual docs stack: VitePress + Vue + TypeScript ([docs/package.json:13](docs/package.json:13)); config is shared Phenotype VitePress via `createPhenotypeConfig` ([docs/.vitepress/config.mts:5](docs/.vitepress/config.mts:5)).
- Landing metadata exists as JSON for an external generator, not an Astro app ([docs/landing.json:1](docs/landing.json:1)).

## Apps, Sidecars, Tools, Landing

- `apps/`: `build`, `landing`, `macos`, `streamlit`; `apps/streamlit` currently contains a checked-in `.venv` surface, which is audit noise.
- `sidecars/`: `omlx-fork`; README frames it as a fat oMlx fork ([README.md:44](README.md:44)).
- `tools/`: `journey-remotion`, but no files found to maxdepth 3.
- `landing/`: present but empty at shallow scan.
- `.benchmarks/`: present but no files found to maxdepth 3.

## Governance Gap Matrix

| File | Status |
|---|---|
| `LICENSE` | Present |
| `LICENSE-MIT` | Absent |
| `LICENSE-APACHE` | Absent |
| `CHANGELOG.md` | Present |
| `AGENTS.md` | Present |
| `ARCHITECTURE.md` | Absent |
| `CLAUDE.md` | Present |
| `SPEC.md` | Absent |
| `STATUS.md` | Absent |
| `CODE_OF_CONDUCT.md` | Present |
| `CONTRIBUTING.md` | Present |
| `SECURITY.md` | Present |
| `deny.toml` | Present, but permissive license allowlist includes copyleft/novel licenses like GPL-3.0-only, WTFPL, Unlicense ([deny.toml:5](deny.toml:5), [deny.toml:15](deny.toml:15), [deny.toml:25](deny.toml:25)). |
| `.editorconfig` | Absent |
| CI | Present: cargo-audit, cargo-deny, CodeQL, docs deploy, scorecard, trufflehog. Cargo workflows key on `Cargo.toml`/`Cargo.lock`, which are absent locally. |

## Hex Readiness Score

2/10. Astro/hex is unusual here because current source is VitePress/static docs plus scaffolded Rust/native ambitions, not an Astro SSR/SSG application. There is a landing JSON contract and generated Astro residue, but no source Astro app, no routes, no content collections, no adapter, no root package scripts, and no verifiable build entrypoint.

## Hand-Rolled / Fragile Patterns

1. Landing is data-driven via `docs/landing.json` rather than a local typed schema or generated checked output; useful, but not validated in root CI ([docs/landing.json:2](docs/landing.json:2)).
2. Docs config depends on external `@phenotype/docs/config` without a local root package/workspace lock shown in this scan ([docs/.vitepress/config.mts:5](docs/.vitepress/config.mts:5)).
3. Rich-media journey references in README point to remote branch assets and non-present local `apps/cli-journeys` paths, creating drift risk ([README.md:100](README.md:100)).
4. Vendored/generated directories are present enough to pollute TODO scans: `apps/landing/node_modules`, `apps/landing/dist`, `docs-site/.vitepress/dist`, `apps/streamlit/.venv`.
5. README documents planned crates/native apps as if part of architecture, but local build manifests are missing, so docs and runnable surface are out of sync.

## 5 SOTA Gaps

1. No reproducible root build/test entrypoint: root npm fails and Cargo manifests are absent.
2. No Astro source despite V4 Astro premise; either remove Astro from the plan or add a real `apps/landing` package with config, pages, tests, and lockfile.
3. No typed governance around landing/docs metadata; `docs/landing.json` needs schema validation in CI.
4. Repo hygiene gap: checked/generated dependency/build artifacts distort search, security scans, and size.
5. CI is security-heavy but not product-build-heavy: no root matrix proving docs, landing, Streamlit, and Rust/native scaffolds build on Mac/Linux.

## Libification Candidates

1. `docs/landing.json` -> typed `landing` manifest package/schema consumed by generator and CI ([docs/landing.json:1](docs/landing.json:1)).
2. `docs/.vitepress/config.mts` -> shared docs config wrapper with local tests for base/custom-domain behavior ([docs/.vitepress/config.mts:7](docs/.vitepress/config.mts:7)).
3. README rich-media stubs -> reusable journey manifest validator; current docs package has `journey:lint` but only under `docs/` ([docs/package.json:11](docs/package.json:11)).
4. `deny.toml` license policy -> org-standard deny preset with tighter app-safe defaults ([deny.toml:7](deny.toml:7)).
5. Dev bootstrap commands -> one root task runner once manifests exist; README currently advertises commands whose files are not present ([README.md:29](README.md:29), [README.md:55](README.md:55)).

## 5 Prioritized V4 Next Steps

1. Decide V4 surface truth: VitePress docs-only, real Astro landing app, or both. If Astro, create `apps/landing/package.json`, `astro.config.mjs`, `src/pages/index.astro`, and lockfile.
2. Add a root build orchestrator: `package.json` or `justfile` that runs docs build, landing build, metadata validation, and source hygiene checks.
3. Remove generated/vendor artifacts from version control or add explicit audit excludes: `apps/landing/node_modules`, `apps/landing/dist`, `docs-site/.vitepress/dist`, `apps/streamlit/.venv`.
4. Add governance files: `.editorconfig`, `ARCHITECTURE.md`, `SPEC.md`, plus align README promised docs with actual files.
5. Tighten CI matrix for Mac/Linux primary: docs build, landing build if added, rich-media manifest lint, deny/audit, and repo hygiene checks that fail on vendored generated artifacts.

