# User Journey Verification Harness for FocalPoint: 2026 Research Brief

## Executive Summary

This brief outlines the 2026 architecture for an automated user journey verification harness that enables agents and reviewers to verify real user-facing behavior through recorded keyframes, GIF/MP4 artifacts, and VLM-based visual equivalence testing.

**Key finding**: FocalPoint's iOS/SwiftUI app requires a hybrid recording approach combining XCTest (UI automation), ScreenCaptureKit (screen recording), and VHS (TUI/CLI demos). The VLM keyframe gallery pattern from hwLedger applies directly.

---

## Part 1: Recording Strategy by Platform

### iOS App (SwiftUI)

| Requirement | 2026 Recommendation | Rationale |
|---|---|---|
| **UI Automation** | **XCTest** + **Accessibility API** | Native Apple framework, stable, integrated into Xcode |
| **Screen Recording** | **ScreenCaptureKit** (native Swift) | Apple's recommended API (iOS 16+), hardware-accelerated |
| **Recording Pipeline** | Custom Swift module in `apps/ios/FocalPointRecorder/` | Reusable, cross-platform (iOS + macOS Catalyst) |

### macOS Catalyst App

| Requirement | 2026 Recommendation | Rationale |
|---|---|---|
| **UI Automation** | **XCTest** (macOS target) | Shares iOS test code with platform-specific adapters |
| **Screen Recording** | **ScreenCaptureKit** (macOS 12.3+) | Native, hardware-accelerated |

### TUI / CLI Demos

| Requirement | 2026 Recommendation | Rationale |
|---|---|---|
| **Terminal Recording** | **VHS (Charmbracelet)** | Declarative .tape files → MP4/GIF, widely adopted |

### Web/Streamlit (Future)

| Requirement | 2026 Recommendation | Rationale |
|---|---|---|
| **Web Recording** | **Playwright** + Chromium | Headed browser automation, screenshot/keyframe extraction |

---

## Part 2: Recording Pipeline Architecture

### Directory Structure

```
FOCALPOINT/
├── apps/
│   ├── ios/
│   │   └── FocalPointUITests/
│   │       ├── recordings/           # Screen recordings (MP4)
│   │       ├── keyframes/           # Extracted frames (PNG)
│   │       ├── manifests/           # Journey manifests (JSON)
│   │       └── tapes/               # VHS .tape files (TUI demos)
│   └── cli/
│       └── journeys/
│           └── tapes/               # CLI VHS tapes
└── docs-site/
    ├── journeys/                    # Journey MD files
    ├── recordings/                  # Public artifacts (symlink or copy)
    │   ├── student-canvas/
    │   │   ├── manifest.json
    │   │   ├── frames/
    │   │   └── verification.json
    │   ├── developer-github/
    │   └── connector-sdk/
    └── public/
        └── journeys/                # Static assets (MP4, GIF, PNG)
```

### Manifest Schema

```json
{
  "id": "student-canvas-onboarding",
  "title": "Student Canvas Onboarding",
  "persona": "Alice (Student)",
  "platform": "ios",
  "duration_seconds": 120,
  "recording_date": "2026-04-30",
  "status": "verified",
  "intent": {
    "summary": "Student links Canvas LMS and creates first focus rule",
    "steps": [
      {
        "index": 1,
        "slug": "install-grant-permissions",
        "intent": "Install app and grant FamilyControls",
        "precondition": "Fresh install, no accounts linked",
        "expected_visible_change": "Permission dialog, then onboarding cards"
      },
      {
        "index": 2,
        "slug": "link-canvas",
        "intent": "OAuth with Canvas LMS",
        "precondition": "Settings > Connectors open",
        "expected_visible_change": "Canvas logo, course selection list"
      },
      {
        "index": 3,
        "slug": "create-first-rule",
        "intent": "Create assignment due-notification rule",
        "precondition": "Rules screen, empty state",
        "expected_visible_change": "Rule editor with trigger/action blocks"
      }
    ]
  },
  "keyframes": [
    {
      "step": 1,
      "path": "/recordings/student-canvas/frame-001.png",
      "timestamp": "0:00",
      "caption": "Fresh install, permission dialog"
    }
  ],
  "verification": {
    "vlm_model": "claude-opus-4-7-20250416",
    "description": "User installs FocalPoint, grants FamilyControls...",
    "equivalence_score": 0.92,
    "status": "PASS",
    "verified_at": "2026-04-30T12:00:00Z"
  }
}
```

