# Getting started

FocalPoint is a connector-first screen-time platform. To get useful value you need two things:

1. The iOS app installed on your device (with `com.apple.developer.family-controls` entitlement).
2. At least one connector authenticated (Canvas LMS is the only shipping connector today).

## Paths

- [Install on iOS](/getting-started/install-ios) — build from source, pair device, install the `.ipa`, grant FamilyControls permission.
- [First rule walkthrough](/getting-started/first-rule) — author, test, and deploy your first rule with Canvas.

## Prerequisites

- **iOS 16+** device (iPhone or iPad).
- **Xcode 15+** with a paid Apple Developer account.
- **FamilyControls entitlement** approved for your team id (Apple review is 1–4 weeks — submit early).
- **Rust 1.82+** and **Task** installed locally (see [`CONTRIBUTING.md`](https://github.com/KooshaPari/FocalPoint/blob/main/CONTRIBUTING.md)).

## No-dependencies evaluation

You can explore the rules engine and audit chain without a device:

```bash
task test
cargo run -p focus-cli -- rule evaluate examples/rules/assignment-focus.toml \
  --events examples/events/canvas-assignment-due.json
```

This runs the rule against a fixture event stream and prints the decision trace — no iOS hardware required.
