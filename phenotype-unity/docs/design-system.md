# Phenotype Unity Design System

## Overview

The `phenotype-unity` workspace is a collection of sibling Unity packages that share a common build contract, design language, and polyrepo pattern under the Phenotype org. Each package targets `net48` and the Unity Built-In Render Pipeline (BRP), and is designed to be consumed as a `<ProjectReference>` by WorldBox / Unity mods.

The workspace follows the hexagonal polyrepo pattern: every package is independently versioned, independently buildable, and depends on siblings only through formal project references.

---

## 1. Package Structure

### 1.1 Core Packages

| Package | Path | Purpose | Maturity |
|---------|------|---------|----------|
| `phenotype-postfx` | `phenotype-postfx/` | Reusable BRP post-processing stack (SSAO, SSGI, Bloom, ACES, LUT) | 70% |
| `phenotype-terrain` | `phenotype-terrain/` | Shared terrain mesh infrastructure (height-field, chunk mesh, LOD) | 20% |
| `phenotype-water` | `phenotype-water/` | Gerstner-wave water system with camera-aware LOD | 20% |

### 1.2 Directory Layout (Per Package)

Each package follows a standard directory contract:

```
<phenotype-{name}>/
├── README.md              # Usage, build instructions, install
├── LICENSE                # MIT (default for Phenotype org)
├── CONTRIBUTING.md        # Phenotype-org standard
├── CODEOWNERS             # Package ownership
├── SECURITY.md            # Security policy
├── STATUS.md              # Build / health status
├── CHANGELOG.md           # Release notes
├── AGENTS.md              # Agent context file
├── package.json           # Unity Package Manager metadata (postfx only)
├── NuGet.config           # NuGet source configuration
├── Taskfile.yml           # Build / test / lint commands
├── <name>.csproj          # Library project
├── <name>.slnx            # Optional solution (water)
├── src/ or Runtime/       # Source code
│   ├── UnityEngineStubs.cs    # Stub when UnityEngine is absent (CI)
│   └── <Domain>/
├── tests/                 # Unit / integration tests
│   └── <name>.tests.csproj
└── _stub/                 # Compiled UnityEngine stub (terrain only)
```

#### Source Folder Convention

