# Connecting Apple Health to FocalPoint

FocalPoint integrates with Apple HealthKit to read your workouts, sleep, and daily step milestones. This guide walks you through the setup and troubleshooting.

## What FocalPoint Reads

- **Workouts:** Running, walking, cycling, swimming, elliptical, rowing — captured with duration, calories, and distance.
- **Sleep:** Total hours asleep, efficiency, bed time, and wake time.
- **Steps:** Daily step count and milestones when you reach 10,000 steps.
- **Heart Rate:** Resting heart rate only (not minute-by-minute data).

## Setup

### 1. Grant Permissions

1. Open FocalPoint → **Settings** (gear icon)
2. Tap **Connectors** → **Apple Health**
3. Tap **Connect**
4. iOS will prompt: "FocalPoint would like to read your health data"
   - Tap **Allow** (FocalPoint reads only; it does not write to Health)
5. Select which data types to share:
   - Toggle on **Workouts, Sleep, Steps, Heart Rate**
   - Tap **Done**

### 2. Start Syncing

Once connected, FocalPoint will:
- Fetch today's workouts, sleep, and steps every 5 minutes (background)
- Emit **apple-health:workout** events when new workouts are detected
- Emit **apple-health:sleep_reported** events when sleep data arrives
- Emit **apple-health:steps_milestone** events when you reach 10K steps

### 3. Create Rules Using Health Events

In the **Rules** screen, create rules triggered by health events:

```toml
[[rules]]
id = "apple-health-workout-credit"
name = "Credit on Apple Health workout"
priority = 50
enabled = true
trigger = { kind = "event", value = "apple-health:workout" }
actions = [
  { type = "grant_credit", amount = 25 },
]
```

## Troubleshooting

### "Apple Health permission denied"
- Go to **Settings (device) → Privacy → Health → FocalPoint**
- Toggle the required data types back on (they may have been disabled)

### "No workouts showing up"
- Make sure you've logged a workout in the Apple Health app (or via another app like Strava)
- FocalPoint reads from the Health app's database; if the workout isn't there, FocalPoint won't see it
- Try logging a manual workout: **Health → Browse → Workouts → Add Data**

### "Sleep data is empty"
- Apple requires workouts/sleep to be logged manually or via a supported device (Apple Watch, Oura Ring)
- iPhone-only users: manually log sleep in **Health → Browse → Sleep → Add Data**

### "Getting prompted every launch"
- This is normal on first install. After the first permission grant, the prompt disappears
- If it persists, toggle Health → FocalPoint off and back on in device Settings

## Privacy

- **On-device storage:** All health data stays in FocalPoint's local database
- **No sharing:** FocalPoint never sends your health data to servers or third parties
- **You control deletion:** Disconnect Apple Health anytime (Settings → Connectors → Apple Health → Disconnect)
  - Tokens are removed from Keychain immediately
  - Event history remains until manually cleared

## macOS Limitations

- HealthKit is available on macOS (via Designed for iPad), but FocalPoint cannot yet initialize observer queries on macOS
- Apple Health sync is currently **iOS-only**
- Workaround: sync via Fitbit (below) or log workouts manually in the web UI

## See Also

- [Connecting Fitbit](./connect_fitbit.md)
- [Gym Routine Starter Pack](../templates/gym-routine.md)
- [Privacy Policy](../../PRIVACY.md)
