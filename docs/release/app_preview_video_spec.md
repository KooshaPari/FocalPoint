# FocalPoint App Preview Video Specification

**Status:** Specification (pre-production)  
**Target:** App Store Preview Video (1–3 required videos)  
**Timeline:** After FamilyControls entitlement approval

## Overview

App Store Preview videos (15–30 seconds each) are **required** for App Store listings on iOS/iPadOS. This specification defines the shot list, audio strategy, and technical requirements for FocalPoint's preview videos.

## Apple Requirements

| Requirement | Value | Note |
|-------------|-------|------|
| Duration | 15–30 seconds each | Shorter is better; 15s recommended |
| Format | MP4 H.264 | Must be `.mp4` or `.mov` |
| Resolution | Device-native | iPhone 6.9", iPad 13", or generic 16:9 |
| Frame rate | 24p, 25p, 30p, or 60p | 30p recommended |
| Codec | H.264 (AVC) | Not HEVC; Apple's specs are strict |
| Orientation | Portrait (iPhone) + Landscape (iPad) | Submit separately per device |
| Audio | Device audio **optional** | No requirement; often silent with text overlays |
| Watermarks | None | No logos, legal text, or app branding |
| Transitions | Fade/dissolve allowed | Avoid spinning/flying 3D effects |

## Recommended Shot List (15 seconds total)

Each video should tell the Coachy story: **"Focus harder. Earn rewards. Become unstoppable."**

### Shot 1: Cold-Launch Wake (0–2s)
- Device on black screen
- Swipe up from lock screen OR tap app icon
- FocalPoint loading screen appears
- Coachy mascot fades in with gentle flame animation
- **CTA overlay:** "Meet Coachy"

### Shot 2: Add a Focus Task (2–5s)
- User taps "+ Add Task" (or equivalent)
- Types a task: **"Finish Q2 Roadmap"**
- Selects duration: **25 minutes**
- Taps "Start Focus Session"
- **CTA overlay:** "Stay Focused"

### Shot 3: Focus Session In-Progress (5–10s)
- Screen shows active focus session
- Timer counting down (1:30 remaining)
- Smooth animation of flame growing/pulsing (subtle)
- Focus streak badge animates on screen
- Credits incrementally increasing (+10 credits)
- **CTA overlay:** "Earn Credits"

### Shot 4: Session Complete + Celebration (10–13s)
- Focus session ends with timer reaching 0:00
- Confetti/celebration animation (if available)
- Earn 50 credits badge pops up
- Flame icon grows to full brightness
- **CTA overlay:** "Level Up"

### Shot 5: Rewards Shop (13–15s)
- Swipe to Rewards tab (or tap)
- Show 3–4 reward items (e.g., "Unlock Advanced Focus", "Custom Colors", "Leaderboard Access")
- User taps one reward
- Redemption confirmation: **"Reward Unlocked!"**
- **CTA overlay:** "Unlock Rewards"

**Final frame (15s):** App name + tagline:  
> **FocalPoint**  
> *Focus Harder. Earn Rewards.*

---

## Alternate Shot List: Variant #2 (iPad Landscape, 20s)

If submitting iPad Preview:

1. **Cold-launch on iPad** (0–2s): Split-screen setup (app + secondary app side-by-side)
2. **Create Session** (2–5s): Drag-to-add task from sidebar
3. **Multi-window Focus** (5–15s): Focus session in main pane, sidebar shows task details + history
4. **Unlock via Leaderboard** (15–18s): Swipe to Leaderboard, user ranks 2nd, earns streak bonus
5. **Celebrate** (18–20s): Reward unlock animation, app name + tagline

---

## Audio Strategy

### Option A: Silent (Recommended)

- Zero device audio
- Text overlays do all communication
- Backing track: **none** (silence is cleaner)
- Advantage: Works globally; no subtitles needed

### Option B: Minimal Soundtrack

- **Music:** Upbeat instrumental (royalty-free, 20–30 BPM)
- Examples: Epidemic Sound, Artlist, YouTube Audio Library
- **No voiceover** (Apple discourages this)
- **SFX only:** Soft "whoosh" on transitions, "ding" on rewards
- Track must not:
  - Include voice (VO)
  - Have copyrighted music
  - Be >30 seconds long
  - Contain speech or dialogue

### Audio Acquisition

- **Royalty-free sources:**
  - Epidemic Sound: $15/month (best; high-quality)
  - Artlist: $20/month (similar)
  - YouTube Audio Library: Free (basic selection)
  - Splice: $8/month (electronic/lo-fi focus)
- **Licensing:** Ensure "App Store" or "Commercial" license

---

## Text Overlays

Use **sans-serif, bold, white text** on semi-transparent dark backgrounds (40–60% opacity).

| Shot | Overlay | Duration | Position |
|------|---------|----------|----------|
| 1 | "Meet Coachy" | 1s | Center |
| 2 | "Stay Focused" | 2s | Bottom |
| 3 | "Earn Credits" | 2s | Bottom-right |
| 4 | "Level Up" | 2s | Center |
| 5 | "Unlock Rewards" | 1s | Bottom |
| Final | App name + tagline | 2s | Center |

---

## Technical Specs for Recording

