# FocalPointAppSnapshotTests

Swift snapshot testing for FocalPoint UI components.

## Overview

This target contains visual regression tests using PointFree's `swift-snapshot-testing` framework. Each snapshot test captures a baseline image of a UI component and detects changes on subsequent test runs.

## Running Tests

### Record Baselines (First Run)

On first run, set the `RECORD` environment variable to generate baseline snapshots:

```bash
RECORD=true xcodebuild test \
  -scheme FocalPointAppSnapshotTests \
  -workspace FocalPoint.xcworkspace \
  -configuration Debug \
  -destination 'generic/platform=iOS Simulator,name=iPhone 15 Pro'
```

Or via fastlane:
```bash
RECORD=true fastlane snapshot
```

### Compare Against Baselines (Subsequent Runs)

```bash
xcodebuild test \
  -scheme FocalPointAppSnapshotTests \
  -workspace FocalPoint.xcworkspace \
  -configuration Debug \
  -destination 'generic/platform=iOS Simulator,name=iPhone 15 Pro'
```

Or via fastlane:
```bash
fastlane snapshot
```

## Test Targets

| Test File | Coverage | FRs Traced |
|-----------|----------|-----------|
| `DesignSystemSnapshotTests.swift` | Color palette, typography, component variants | FR-UI-001 |
| `MascotUISnapshotTests.swift` | Coachy mascot states (idle, engaged, celebrating, warning) | FR-MASCOT-001 |
| `OnboardingSnapshotTests.swift` | Welcome, permissions, connector setup, first rule, completion | FR-ONBOARD-001 |
| `EnforcementSnapshotTests.swift` | Rule list, creation form, focus block, penalty display | FR-ENFORCE-001 |
| `CoreTabsSnapshotTests.swift` | 7 tab views: Home, Today, Focus Mode, Tasks, Wallet, Stats, Settings | FR-TAB-001 |

**Total baseline snapshots:** 23+ images recorded in `__Snapshots__/`

## Baseline Images

All baseline snapshots are stored in `__Snapshots__/` subdirectory:

```
__Snapshots__/
  DesignSystemSnapshotTests.testPaletteColorsLight.1.png
  DesignSystemSnapshotTests.testPaletteColorsDark.1.png
  DesignSystemSnapshotTests.testTypographyScale.1.png
  ... (and more)
```

**Important:** Commit these PNG files to git. CI will compare test runs against them.

## Workflow

### Intentional Visual Changes

When you intentionally change the UI (design system, layout, colors), you need to re-record snapshots:

1. Make your UI changes
2. Run tests with `RECORD=true` to generate new baselines
3. Review the diff in `__Snapshots__/`
4. Commit the updated PNG files to git

### Unintentional Regressions

If a test fails because a snapshot doesn't match, review the diff:

1. Xcode shows you: "Expected [old image] vs Actual [new image]"
2. Either fix the code (revert the regression) or intentionally update the baseline

## Test Helpers

### SnapshotTestHelpers.swift

- `MockCoreHolder` — Mock FFI core for deterministic snapshots
- `assertViewSnapshot()` — Assertion helper with iPhone 13 Pro preset
- Device configs: iPhone 13 Pro (light/dark)

## Device Configuration

All tests use **iPhone 13 Pro** (390x844) at regular (1x) scale:
- Safe area: top 47pt, bottom 34pt
- Light and dark modes supported

To test other devices, extend `ViewImageConfig` in `SnapshotTestHelpers.swift`:

```swift
static let iPhoneSE = ViewImageConfig(
    size: .init(width: 375, height: 667),
    safeAreaInsets: .init(top: 20, left: 0, bottom: 0, right: 0),
    traits: .init(userInterfaceStyle: .light)
)
```

## Determinism

All snapshots are **deterministic** — same test input always produces the same output:

- No random data
- No timestamps
- No live network calls
- Mock data is fixed (e.g., 3 rules, balance of 4.5 hours)

This ensures CI and local runs produce identical results.

## Troubleshooting

### "Snapshot not found"

**Cause:** You haven't run the test with `RECORD=true` yet.

**Fix:**
```bash
RECORD=true fastlane snapshot
```

### "Snapshot mismatch (diff shown in Xcode)"

**Cause:** UI changed, but snapshot baseline is outdated.

**Options:**
1. Revert the UI change (if unintended regression)
2. Update the baseline: `RECORD=true fastlane snapshot`

### "Test hangs / timeout"

**Cause:** Simulator is not responsive.

**Fix:**
```bash
killall "Simulator"
xcodebuild test -scheme FocalPointAppSnapshotTests ...
```

## Performance Notes

- First run (record): ~30 sec (generates 23+ images)
- Subsequent runs (compare): ~20 sec
- Snapshots are PNGs (lossless); git stores them efficiently

## Integration with CI

Add to your GitHub Actions workflow:

```yaml
- name: Snapshot tests
  run: |
    fastlane snapshot
```

GitHub will auto-detect PNG diffs in pull requests.

## References

- [PointFree swift-snapshot-testing](https://github.com/pointfreeco/swift-snapshot-testing)
- [fastlane snapshot command](https://docs.fastlane.tools/actions/snapshot/)
