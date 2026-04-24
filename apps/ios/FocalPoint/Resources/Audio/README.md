# Audio Assets — FocalPoint Tier-0

This directory contains sound effects and voice samples for the FocalPoint mascot system.

## ⚠️ IMPORTANT: Audio Files Not Committed to Git

Audio assets (`.m4a` files) are **NOT** committed to this repository. They are:

1. **Large** (binary files, ~50–100 KB each).
2. **Generated at build time** or manually placed by the audio engineer.
3. **Gracefully handled if missing** — the app degrades to silent mode.

## Directory Structure

```
Resources/Audio/
  SFX/                          # 19 sound effects (*.m4a files)
    session-start-chime.m4a
    focus-ambient-loop.m4a
    session-complete-fanfare.m4a
    ... (16 more)
  
  Simlish/                      # 16 phoneme samples (*.m4a files)
    phoneme-m.m4a
    phoneme-a.m4a
    ... (14 more)
  
  SOUND_SOURCES.md              # Sourcing guide (this file)
  README.md                      # You are here
```

## Adding Audio Files

### Option 1: Manual Placement (Fastest for MVP)

1. Open `SOUND_SOURCES.md` and locate the sound you need.
2. Download or record the audio sample.
3. Convert to **AAC (m4a) at 48 kHz, 128 kbps** using ffmpeg:
   ```bash
   ffmpeg -i input.wav -c:a aac -b:a 128k -sample_rate 48000 output.m4a
   ```
4. Place the file in the appropriate subdirectory (`SFX/` or `Simlish/`).
5. **Add to Xcode project:**
   - Drag the `.m4a` file into the `FocalPoint` target in Xcode.
   - Ensure "Copy items if needed" is checked.
   - Target: `FocalPoint` (not FocalPointTests or other targets).

### Option 2: Automated Download (Future CI Integration)

Create a build phase script (see `scripts/inject-audio.sh` in `SOUND_SOURCES.md`).

## Missing Audio Handling

If a sound file is missing at runtime, the app:

1. **Logs a warning** to the console (once per file).
2. **Continues silently** — no crash, no user-facing error.
3. **Still plays haptics** (if enabled) — users feel feedback even without sound.

This ensures the app is always usable, even during audio asset development.

## Verification

To verify all expected audio files are present:

```bash
# From the FocalPoint root:
ls Resources/Audio/SFX/*.m4a | wc -l    # Should be 19
ls Resources/Audio/Simlish/*.m4a | wc -l # Should be 16
```

## Build & App Bundle

- **App bundle size impact:** ~1.4 MB for all audio (900 KB SFX + 480 KB phonemes).
- **Load time:** Minimal; files are loaded on-demand or cached.
- **User-facing:** Users do NOT download audio separately; it's bundled with the app.

## Further Reading

- Sourcing details: `SOUND_SOURCES.md`
- Specification: `docs/mascot/sound_design_spec_2026_04.md` (project root)
- Playback code: `Sources/MascotUI/Audio/SoundEffectPlayer.swift`