### On-Device Recording (Recommended)

1. **Device Setup:**
   - Use real iPhone 15 Pro or later (or simulator with high res)
   - Landscape (if iPad) or portrait (iPhone)
   - 100% brightness for clarity
   - Airplane mode (no notifications)

2. **Capture Tool:**
   - **macOS:** QuickTime Player → File → New Screen Recording
   - **iOS:** Settings → Control Center → Add Screen Recording, then record
   - Alternative: **ScreenFlow** or **Camtasia** (paid, better quality)

3. **Post-Production:**
   - **Final Cut Pro** or **Adobe Premiere** (professional)
   - **iMovie** or **DaVinci Resolve** (free; Resolve is excellent)
   - Export as **MP4, H.264, 30fps, 1080p minimum**

4. **Frame Rate:**
   - Minimum: 30fps (cinematic)
   - Preferred: 60fps (buttery smooth)

### Simulator-Based Recording

- Pros: Reproducible, no device needed
- Cons: Cannot show actual Family Controls behavior
- **Acceptable for:** Onboarding flow, reward screens, settings

---

## Rejection Risk Mitigation

**Avoid these common rejections:**

1. ✗ **Device notifications visible** → Use Airplane mode
2. ✗ **Status bar at top** → Crop or use device frame overlay
3. ✗ **Third-party app visible** → Ensure FocalPoint fills entire screen
4. ✗ **Watermarks or logos** → Remove before export
5. ✗ **Crashes or glitches** → Record multiple takes; use best
6. ✗ **Copyrighted music** → Use royalty-free only
7. ✗ **Misleading claims** → Match video to actual features
8. ✗ **Spam or clickbait overlays** → Keep text minimal and true

---

## Production Schedule

**Phase 1: Planning** (1 week before shoot)
- [ ] Finalize shot list with product team
- [ ] Brief designer on text overlay style
- [ ] Source royalty-free music (if needed)
- [ ] Set up test focus session scenario

**Phase 2: Recording** (1 day)
- [ ] Record all 5 shots, multiple takes each
- [ ] Verify audio (if using)
- [ ] Backup footage to two locations

**Phase 3: Editing** (2–3 days)
- [ ] Assemble shots in timeline
- [ ] Add transitions (fade/dissolve only)
- [ ] Add text overlays and time them
- [ ] Color-grade for consistency
- [ ] Export to MP4 H.264

**Phase 4: Testing & Upload** (1 day)
- [ ] Play on real device (AirPlay to Apple TV if possible)
- [ ] Verify playback quality
- [ ] Upload to App Store Connect via fastlane `:deliver` lane
- [ ] Request App Review

---

## Fastlane Integration

Once video is recorded and edited:

```ruby
# fastlane/Fastfile — :deliver_preview lane

lane :deliver_preview do
  # Requires: FAMILY_CONTROLS_ENTITLEMENT_APPROVED=yes
  unless ENV["FAMILY_CONTROLS_ENTITLEMENT_APPROVED"] == "yes"
    UI.error("❌ Entitlement not approved. Cannot upload preview video.")
    UI.abort_with_message!("FamilyControls approval required.")
  end

  deliver(
    app_identifier: "com.example.focalpoint",
    app_review_information: {
      first_name: "Coachy",
      last_name: "Bot",
      phone_number: ENV["CONTACT_PHONE"],
      email_address: "support@focalpoint.app"
    },
    app_preview_update_message: "See Focus Sessions in action.",
    preview_video_path: "apps/ios/FocalPoint/fastlane/preview_video_1_iphone.mp4",
    skip_screenshots: true,
    skip_metadata: true
  )
end
```

---

## FAQ

**Q: Do I need multiple videos?**  
A: No—one video is sufficient. But 2–3 videos (e.g., iPhone + iPad) increase approval chances.

**Q: Can I use video from the App Store of a competitor?**  
A: No. Apple will reject it immediately. Use 100% your own app footage.

**Q: What if Coachy animation isn't ready?**  
A: Record without the flame animation. Focus on the interaction flow (tapping, swiping, rewards). Animation is nice-to-have, not required.

**Q: Can I re-use video from demo or marketing?**  
A: Only if it's 100% actual app footage. No actors, scripts, or staged scenarios.

**Q: How do I distribute the final video to the team?**  
A: Upload to a shared Google Drive or Figma project. Commit the final `.mp4` to a `design-assets/` branch of the repo (don't add large files to main).

---

## Successor Actions

Once entitlement is approved:

1. **Update this spec** with actual shot timing from your app
2. **Record on real device** (iPhone 15 Pro or later)
3. **Edit and color-grade** using DaVinci Resolve (free)
4. **Test on Apple TV** via AirPlay
5. **Upload via fastlane** `:deliver` lane
6. **Monitor for rejection** via App Store Connect notifications

---

## Resources

- **Apple Preview Video Guidelines:** https://developer.apple.com/app-store/app-previews/
- **Royalty-Free Music:** Epidemic Sound, YouTube Audio Library
- **Editing Software:** DaVinci Resolve (free), Final Cut Pro ($300)
- **Fastlane Deliver Docs:** https://docs.fastlane.tools/actions/deliver/
