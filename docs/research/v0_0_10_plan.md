# FocalPoint v0.0.10 Roadmap

**Status:** Draft (pre-rc)  
**Previous Release:** v0.0.9 (aed0aaa, W-49)  
**Commits Since:** 2 (refactor + CI chore)

---

## Themes

### 1. MockFamilyControls Testability — Polish Demo
Refine the MockFamilyControls adapter introduced in feat/connectors. iOS app integration tests demand robust child-restriction mocking.

**Acceptance Criteria:**
- MockFamilyControls adapter passes 5+ iOS integration tests (UTCoreServices, authorization chains)
- Demo runs cleanly in TestFlight with no crashes on iOS 16.4+
- Test fixtures cover: approved/denied schedule transitions, geofence edge cases

**WBS (agent actions):**
1. Review existing MockFamilyControls impl; identify brittle test paths (2 tool calls)
2. Add 3+ comprehensive test cases for schedule enforcement (4 tool calls)
3. Polish error messages + validation paths (2 tool calls)
4. Verify TestFlight build passes (1 tool call)

**Estimated:** ~3 min (9 agent actions)

---

### 2. Coachy Mascot — Tier-1 → Tier-2
Promote Coachy from static/parametric SVG to rich animated state machine. Export Rive binary; add 20+ emotional/contextual states.

**Acceptance Criteria:**
- Rive project exports deterministically (no binary drift)
- 20-state emotion matrix (happy, encouraging, concerned, idle, etc.)
- All states render <200ms on real iPhone 15 Pro + iPhone 13 mini
- Audio cue sync (8 cues mapped to state transitions)

**WBS (agent actions):**
1. Set up Rive exporter pipeline; validate deterministic output (3 tool calls)
2. Model 20-state emotion matrix in Rive; export binary (5 tool calls)
3. Integrate binary into iOS app; wire state transitions (4 tool calls)
4. Profile render perf; optimize if >200ms (3 tool calls)
5. Bind audio cues to state changes (2 tool calls)

**Estimated:** ~6 min (17 agent actions)

---

### 3. Audio System — Real Device Integration
Extend deterministic synthesized cues (existing: 8 cues + 8 haptics) to real devices. Verify Taptic Engine + speaker output on iOS hardware.

**Acceptance Criteria:**
- 8 audio cues play correctly on iOS 15+ devices
- 8 haptic patterns trigger without stuttering (Taptic Engine latency <50ms)
- Fallback to speaker on non-haptic devices
- Test matrix: iPhone 15 Pro, iPhone 13, iPhone 12 mini

**WBS (agent actions):**
1. Audit existing audio pipeline (synthesis, playback API) (2 tool calls)
2. Add hardware device detection + Taptic capability checks (2 tool calls)
3. Real device audio testing loop: iPhone 15 Pro, 13, 12 mini (6 tool calls)
4. Profile Taptic latency; add fallback paths if >50ms (3 tool calls)

**Estimated:** ~5 min (13 agent actions)

---

### 4. Connector Test Expansion
Strava, Notion, Linear, Readwise adapters drafted in earlier waves. Add test coverage for OAuth flows + data normalization.

**Acceptance Criteria:**
- 4+ connectors (Strava, Notion, Linear, Readwise) pass OAuth mock tests
- Each connector normalizes ≥10 data fields to FocalPoint schema
- Mock server responses match real API payloads (captured in fixtures)
- Full coverage of happy-path + auth-failure scenarios

**WBS (agent actions):**
1. Audit existing connector stubs; identify test gaps (2 tool calls)
2. Write mock OAuth flows for each connector (6 tool calls: 1.5/connector)
3. Add data normalization tests; verify schema alignment (4 tool calls)
4. Capture real API payloads into test fixtures (2 tool calls)
5. Run connector test suite; document any external blockers (1 tool call)

**Estimated:** ~4 min (15 agent actions)

---

### 5. Apple FamilyControls Entitlement — External Blocker
Apple App Review still blocks manual entitlement request. Status: awaiting Apple's guidance on automation.

**Acceptance Criteria:**
- If Apple lifts restriction: port MockFamilyControls code → real FamilyControls API (straightforward)
- If Apple maintains freeze: document workaround paths (MDM, TestFlight enterprise build)
- No code changes until Apple unblocks

**WBS (agent actions):**
1. Check Apple Developer forum + escalation queue (1 tool call)
2. Document current entitlement status + workaround paths in RFC (1 tool call)

**Estimated:** ~1 min (2 agent actions)

---

## Risks & Known Issues

| Risk | Mitigation |
|------|-----------|
| **Rive binary drift** (non-determinism) | Automate export pipeline; pin Rive version in CI |
| **Taptic latency on older hardware** (iPhone 12 mini) | Profile early; use async dispatch if >50ms |
| **OAuth mock fidelity** | Capture real payloads; store in fixtures; regenerate quarterly |
| **Apple FamilyControls entitlement freeze** | External blocker; document fallback paths; revisit Q3 2026 |
| **cargo-deny advisories carryover** | Inherit from v0.0.9; audit + fix in v0.0.11 phase |

---

## Non-Blocking Follow-ups (v0.0.11+)

- **Coachy voice synthesis:** Multi-state response synth (already prototyped; needs refinement)
- **SQLite schema migrations:** Extend Versioned trait (templates-registry already has foundation)
- **Plugin SDK:** WASM sandbox phase 2 (phase 1 scaffolded; expand capability manifest)

---

## Timeline & Effort

**Total WBS effort:** ~19 min wall clock (~59 agent actions)  
**Phases:**
1. **Fast path (Themes 1–2):** 9 min, unblocked, launch immediately
2. **Parallel (Themes 3–4):** 9 min, ~2 subagent swarm, start after phase 1 review
3. **Blocker check (Theme 5):** 1 min, embedded in parallel phase

**No version bump until rc gate passes.** Branch: pre-rc/v0.0.10 (feature branches per CLAUDE.md discipline).

---

## Backlog & Deprecations

- **Already shipped (v0.0.9+):** MockFamilyControls adapter, Rive+Lottie pipelines, 8-cue audio + haptics, 20-state mascot (SVG), plugin SDK phase 1, HealthKit + Fitbit connectors, dynamic island, onboarding v2, backup/restore, i18n
- **Deprecated:** Legacy mascot debug view (replaced by Rive integration)
- **Parking lot:** Stashed Stashly-Migrations pattern (schema_version adopted in v0.0.9 refactor; full migration framework in v0.0.11)
