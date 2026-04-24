# FocalPoint — App Store Metadata Package

**Date Created:** 2026-04-23  
**Version:** 1.0  
**Status:** Ready for App Store Connect entry

---

## 1. Basic Metadata

### App Name
**FocalPoint** (170 characters max)

**Why:** Short, memorable, domain-friendly (focalpoint.app); clear brand name.

---

## 2. Subtitle
(Optional; iOS 15+ feature)

**Subtitle:**
```
Rules engine for focused productivity
```

**Character Count:** 45 / 30 max

**Alternative (if too long):**
```
Connector-first focus engine
```

**Character Count:** 30 / 30

---

## 3. App Description

### Short Description (170 characters max for marketing / search results)

```
Focus rules engine powered by Canvas, Calendar, and GitHub connectors.
Build sustainable study habits with explainable enforcement.
```

**Character Count:** 141 / 170

---

### Full Description (4000 characters max)

```
FocalPoint is a connector-first screen-time management platform that transforms how you build focus habits. Instead of blocking apps arbitrarily, FocalPoint intelligently enforces focus policies based on your calendar, task assignments, and external connectors.

## Why FocalPoint?

Most screen-time managers are simple blockers: they lock apps during set times, period. FocalPoint goes deeper. It evaluates *your context*—when you finish a Canvas assignment, when your calendar shows a meeting, when your streak is on the line—and adjusts your focus rules accordingly. Results are explainable. Every time a rule fires, you see why.

## Core Features

**Connector-First Architecture**
- Connect Canvas LMS to sync assignment due dates
- Sync Google Calendar and device calendar for meeting-aware rules
- Optional GitHub integration for development-focused sessions
- Build custom rules triggered by real external events, not arbitrary time windows

**Rules Engine**
- Create if-then-else logic: "If assignment due today, lock social apps until 3pm"
- Evaluate rules in milliseconds with deterministic explanations
- Cooldown prevents re-triggering the same rule within a time window
- See exactly which rule fired and why every time

**Reward & Penalty Ledger**
- Earn credits for consecutive focus sessions and meeting deadlines
- Accumulate penalty points for missed deadlines or boundary violations
- Use credits to unlock apps temporarily (bypass budget)
- Visual progress tracking keeps you motivated

**Explainability & Audit Trail**
- Every state change is recorded in a tamper-evident audit chain
- Understand the full history of your rules, decisions, and state
- Export your data anytime (SQLite database)

**Privacy-First Design**
- All data stored locally on your device
- OAuth tokens stored securely in iOS Keychain
- No FocalPoint servers; no cloud sync (Phase 1)
- Your rules, your data, your device

## Privacy & Security

✓ No analytics or crash reporting SDKs
✓ No user tracking across apps
✓ GDPR/CCPA/LGPD compliant
✓ Fully local-first (Phase 1)
✓ Open-source Rust core (see GitHub)

## Who Uses FocalPoint?

**Self-Regulators** — Adults enforcing their own focus policies
**Students** — Canvas-driven study sessions with streak-based rewards
**Knowledge Workers** — Calendar-aware rules for deep work windows

## Getting Started

1. Create your first task or goal
2. (Optional) Connect Canvas, Google Calendar, GitHub
3. Build a simple rule: "Lock distracting apps during focus hours"
4. Watch your focus patterns emerge with explainable decisions

## Support

Questions? Open an issue on GitHub or email support@focalpoint.app

---

*FocalPoint v0.2.0 — Free, open-source, privacy-first. Your rules, your focus.*
```

**Character Count:** 1,892 / 4000

---

## 4. Keywords

### Keyword List (100 characters max, comma-separated)

```
focus,productivity,screen time,rules,calendar,canvas,study,timer
```

**Character Count:** 68 / 100

### Keyword Rationale

| Keyword | Reason |
|---------|--------|
| focus | Primary use case; high search volume |
| productivity | Category alignment (Productivity app) |
| screen time | Core feature; App Store search term |
| rules | Core differentiator (rules engine vs. blockers) |
| calendar | Major connector; user expectation |
| canvas | Specific integration (Canvas LMS) |
| study | Primary persona (students) |
| timer | Common productivity app search term |

