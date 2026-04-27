# Lock Screen Widget Guide

FocalPoint supports iOS 16+ lock screen and StandBy widgets for quick at-a-glance access to your focus stats without opening the app.

## Overview

Three lock-screen widget families are available:

### 1. **Accessory Circular** (Circular Badge)

Displays your current focus streak in a circular widget.

- **Size**: Fits perfectly on the lock screen beside the time
- **Display**: Flame icon + streak count (e.g., "🔥 7")
- **Refresh Rate**: Every 15 minutes
- **Best For**: Quick streak check; motivational reminder

<!-- ![](./assets/lock-screen-circular.png) -->

### 2. **Accessory Rectangular** (Text Bar)

Shows credits balance and next focus session time in a single line.

- **Size**: Full-width text bar below the time
- **Display**: "N credits · M min next focus"
- **Refresh Rate**: Every 15 minutes
- **Best For**: Track available credits and session timing

Example: `850 credits · 15 min next`

<!-- ![](./assets/lock-screen-rectangular.png) -->

### 3. **Accessory Inline** (Minimal Text)

Compact streak and credit display in the notification area.

- **Size**: Single line (≤60 characters)
- **Display**: "🔥 7 · 85¢" (streak and credits)
- **Refresh Rate**: Every 15 minutes
- **Best For**: Ultra-compact info; fits anywhere on lock screen

<!-- ![](./assets/lock-screen-inline.png) -->

## Installation

### Add to Lock Screen (iOS 16+)

1. **Long-press** the lock screen
2. Tap **Customize** at the bottom
3. Tap the **+** button to add a widget
4. Search for **FocalPoint**
5. Select one of three widget families:
   - **Streak Badge** (circular)
   - **Credits & Focus** (rectangular)
   - **Streak & Credits** (inline)
6. Tap to add; adjust position as needed

### StandBy (iOS 17+)

Lock-screen widgets automatically appear in StandBy mode when your iPhone is docked and charging.

- **Circular** widgets display prominently in the center
- **Rectangular** widgets stack vertically
- **Inline** widgets appear in the status bar

All three families scale beautifully for the larger StandBy display.

## Data Source

All lock-screen widgets pull from your local FocalPoint database (shared via App Group container). No internet connection required.

- **Streak Count**: From `wallet_streaks` table (focus streak)
- **Credits Balance**: Calculated from wallet `earned - spent`
- **Refresh**: Every 15 minutes or when you interact with the app

## Customization

Widgets respect your system appearance:

- **Light Mode**: High-contrast text on light background
- **Dark Mode**: Light text on dark background
- **Dynamic Type**: Text scales with system accessibility settings

## Troubleshooting

### Widget shows "0 credits" or "0 streak"

- Open the FocalPoint app and check your balance
- Wait 15 minutes for the widget to refresh
- Restart your iPhone if data doesn't update

### Widget missing from lock screen selection

- Ensure you're on iOS 16 or later
- Reinstall FocalPoint and try adding the widget again
- Check that FocalPoint has App Group entitlements

### Streak or credits incorrect

- Verify the data is accurate in the FocalPoint app
- Kill and reopen the FocalPoint app to force a refresh
- Check that your database hasn't been corrupted (rare)

## Performance

Lock-screen widgets are highly optimized:

- **Battery**: Minimal overhead; updates only every 15 minutes
- **Storage**: No additional app data required; uses existing local database
- **Network**: Completely offline; no syncing or API calls
- **Memory**: Lightweight rendering; <2 MB memory footprint per widget

## Privacy

Your lock-screen widget data remains entirely on-device:

- No telemetry or analytics
- No cloud sync (unless you enable it in Settings)
- Database remains encrypted at rest
- Widget cannot access full task list; only aggregated stats