- **`src/`** — Used by `phenotype-terrain` and `phenotype-water` (C# class-library style).
- **`Runtime/`** — Used by `phenotype-postfx` (Unity package style with `.asmdef` and `.meta` files).

Both layouts are valid; the choice depends on whether the package is intended to be dropped into a Unity `Assets/` folder or referenced as a project.

---

## 2. Shared Components

### 2.1 UnityEngine Reference Contract

All packages compile against the Unity BRP (`UnityEngine.CoreModule.dll`). Because the engine DLL is not available in CI, every package must provide a **fallback stub** that satisfies the compiler when the real DLL is absent.

#### Two Stub Patterns

| Pattern | Package | Description |
|---------|---------|-------------|
| **Inline stub** | `phenotype-water` | `src/UnityEngineStubs.cs` — included via `<Compile Include="...">` when `UnityEngine.CoreModule.dll` is missing. |
| **Compiled stub** | `phenotype-terrain` | `_stub/` contains a pre-built `UnityEngine.CoreModule.dll` generated from a separate stub project. Removed from compilation when the real DLL is present. |

#### Conditional Compilation (MSBuild)

```xml
<!-- Terrain: use compiled stub -->
<ItemGroup Condition="Exists('$(WorldBoxManaged)/UnityEngine.CoreModule.dll')">
  <Reference Include="UnityEngine.CoreModule">
    <HintPath>$(WorldBoxManaged)/UnityEngine.CoreModule.dll</HintPath>
  </Reference>
  <Compile Remove="_stub/**/*.cs" />
</ItemGroup>

<!-- Water: use inline stub -->
<ItemGroup Condition="Exists('$(WorldBoxManaged)/UnityEngine.CoreModule.dll')">
  <Reference Include="UnityEngine.CoreModule">
    <HintPath>$(WorldBoxManaged)/UnityEngine.CoreModule.dll</HintPath>
    <Private>false</Private>
  </Reference>
</ItemGroup>
<ItemGroup Condition="!Exists('$(WorldBoxManaged)/UnityEngine.CoreModule.dll')">
  <Compile Include="src\UnityEngineStubs.cs" />
</ItemGroup>
```

**Rule**: The stub must cover the exact surface area the package uses. If a new Unity API is introduced, the stub must be updated in the same PR.

### 2.2 Directory.Build.props

The `WorldBoxManaged` MSBuild property is the shared build contract across all packages. It points to the WorldBox `Managed/` directory containing Unity engine assemblies.

```powershell
# Environment or Directory.Build.props
$env:WorldBoxManaged = "C:/Program Files (x86)/Steam/steamapps/common/worldbox/worldbox_Data/Managed"
```

Each `.csproj` declares:

```xml
<ItemGroup Condition="Exists('$(WorldBoxManaged)/UnityEngine.CoreModule.dll')">
  <Reference Include="UnityEngine.CoreModule">
    <HintPath>$(WorldBoxManaged)/UnityEngine.CoreModule.dll</HintPath>
  </Reference>
</ItemGroup>
```

---

## 3. Design Tokens

### 3.1 Naming Conventions

| Layer | Convention | Example |
|-------|-----------|---------|
| **Namespace** | `Phenotype.{Domain}` | `Phenotype.Water`, `Phenotype.Terrain`, `Phenotype.PostFx` |
| **Assembly** | `Phenotype.{Domain}` or `phenotype-{name}` | `Phenotype.Terrain`, `phenotype-water` |
| **Class** | PascalCase, domain suffix | `GerstnerWaveBank`, `ChunkMeshBuilder`, `PostStack` |
| **Interface** | PascalCase, `I` prefix | `IPostFxPass`, `ILutPipeline` |
| **Method** | PascalCase (C#) | `GetComponent<T>`, `SetFloat` |
| **Shader** | `Brp{Pass}.shader` | `BrpACES.shader`, `BrpBloom.shader` |
| **Asset file** | kebab-case | `phenotype-postfx-variants.shadervariants` |

### 3.2 Folder Structure Tokens

```
src/
  ├── <Domain>.csproj              # Library entry point
  ├── <Domain>/
  │   ├── Rendering/              # GPU-facing code
  │   ├── Materials/              # Material property definitions
  │   └── <CoreFeature>.cs        # Top-level component
  └── UnityEngineStubs.cs         # CI stub (optional)

tests/
  └── <Domain>.tests.csproj        # Test project

Runtime/                          # Unity-package style
  ├── <Domain>.asmdef             # Assembly definition
  ├── Shaders/                    # Shader assets
  ├── Ports/                      # Render-pipeline adapters
  └── <Pass>.cs                   # Pass implementations
```

---

## 4. Integration Patterns

### 4.1 Dependency Graph

```
                    phenotype-terrain
                          |
                    phenotype-water
                          |
                    phenotype-postfx
```

- `phenotype-water` references `phenotype-terrain` via `<ProjectReference>` for shared LOD and mesh infrastructure.
- `phenotype-postfx` is currently independent (no package references). It consumes the camera via `Camera.main` and `OnRenderImage`.

### 4.2 ProjectReference Contract

```xml
<!-- In consuming .csproj -->
<ProjectReference Include="../phenotype-terrain/phenotype-terrain.csproj" />
```

**Rules**:
1. Always reference the **library** project, never the test project.
2. Test projects are excluded from the library via `<Compile Remove="tests/**/*.cs" />`.
3. The `net48` / `$(WorldBoxManaged)` contract is stable — consuming mods must not break when a sibling updates.

### 4.3 Shader Variant Preservation

Post-processing packages ship a `*.shadervariants` asset. When included in an AssetBundle, it prevents Unity's shader stripper from removing passes that are not statically reachable.

```
Runtime/phenotype-postfx-variants.shadervariants  # Include in AssetBundle
```

---

## 5. Build and Test Conventions

### 5.1 Target Framework

| Target | Value | Notes |
|--------|-------|-------|
| `TargetFramework` | `net48` | Unity / Mono compatibility |
| `LangVersion` | `10.0` or `latest` | C# 10 nullable annotations preferred |
| `Nullable` | `annotations` | Partial nullable analysis for Unity interop |

### 5.2 Build Commands

```powershell
# Set the Unity engine path
$env:WorldBoxManaged = "C:/Program Files (x86)/Steam/steamapps/common/worldbox/worldbox_Data/Managed"

# Build library
dotnet build phenotype-terrain.csproj -c Release
dotnet build phenotype-water.csproj -c Release
dotnet build phenotype-postfx/package.json  # UPM-style

# Build without Unity (CI stub)
dotnet build phenotype-terrain.csproj -c Release   # Uses _stub/
dotnet build phenotype-water.csproj -c Release   # Uses UnityEngineStubs.cs
```

### 5.3 Test Conventions

| Package | Test Framework | Test Project | Notes |
|---------|---------------|--------------|-------|
| `phenotype-terrain` | xUnit | `tests/phenotype-terrain.tests.csproj` | TBD |
| `phenotype-water` | xUnit | `tests/phenotype-water.tests.csproj` | `xunit` 2.9.3, `xunit.runner.visualstudio` 2.8.2 |
| `phenotype-postfx` | NUnit | `tests/PostStackSourceTests/PostStackSourceTests.csproj` | `net9.0` for source tests |

```powershell
# Water tests
dotnet test tests/phenotype-water.tests.csproj -c Release

# PostFX tests (source compilation)
dotnet test tests/PostStackSourceTests/PostStackSourceTests.csproj
```

### 5.4 Quality Gate (Taskfile)

Every package provides a `Taskfile.yml` with the same task contract:

```bash
task build      # dotnet build <project>.csproj -c Release
task test       # dotnet test <tests>.csproj -c Release
task format     # dotnet format --verify-no-changes
task format-fix # dotnet format --verbosity diagnostic
task lint       # alias for format
task            # default: build + test + lint
```

### 5.5 CI / Health

- GitHub Actions CI badge in `README.md`.
- `STATUS.md` tracks build health and audit state.
- `.csproj` excludes `tests/` and `obj/` from compilation to prevent cross-contamination.

---

## 6. Contributing

1. Keep the `net48` / `$(WorldBoxManaged)` contract stable.
2. Update UnityEngine stubs when adding new Unity API surface.
3. Reference packages via `<ProjectReference>`, never by file glob.
4. Add tests for new pass logic; use the same test framework as the package.
5. Run `task` (build + test + lint) before opening a PR.

---

*Last updated: 2026-06-08*