**Alternatives (if repositioning):**
- `app blocker,focus timer,habit tracker,goal setting,time management`

---

## 5. Support & Privacy URLs

### Required URLs

**Support URL:**
```
https://github.com/KooshaPari/FocalPoint/issues
```

OR (once support site exists):
```
https://support.focalpoint.app
```

**Privacy Policy URL:**
```
https://focalpoint.app/privacy
```

Must resolve to the full text of `PRIVACY.md`.

**Marketing URL (Optional):**
```
https://focalpoint.app
```

(Omit if no landing page exists; not required for submission.)

**App Website (Optional):**
```
https://github.com/KooshaPari/FocalPoint
```

---

## 6. Version Release Notes

### Release Notes (up to 4000 characters, shown in App Store updates)

```
FocalPoint v0.2.0 — Connector Edition

🎯 NEW: End-to-end focus engine live
  - Create rules with Canvas, Calendar, GitHub connectors
  - Explainable rule evaluations (see why each rule fired)
  - Reward & penalty ledger with visual progress tracking

🔧 IMPROVEMENTS:
  - Background sync via BGTaskScheduler (respects battery/network)
  - iOS Keychain secure token storage
  - Audit chain with tamper detection
  - EventKit calendar permissions flow

✅ VERIFIED:
  - 22/26 functional requirements shipped
  - Privacy manifest (PrivacyInfo.xcprivacy) included
  - FamilyControls entitlement ready
  - GDPR/CCPA/LGPD compliant

📋 KNOWN LIMITATIONS (Phase 2+):
  - Parental controls UI deferred
  - Multi-device sync deferred
  - Android deferred beyond Phase 2
  - LLM suggestions (Minimax/Kimi) deferred

Questions? Open an issue: https://github.com/KooshaPari/FocalPoint/issues

---
Privacy-first, open-source, local-only. Your rules. Your focus.
```

**Character Count:** 755 / 4000

---

## 7. Category Selection

### Primary Category
**Productivity**

**Secondary Category**
**Health & Fitness** (due to optional health app integrations in Phase 2+)

### Category Rationale

| Category | Fit | Reason |
|----------|-----|--------|
| Productivity | ✅ Excellent | Focus management, task/goal tracking, time blocking |
| Health & Fitness | ⚠️ Secondary | Optional health data integrations (sleep, steps) in Phase 2 |
| Utilities | ❌ Not recommended | Too generic; misses discovery opportunities |
| Education | ❌ Not recommended | Not a teaching platform; would be misleading |
| Parental Controls | ❌ Not for v1 | Correct for Phase 2 (explicit parental features) |

---

## 8. Age Rating (IARC Questionnaire)

### Questionnaire Answers

| Section | Question | Answer | Notes |
|---------|----------|--------|-------|
| **Content** | Violence | No | N/A |
| | Gambling | No | No loot boxes, gacha, or casino mechanics |
| | Alcohol, Tobacco, Drugs | No | N/A |
| | Profanity | No | App content is user-generated rules/tasks |
| | Scary/Horrifying | No | N/A |
| | Sexual Content | No | N/A |
| | Nude/Suggestive | No | N/A |
| **Targeting** | Directed at Children | No | Primary audience: students + adults |
| | Children's Privacy | Conditional | If minors use: no PII collection; parental consent for FamilyControls |
| **Features** | Location Data | No | Local-only; no GPS tracking |
| | User Identification | Yes | Task titles, calendar events; OAuth tokens to Canvas/Google/GitHub |
| | Third-Party Services | Yes | Canvas LMS, Google Calendar, GitHub APIs |
| | User-Generated Content | Partial | Rules, tasks are user-generated; not shared publicly (Phase 1) |
| | In-App Purchases | No | Free; monetization deferred to Phase 3+ |
| | Medical Claims | No | Penalty escalation is behavioral, not clinical |
| | Financial Transactions | No | No purchases, subscriptions, or payment processing |

### Resulting Age Rating
**4+** (unrestricted)

---

## 9. App Icon Specifications

### Master Icon Requirements

**Dimensions:** 1024×1024 px @ 72 ppi  
**Format:** PNG or JPEG (sRGB color space)  
**File Size:** < 100 MB (typical ~200–500 KB)