---

## Part 3: VLM Verification Integration

### Process

1. **Record**: Capture MP4 via ScreenCaptureKit/XCTest
2. **Extract**: `ffmpeg` extracts keyframes (1 frame / 3 seconds)
3. **Describe**: Claude Opus 4.7 describes keyframe sequence
4. **Judge**: Claude Sonnet 4.6 scores equivalence vs. intent
5. **Store**: `verification.json` alongside manifest

### API Integration

```python
# Pseudocode for verification pipeline
async def verify_journey(manifest_path: Path) -> VerificationResult:
    manifest = load_manifest(manifest_path)
    keyframes = load_keyframes(manifest.keyframe_paths)

    # VLM description
    description = await claude.describe_keyframes(
        images=keyframes,
        system_prompt="Describe what happens in this iOS app interaction:"
    )

    # Judge equivalence
    score = await claude.judge_equivalence(
        intent=manifest.intent.summary,
        description=description
    )

    return VerificationResult(
        description=description,
        score=score,
        status="PASS" if score >= 0.8 else "FAIL"
    )
```

---

## Part 4: Journey Types for FocalPoint

### Planned Journeys

| Journey | Platform | Priority | Status |
|---|---|---|---|
| Student Canvas Onboarding | iOS | P0 | Draft |
| Developer GitHub Integration | iOS | P0 | Draft |
| Connector SDK Author Flow | CLI/Mac | P1 | Draft |
| Morning Brief Ritual | iOS | P1 | Planned |
| Evening Shutdown Ritual | iOS | P1 | Planned |
| Focus Session (PR Review) | iOS | P2 | Planned |
| Rule Pack Import | iOS | P2 | Planned |
| Coachy Procrastination Detection | iOS | P2 | Planned |
| Streak Achievement Celebration | iOS | P3 | Aspirational |
| Apple Health Sleep Integration | iOS | P3 | Aspirational |

---

## Part 5: CLI Recording with VHS

### Tape Template

```tape
Output student-canvas-onboarding.mp4

Set FontSize 14
Set Height 20
Set Width 100
Set Theme Catppuccin Mocha

Type "focalpoint --version"
Sleep 1s
Enter

Type "focalpoint connector list"
Sleep 2s
Enter

Type "focalpoint rule create --name 'Canvas Due Soon'"
Sleep 1s
Enter

Type "# Trigger: canvas.assignment.due_soon"
Sleep 0.5s
Enter

Type "# Action: show_focus_view + block_social"
Sleep 0.5s
Enter

Type "focalpoint rule save"
Sleep 2s
Enter

Type "exit"
Enter
```

---

## Part 6: Acceptance Criteria

For each journey:

- [ ] MP4 recording exists (or VHS .tape file for CLI)
- [ ] Keyframes extracted (8–15 PNG files per journey)
- [ ] Manifest JSON with steps, intents, preconditions
- [ ] Verification.json with VLM description and equivalence score
- [ ] Journey MD file with JourneyViewer component
- [ ] JourneyViewer displays: video/GIF, keyframe gallery, verification badge
- [ ] Links to next steps / related journeys

---

## Part 7: References

- [hwLedger Journey Harness](../hwLedger/docs-site/research/12-ui-journey-harness-2026.md)
- [Charmbracelet VHS](https://github.com/charmbracelet/vhs)
- [Apple ScreenCaptureKit](https://developer.apple.com/documentation/screencapturekit/)
- [XCTest Documentation](https://developer.apple.com/documentation/xctest)
- [Claude Vision API](https://platform.anthropic.com/docs/en/build-with-claude/vision)
- [SVG Graphics Style Guide](./14-svg-graphics-style-guide.md) — **Critical**: Understanding the distinction between mockups, supporting graphics, and real recordings

---

**Research completed**: April 30, 2026
**Status**: Ready for implementation planning
**Next steps**: Prioritize P0 journeys (Student Canvas, Developer GitHub) for recording
