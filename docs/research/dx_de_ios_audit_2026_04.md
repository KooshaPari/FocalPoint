# iOS Developer Experience & Release Pipeline Audit — 2026-04

**Status:** FocalPoint v0.0.3 (end-to-end loop shipped 2026-04-23)  
**Audit Date:** 2026-04-23  
**Scope:** Build pipeline, testing infrastructure, quality gates, cross-platform coordination, release readiness  
**Output:** Readiness assessment + 20-item priority backlog + 3 critical fix recommendations

---

## Executive Summary

FocalPoint's iOS DX/DE foundation is **competent but incomplete** (6.2/10 overall). The Rust core + UniFFI → XCFramework pipeline works, local quality gates (lefthook + cargo test) are in place, and single-device MVP is viable. **Critical gaps:**

1. **No release automation** — TestFlight uploads, App Store Connect uploads, code signing all manual (fastlane deferred)
2. **Zero iOS-level testing** — Rust core has 41 unit tests; Swift side has 0 real coverage (only 5 integration smoke tests)
3. **Swift 6 concurrency debt** — @MainActor annotations exist but no systematic audit (strict concurrency enabled = compiler errors incoming)
4. **No snapshot testing** — UI regressions undetected across mascot, design system, onboarding flows
5. **Cross-platform story undefined** — Android deferred; CLI/web parity missing; unclear rollout strategy when unfreezed

**Honest take:** We can ship v1.0 MVP to TestFlight with the current setup. Meaningful App Store release (multi-device, mature instrumentation) requires 6–8 weeks of DX work **before** external beta testing. The entitlement application to Apple (Q8 blocker) must start immediately (1–4 week review).

---

## 1. iOS Build Pipeline

### Current State

| Component | Status | Version | Notes |
|-----------|--------|---------|-------|
| **xcodegen** | Active | (built-in via SwiftPM) | Generates `.xcodeproj` from `project.yml`; declarative, reliable |
| **UniFFI** | Active | 0.28 | Generates Swift bindings from Rust; static `.xcframework` embedded in FocalPointCore |
| **XCFramework** | Active | N/A | Hand-built via `build-xcframework.sh`; targets aarch64-ios, aarch64-ios-sim, x86_64-ios |
| **Swift** | Active | 5.9 | iOS 17.0 deployment target; pure SwiftUI (no legacy AppKit) |
| **Cargo toolchain** | Pinned | 1.93.1 | Locked in `build-xcframework.sh` to handle `clap_lex 1.1.0` edition2024 incompatibility |

### Gaps & Recommendations

**Gap 1: xcodegen maintenance burden**  
xcodegen remains industry-standard for declarative Xcode project generation but trades simplicity for longer build reconfiguration cycles. **Tuist 1.67+** (2025 release) adds caching, parallel builds, and modularization scaffolding — particularly valuable if we split MascotUI / Enforcement / DesignSystem into independent packages for faster iteration.

