# Connecting Fitbit to FocalPoint

FocalPoint integrates with your Fitbit account to read workouts, sleep, steps, and resting heart rate. This guide covers setup, rate limits, and troubleshooting.

## What FocalPoint Reads

- **Workouts:** Activity type, duration, calories burned, distance (via Fitbit API `/activities/date/today`)
- **Sleep:** Total hours, sleep efficiency, bed/wake times (via `/sleep/date/today`)
- **Steps:** Daily step count and milestones at 10,000 steps
- **Heart Rate:** Resting heart rate (via `/activities/heart/date/today/1d`)

## Setup

### 1. Start OAuth Flow

1. Open FocalPoint → **Settings** (gear icon)
2. Tap **Connectors** → **Fitbit**
3. Tap **Connect**
4. FocalPoint opens Fitbit's login page in a web view
5. Log in with your Fitbit account
6. Grant FocalPoint permission to read activity, sleep, and heart rate data
7. You're redirected back to FocalPoint with an access token

### 2. Sync Begins

Once authorized, FocalPoint will:
- Sync today's Fitbit data immediately
- Check for new data every 5 minutes (background polling)
- Emit **fitbit:workout_completed** events for logged activities
- Emit **fitbit:sleep_reported** events for sleep sessions
- Emit **fitbit:daily_steps_milestone** when steps ≥ 10,000
- Emit **fitbit:heart_rate_resting** for resting heart rate

### 3. Create Rules Using Fitbit Events

```toml
[[rules]]
id = "fitbit-workout-credit"
name = "Credit on Fitbit workout"
priority = 50
enabled = true
trigger = { kind = "event", value = "fitbit:workout_completed" }
actions = [
  { type = "grant_credit", amount = 30 },
]

[[rules]]
id = "fitbit-weekly-streak"
name = "Weekly consistency"
trigger = { kind = "event", value = "fitbit:workout_completed" }
conditions = [
  { kind = "custom", params = { rule = "weekly_count_gte_3" } },
]
actions = [
  { type = "streak_increment", name = "weekly_consistency" },
]
```

## Rate Limits

Fitbit's API enforces a **150 requests per hour, per user** quota.

- FocalPoint makes 3 requests per sync cycle (activities, sleep, heart rate)
- At 5-minute intervals, that's 36 requests/hour — well within the limit
- If you hit the limit, Fitbit returns 429 Too Many Requests; FocalPoint backs off and retries after the reset window

**Tip:** If you run multiple devices with Fitbit connected, the quota is shared across all devices on your account.

## Troubleshooting

### "Authorization failed"
- Check that your Fitbit email/password are correct
- Make sure you completed the OAuth flow (you should see a success message in FocalPoint)

### "No workouts appearing"
- Confirm your workouts are logged in the Fitbit app or a connected device (Fitbit watch, etc.)
- FocalPoint syncs today's data only; workouts from yesterday won't appear in the current sync
- Manually log a test workout in the Fitbit app and wait 5 minutes

### "401 Unauthorized after a few days"
- Fitbit tokens expire in 8 hours by default; FocalPoint should refresh them automatically
- If you see repeated 401 errors, the refresh token may have been revoked
- **Fix:** Go to Settings → Connectors → Fitbit → Disconnect → Reconnect

### "401 Unauthorized from day one"
- Check that your Fitbit client ID and secret are correctly set in the environment
- FocalPoint requires `FOCALPOINT_FITBIT_CLIENT_ID` to be configured at startup
- Contact the FocalPoint team if you're a self-hosted user

### "Getting rate limited (429)"
- Wait 1 hour and try again; Fitbit's quota resets hourly
- If you have multiple devices syncing Fitbit, coordinate to spread requests

## Privacy

- **OAuth tokens:** Stored securely in iOS Keychain; never transmitted to FocalPoint servers
- **Data storage:** All synced data stays in FocalPoint's local database
- **No sharing:** FocalPoint never uploads your health data to any service
- **Disconnection:** Go to Settings → Connectors → Fitbit → Disconnect
  - Access token is revoked and removed immediately
  - Event history retained until manually cleared

## Rate Limit Headers

FocalPoint monitors Fitbit's rate-limit headers:
- `X-RateLimit-Limit: 150`
- `X-RateLimit-Remaining: [count]`
- `X-RateLimit-Reset: [unix-timestamp]`

If remaining quota falls below 5, FocalPoint logs a warning and increases the sync interval to 10 minutes until the quota resets.

## See Also

- [Connecting Apple Health](./connect_apple_health.md)
- [Gym Routine Starter Pack](../templates/gym-routine.md)
- [Privacy Policy](../../PRIVACY.md)
- Fitbit API Docs: https://dev.fitbit.com/api/
