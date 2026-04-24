# Demo Mode: Populating FocalPoint with Fixture Data

For designers, testers, and screenshot generation, FocalPoint includes a **demo mode** that seeds realistic fixture data without needing actual account integrations.

## Quick Start

### CLI: `focus demo seed`

Populate the database with demo data:

```bash
# Seed the default database (~/.../focalpoint/core.db)
focus demo seed

# Or specify a path (useful for screenshot fixtures)
focus demo seed --db /path/to/test.db
```

**What gets seeded:**
- **10 example tasks** (varied priorities, due dates)
- **5 example rules** (Canvas, GitHub, Fitbit rewards, streak bonuses)
- **Wallet:** 85 credits + 7-day focus streak
- **3 connector configs:** GitHub, Canvas, Fitbit (marked "connected")
- **~30 audit records** across 14 days (wallet grants, sessions, rule fires)
- **14 ritual completions:** 7 days × 2 rituals (morning brief + evening shutdown)

All demo records are marked with a `demo_marker` flag in the audit log, so they can be selectively reset without affecting real user data.

### iOS Settings: Developer Toggle

In **Settings > Developer (DEBUG)**, two buttons appear:

- **Load demo data:** Calls `focus demo seed` (placeholder in v0.0.1)
- **Reset demo data:** Calls `focus demo reset` (placeholder in v0.0.1)

## Fastlane Screenshots

The `fastlane screenshots` lane automatically seeds demo data before building the app:

```bash
cd apps/ios/FocalPoint
fastlane screenshots
```

After screenshots are captured, demo data is reset automatically.

## Reset Demo Data

Clear demo data while preserving real user data:

```bash
focus demo reset

# Or with a specific database path
focus demo reset --db /path/to/test.db
```

## Design & Testing Workflow

1. Start with a fresh database: `focus demo seed`
2. Verify UI with populated tasks, rewards, and audit trail
3. Run snapshot tests or manual screenshots
4. Reset: `focus demo reset`
5. Verify non-demo data remains (if you had any)

## Implementation Status

- ✅ Rust seed harness (`focus-demo-seed` crate, 6+ tests)
- ✅ CLI subcommands `focus demo seed` and `focus demo reset`
- ✅ iOS Settings Developer toggle (DEBUG only)
- ✅ Fastlane integration points
- ⏳ FFI bindings (phase 2): wire iOS buttons to Rust implementation
- ⏳ SQLite data insertion (phase 2): seed actual task/rule/wallet rows

See: [FR-DEMO-001](https://github.com/KooshaPari/FocalPoint/issues) for full spec.