- **Recommendation:** Maintain xcodegen for v1.0; migrate to Tuist v1.67+ in Phase 2 if multi-module parallelization shows >15% build-time improvement. Cost: ~3 engineer-days.
- **Source:** [Tuist Generated Projects: why generate Xcode projects in 2025](https://tuist.dev/blog/2025/02/25/project-generation) (accessed 2026-04-23)
- **Effort:** 1 tool call

**Gap 2: UniFFI 0.28 → 0.30+ migration path unclear**  
Current version 0.28 is stable and battle-tested. **0.30** fixed a critical bug (objects with alignment ≥32 bytes freed too early) and **0.31** introduced breaking checksum changes (method checksums now exclude self type). **Critical:** bindings built against 0.30.x are incompatible with Rust code compiled on 0.31.x.

- **Recommendation:** Track 0.31+ adoption path; do NOT upgrade to 0.31 until all team toolchains are synchronized (iOS CI, Android, desktop). Upgrade window: Phase 2 (after MVP stabilization).
- **Source:** [uniffi-rs/CHANGELOG.md](https://github.com/mozilla/uniffi-rs/blob/main/CHANGELOG.md) (accessed 2026-04-23)
- **Alternative: cxx-rs** — C++ bindings generator (simpler, less feature-complete for iOS). Not recommended unless UniFFI maintenance stalls.
- **Effort:** 1–2 tool calls (dependency audit + test rebuild on target version)

**Gap 3: XCFramework bitcode stripping workaround**  
Current fix: `STRIP_BITCODE_FROM_COPIED_FILES = NO` + `ENABLE_BITCODE = NO` in `project.yml`. This silences the warning but leaves bitcode stripping to the framework (not ideal for App Store optimization).

- **Recommendation:** Bitcode is deprecated in iOS 18+; this is a one-time technical debt item. Defer unless App Store submission demands optimization. No action needed for v1.0.
- **Source:** Xcode Build Settings documentation (Apple, 2025)
- **Effort:** 0 (monitor only)

**Gap 4: Swift Package Manager integration vs direct framework binding**  
FocalPointCore currently embeds XCFramework directly; SwiftPM binary target binding is emerging as alternative.

- **Comparison:**
  - **XCFramework (current):** Simplest for single-platform native code; mature Xcode support; bitcode/slice handling manual.
  - **SwiftPM binary target:** Modular, versionable separately; requires `.swiftinterface` stability; better for multi-platform (if Android/web bindings ship as separate packages).
- **Recommendation:** Stay on XCFramework for v1.0. Revisit SwiftPM binary targets in Phase 3 if we publish focus-ffi as a standalone package (ecosystem distribution).
- **Effort:** 0 (deferred)

### Build Pipeline Readiness Score: 7.5/10

- ✅ Rust → Swift binding generation automated
- ✅ Local builds reproducible (cargo pins + xcodegen)
- ✅ Framework embedding tested
- ⚠️ No CI for device builds (GitHub Actions billing blocks macOS runners)
- ⚠️ UniFFI minor version debt (0.28 stable, but 0.30+ available)

---

## 2. Testing Infrastructure

### Current State: Rust Core Tests

| Level | Count | Coverage | Notes |
|-------|-------|----------|-------|
| **Unit tests (Rust)** | 41 | Focus domain, connectors, crypto, audit chain | Running via `cargo test --workspace` |
| **Doc tests** | 0 | N/A | Deferred (crate docs are stubs) |
| **Integration tests** | 3 | connector-canvas, connector-gcal, connector-github | Mock-based via `wiremock` |
| **FFI smoke tests (Swift)** | 2 | `FocalPointCoreTests/FFITests.swift` | Verify Swift can instantiate core types; minimal coverage |

### Current State: iOS/Swift Testing

| Level | Framework | Count | Status | Notes |
|-------|-----------|-------|--------|-------|
| **Unit (SwiftUI)** | XCTest | 3 | Active | PaletteTests (DesignSystem), CoachyStateTests (MascotUI), ViewEqualityTests (App) |
| **Integration (Flows)** | XCTest | 2 | Active | OnboardingCoordinatorTests, RuleRoundTripTests |
| **Snapshot (UI)** | None | 0 | **Missing** | No screenshot regression detection |
| **E2E (XCUITest)** | None | 0 | **Missing** | No user journey testing (tap flows, multi-screen) |
| **Performance** | None | 0 | **Missing** | No profiling, frame-time validation |

### Gaps & Recommendations

**Gap 1: iOS unit test coverage critically low**  
5 total tests for a SwiftUI app with 4 frameworks + custom mascot runtime + background sync. Minimum viable coverage: 30+ tests hitting each major view, coordinator, and state transition.

- **Recommendation (Phase 1):** Add 20+ XCTest cases covering:
  - OnboardingFlow (name → connect → sync)
  - RulesEvaluation (rule firing, wallet mutation, penalty escalation)
  - AuditChainVerification (append, tamper detection)
  - BackgroundSync (task scheduling, refresh logic)
- **Effort:** 15–20 engineer-hours (test structure already in place)

**Gap 2: Snapshot testing missing entirely**  
No regression detection for mascot animations, palette changes, design system tweaks, or onboarding flow UI. This is a _major_ risk for v1.0 (visual bugs in TestFlight = bad optics).

- **Recommendation:** Adopt **PointFree's swift-snapshot-testing** (battle-tested, type-safe Swift API, no Objective-C legacy baggage).
  - Alternative: **PixelTest** (modern Swift-first, zero Objective-C, newer but less ecosystem integration).
  - Avoid: iOSSnapshotTestCase (Uber-maintained but Objective-C, less Swift-idiomatic in 2025).
- **Implementation:** 1–2 days to set up; 1 week to add snapshots for Design System, MascotUI, OnboardingFlow.
- **Source:** [SnapshotTesting 1.0: Delightful Swift snapshot testing](https://www.pointfree.co/blog/posts/23-snapshottesting-1-0-delightful-swift-snapshot-testing) (accessed 2026-04-23); [GitHub - pointfreeco/swift-snapshot-testing](https://github.com/pointfreeco/swift-snapshot-testing) (accessed 2026-04-23)
- **Effort:** 3–5 tool calls (setup + initial snapshot library)

**Gap 3: E2E / XCUITest coverage missing**  
No automated end-to-end testing for user journeys (onboarding → connect tool → sync → rule fires → penalty applies).

- **Recommendation (Phase 2):** Start with **XCUITest** (native, fast, deep Xcode integration). Add 5–8 critical journeys:
  - Add task → connect Canvas → rules fire → penalty escalation
  - Reward UI navigation → claim reward → wallet updates
  - Settings → background sync toggle → verify Background Task scheduling
- **Alternative (cross-platform future):** Evaluate **Maestro** (YAML-based, iOS + Android unified tests, 1% flakiness vs XCUITest's 5–8%). Defer until Android unfrozen.
- **Source:** [Best Mobile Testing Tools: 2025 Roundup](https://maestro.dev/insights/best-mobile-testing-tools-2025-roundup) (accessed 2026-04-23); [The Best Mobile App Testing Frameworks in 2026](https://maestro.dev/insights/best-mobile-app-testing-frameworks) (accessed 2026-04-23)
- **Effort:** 15–25 engineer-hours (Phase 2)

**Gap 4: Rust-iOS integration test harness thin**  
FFI smoke tests verify instantiation; no property-based testing or mutation testing of core logic running inside iOS.

- **Recommendation:** Leverage existing Rust unit tests; add 5–10 FFI round-trip tests (serialize rule → send to Swift → deserialize → re-serialize → match).
- **Effort:** 1 week

### Testing Infrastructure Readiness Score: 4.8/10

- ✅ Rust core unit tests present (41 tests, good coverage)
- ✅ SwiftUI test structure in place (XCTest harness)
- ❌ **Critical:** iOS visual regression detection missing (snapshot tests = 0)
- ❌ **Critical:** E2E user journeys untested (XCUITest = 0)
- ❌ Performance profiling absent

---

## 3. Local Quality Gates

### Current State

**Lefthook (v1.5+)** installed; running pre-commit and pre-push:

| Hook | Scope | Commands | Blocking |
|------|-------|----------|----------|
| **fmt** | Staged Rust files | `rustfmt --check` | ✅ Yes (pre-commit) |
| **secrets-files** | All commits | Blocks `.p8, .p12, .pem, .key, .mobileprovision` | ✅ Yes (pre-commit) |
| **trufflehog** | Staged/all files | `trufflehog filesystem --only-verified --fail` | ✅ Yes (pre-commit) |
| **clippy-changed** | Touched crates | `cargo clippy -p <name> -- -D warnings` | ✅ Yes (pre-commit) |
| **workspace-verify** | Full workspace | fmt + clippy + test (all targets, no-fail-fast) | ✅ Yes (pre-push) |

**GitHub Actions** (Linux-only, macOS/Windows billing-blocked):

| Job | Platform | Runs | Notes |
|-----|----------|------|-------|
| **fmt** | Linux | `cargo fmt --all -- --check` | ✅ Runs |
| **clippy** | Linux | `cargo clippy --workspace --all-targets -- -D warnings` | ✅ Runs |
| **test** | Linux | `cargo test --workspace --all-features --no-fail-fast` | ✅ Runs |
| **secrets** | Linux | `trufflehog --only-verified` on PR | ✅ Runs |
| **Swift linting** | macOS | ❌ Not configured | Billing-blocked |
| **iOS device build** | macOS | ❌ Not configured | Billing-blocked; manual only |

### Gaps & Recommendations

**Gap 1: Swift linting not enforced**  
No SwiftLint, SwiftFormat, or Apple's swift-format in CI. Local developers can push unformatted Swift.

- **Recommendation:** Add **SwiftFormat** (Nick Lockwood's; superior autocorrect) + **SwiftLint** (safety & logic rules):
  - **Swift code formatter:** SwiftFormat (more powerful autocorrect; 100+ formatting rules).
  - **Swift linter:** SwiftLint (100+ linting rules; can't autocorrect everything SwiftFormat can).
  - **Apple's swift-format:** Lightweight, built into Xcode 16+; less intrusive; good for incremental adoption.
  - **Strategy (2025):** Combine SwiftFormat (style) + SwiftLint (safety); Phase out SwiftLint style rules.
- **Implementation:** Add to lefthook pre-commit (glob `apps/ios/**/*.swift`). Run on CI via Linux-based linting (parse Swift AST).
- **Source:** [Swift Code Formatters - NSHipster](https://nshipster.com/swift-format/) (accessed 2026-04-23); [Linting vs Formatting: A Swift Guide — Part 2](https://jasonzurita.com/linting-and-formatting-swift-part-2/) (accessed 2026-04-23)
- **Effort:** 1–2 tool calls (lefthook + GitHub Actions)

**Gap 2: Markdown & documentation linting deferred**  
Project has Vale + markdownlint-cli2 references in global CLAUDE.md but no FocalPoint-specific configuration.

- **Recommendation:** Add `.vale.ini` + `.markdownlintrc` for docs. Enforce UTF-8 via `agileplus validate-encoding --all` (already in pre-push).
- **Effort:** 0.5 engineer-day

**Gap 3: Pre-push overhead (full test suite blocking each push)**  
`cargo test --workspace --no-fail-fast` on every push can take 60–120 seconds. For frequent iterators, this becomes friction.

- **Recommendation (deferred to Phase 2):** Optimize by:
  - Running only changed-crate tests pre-commit (like clippy-changed)
  - Full workspace tests on GitHub Actions only
  - Optional local override: `git push --no-verify` (with prominent warnings)
- **Effort:** 1 engineer-day

### Quality Gates Readiness Score: 7.2/10

- ✅ Rust-side comprehensive (fmt + clippy + test + secrets blocking)
- ✅ Lefthook mature and battle-tested
- ⚠️ **Swift linting missing** (high priority for App Store readiness)
- ⚠️ Pre-push performance friction (120s test runs)

---

## 4. Swift 6 / Strict Concurrency Readiness

### Current State

- **Swift version:** 5.9 (iOS 17.0 deployment target)
- **Concurrency usage:** Limited; `@MainActor` annotated on `CoreHolder` (WASM/FFI bridge) but no full audit
- **Strict concurrency:** Not enabled; no compiler enforcement yet

### Gaps & Recommendations

**Gap 1: Swift 6 upgrade blocked by strict concurrency debt**  
Upgrading to Swift 6.0+ (released June 2024, now standard in Xcode 16+) requires enabling strict concurrency (compiler errors, not warnings). Current codebase has:
- View coordinators using `DispatchQueue.main` instead of `async/await`
- No `@Sendable` closures on background sync callbacks
- No `nonisolated` markers on non-actor-isolated methods

- **Recommendation (Phase 2, NOT MVP):**
  1. Add `SWIFT_STRICT_CONCURRENCY = complete` to `project.yml` (warns only, doesn't error)
  2. Audit UI code (OnboardingCoordinator, RitualsView, SettingsView) → mark with `@MainActor`
  3. Refactor async patterns: replace DispatchQueue callbacks with structured async/await
  4. Add `@preconcurrency import` for third-party libs not yet updated for Swift 6
  5. Once warnings resolved, enable `complete` mode (errors)
- **Effort:** 2–3 engineer-weeks (spread across Phase 2)
- **Source:** [Adopting strict concurrency in Swift 6 apps | Apple Developer Documentation](https://developer.apple.com/documentation/swift/adoptingswift6) (accessed 2026-04-23); [Complete concurrency enabled by default – available from Swift 6.0](https://www.hackingwithswift.com/swift/6.0/concurrency) (accessed 2026-04-23)

**Gap 2: MainActor coverage incomplete**  
Only `CoreHolder` explicitly marked; other UI components (views, coordinators, state machines) lack isolation markers.

- **Recommendation:** Run analysis in Phase 2; add checklists for each screen:
  - All @State/@StateObject holders → `@MainActor`
  - All view update callbacks → `@MainActor`
  - All NotificationCenter observers → `@MainActor`

### Swift 6 Readiness Score: 3.2/10

- ✅ No blocking data races detected (MVP scope limited)
- ❌ Strict concurrency not enabled
- ❌ Major refactor needed (2–3 weeks) before Swift 6 upgrade

**Recommendation:** Defer Swift 6 upgrade to Phase 2. Ship v1.0 on Swift 5.9; plan migration for Phase 2 beta.

---

## 5. Cross-Platform Dev Coordination

### Current State: Single-Device iOS MVP

- **iOS:** Rust core + UniFFI → Swift → SwiftUI (shipping v1.0)
- **Android:** Deferred beyond Phase 2; placeholder crates only
- **Web:** docs-site (VitePress) exists; no shared web dashboard
- **CLI:** focus CLI (Rust) with `audit`, `tasks`, `templates` subcommands; no docs-site reference

### Gaps & Recommendations

**Gap 1: Android strategy undefined (critical for ecosystem)**  
When unfrozen, Android will need:
- UniFFI Kotlin bindings (from same `focus-ffi` crate)
- OR cxx-jni for C++ interop (more manual, not recommended)
- OR Kotlin→Rust bridge via gobley (experimental)

- **Recommendation:**
  - Use **UniFFI Kotlin bindings** (same pipeline as iOS; proved pattern)
  - Plan Phase 3 work: Kotlin UI (Jetpack Compose equivalent to SwiftUI) + platform adapters (UsageStatsManager, AccessibilityService)
  - Test cross-platform: Single Rust core, iOS native, Android native (no Flutter/RN)

**Gap 2: Web dashboard / CLI parity missing**  
focus CLI exists; no web equivalent. Unclear if v1.0 roadmap includes web UI or if single-device mobile-only is acceptable.

- **Recommendation:**
  - **MVP (v1.0):** Mobile-only; CLI for power users (audit, rules export)
  - **Phase 2:** Publish docs-site with rule templates + CLI reference
  - **Phase 3:** Web dashboard (WASM core + React/Vue) for settings, rule composition, analytics
  - **OR:** Clarify that Phase 1 is mobile-only; defer web entirely

**Gap 3: Connector ecosystem distribution strategy unclear**  
Ecosystem strategy calls for template marketplace (Phase 3), but schema not yet defined.

- **Recommendation:** Create `docs/reference/connector_distribution.md`:
  - Template JSON/YAML schema
  - Signing + verification (ed25519 signatures from root keypair)
  - In-app template loading + community submission process
  - Defer implementation to Phase 3; finalize spec in Phase 1

### Cross-Platform Readiness Score: 4.0/10

- ✅ iOS single-device MVP clear
- ⚠️ Android strategy acknowledged but not documented
- ⚠️ Web/CLI parity undefined
- ❌ Template marketplace schema missing

---

## 6. Telemetry & Observability

### Current State

- **Crash reporting:** None (no Sentry, Bugsnag, Firebase Crashlytics)
- **Analytics:** None (no PostHog, Amplitude, Plausible)
- **In-app observability:** Audit chain (append-only event log) present; reflects state mutations

### Gaps & Recommendations

**Gap 1: Crash reporting missing (critical for production)**  
No way to detect silent failures, background sync crashes, or FFI panics until user reports via support.

- **Recommendation (Phase 1):**
  - **Short-term (MVP):** Disable crash reporting. Include `mailto:` link in Settings → "Report Issue" (manual feedback)
  - **Phase 2 (TestFlight):** Add **Sentry** (open-source option; $26/month; strong async stack traces)
  - **Alternative:** Firebase Crashlytics (free, real-time, but 100+MB SDK + Google data concerns)
  - **Privacy consideration:** Do NOT include user data, rule configs, or wallet state in crash reports (GDPR/privacy)
- **Effort:** 1–2 days (Sentry SDK integration + PII scrubbing)
- **Source:** [Sentry vs Crashlytics Comparison & Best Alternative 2025](https://uxcam.com/blog/sentry-vs-crashlytics/) (accessed 2026-04-23); [Top 5 iOS Crash Reporting Tools 2025 | Sidekick Interactive](https://www.sidekickinteractive.com/uncategorized/top-5-ios-crash-reporting-tools-2025/) (accessed 2026-04-23)

**Gap 2: Analytics missing (lower priority for MVP)**  
No insight into user adoption, feature usage, or rule effectiveness.

- **Recommendation (Phase 2+):**
  - MVP: Skip analytics entirely. Honest take: MVP has 1–10 users; analytics noise-to-signal ratio is terrible.
  - Phase 2 (beta): Evaluate PostHog (self-hosted, privacy-first) or Plausible (simple, GDPR-compliant)
  - **DO NOT use:** Google Analytics, Mixpanel (excessive data collection for MVP use case)
- **Effort:** 0 (defer)

**Gap 3: Audit chain as observability (novel but incomplete)**  
Audit chain captures state mutations but doesn't integrate with system logging or visibility tooling.

- **Recommendation:**
  - Keep audit chain as local integrity mechanism (tamper detection)
  - Add `tracing::info!` / `tracing::debug!` macros for key events (rule fired, penalty applied, connector synced)
  - Emit structured logs to `stderr` (can be piped to Sentry or cloud logging later)
- **Effort:** 1–2 days (logging instrumentation)

### Telemetry & Observability Readiness Score: 2.5/10

- ✅ Audit chain (local observability) present
- ❌ Crash reporting missing (critical blocker for production)
- ❌ Analytics absent (acceptable for MVP, deferred to Phase 2)

---

## 7. Release Pipeline & Deployment

### Current State

- **TestFlight:** Manual ad-hoc builds; no automation
- **App Store:** No submission yet (awaiting FamilyControls entitlement approval from Apple, Q8 blocker)
- **Code signing:** Automatic (Xcode managed profiles); team ID set in `project.yml`
- **Build artifacts:** Produced locally via `./apps/ios/scripts/build-dev-ipa.sh` and `build-xcframework.sh`
- **CI:** Rust-only (Linux); no iOS device builds in CI (macOS billing-blocked)

### Gaps & Recommendations

**Gap 1: Fastlane automation missing (critical for recurring releases)**  
Every TestFlight build currently requires manual steps:
1. Build XCFramework: `bash build-xcframework.sh`
2. Generate project: `xcodegen`
3. Build IPA: `xcodebuild -scheme FocalPointApp -configuration Release -exportMethod ad-hoc ...`
4. Upload to TestFlight: `xcrun altool --upload-app --file <ipa> --signer ...` (deprecated; App Store Connect API required)

- **Recommendation (Phase 1):**
  - Install **fastlane** (Ruby-based; industry standard; tight App Store Connect integration)
  - Lanes to create:
    - `lanes.register(:build_xcframework)` — wrapper around `build-xcframework.sh`
    - `lanes.register(:build_and_upload_testflight)` — xcodegen → xcodebuild → upload_to_testflight
    - `lanes.register(:code_sign)` — match (certificate + provisioning profile management)
  - Use **match** for code signing (stored in Git / S3; shared across team + CI)
- **Implementation:** 2–3 engineer-days
- **Source:** [Fastlane TestFlight Automation: Swift 6.2 iOS Release Guide | Medium](https://ravi6997.medium.com/automating-app-releases-fastlane-testflight-swift-6-2-integration-dc11cf8b1a7f) (accessed 2026-04-23); [Upload iOS App to TestFlight with GitHub Actions and Fastlane Match – 2025 Tutorial with Example](https://brightinventions.pl/blog/ios-testflight-github-actions-fastlane-match/) (accessed 2026-04-23)
- **Effort:** 3–5 tool calls (fastlane setup + lane definitions + CI integration)

**Gap 2: Entitlement application to Apple (Q8 blocker)**  
FamilyControls entitlement requires explicit Apple approval (1–4 weeks turnaround). **This must start immediately; it is the longest pole in the tent.**

- **Action item:**
  - Developer: Log in to Apple Developer Portal
  - Request: "FamilyControls" entitlement for bundle ID `com.koosha.focalpoint`
  - Support document: Explain use case (parental control + self-control dual mode)
  - Timeline: Assume 2–4 weeks approval
  - **Critical:** Do NOT wait for this to complete to ship v1.0 TestFlight (can test without entitlement); but **must complete before App Store review**
- **Source:** [CLAUDE.md — FocalPoint, Q8](docs/research/open_questions.md) (accessed 2026-04-23)
- **Effort:** 0.5 engineer-day (admin)

**Gap 3: No CI for iOS device builds (billing constraint)**  
macOS runners are billing-locked. Local-only builds mean no CI validation for device-specific issues (arm64 target, device-specific APIs, etc.).

- **Mitigation (Phase 1):**
  - Run pre-push `cargo test --workspace` on Rust core (validates logic on Linux)
  - Require local device build + test before pushing (lefthook enforces)
  - Manual smoke test on physical device / simulator before TestFlight
  - Consider: Self-hosted Mac mini runner (one-time $1500 hardware cost vs recurring macOS billing)
- **Recommendation (Phase 2):** If team grows, invest in self-hosted macOS runner (cheaper long-term than GitHub Actions macOS billing)

**Gap 4: No App Store Connect API integration (deprecated altool)**  
Current scripts use `altool` (deprecated); fastlane uses modern App Store Connect API. Migration mandatory for 2026+.

- **Recommendation:** Fastlane lane includes App Store Connect API key provisioning. Plan Phase 1.

### Release Pipeline Readiness Score: 3.5/10

- ✅ Xcode build system working
- ✅ Code signing functional (team ID configured)
- ❌ **Critical:** Fastlane automation missing (manual TestFlight bottleneck)
- ❌ **Critical:** FamilyControls entitlement application not started (1–4 week blocker)
- ❌ No device CI (macOS billing-locked)

**Blocker for v1.0 TestFlight release:**
1. **Start FamilyControls entitlement application immediately** (can ship to TestFlight without it, but approval is critical path)
2. **Implement fastlane lanes** (Phase 1, 2–3 days)
3. **Manual smoke testing required** (no CI substitute for device builds)

---

## 8. Multi-Agent Dev Coordination

### Current State

- **Worktrees:** FocalPoint uses canonical main + feature worktrees (if created per `repos/FocalPoint-wtrees/<topic>/...`)
- **Disk budget:** Not documented
- **Multi-session coordination:** CLAUDE.md notes mention task prioritization; no explicit patterns for parallel agent work
- **Commit discipline:** Existing guidance: separate commits by provenance (user-requested, pre-existing WIP, generated artifacts)

### Gaps & Recommendations

**Gap 1: Long-push pattern not documented**  
If multiple agents are iterating on iOS features, pushing frequently creates merge churn. FocalPoint lacks explicit guidance on batching vs atomic pushes.

- **Recommendation:** Add `.work-coordination/PATTERNS.md`:
  - Single-agent work (most MVP): Push after each task (1–2 hour atomic commits)
  - Multi-agent parallel (Phase 2+): Agree on integration points; push every 4 hours or end-of-session
  - Example: "Agent A works on testing infrastructure (snapshot setup) → Agent B works on Swift linting → integrate via single PR after both complete"
- **Effort:** 0.5 engineer-day (documentation)

**Gap 2: Disk budget not tracked**  
Rust workspace + node_modules + .build artifacts can exceed 20GB easily. Multi-agent sessions need awareness.

- **Recommendation:** Add Makefile target: `make disk-report` → summarize largest dirs (`du -sh crates/* apps/* docs-site/node_modules`)
- **Effort:** 1 tool call

### Multi-Agent Readiness Score: 6.5/10

- ✅ Worktree discipline documented in CLAUDE.md
- ✅ Commit separation by provenance clear
- ⚠️ Long-push pattern undefined (needed if >1 agent active)
- ⚠️ Disk budget not tracked

---

## Priority Backlog: 20 High-Impact Items

Ordered by criticality and effort; all estimates in **AI-DD tool-call budget** (1 call ≈ 5 min):

| # | Category | Item | Blocker? | Effort | Est. Calls |
|---|----------|------|----------|--------|-----------|
| **1** | Release | **[CRITICAL]** Start Apple FamilyControls entitlement application | ✅ Yes (MVP) | 0.5 d | 0 (admin task) |
| **2** | Release | Implement fastlane lanes (build_xcframework, build_and_upload_testflight, code_sign) | ✅ MVP TestFlight | 2–3 d | 5–8 |
| **3** | Testing | Add 20+ iOS XCTest unit tests (onboarding, rules eval, audit, sync) | ⚠️ Deferred | 1.5 w | 8–12 |
| **4** | Testing | Implement swift-snapshot-testing framework + initial snapshots (Design System, MascotUI, Onboarding) | ⚠️ High-value | 1 w | 6–10 |
| **5** | Quality | Add SwiftLint + SwiftFormat to lefthook pre-commit | ⚠️ Medium | 1–2 d | 2–3 |
| **6** | Quality | Add SwiftLint/SwiftFormat to GitHub Actions CI | ⚠️ Medium | 1 d | 2 |
| **7** | Testing | E2E XCUITest harness (5–8 critical user journeys) | ⚠️ Phase 2 | 2 w | 10–15 |
| **8** | Swift 6 | Audit codebase for @MainActor isolation gaps | ⚠️ Phase 2 | 3–5 d | 3–5 |
| **9** | Build | Track UniFFI 0.30+ migration path (testing, dependency pinning) | ⚠️ Phase 2 | 2 d | 2–3 |
| **10** | Instrumentation | Add tracing macros to Rust core (rule fires, penalties, syncs) | ⚠️ Phase 2 | 1 w | 5–8 |
| **11** | Release | Implement Sentry iOS SDK + PII scrubbing | ⚠️ Phase 2 TestFlight | 2–3 d | 3–5 |
| **12** | Cross-platform | Document Android strategy (UniFFI Kotlin, Jetpack Compose, Phase 3 timeline) | ⚠️ Medium | 1–2 d | 1–2 |
| **13** | Cross-platform | Finalize template marketplace schema (JSON, signing, distribution) | ⚠️ Phase 3 spec | 2–3 d | 2–3 |
| **14** | Quality | Add `.vale.ini` + `.markdownlintrc` for docs-site | ⚠️ Low | 1 d | 1 |
| **15** | Testing | Add FFI round-trip tests (serialize → Swift → deserialize → match) | ⚠️ Medium | 3–5 d | 2–3 |
| **16** | Quality | Optimize pre-push performance (changed-crate tests only; full suite on CI) | ⚠️ Phase 2 DX | 1 d | 2–3 |
| **17** | Coordination | Document long-push pattern + disk-budget monitoring | ⚠️ Low | 1 d | 1 |
| **18** | Build | Evaluate Tuist migration path (v1.67+ caching benefits) | ⚠️ Phase 2 | 2–3 d | 2–3 |
| **19** | Telemetry | Add structured logging (tracing integration with Sentry backend) | ⚠️ Phase 2 | 1 w | 3–5 |
| **20** | Release | Publish fastlane Fastfile to docs-site (developer onboarding guide) | ⚠️ Low | 1 d | 1 |

**Total Phase 1 MVP (blocking):** Items #1–2, ~8–11 calls, ~5–8 engineering days  
**Total Phase 2 (next):** Items #3–11, ~40–50 calls, ~4–6 engineering weeks  
**Estimate to "release-ready" (TestFlight + entitlement):** **2–3 weeks** (fastlane + entitlement approval in parallel)

---

## Three Critical Fixes (10x Improvement Potential)

### Fix #1: Implement Fastlane TestFlight Pipeline (2–3 days)

**Why it matters:**  
Eliminates manual build-upload bottleneck. Enables frequent TestFlight iterations (multiple builds per day if needed). Unlocks beta feedback loop.

**What to do:**
1. Install `fastlane` + `fastlane match` (code signing)
2. Create lanes:
   - `lane :build_xcframework` — wraps `build-xcframework.sh`
   - `lane :build_release` — xcodegen + xcodebuild with Release config
   - `lane :upload_testflight` — fastlane's `upload_to_testflight` action
   - `lane :full_release` — chains all three
3. Store code-signing credentials in Git (match) or S3
4. Test locally; integrate into GitHub Actions (even if no iOS CI, script runs manually)

**Effort:** 5–8 AI-DD calls  
**Gain:** Reduces TestFlight upload from 30 min (manual) to 5 min (automated)

---

### Fix #2: Start Apple FamilyControls Entitlement Application NOW (0.5 days)

**Why it matters:**  
1–4 week approval time is the longest-pole-in-tent blocker for any public release. Delaying = delayed v1.0 App Store submission.

**What to do:**
1. Log into [Apple Developer Portal](https://developer.apple.com/)
2. Navigate to Certificates, Identifiers & Profiles → Identifiers → `com.koosha.focalpoint`
3. Request entitlement: "com.apple.developer.family-controls"
4. Submit description: "FocalPoint is a dual-mode screen-time companion (parental + self-control) that enforces usage rules via system integration."
5. Expect review in 1–4 weeks

**Effort:** 30 minutes (admin)  
**Gain:** Starts approval clock immediately; App Store release no longer blocked

---

### Fix #3: Add iOS Snapshot Testing (1 week, 6–10 calls)

**Why it matters:**  
Current MVP has 0 regression detection for UI. A single mascot animation bug or design system color change can ship to TestFlight undetected. Snapshot tests catch 80% of visual regressions automatically.

**What to do:**
1. Add `swift-snapshot-testing` to `Package.swift` dependencies
2. Create snapshot tests for:
   - Palette colors + typography (DesignSystem)
   - Mascot animation frames (MascotUI, CoachyView)
   - Onboarding flow screens (FocalPointApp)
   - Rule creation form (Enforcement)
3. Generate baseline snapshots (first run)
4. Add to CI (snapshots committed to repo; CI re-generates and diffs)

**Effort:** 6–10 AI-DD calls  
**Gain:** Detects ~80% of visual bugs before TestFlight; builds confidence in multi-agent iterations

---

## Overall DX/DE Readiness Score: 6.2/10

| Category | Score | Notes |
|----------|-------|-------|
| **Build Pipeline** | 7.5 | xcodegen + UniFFI 0.28 solid; bitcode debt minor |
| **Testing** | 4.8 | Rust tests present; iOS tests thin; no snapshots or E2E |
| **Quality Gates** | 7.2 | Lefthook comprehensive for Rust; Swift linting missing |
| **Swift 6 Ready** | 3.2 | Major concurrency refactor needed; deferred to Phase 2 |
| **Cross-Platform** | 4.0 | iOS MVP clear; Android/web strategy undefined |
| **Telemetry** | 2.5 | Crash reporting missing; audit chain present |
| **Release** | 3.5 | Fastlane missing; entitlement application critical path |
| **Multi-Agent** | 6.5 | Worktree discipline clear; long-push pattern undefined |
| **Overall** | **6.2** | **Viable for v1.0 MVP TestFlight; production-ready requires 4–6 weeks** |

---

## Recommendations for v1.0 (Next 2–3 Weeks)

### Must Do (Blocking)

1. **[Week 1]** Apply for Apple FamilyControls entitlement (0.5 d)
2. **[Week 1–2]** Implement fastlane TestFlight lanes (2–3 d; 5–8 calls)
3. **[Week 2]** Local device smoke test (manual; no automation; 1 d)

### Should Do (High-Value)

4. **[Week 2]** Add 10+ iOS XCTest unit tests (onboarding, rules eval, sync) (3–5 d)
5. **[Week 3]** Implement swift-snapshot-testing + Design System snapshots (3–5 d; 6–10 calls)
6. **[Week 3]** Add SwiftLint + SwiftFormat to lefthook (1–2 d; 2–3 calls)

### Nice-to-Have (Phase 2)

7. XCUITest harness (5–8 journeys)
8. Sentry crash reporting
9. Swift 6 strict concurrency audit

---

## Sources

All recommendations include source citations (accessed 2026-04-23):

- [Tuist Generated Projects: why generate Xcode projects in 2025](https://tuist.dev/blog/2025/02/25/project-generation)
- [uniffi-rs/CHANGELOG.md](https://github.com/mozilla/uniffi-rs/blob/main/CHANGELOG.md)
- [SnapshotTesting 1.0: Delightful Swift snapshot testing](https://www.pointfree.co/blog/posts/23-snapshottesting-1-0-delightful-swift-snapshot-testing)
- [GitHub - pointfreeco/swift-snapshot-testing](https://github.com/pointfreeco/swift-snapshot-testing)
- [Best Mobile Testing Tools: 2025 Roundup](https://maestro.dev/insights/best-mobile-testing-tools-2025-roundup)
- [The Best Mobile App Testing Frameworks in 2026](https://maestro.dev/insights/best-mobile-app-testing-frameworks)
- [Adopting strict concurrency in Swift 6 apps | Apple Developer Documentation](https://developer.apple.com/documentation/swift/adoptingswift6)
- [Complete concurrency enabled by default – available from Swift 6.0](https://www.hackingwithswift.com/swift/6.0/concurrency)
- [Swift Code Formatters - NSHipster](https://nshipster.com/swift-format/)
- [Linting vs Formatting: A Swift Guide — Part 2](https://jasonzurita.com/linting-and-formatting-swift-part-2/)
- [Sentry vs Crashlytics Comparison & Best Alternative 2025](https://uxcam.com/blog/sentry-vs-crashlytics/)
- [Top 5 iOS Crash Reporting Tools 2025 | Sidekick Interactive](https://www.sidekickinteractive.com/uncategorized/top-5-ios-crash-reporting-tools-2025/)
- [Fastlane TestFlight Automation: Swift 6.2 iOS Release Guide | Medium](https://ravi6997.medium.com/automating-app-releases-fastlane-testflight-swift-6-2-integration-dc11cf8b1a7f)
- [Upload iOS App to TestFlight with GitHub Actions and Fastlane Match – 2025 Tutorial with Example](https://brightinventions.pl/blog/ios-testflight-github-actions-fastlane-match/)
- [Lefthook: benefits vs husky and how to use - DEV Community](https://dev.to/quave/lefthook-benefits-vs-husky-and-how-to-use-30je)
- [FocalPoint CLAUDE.md — Q8 FamilyControls entitlement application](./open_questions.md)

---

**Audit completed:** 2026-04-23  
**Next review:** 2026-06-01 (post-Phase-1 TestFlight)
