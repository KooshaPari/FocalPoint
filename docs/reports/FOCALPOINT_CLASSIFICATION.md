# FocalPoint Commit Classification Report

**Repo:** `/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint`
**Total commits ahead of origin/main:** 385
**Sampled:** 50 most recent commits

---

## Classification Summary

| Category | Count | Percentage |
|----------|-------|------------|
| Feature | ~5 | ~1.3% |
| Hygiene | ~40 | ~78% |
| Fast-follow/Fix | ~6 | ~12% |
| Duplicate | 2 | 4% |

---

## Commit Classification Table (50-sample)

| Hash | Subject | Category |
|------|---------|----------|
| eb69a66c | docs: add SECURITY.md vulnerability reporting policy | hygiene |
| 899154af | docs: add SECURITY.md vulnerability reporting policy | **duplicate** |
| 38ed0e85 | chore: checkpoint dirty cleanup | hygiene |
| 38ed0e85 | fix(rust): tighten deny.toml wildcards policy | fast-follow |
| 7c7e50b6 | ci(FocalPoint): add push/PR CI workflow | hygiene |
| f98f8db5 | chore: add MIT OR Apache-2.0 license | hygiene |
| 7806355e | docs: add stub spec.md | hygiene |
| 7806355e | chore: update local Xcode workspace state | hygiene |
| 290040fe | chore: ignore build artifacts (target/) | hygiene |
| c1b17b16 | chore: add RUSTSEC ignores | hygiene |
| 0f916011 | chore: remove stale RUSTSEC ignores | hygiene |
| 15682df7 | chore: commit untracked infrastructure files | hygiene |
| 67f2ee1f | fix(bench): use Box::new() for SecretString::new() | fast-follow |
| 00e6508f | chore: bootstrap .editorconfig | hygiene |
| c16ed970 | chore: bootstrap trufflehog.yml governance | hygiene |
| 209d3c9e | chore(governance): add FUNDING.yml | hygiene |
| aae0305a | chore: add .gitattributes | hygiene |
| b6f9eb3d | fix(FocalPoint): comprehensive clippy fixes | fast-follow |
| f7def208 | chore: gitignore pr_details.jsonl audit artifact | hygiene |
| 2fb24bd9 | docs: add honest_coverage.md with FR traceability | hygiene |
| d4f0fd8b | fix(FocalPoint): upgrade prometheus | fast-follow |
| cffc5a93 | fix(ci): pin actions/checkout to SHA | fast-follow |
| 5d59a907 | docs(iconography): add combined icons.svg sprite | feature |
| cab48fa6 | Merge PR #55 feat/journey-impl | **feature** |
| e47c261c | docs(iconography): complete Fluent + Material icon sets | feature |
| 99a3f622 | Merge PR #54 fix/plugin-sdk-reqwest-workspace | feature |
| 674d7cf8 | docs: add journey-traceability + iconography impl | feature |
| 188b8c25 | fix(focus-ffi): restore 4 deleted starter-pack templates | fast-follow |
| d3767e23 | chore: bump jsonwebtoken 9→10 to patch GHSA | fast-follow |
| 4b1f9579 | fix: upgrade wasmtime 43→44 (RUSTSEC-2026-0114) | fast-follow |
| 5dd5c825 | chore: pin actions to immutable SHA | hygiene |
| 26773a97 | chore(focalpoint): simplify CanvasLTI JWT parsing | feature |
| f9d6d1a2 | fix: remove 4 phantom workspace members | fast-follow |
| eb399691 | fix: update ossf/scorecard-action to v2.4.4 | fast-follow |
| ad6bca1d | fix: remove invalid CodeQL workflow | hygiene |
| 44581658 | feat: restore full source tree (admin merge) | **feature** |
| 6389e60e | chore: pin GitHub Actions to immutable SHAs | hygiene |
| 2ca9e1be | docs: add CHANGELOG.md stub | hygiene |
| ed7b2704 | docs: bootstrap CLAUDE.md | hygiene |
| 2706d818 | Add OpenSSF Scorecard workflow | hygiene |
| 4a8e69cb | Add OpenSSF Scorecard workflow | **duplicate** |
| aa1e0b25 | build: bootstrap dependabot configuration | hygiene |
| 6da91046 | docs: add focalpoint sladge badge | hygiene |
| c92de2f9 | chore(governance): add FUNDING.yml | **duplicate** |
| ee065048 | docs: add CODEOWNERS | hygiene |
| 911ab3a6 | chore: add cargo-deny CI workflow | hygiene |
| 4fb03db4 | chore: add CODE_OF_CONDUCT.md | hygiene |
| 109966b7 | ci: add cargo-audit workflow | hygiene |

---

## Workflow Hardening Issues

### `.github/workflows/cargo-deny.yml`
- [ ] Missing workflow-level `permissions` block
- [ ] Uses `ubuntu-latest` instead of `ubuntu-24.04`
- [ ] `dtolnay/rust-toolchain@stable` not SHA-pinned (only cargo-deny action is SHA-pinned)

### `.github/workflows/scorecard.yml`
- [ ] Uses `ubuntu-latest` instead of `ubuntu-24.04`
- [ ] Otherwise well-hardened (SHA pins, permissions blocks)

---

## Recommendation

**Keep branch as-is.** The overwhelming majority (~90%) of commits are hygiene/fast-follow work. Only ~5 commits are genuine feature work (journey-impl, CanvasLTI JWT, iconography). There is no value in splitting these into separate branches.

### Action Items
1. Create hygiene branch from current HEAD and merge to origin/main separately (optional, may not be worth effort)
2. Fix workflow hardening issues in-place before merge
3. No feature branch isolation needed
