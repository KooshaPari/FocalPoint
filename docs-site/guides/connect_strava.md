# Connecting Strava

Strava is a platform for logging and tracking athletic activities — runs, cycling, swimming, and more. Connect FocalPoint to Strava to automatically track workout completions and personal records.

## Prerequisites

- A Strava account (free or paid)
- FocalPoint v0.0.1 or later
- iOS 14+ or Android 8+

## What Events Are Generated?

When you connect Strava, FocalPoint listens for two event types:

### 1. **strava:activity_completed**

Fires when you finish any Strava activity (run, ride, swim, workout).

**Event payload includes:**
- Activity name and sport type (Run, Ride, Swim, Workout, etc.)
- Distance (in meters)
- Moving time (in seconds)
- Elevation gain (in meters)
- Average and max speed
- Credit awarded: **25 points**

### 2. **strava:pr_earned**

Fires when an activity sets a new personal record (PR) for any segment.

**Event payload includes:**
- PR count for the activity
- Activity details (name, sport type, distance)
- Credit awarded: **50 points** (recognition of exceptional effort)

## OAuth2 Authorization

FocalPoint uses **OAuth2 authorization code flow** to connect Strava securely.

### Scopes

- `read` — Access your basic athlete info
- `activity:read` — Read your activities and activity details

### Flow

1. Tap "Connect Strava" in the Connectors list
2. Authorize FocalPoint on Strava's OAuth screen (`https://www.strava.com/oauth/authorize`)
3. FocalPoint receives an access token and securely stores it in your device's keychain
4. Sync begins automatically every 5 minutes

## Rate Limiting

Strava enforces rate limits:

- **Per 15 minutes:** 100 API requests
- **Per day:** 1,000 API requests

FocalPoint batches activity syncs and backs off on `429 Too Many Requests` responses. Monitor health in Settings → Connectors → Strava.

## Supported Activities

Strava's `sport_type` field recognizes:

- **Run, Trail Run, Virtual Run**
- **Ride, E-Bike Ride, Gravel Ride, Mountain Bike, Road Bike**
- **Swim, Open Water Swim**
- **Hike, Trail Run, Alpine Ski, Backcountry Ski**
- **Workout, Weight Training, Yoga**
- And 30+ more (full list: [Strava docs](https://developers.strava.com/docs/reference/#api-models-ActivityType))

## Troubleshooting

### "Token invalid or expired"

1. Go to Settings → Connectors → Strava
2. Tap "Reconnect"
3. Re-authorize on Strava's OAuth screen

### "Rate limited (429)"

FocalPoint backs off automatically. Wait ~1 minute and try again. If persistent:

1. Check your Strava app — activities may have synced from a different device
2. Reduce sync frequency in Settings (default: 5 min)
3. Review your API quota at [Strava's developer settings](https://www.strava.com/settings/api)

### Activities not appearing

1. **Ensure activities are marked as private in Strava** — public activities may have privacy filters
2. **Check sync status:** Settings → Connectors → Strava → "Last sync:" should show a recent timestamp
3. **Force sync:** Pull down to refresh in the Activities tab (if available)

## Privacy & Data Storage

- **Your tokens are stored locally** in your device's secure enclave (iOS Keychain / Android Keystore)
- **Activity data is cached locally** in FocalPoint's SQLite store
- **No data is shared** with third parties
- **You can revoke access anytime** in Settings → Connectors → Strava → Disconnect

## See Also

- [Connector Overview](./connector_overview.md)
- [Fitbit Connector](./connect_fitbit.md)
- [Managing Connectors](./managing_connectors.md)
