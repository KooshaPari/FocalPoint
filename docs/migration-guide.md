# Polyrepo to Monorepo Migration Guide

## phenotype-postfx + phenotype-terrain + phenotype-water &rarr; phenotype-unity

This guide describes the consolidation of three independent Phenotype Unity packages into a single `phenotype-unity` workspace.

---

## Table of Contents

1. [Prerequisites](#1-prerequisites)
2. [Step-by-Step Migration Instructions](#2-step-by-step-migration-instructions)
3. [Common Pitfalls](#3-common-pitfalls)
4. [Verification Steps](#4-verification-steps)
5. [Rollback Plan](#5-rollback-plan)

---

## 1. Prerequisites

### Required Tools

| Tool | Version | Purpose |
|------|---------|---------|
| `git` | 2.40+ | History preservation, subtree merges |
| `dotnet` | 8.0+ | Build and test .NET projects |
| `task` (Taskfile) | 3.35+ | Run automation targets |
| `python3` | 3.10+ | JSON validation scripts |
| `Unity` | 2021.3 LTS+ | Verify package.json compatibility |
| `xcodebuild` or `msbuild` | Latest | Verify .csproj compilation on macOS / Windows |

### Access Requirements

- **Write access** to the target monorepo repository (`phenotype-unity`).
- **Admin access** to the three source repositories to archive them after migration.
- **CI/CD pipeline access** to update or disable old repository workflows.

### Pre-Migration Checklist

- [ ] All three source repositories have a clean `main` branch with no unmerged PRs.
- [ ] CI is green on `main` for `phenotype-postfx`, `phenotype-terrain`, and `phenotype-water`.
- [ ] A `phenotype-unity` repository exists (even if empty) with the correct default branch.
- [ ] The `WorldBoxManaged` MSBuild property path is documented and accessible on the migration workstation.
- [ ] A backup of all three repositories is stored locally or in a separate remote.

---

## 2. Step-by-Step Migration Instructions

### Phase 0: Create the Monorepo Scaffold

1. **Clone the target repository** (or create it):

   ```bash
   git clone git@github.com:Phenotype-org/phenotype-unity.git
   cd phenotype-unity
   ```

2. **Create the monorepo directory structure**.
   Each source repository is placed into a dedicated subdirectory to preserve history and avoid file collisions:

   ```
   phenotype-unity/
   ├── .editorconfig
   ├── .gitignore
   ├── LICENSE
   ├── README.md
   ├── Taskfile.yml                    # Root orchestration
   ├── nuget.config                    # Unified NuGet source
   ├── packages/
   │   └── postfx/                     # phenotype-postfx
   │       ├── package.json
   │       ├── Runtime/
   │       └── tests/
   ├── libs/
   │   ├── terrain/                    # phenotype-terrain
   │   │   ├── src/
   │   │   ├── tests/
   │   │   └── phenotype-terrain.csproj
   │   └── water/                      # phenotype-water
   │       ├── src/
   │       ├── tests/
   │       ├── phenotype-water.csproj
   │       └── phenotype-water.slnx
   └── docs/
       └── migration-guide.md
   ```

3. **Write a root `.gitignore`** that covers all three projects:

   ```gitignore
   # .NET
   bin/
   obj/
   *.dll
   *.pdb
   .vs/

   # Unity
   Library/
   Temp/
   Logs/
   *.csproj.meta
   *.sln.meta

   # OS
   .DS_Store
   Thumbs.db
   ```

### Phase 1: Migrate Repository Histories

Use `git subtree` to preserve per-project history. This is non-destructive and keeps commit SHAs intact.

1. **Add `phenotype-terrain` history**:

   ```bash
   git subtree add \
     --prefix libs/terrain \
     git@github.com:Phenotype-org/phenotype-terrain.git \
     main
   ```

2. **Add `phenotype-water` history**:

   ```bash
   git subtree add \
     --prefix libs/water \
     git@github.com:Phenotype-org/phenotype-water.git \
     main
   ```

3. **Add `phenotype-postfx` history**:

   ```bash
   git subtree add \
     --prefix packages/postfx \
     git@github.com:Phenotype-org/phenotype-postfx.git \
     main
   ```

> **Note:** If you do not need full history, you can use `git subtree add --squash` to collapse each repo into a single commit.

### Phase 2: Fix Cross-Project References

`phenotype-water` currently references `phenotype-terrain` via a relative path outside its own repository:

```xml
<!-- OLD: phenotype-water.csproj -->
<ProjectReference Include="../phenotype-terrain/phenotype-terrain.csproj" />
```

In the monorepo, this becomes an intra-workspace reference:

```xml
<!-- NEW: libs/water/phenotype-water.csproj -->
<ProjectReference Include="../terrain/phenotype-terrain.csproj" />
```

Apply the change:

```bash
sed -i 's|../phenotype-terrain/phenotype-terrain.csproj|../terrain/phenotype-terrain.csproj|g' \
  libs/water/phenotype-water.csproj
```

### Phase 3: Unify Build Configuration

1. **Create a root `Directory.Build.props`** at the repository root so all .NET projects share the same `WorldBoxManaged` contract and tooling versions:

   ```xml
   <Project>
     <PropertyGroup>
       <!-- Shared Unity reference root -->
       <WorldBoxManaged Condition="'$(WorldBoxManaged)' == ''">
         C:/Program Files (x86)/Steam/steamapps/common/worldbox/worldbox_Data/Managed
       </WorldBoxManaged>
       <!-- Tooling defaults -->
       <LangVersion>10.0</LangVersion>
       <Nullable>annotations</Nullable>
       <TreatWarningsAsErrors>false</TreatWarningsAsErrors>
     </PropertyGroup>
   </Project>
   ```

   > Update the `WorldBoxManaged` default path to match your team’s standard installation directory.

2. **Consolidate `NuGet.config`**.
   Both `phenotype-terrain` and `phenotype-water` contain identical `NuGet.config` files. Move a single copy to the repository root and delete the nested copies:

   ```bash
   cp libs/terrain/NuGet.config ./nuget.config
   git rm libs/terrain/NuGet.config
   git rm libs/water/NuGet.config
   ```

3. **Create a root `Taskfile.yml`** that orchestrates all three projects:

   ```yaml
   version: "3"

   vars:
     DOTNET_VERSION: "8.0"

   tasks:
     build:
       desc: "Build all .NET projects"
       cmds:
         - dotnet build libs/terrain/phenotype-terrain.csproj -c Release
         - dotnet build libs/water/phenotype-water.slnx -c Release

     test:
       desc: "Run all test suites"
       cmds:
         - task: test-terrain
         - task: test-water
         - task: test-postfx

     test-terrain:
       cmds:
         - dotnet test libs/terrain/tests/phenotype-terrain.tests.csproj -c Release

     test-water:
       cmds:
         - dotnet test libs/water/tests/phenotype-water.tests.csproj -c Release

     test-postfx:
       cmds:
         - dotnet test packages/postfx/tests/PostStackSourceTests.csproj -c Release
         - dotnet test packages/postfx/tests/PostStackVariantTests/PostStackVariantTests.csproj -c Release

     format:
       desc: "Check C# formatting across all projects"
       cmds:
         - dotnet format --verify-no-changes --verbosity diagnostic

     format-fix:
       desc: "Auto-fix C# formatting across all projects"
       cmds:
         - dotnet format --verbosity diagnostic

     validate:
       desc: "Validate package.json and shader existence for postfx"
       dir: packages/postfx
       cmds:
         - python3 -c "import json, sys; json.load(open('package.json'))"
         - |
           SHADERS=$(python3 -c "import re; text=open('Runtime/PostStack.cs').read(); matches=re.findall(r'\"Shaders/[^\"]+\"', text); [print(m[1:-1]) for m in matches]")
           for rel_path in $SHADERS; do
             if [ -f "Runtime/$rel_path" ] || [ -f "Runtime/${rel_path}.shader" ]; then echo "  OK $rel_path"; else echo "  MISSING: $rel_path"; exit 1; fi
           done

     default:
       desc: "Full quality gate"
       cmds:
         - task: build
         - task: test
         - task: format
         - task: validate
   ```

### Phase 4: Merge Shared Metadata

1. **Deduplicate standard files**.
   All three repositories contain identical `LICENSE` (MIT), `CODEOWNERS`, `CONTRIBUTING.md`, `SECURITY.md`, and `CLAUDE.md` files. Retain a single copy at the repository root and delete the nested duplicates:

   ```bash
   # Keep the root copies
   git rm libs/terrain/LICENSE libs/terrain/CODEOWNERS libs/terrain/CONTRIBUTING.md libs/terrain/SECURITY.md
   git rm libs/water/LICENSE libs/water/CODEOWNERS libs/water/CONTRIBUTING.md libs/water/SECURITY.md
   git rm packages/postfx/LICENSE packages/postfx/CONTRIBUTING.md packages/postfx/SECURITY.md
   ```

2. **Update `README.md`** at the repository root to describe the unified workspace:

   ```markdown
   # phenotype-unity

   Monorepo workspace for Phenotype Unity packages.

   | Package | Path | Description |
   |---------|------|-------------|
   | `postfx` | `packages/postfx/` | BRP post-processing stack (SSAO, SSGI, Bloom, ACES, LUT) |
   | `terrain` | `libs/terrain/` | Shared terrain mesh infrastructure |
   | `water` | `libs/water/` | Gerstner-wave water system |

   ## Build

   ```bash
   task build
   ```

   ## Test

   ```bash
   task test
   ```
   ```

### Phase 5: Clean Up Stale Artifacts

1. Remove per-project `_stub` / `obj` / `bin` artifacts that may have been carried over by `git subtree`:

   ```bash
   git rm -rf libs/terrain/_stub/obj/ libs/terrain/bin/ libs/terrain/obj/
   git rm -rf libs/water/bin/ libs/water/obj/
   ```

2. Add entries to the root `.gitignore` so these directories are never tracked again.

### Phase 6: Commit the Consolidation

1. Stage all changes:

   ```bash
   git add -A
   git commit -m "feat: consolidate phenotype-postfx, terrain, water into phenotype-unity workspace

   - Migrated three repositories via git subtree
   - Fixed water -> terrain project reference for monorepo layout
   - Unified NuGet.config, Taskfile, and Directory.Build.props
   - Deduplicated shared metadata (LICENSE, CODEOWNERS, CONTRIBUTING.md)
   - Added root README with workspace overview"
   ```

2. Push to the migration branch:

   ```bash
   git checkout -b phenotype-unity/migration-guide
   git push -u origin phenotype-unity/migration-guide
   ```

---

## 3. Common Pitfalls

### 3.1 Broken Relative Project References

**Problem:** `phenotype-water` references `phenotype-terrain` via `../phenotype-terrain`. After moving into `libs/water/`, this path resolves to the monorepo root, not to the terrain library.

**Fix:** Always update `<ProjectReference>` paths immediately after the `git subtree add`. Use a root `Directory.Build.props` or a shared MSBuild variable for cross-package references.

### 3.2 Colliding File Names

**Problem:** `phenotype-terrain` and `phenotype-water` both contain `NuGet.config`, `Taskfile.yml`, and `README.md` at their repo roots. A flat merge would overwrite them.

**Fix:** Use `git subtree` with `--prefix` so each repository lives in a dedicated subdirectory. Only deduplicate files that are truly identical and organization-wide (e.g., `LICENSE`, `CODEOWNERS`).

### 3.3 Missing Unity Reference Assemblies

**Problem:** `phenotype-terrain` and `phenotype-water` compile against `UnityEngine.CoreModule.dll` via `$(WorldBoxManaged)`. In the monorepo, the default path may differ between workstations.

**Fix:** Centralize the `WorldBoxManaged` property in a root `Directory.Build.props` and document how to override it locally:

```bash
# Local override (never commit)
dotnet build -p:WorldBoxManaged="/custom/path/to/Managed"
```

### 3.4 Shader Path Mismatches in postfx

**Problem:** `phenotype-postfx` validates shader existence by scanning `Runtime/PostStack.cs` for `"Shaders/..."` strings. If the `Runtime/` directory is moved or renamed during migration, the validation script breaks.

**Fix:** Keep the `packages/postfx/Runtime/` directory name unchanged. If you must rename it, update the Python regex in `Taskfile.yml` as well.

### 3.5 Ignored Test Projects

**Problem:** Both `phenotype-terrain` and `phenotype-water` exclude test files from their library `.csproj` files using `Compile Remove="tests/**/*.cs"`. When running `dotnet test` from the monorepo root, the test projects may not be discovered if the working directory is wrong.

**Fix:** Reference the explicit `.csproj` paths in the root `Taskfile.yml` (as shown in Phase 3) rather than relying on glob discovery.

### 3.6 Branch Name Collision

**Problem:** The migration branch `phenotype-unity/migration-guide` uses a slash, which Git treats as a directory-like namespace. Some CI systems or legacy scripts may not handle slash-containing branch names correctly.

**Fix:** Verify that your CI provider (GitHub Actions, GitLab CI, etc.) supports slash-delimited branch filters before pushing. GitHub Actions supports them natively via `branches: ['phenotype-unity/migration-guide']`.

---

## 4. Verification Steps

### 4.1 File Structure Verification

```bash
# Verify all three packages are present in their expected locations
ls -d packages/postfx/Runtime/ libs/terrain/src/ libs/water/src/ || echo "FAIL: missing directories"

# Verify deduplicated metadata exists only at root
[ -f LICENSE ] && [ -f CODEOWNERS ] && [ -f CONTRIBUTING.md ] && echo "OK: root metadata"
[ -f libs/terrain/LICENSE ] && echo "FAIL: stale LICENSE in terrain" || echo "OK: no stale LICENSE"
```

### 4.2 Build Verification

```bash
# Build both .NET libraries
export WorldBoxManaged="/path/to/worldbox_Data/Managed"
dotnet build libs/terrain/phenotype-terrain.csproj -c Release
dotnet build libs/water/phenotype-water.slnx -c Release
```

Expected: `Build succeeded` with `0 Warning(s)`.

### 4.3 Reference Resolution Verification

```bash
# Confirm water resolves terrain from the monorepo, not from an external path
dotnet build libs/water/phenotype-water.slnx -c Release -v:n \
  | grep -E "(ProjectReference|phenotype-terrain)"
```

Expected output must contain `../terrain/phenotype-terrain.csproj`, not `../phenotype-terrain/phenotype-terrain.csproj`.

### 4.4 Test Verification

```bash
# Run all test suites
task test
```

Expected: All three test projects pass with no failures.

| Suite | Expected Result |
|-------|----------------|
| `libs/terrain/tests` | `Total tests: N` — all passed |
| `libs/water/tests` | `Total tests: N` — all passed |
| `packages/postfx/tests/PostStackSourceTests` | `Total tests: N` — all passed |
| `packages/postfx/tests/PostStackVariantTests` | `Total tests: N` — all passed |

### 4.5 Formatting Verification

```bash
# Verify C# formatting is consistent across all projects
task format
```

Expected: `dotnet format` reports `0 errors` and exits with code `0`.

### 4.6 PostFX Package Validation

```bash
# Validate package.json and shader existence
cd packages/postfx
task validate
```

Expected: `Valid JSON`, required fields printed, and `OK` for every shader path.

### 4.7 Git History Verification

```bash
# Verify per-project history was preserved by git subtree
git log --oneline --all -- packages/postfx/ | head -5
git log --oneline --all -- libs/terrain/ | head -5
git log --oneline --all -- libs/water/ | head -5
```

Expected: Each directory shows commits from its original repository, not just a single squashed commit.

### 4.8 CI Pipeline Verification

1. Open a PR from `phenotype-unity/migration-guide` to `main`.
2. Confirm that CI triggers correctly (build, test, format, validate).
3. Verify the PR diff contains only the expected structural changes and no duplicate binary artifacts.

---

## 5. Rollback Plan

### 5.1 Immediate Rollback (Before PR Merge)

If the migration branch contains critical errors that cannot be fixed quickly:

```bash
# Delete the local and remote migration branch
git checkout main
git branch -D phenotype-unity/migration-guide
git push origin --delete phenotype-unity/migration-guide
```

The three source repositories remain untouched and fully functional.

### 5.2 Partial Rollback (After PR Merge, Monorepo Issues)

If the monorepo is merged but breaks downstream consumers:

1. **Revert the monorepo commit**:

   ```bash
   git revert -m 1 <merge-commit-sha>
   git push origin main
   ```

   This restores the monorepo to its pre-merge state while preserving the revert in history.

2. **Re-enable CI on the source repositories**:
   - Re-activate GitHub Actions workflows in `phenotype-postfx`, `phenotype-terrain`, and `phenotype-water`.
   - Remove any archive settings or redirection notices.

3. **Notify downstream consumers** to revert their project references from the monorepo back to the individual repositories.

### 5.3 Full Rollback (Monorepo Abandonment)

If the monorepo experiment fails after extended use:

1. **Create a snapshot** of the monorepo state:

   ```bash
   git tag archive/phenotype-unity-final <final-commit-sha>
   git push origin archive/phenotype-unity-final
   ```

2. **Split history back into independent repositories** using `git subtree split`:

   ```bash
   # Extract postfx history
   git subtree split --prefix=packages/postfx --branch postfx-history
   git push git@github.com:Phenotype-org/phenotype-postfx.git postfx-history:main

   # Extract terrain history
   git subtree split --prefix=libs/terrain --branch terrain-history
   git push git@github.com:Phenotype-org/phenotype-terrain.git terrain-history:main

   # Extract water history
   git subtree split --prefix=libs/water --branch water-history
   git push git@github.com:Phenotype-org/phenotype-water.git water-history:main
   ```

3. **Update downstream references** to point back to the original repositories.
4. **Archive the monorepo** with a final README explaining the rollback.

### 5.4 Rollback Decision Matrix

| Scenario | Action | Time to Recover |
|----------|--------|-----------------|
| Pre-merge CI failure | Delete branch, fix, retry | 10 minutes |
| Post-merge build break | Revert merge commit on `main` | 30 minutes |
| Cross-package reference regression | Fix `ProjectReference` path + fast-follow PR | 1 hour |
| Full workspace abandonment | `git subtree split` + re-push to source repos | 2–4 hours |

---

## Appendix: Quick Reference Commands

```bash
# Create migration branch
git checkout -b phenotype-unity/migration-guide

# Add source repo histories
git subtree add --prefix libs/terrain git@github.com:Phenotype-org/phenotype-terrain.git main
git subtree add --prefix libs/water git@github.com:Phenotype-org/phenotype-water.git main
git subtree add --prefix packages/postfx git@github.com:Phenotype-org/phenotype-postfx.git main

# Fix water -> terrain reference
sed -i 's|../phenotype-terrain/phenotype-terrain.csproj|../terrain/phenotype-terrain.csproj|g' \
  libs/water/phenotype-water.csproj

# Run full quality gate
task default

# Push
git push -u origin phenotype-unity/migration-guide
```
