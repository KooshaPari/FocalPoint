# Sound Sources & Attribution Guide

This file documents where each sound effect and phoneme sample should be sourced from for FocalPoint Tier-0 implementation.

**Status:** Tier-0 ships code only; audio assets are NOT committed to git. Download and place files at build time using the URLs below.

## 19 Sound Effects

Each sound should be encoded as **AAC (m4a) at 48 kHz, 128 kbps**.

| Sound ID | Description | Recommended Source | URL / Search |
|----------|-------------|-------------------|---|
| `session-start-chime` | Clear, high-pitched bell chime | Zapsplat | https://www.zapsplat.com/sound-effect-categories/bells/ (search: "meditation bell strike") |
| `focus-ambient-loop` | Soft ambient drone, 60s loop | Freesound or Zapsplat | Ambient rain loop or brown noise loop (~40 Hz) |
| `session-complete-fanfare` | Triumphant 3-note ascending brass | Zapsplat | https://www.zapsplat.com/sound-effect-categories/success-failure/ |
| `streak-extension-sparkle` | Sparkling, magical ascending tones | Zapsplat | https://www.zapsplat.com/sound-effect-categories/magic-sfx/ |
| `credit-earned-coin-clink` | Bright coin drop/clink sound | Zapsplat | https://www.zapsplat.com/sound-effect-categories/coins-money/ |
| `rule-fire-whoosh` | Swooshing air sound, brief wind gust | Zapsplat | https://www.zapsplat.com/sound-effect-categories/whoosh-sfx/ |
| `penalty-escalates-warning-tone` | Three ascending warning beeps | Freesound or Zapsplat | Alert beep ×3 or microwave beep sound |
| `lockdown-engaged-lock-thunk` | Heavy mechanical lock click | Zapsplat | https://www.zapsplat.com/sound-effect-categories/mechanical-sfx/ |
| `bubble-appear-soft-pop` | Soft, gentle pop sound | Zapsplat | https://www.zapsplat.com/sound-effect-categories/pop-bubble-sfx/ |
| `bubble-dismiss-soft-fade` | Tiny descending tone, pout or sad trombone | Freesound | Search: "sad trombone short" or "pout sound" |
| `coachy-speak-mouth-click` | Tiny lip/mouth click (consonant articulation) | Freesound | Search: "mouth click subtle" |
| `launch-wake-yawn-stretch` | Cute yawn + stretch sound combined | Freesound or Zapsplat | Search: "yawn stretch" or creature yawn |
| `sleep-disappointed-sigh` | Sad sigh sound, cute/cartoonish | Freesound | Search: "sigh disappointed" |
| `auth-sparkle` | Brief shimmer/sparkle (pixie dust) | Zapsplat | https://www.zapsplat.com/sound-effect-categories/magic-sfx/ |
| `sync-whoosh` | Whooshing, data-transfer-like sound | Zapsplat | https://www.zapsplat.com/sound-effect-categories/sci-fi-sfx/ |
| `think-hum` | Soft, intellectual hum (computer thinking) | Freesound | Search: "computer processing hum" or "thinking tone" |
| `task-clink` | Single, bright metallic clink | Zapsplat | https://www.zapsplat.com/sound-effect-categories/metal-sfx/ |
| `install-pop` | Cheerful pop or plop, cork popping | Zapsplat | https://www.zapsplat.com/sound-effect-categories/pop-bubble-sfx/ |
| `stats-whoosh` | Smooth data-transfer whoosh | Zapsplat | https://www.zapsplat.com/sound-effect-categories/sci-fi-sfx/ |

## 16 Simlish Phonemes

Each phoneme should be a clean, isolated sound sample, 0.06–0.15 seconds long, at **48 kHz, 16-bit PCM WAV**.

