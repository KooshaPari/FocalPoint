# FocalPoint — AGENTS.md

## Project Overview

macOS Focus management / window management app for Mac.

## Stack

- Language: Objective-C (per GitHub language detection)
- Platform: macOS (AppKit)
- Build system: Xcode (verify locally)
- Note: GitHub language detection shows "Objective-C" — verify actual implementation language

## Key Commands

```bash
# Verify build system
ls -la *.xcodeproj *.xcworkspace Makefile Podfile 2>/dev/null

# If Xcode project
# xcodebuild -project FocalPoint.xcodeproj -scheme FocalPoint -configuration Release build

# If CocoaPods
# pod install && xcodebuild -workspace *.xcworkspace ...
```

## Notes

- **Active** — verify language (Objective-C vs Swift) and build system locally before running commands
- Legacy macOS app — may need Xcode version compatibility check
