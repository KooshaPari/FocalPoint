# iOS Build Status — Smoke Verify Post Toolchain Fix

**Date:** 2026-04-26
**Last toolchain fix:** commit `45dc356` — fix(ios): pin IPHONEOS_DEPLOYMENT_TARGET=15.0 + use rustup 1.93.0 for xcframework rebuild
**Verified by:** read-only smoke build from agent worktree (no source mutations)

## Artifact Freshness

| Artifact | Size | mtime |
|----------|------|-------|
| `Frameworks/FocusFFI.xcframework/Info.plist` | 1.3KB | 2026-04-26 01:36 |
| `Frameworks/FocusFFI.xcframework/ios-arm64/libfocus_ffi.a` | 72MB | 2026-04-26 01:35 |
| `Frameworks/FocusFFI.xcframework/ios-arm64_x86_64-simulator/libfocus_ffi.a` | 143MB (per session notes) | 2026-04-26 |

Xcframework artifacts are fresh from today's rebuild. Toolchain fix is applied on disk.

## Smoke Build Result

**Command:**
```
xcodebuild -project apps/ios/FocalPoint/FocalPoint.xcodeproj \
  -scheme FocalPointApp -configuration Debug \
  -sdk iphonesimulator -destination 'generic/platform=iOS Simulator' \
  CODE_SIGNING_ALLOWED=NO build
```

**Result:** BUILD FAILED — but **not** due to the toolchain. The xcframework links cleanly; failure is Swift-source binding drift.

## Errors (6 total, all in target `FocalPointCore`)

| # | File | Line | Error |
|---|------|------|-------|
| 1 | `focus_ffi.swift` | 4500 | `'MonthlyRetroDto' is ambiguous for type lookup` |
| 2 | `focus_ffi.swift` | 4500 | (same — second occurrence) |
| 3 | `focus_ffi.swift` | 6692 | `'WeeklyReviewDto' is ambiguous for type lookup` |
| 4 | `focus_ffi.swift` | 6692 | (same — second occurrence) |
| 5 | `FocalPointCore.swift` | 29 | `invalid redeclaration of 'WeeklyReviewDto'` |
| 6 | `FocalPointCore.swift` | 73 | `invalid redeclaration of 'MonthlyRetroDto'` |

## Root Cause

UniFFI now emits `WeeklyReviewDto` and `MonthlyRetroDto` from the regenerated bindings (`focus_ffi.swift`, last regen via today's xcframework build). The hand-written `FocalPointCore.swift` (last touched 2026-04-24, commit `7e53d8d`) still carries placeholder definitions of those same structs. They collide.

The header comment in `FocalPointCore.swift` already acknowledges this pattern:
> "UniFFI-generated bindings (focus_ffi.swift) now provide the real `FocalPointCore` class. Keep Swift-only placeholder types (RuleId, ActiveRule) that the UI layer already references; they are not in the UDL."

The two DTOs are now in the UDL, so the placeholders must be removed. This is post-toolchain-fix drift, not the previous (resolved) toolchain breakage.

## Expected vs Unexpected

- **Expected:** Toolchain fix is good. Linker resolves `libfocus_ffi.a`. Other targets (DesignSystem, Mascot, etc.) compile fine.
- **Unexpected (new):** Two stale Swift-side placeholder structs need to be deleted from `FocalPointCore.swift` (lines 29-72 for `WeeklyReviewDto`, lines 73-~end for `MonthlyRetroDto`).

## What The User Can Do In Xcode Now

1. **Open Xcode.** The xcframework integration IS fixed — Xcode will no longer fail at the FFI link stage.
2. **You will hit 2 Swift compile errors** in `apps/ios/FocalPoint/Sources/FocalPointCore/FocalPointCore.swift`:
   - Line 29: redeclaration of `WeeklyReviewDto`
   - Line 73: redeclaration of `MonthlyRetroDto`
3. **Fix:** Delete those two struct definitions from `FocalPointCore.swift`. The UniFFI-generated versions in `focus_ffi.swift` are now authoritative. Keep the other placeholders (`RuleId`, `ActiveRule`) — they remain UI-only and not in the UDL.
4. After deletion: rebuild. Expected result: clean build, ready to run on simulator.

## Code Signing

This smoke build used `CODE_SIGNING_ALLOWED=NO`. User's interactive Xcode runs will sign normally with their dev profile — no concern.

## Disk

DerivedData growth during this verify: ~1GB (within budget; pre-build free 36GiB).