| Phoneme | Filename | Source Strategy | Notes |
|---------|----------|-----------------|-------|
| /m/ | `phoneme-m.m4a` | Record "mmmm" sustain or synthesize | 100–200 Hz, low hum, 0.1s |
| /a/ | `phoneme-a.m4a` | Record "ahhh" sustain or synthesize | 200–400 Hz, open vowel, 0.12s |
| /b/ | `phoneme-b.m4a` | Record "buh" stop or synthesize | 150–300 Hz, 0.08s |
| /e/ | `phoneme-e.m4a` | Record "ehhh" sustain or synthesize | 300–500 Hz, 0.12s |
| /i/ | `phoneme-i.m4a` | Record "eee" sustain or synthesize | 400–600 Hz, high vowel, 0.12s |
| /g/ | `phoneme-g.m4a` | Record "guh" velar stop or synthesize | 100–300 Hz, 0.08s |
| /o/ | `phoneme-o.m4a` | Record "ohh" sustain or synthesize | 200–400 Hz, 0.12s |
| /u/ | `phoneme-u.m4a` | Record "ooo" sustain or synthesize | 100–300 Hz, low vowel, 0.12s |
| /n/ | `phoneme-n.m4a` | Record "nnn" nasal or synthesize | 100–200 Hz, 0.1s |
| /p/ | `phoneme-p.m4a` | Record "puh" bilabial stop or synthesize | 150–300 Hz, 0.06s |
| /ʃ/ (sh) | `phoneme-sh.m4a` | Record "shhhh" fricative or synthesize | 2000–4000 Hz, high-freq hiss, 0.15s |
| /t/ | `phoneme-t.m4a` | Record "tuh" alveolar stop or synthesize | 200–400 Hz, 0.06s |
| /d/ | `phoneme-d.m4a` | Record "duh" voiced alveolar or synthesize | 200–400 Hz, 0.08s |
| /w/ | `phoneme-w.m4a` | Record "wuh" glide or synthesize | 100–300 Hz, 0.08s |
| /l/ | `phoneme-l.m4a` | Record "luh" lateral or synthesize | 200–400 Hz, 0.10s |
| /sil/ | `phoneme-sil.m4a` | 50ms silence | Zero samples, 0.05s |

### Phoneme Sourcing Strategies

1. **Record in-house** (1–2 hours effort):
   - Use Audacity (free) or a DAW like Reaper.
   - Record each phoneme as a clean sample.
   - Trim to the durations above.
   - Export as WAV, then convert to m4a via ffmpeg.

2. **Synthesize with AVSpeechSynthesizer** (30 min effort):
   - Write a Swift script that generates each phoneme using `AVSpeechSynthesizer`.
   - Output to WAV, then convert to m4a.
   - Example approach: create a test app that iterates through phoneme strings, records them, saves files.

3. **Search Freesound / Splice packs** (30 min effort):
   - Many producers share isolated phoneme sets.
   - Search for "phoneme pack" or "voice synthesis samples".
   - Ensure Creative Commons or commercial-use license.

**Recommendation for Tier-0:** Use option 2 (synthesize with AVSpeechSynthesizer). Fastest ROI.

## Build-Time Audio Injection Script

During app build (as a Run Script phase in Xcode), download or generate these files:

```bash
#!/bin/bash
# scripts/inject-audio.sh — Run as a build phase (Xcode target → Build Phases → New Run Script Phase)

AUDIO_DIR="apps/ios/FocalPoint/Resources/Audio"
SFX_DIR="$AUDIO_DIR/SFX"
PHONEME_DIR="$AUDIO_DIR/Simlish"

mkdir -p "$SFX_DIR" "$PHONEME_DIR"

# TODO: Add script to download from Zapsplat / Freesound
# For Tier-0 MVP, this is stubbed.
# Audio engineer will manually place files here.

echo "Audio injection complete (stub)"
```

## Privacy & Licensing

- **Zapsplat:** Free tier (non-commercial) or $30/year commercial license.
- **Freesound:** Creative Commons licensed; check specific license of each sound (most allow commercial reuse with attribution).
- **Splice:** $9–15/month subscription; covers commercial use of all sounds.

**FocalPoint approach:** Start with Zapsplat free tier for MVP. Upgrade to commercial license once past beta.

## Attribution

All sourced sounds must be credited in the app (either in Settings → About or in bundled `ATTRIBUTIONS.txt`). Check the license of each sound for attribution requirements.