### Design Guidelines

✓ **Do:**
- Use brand colors (if defined; else use professional palette)
- Keep essential design within 940×940 px inner circle
- Ensure contrast ratio ≥ 4.5:1 for text/graphics
- Test on physical devices (not just simulator)
- Include rounded corners in design (Apple adds system corners)

✗ **Don't:**
- Use transparency/alpha channel (Apple adds corners automatically)
- Place essential content within 50 px of edge
- Use animated GIFs or video
- Include text that would be unreadable at small sizes
- Use copyrighted material without rights

### Icon Variants (if multi-file app)

Not applicable for FocalPoint v1 (single icon). Phase 2+ may need app clip icons.

---

## 10. Screenshots

### Screenshot Specifications

**iPhone Screenshots:**
- Dimensions: 1284×2778 px (6.7" Super Retina XS) or 1290×2796 px
- Max 5 screenshots (landscape + portrait)
- Safe area: avoid essential content in top/bottom 100 px (status/notch)

**iPad Screenshots:**
- Dimensions: 2732×2048 px (13" iPad)
- Max 5 screenshots
- Landscape orientation recommended

**File Format:** PNG or JPEG (sRGB)

### Screenshot Content Plan

| Screenshot # | iPhone (Portrait) | Content | Purpose |
|--------------|-------------------|---------|---------|
| 1 | Yes | Home screen with task/rule overview | Immediate UVP: "Create focus rules" |
| 2 | Yes | Rule creation flow (step 1: trigger selection) | Show rules engine core feature |
| 3 | Yes | Connector auth screen (Canvas OAuth) | Highlight Canvas integration |
| 4 | Yes | Reward/penalty ledger with visual progress | Show feedback loop (motivation) |
| 5 | Yes | Rule explanation screen (why rule fired) | Emphasize explainability/transparency |

### iPad Screenshots

| Screenshot # | iPad (Landscape) | Content | Purpose |
|---------------|------------------|---------|---------|
| 1 | Yes | Dashboard with rules + connectors visible | Wide-screen layout advantage |
| 2 | Yes | Audit chain view (full history) | Tamper-evident appeal to power users |
| 3 | Yes | Settings / connector management | Configuration options |
| 4 | Yes | (Mascot animation — if Coachy integrated) | Visual delight |
| 5 | Yes | (Reward/penalty ledger on iPad) | Large-screen UX |

### Screenshot Best Practices

- **Text Overlay:** Keep minimal; let UI speak
- **Consistency:** Use same font, color scheme across screenshots
- **Device Realistic:** Show iOS status bar, notch, home indicator
- **No Marketing Fluff:** Avoid floating icons or stock photos

---

## 11. App Preview Video (Optional)

### Video Specifications

- **Duration:** 15–30 seconds (ideal: 20–25s)
- **Aspect Ratio:** 1.97:1 (iPhone) or 4:3 (iPad)
- **File Format:** MOV (ProRes) or MP4 (H.264)
- **Codec:** H.264 @ 24–30 fps, stereo audio
- **Max File Size:** 500 MB

### Preview Script (Example)

```
[0–2s] Hero Shot: Home screen with "Create New Rule" button
  VOICEOVER: "FocalPoint — focus rules powered by your calendar and tasks."

[2–5s] Rule Creation: Select trigger (Canvas assignment due)
  VOICEOVER: "Create rules that understand your context."

[5–8s] Rule Condition: Select condition (before 3pm) and action (lock social apps)
  TEXT OVERLAY: "If assignment due today → lock distraction apps before 3pm"

[8–10s] Connector Sync: Calendar event appears + rule evaluates
  VOICEOVER: "Your rules adapt to your real world."

[10–12s] Reward/Penalty Ledger: Visual progress, streak counter
  TEXT OVERLAY: "7-day study streak 🔥 — Unlock 2× credits"

[12–15s] Audit Explanation: User taps a rule decision, sees explanation
  VOICEOVER: "Every decision is explainable."

[15–20s] Montage: Settings, connectors, audit chain
  TEXT OVERLAY: "Privacy-first. Local-only. Your rules, your focus."

[20s] App icon + "FocalPoint" + "Available on the App Store"
```

---

## 12. Promotional Text (Optional)

Appears in-app on feature carousel (when Apple promotes your app).

**Promotional Text (170 characters max):**

```
Build focus habits with rules that understand your calendar, tasks, and goals.
```

**Character Count:** 90 / 170

---

## 13. Subtitle (iOS 15+)

If included on App Store product page:

```
Connector-first focus engine
```

**Character Count:** 30 / 30

---

## 14. Localization (Future)

For Phase 2+ expansion into non-English markets:

**Priority Languages (by user demand):**
1. Spanish (es_ES, es_MX)
2. French (fr_FR)
3. German (de_DE)
4. Japanese (ja_JP)

Each requires:
- Translated app name, description, keywords
- Localized screenshots with text overlays
- Localized app icon (if text-based)

---

## 15. Content Rating (ESRB / Localized)

### ESRB Rating (North America)
**Rating:** T (Teen) → Actually **E** (Everyone) based on IARC answers

**Age:** Unrestricted (4+)

### Pan European Game Information (PEGI)
**Rating:** 3 (suitable for ages 3+)

### Entertainment Software Rating Board (ESRB)
**Rating:** E (Everyone) — no concerning content

---

## 16. Checklist Before Submission

- [ ] App name finalized and under 170 characters
- [ ] Subtitle (if using) finalized
- [ ] Short description (170 chars) finalized
- [ ] Full description (4000 chars) finalized and proofread
- [ ] Keywords (100 chars) entered
- [ ] Support URL pointing to active support resource
- [ ] Privacy Policy URL pointing to published policy
- [ ] App icon (1024×1024) created and tested on device
- [ ] iPhone screenshots (1284×2778) created and approved
- [ ] iPad screenshots (2732×2048) created and approved
- [ ] App preview video (optional) filmed and edited
- [ ] Release notes written
- [ ] Category (Productivity) + secondary (Health & Fitness) selected
- [ ] IARC age rating questionnaire completed
- [ ] IARC rating received (4+)
- [ ] Build uploaded to App Store Connect
- [ ] Metadata review on App Store Connect (no warnings/errors)

---

## 17. Copy-Ready Text Snippets

### For Your Website / Marketing

**Hero Copy:**
```
FocalPoint — Rules Engine for Focused Productivity

Build sustainable focus habits powered by Canvas, Calendar, and GitHub.
No arbitrary blockers. Just intelligent enforcement based on your real context.
```

**Sub-headline:**
```
Explainable. Local-first. Privacy-safe.
```

**Feature Bullets:**
```
• Connector-first rules engine (Canvas, Calendar, GitHub)
• Explainable enforcement (see why each rule fired)
• Reward & penalty ledger with streak tracking
• Tamper-evident audit trail
• Local-first privacy (no cloud sync, Phase 1)
```

**Call-to-Action:**
```
Download FocalPoint on the App Store
```

---

## 18. Notes & Considerations

### For App Store Connect Entry

1. **Release Cadence:** Set to automatic release (or manual, if wanting phase-in)
2. **Price Tier:** Free (Tier 0; monetization deferred to Phase 3)
3. **Family Sharing:** Enabled (applies to iPhone + iPad)
4. **Build Selection:** Choose latest build from TestFlight
5. **Review Notes:** Reference FamilyControls entitlement approval + privacy manifest
6. **Auto-Renewal:** N/A (no subscription)

### Common Rejection Reasons (Avoid)

- ❌ Misleading marketing copy (avoid "blocker" if you mean "policy enforcer")
- ❌ Broken support URL (test before submission)
- ❌ Unrelated screenshots (show the actual app, not mockups)
- ❌ Missing privacy policy (required)
- ❌ Vague description (be specific about features)

### Post-Submission Monitoring

- Watch approval queue for 24–48 hours
- Be ready to respond to reviewer feedback within 5 days
- If rejected, fix issues immediately and resubmit (not an appeal)
- Approval usually comes within 24–48 hours after resubmission

---

**Word Count:** 2,245 words  
**Revision:** 1.0  
**Ready for App Store Connect:** Yes, upon PrivacyInfo.xcprivacy validation and entitlement approval.
