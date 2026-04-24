# FocalPoint App Icon Design

**Status:** Procedural placeholder (GA-ready)  
**Target:** App Store submission, iOS 17+

## Overview

FocalPoint uses a procedurally-generated Coachy-inspired flame icon for App Store submission. This document describes the design rationale, implementation, and replacement strategy for a human-designed icon.

## Design Rationale

The procedural flame icon is a **temporary-by-design** placeholder that:
- Requires **no designer involvement** for generation (Rust + pure pixel math)
- Renders **instantly** at all required sizes (1024×1024 down to 58×58)
- Uses **stable color tokens** from the design system
- Maintains **perfect pixel fidelity** across all scales (no anti-alias blur)
- Generates a **deterministic SHA-256 hash** for verification

### Why Not Hand-Designed Yet?

1. **Entitlements blocker:** FamilyControls approval from Apple is required before App Store submission. Until approval, icon design effort is premature.
2. **Fast iteration:** Procedural generation allows icon tweaks in 10 seconds (edit RGB values, re-render).
3. **Staging:** This placeholder proves the full pipeline (icon generation → XCAssets → fastlane → App Store).

## Implementation

### Generator Binary

**Location:** `crates/focus-icon-gen/`  
**Binary:** `focalpoint-icon-gen`

```bash
# Render all sizes to XCAssets directory
focalpoint-icon-gen

# Preview mode (PNG to stdout, hash to stderr)
focalpoint-icon-gen --preview

# Custom output directory
focalpoint-icon-gen --output-dir /custom/path
```

### Icon Generation Pipeline

1. **Color Tokens** (from design_system_tokens.md):
   - Flame primary: `#FF8C00` (orange)
   - Flame secondary: `#DC143C` (crimson red)
   - Background dark: `#1E1E28` (dark blue-black)
   - Background light: `#3C3C50` (lighter blue-gray)

2. **Algorithm**:
   - Background: vertical linear gradient (dark → light)
   - Flame: teardrop-shaped silhouette with tapered width
   - Flame gradient: orange at base → red at tip
   - All pixels computed using pure arithmetic (no vector rasterization)

3. **Output Sizes** (11 total):
   - `1024×1024` (App Store, macOS)
   - `512×512`, `256×256` (fallback)
   - `180×180`, `152×152`, `120×120`, `114×114` (iPhone)
   - `167×167`, `76×76` (iPad)
   - `80×80`, `58×58` (Spotlight/Settings)

4. **XCAssets Manifest**:
   - Generates `Contents.json` with idiom/scale/size metadata
   - Automatically registers all @1x/@2x/@3x variants
   - Integrates with Xcode project without manual editing

### Tests

**4 tests trace to FR-APPSTORE-001 through FR-APPSTORE-004:**

```bash
cargo test -p focus-icon-gen
```

- **FR-APPSTORE-001:** Icon hash remains stable across re-renders
- **FR-APPSTORE-002:** All 11 required sizes are generated
- **FR-APPSTORE-003:** `Contents.json` is valid JSON with required fields
- **FR-APPSTORE-004:** PNG output has correct signature and structure

## Replacing with Human-Designed Icon

When a designer creates a polished 1024×1024 PNG icon:

### Quick Replace (No Code Changes)

```bash
# Place 1024×1024 PNG at project root
cp designer-icon-1024x1024.png Resources/app-icon-override.png

# Regenerate all sizes
focalpoint-icon-gen --override
```

### Full Custom Implementation

If you need full control over icon generation:

1. Add to `crates/focus-icon-gen/src/lib.rs`:
   ```rust
   pub fn render_from_file(override_path: &Path) -> Result<Vec<u8>> {
       // Load PNG, downsample to all required sizes, write XCAssets
   }
   ```

2. Update `src/bin.rs`:
   ```rust
   if args.override {
       gen.render_from_file(args.input_file)?;
   }
   ```

3. Regenerate:
   ```bash
   focalpoint-icon-gen --override --input-file designer-icon.png
   ```

## Color Tokens

From `design_system_tokens.md`:

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| Flame Primary | `#FF8C00` | `255, 140, 0` | Flame base (orange) |
| Flame Secondary | `#DC143C` | `220, 20, 60` | Flame tip (red) |
| BG Dark | `#1E1E28` | `30, 30, 40` | Background top |
| BG Light | `#3C3C50` | `60, 60, 80` | Background bottom |

## Integration

- **Fastlane:** `:screenshots` lane calls `focalpoint-icon-gen` before launching simulator
- **CI:** Icon generation runs in `fastlane ci` (verify XCAssets consistency)
- **Xcode:** `Build Phases` > Add Run Script to auto-generate icons pre-build (optional)

## FAQ

**Q: Why procedural instead of SVG?**  
A: Procedural pixel math is simpler to maintain than SVG parsing/rendering. No external deps needed beyond `png` crate.

**Q: Can I tweak colors?**  
A: Edit `IconGenerator::default()` in `src/lib.rs`, save, recompile. Takes ~2s.

**Q: Does the icon change if I run it twice?**  
A: No. The icon hash is deterministic. Same input → identical pixel data → identical PNG.

**Q: What if I want to distribute a custom PNG in CI?**  
A: Commit the 1024 PNG to a `design-assets/` branch and load it in fastlane:
```ruby
icon_path = "design-assets/app-icon-1024.png"
sh("focalpoint-icon-gen --override --input-file #{icon_path}")
```

## Next Steps

1. **Entitlements approval:** Request FamilyControls from Apple (2-4 weeks)
2. **Design brief:** Once approved, brief designer on icon requirements
3. **Replace placeholder:** Update `src/lib.rs` with human-designed icon
4. **Fastlane integration:** Call `focalpoint-icon-gen` in `:deliver` lane (gated on entitlement approval)
