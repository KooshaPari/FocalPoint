# Connector Domain Catalog

> Aspirational target list for FocalPoint's connector ecosystem. Traces to
> **FR-ECO-CATALOG-001**. See `STRATEGY.md` for the classification model and
> `crates/focus-connectors/src/lib.rs` for the `VerificationTier` enum.
>
> **No Rust code changes** accompany this doc — this is a roadmap artifact.
> Every connector listed here is a future crate (or MCP mapping, or local
> webhook), not an existing one. Canvas is the only shipped connector today.

## Summary matrix

| Connector | Tier | Auth | Events | Risk | Migrates from |
|---|---|---|---|---|---|
| Canvas LMS | Official | OAuth2 (`url:GET`, `url:POST`) | 4 | Low | Canvas mobile |
| Blackboard Learn | Verified | OAuth2 (REST learn-api) | 4 | Medium | Bb mobile |
| Gradescope | Verified | Scraped session cookie | 3 | High | Gradescope web |
| Khan Academy | Verified | OAuth1.0a (legacy) | 3 | Medium | KA app |
| MacroFactor | Verified | API key (beta) + export | 4 | High | MacroFactor app |
| FlexAI | Verified | API key / webhook | 3 | High | FlexAI app |
| Strava | Verified | OAuth2 (`activity:read_all`) | 5 | Low | Strava widget |
| Whoop | Verified | OAuth2 (`read:recovery`, `read:sleep`, `read:workout`) | 5 | Low | Whoop widget |
| Apple Health | Official | HealthKit entitlement (on-device) | 6 | Low | iOS Health widget |
| YNAB | Verified | OAuth2 PAT (`read-only`) | 4 | Low | YNAB mobile |
| Copilot Money | Verified | Export webhook (no public API) | 3 | High | Copilot app |
| Monarch Money | Verified | Unofficial GraphQL | 3 | High | Monarch app |
| Google Calendar | Verified | OAuth2 (`calendar.readonly`, `calendar.events`) | 4 | Low | GCal widget |
| Apple EventKit | Official | EventKit entitlement (on-device) | 4 | Low | iOS Calendar |
| Outlook / MS Graph | Verified | OAuth2 (`Calendars.Read`) | 4 | Low | Outlook |
| Todoist | Verified | OAuth2 (`data:read`) | 4 | Low | Todoist |
| TickTick | Verified | OAuth2 (`tasks:read`) | 4 | Medium | TickTick |
| Reclaim.ai | Verified | API key | 3 | Medium | Reclaim (subsumed) |
| Sunsama | Verified | No public API; export JSON | 3 | High | Sunsama (subsumed) |
| GitHub | Verified | OAuth2 (`repo`, `read:user`) | 5 | Low | GH mobile |
| Linear | Verified | OAuth2 (`read`) | 4 | Low | Linear |
| Any MCP server | MCP-bridged | MCP session (stdio/http) | N (mapped) | Medium | Arbitrary |
| Home Assistant | Private | Local webhook + long-lived token | 3 | Low | HA companion |
| Custom-CLI webhook | Private | Localhost HTTP + shared secret | N | Low | Ad-hoc scripts |

Event counts are nominal per-connector canonical event types; MCP and
custom-CLI are user-defined.

---

## Learning

### Canvas LMS — Official
Homepage: <https://www.instructure.com/canvas> · Docs: <https://canvas.instructure.com/doc/api/>

- **VerificationTier:** Official (already shipped — `crates/connector-canvas`)
- **Auth:** OAuth2 Authorization Code. Instance base URL per school.
- **Canonical events:**
  - `canvas:assignment_due_soon`
  - `canvas:assignment_graded`
  - `canvas:announcement_posted`
  - `canvas:submission_comment`
- **Payload sketch:**
  ```yaml
  assignment_name: "PHYS 221 HW 7"
  course_code: "PHYS221"
  due_at: "2026-04-25T23:59:00Z"
  due_at_human: "tomorrow 11:59 PM"
  points_possible: 50
  submission_status: "unsubmitted"
  ```
- **Templates:**
  ```yaml
  - name: "Canvas: lock social when assignment due <24h"
    trigger: "canvas:assignment_due_soon"
    conditions: [{ kind: "confidence_gte", params: { min: 0.8 } }]
    actions: [{ kind: "block", profile: "social", duration_seconds: 3600, rigidity: "hard" }]
    explanation: "Instagram stays blocked because {event.payload.assignment_name} is due {event.payload.due_at_human}."
    coachy: "stern/focus"
  - name: "Canvas: celebrate when graded A"
    trigger: "canvas:assignment_graded"
    conditions: [{ kind: "payload_gte", params: { field: "score_ratio", min: 0.9 } }]
    actions: [{ kind: "reward_xp", amount: 25 }]
    explanation: "Nice — {event.payload.assignment_name} graded {event.payload.score_ratio}."
    coachy: "proud/celebrate"
  ```
- **Why:** Students spend the most blockable time near assignment deadlines.
- **Replaces/complements:** Canvas mobile push notifications (complements).
- **Risk:** Low — stable API, pagination well-documented.

### Blackboard Learn — Verified
Homepage: <https://www.blackboard.com> · Docs: <https://developer.blackboard.com/portal/displayApi>

- **Auth:** OAuth2 via Learn REST API. 3LO app registration per institution.
- **Events:** `blackboard:assignment_due_soon`, `blackboard:grade_posted`, `blackboard:announcement_posted`, `blackboard:discussion_reply`
- **Payload:** `{ course_id, title, due_at, grade?, category }`
- **Templates:**
  ```yaml
  - name: "Blackboard: study mode when >2 assignments due this week"
    trigger: "blackboard:assignment_due_soon"
    conditions: [{ kind: "window_count_gte", params: { window_hours: 168, min: 2 } }]
    actions: [{ kind: "activate_profile", profile: "study", rigidity: "semi" }]
    explanation: "You have {window_count} assignments due this week."
    coachy: "stern/focus"
  ```
- **Why:** Bb is the second-largest US LMS; parity with Canvas is table-stakes.
- **Risk:** Medium — per-tenant OAuth apps, rate limits per institution.

### Gradescope — Verified
Homepage: <https://www.gradescope.com>

- **Auth:** No public API. Session-cookie scraping or user-uploaded export.
- **Events:** `gradescope:submission_graded`, `gradescope:regrade_response`, `gradescope:assignment_released`
- **Payload:** `{ course, assignment, score, total, released_at }`
- **Templates:** grade-posted celebration; regrade-response surfaces as a
  notification-only rule (no block).
- **Why:** Dominates CS/engineering grading.
- **Risk:** High — scraping breaks on UI changes; no stable contract.

### Khan Academy — Verified
Homepage: <https://www.khanacademy.org> · Docs: <https://api-explorer.khanacademy.org/>

- **Auth:** OAuth1.0a (legacy).
- **Events:** `khan:streak_at_risk`, `khan:mastery_achieved`, `khan:skill_completed`
- **Payload:** `{ skill_slug, mastery_level, streak_days }`
- **Templates:** "block social until 15 min of Khan done"; streak-at-risk ping.
- **Why:** K-12 parent + self-learner segment.
- **Risk:** Medium — OAuth1 is clunky; API is nominally deprecated but still works.

---

## Fitness & nutrition

### MacroFactor — Verified
Homepage: <https://macrofactorapp.com/>

- **Auth:** Currently no official public API. Beta partner program + CSV
  export. Treat as "user imports daily digest" for now.
- **Events:** `macrofactor:daily_macros_hit`, `macrofactor:daily_macros_missed`, `macrofactor:weight_logged`, `macrofactor:expenditure_adjusted`
- **Payload:**
  ```yaml
  date: "2026-04-23"
  protein_g: 180
  protein_target_g: 170
  calories: 2410
  calories_target: 2500
  adherence: "hit"
  ```
- **Templates:**
  ```yaml
  - name: "MacroFactor: dessert-app block when under-protein"
    trigger: "macrofactor:daily_macros_missed"
    conditions: [{ kind: "payload_lt", params: { field: "protein_g", max: 120 } }]
    actions: [{ kind: "block", profile: "doordash", duration_seconds: 14400, rigidity: "semi" }]
    explanation: "Protein was {event.payload.protein_g}g / {event.payload.protein_target_g}g. Eat real food before ordering dessert."
    coachy: "stern/mild"
  - name: "MacroFactor: reward streak on hit"
    trigger: "macrofactor:daily_macros_hit"
    actions: [{ kind: "reward_xp", amount: 10 }]
    explanation: "Macros hit — streak at {event.payload.streak_days}."
    coachy: "proud/cheer"
  ```
- **Why:** Evidence-based nutrition coaching; FocalPoint can make adherence
  enforceable, not just visible.
- **Risk:** High — no public API; depends on export or partnership.

### FlexAI — Verified
Homepage: <https://flex.ai>

- **Auth:** API key or webhook delivery from FlexAI workouts.
- **Events:** `flexai:workout_logged`, `flexai:workout_skipped`, `flexai:pr_achieved`
- **Payload:** `{ session_id, duration_min, volume_lbs, pr?: {lift, weight} }`
- **Templates:** reward on `workout_logged`; shame-block social on 3-day skip.
- **Why:** Adaptive hypertrophy programming — adherence matters.
- **Risk:** High — early-stage company, API may not be public yet.

### Strava — Verified
Homepage: <https://strava.com> · Docs: <https://developers.strava.com/>

- **Auth:** OAuth2, scopes `activity:read_all`, `profile:read_all`.
- **Events:** `strava:activity_recorded`, `strava:weekly_volume_low`, `strava:kudos_received`, `strava:pr_achieved`, `strava:streak_at_risk`
- **Payload:** `{ activity_type, distance_m, moving_time_s, elevation_gain_m, suffer_score? }`
- **Templates:** morning run → unlock social for 30 min; weekly-volume-low → block Netflix.
- **Why:** Canonical endurance-athlete connector.
- **Risk:** Low — well-documented, rate-limits are 200/15min & 2k/day.

### Whoop — Verified
Homepage: <https://whoop.com> · Docs: <https://developer.whoop.com/>

- **Auth:** OAuth2, scopes `read:recovery`, `read:sleep`, `read:workout`, `read:cycles`.
- **Events:** `whoop:recovery_low`, `whoop:sleep_poor`, `whoop:strain_high`, `whoop:workout_logged`, `whoop:cycle_start`
- **Payload:** `{ recovery_score, hrv_ms, rhr_bpm, sleep_performance, strain }`
- **Templates:**
  ```yaml
  - name: "Whoop: force wind-down when recovery red"
    trigger: "whoop:recovery_low"
    conditions: [{ kind: "payload_lt", params: { field: "recovery_score", max: 34 } }]
    actions: [{ kind: "activate_profile", profile: "wind_down", rigidity: "hard" }]
    explanation: "Recovery {event.payload.recovery_score}%. Phone goes to bed at 10pm tonight."
    coachy: "gentle/firm"
  ```
- **Why:** Quantified recovery → bedtime enforcement; the killer app.
- **Risk:** Low — stable OAuth, 100 req/min.

### Apple Health — Official
Homepage: <https://developer.apple.com/health-fitness/>

- **Auth:** On-device HealthKit entitlement. No OAuth; user grants per-type.
- **Events:** `applehealth:sleep_under_target`, `applehealth:steps_target_hit`, `applehealth:mindful_minutes_logged`, `applehealth:workout_logged`, `applehealth:heart_rate_elevated`, `applehealth:standing_goal_missed`
- **Payload:** `{ metric, value, unit, target?, date }`
- **Templates:** sleep-under-target → next-day caffeine-app soft-block;
  mindful-minutes → unlock reward.
- **Why:** On-device, privacy-preserving, superset of most wearables.
- **Risk:** Low — shipping native, no external API.

---

## Financial

### YNAB — Verified
Homepage: <https://ynab.com> · Docs: <https://api.ynab.com/>

- **Auth:** Personal Access Token or OAuth2. Read-only scope.
- **Events:** `ynab:category_overspent`, `ynab:transaction_large`, `ynab:age_of_money_low`, `ynab:budget_reconciled`
- **Payload:** `{ category_name, budgeted, activity, balance, month }`
- **Templates:**
  ```yaml
  - name: "YNAB: block shopping apps when Dining overspent"
    trigger: "ynab:category_overspent"
    conditions: [{ kind: "payload_eq", params: { field: "category_name", eq: "Dining Out" } }]
    actions: [{ kind: "block", profile: "shopping", duration_seconds: 86400, rigidity: "semi" }]
    explanation: "Dining Out is {event.payload.balance} over. No Amazon until month rollover."
    coachy: "stern/adult"
  ```
- **Why:** Envelope budgeting → enforceable spending discipline.
- **Risk:** Low — stable, generous rate limit (200/hour).

### Copilot Money — Verified
Homepage: <https://copilot.money>

- **Auth:** No public API. Scheduled email export or manual JSON webhook.
- **Events:** `copilot:category_overspent`, `copilot:transaction_large`, `copilot:recurring_detected`
- **Payload:** `{ merchant, amount_usd, category, recurring? }`
- **Templates:** same shape as YNAB; user chooses one.
- **Why:** iOS-first budgeting; younger demo than YNAB.
- **Risk:** High — no public API, export-driven.

### Monarch Money — Verified
Homepage: <https://monarchmoney.com>

- **Auth:** Unofficial GraphQL (community-reverse-engineered). Not officially
  supported.
- **Events:** `monarch:budget_overspent`, `monarch:net_worth_drop`, `monarch:transaction_large`
- **Payload:** `{ category, budgeted, spent, net_worth? }`
- **Templates:** budget-overspent → shopping block; net-worth-drop → pause
  crypto-app access.
- **Risk:** High — TOS-adjacent; may break without notice.

---

## Calendar & task

### Google Calendar — Verified
Homepage: <https://calendar.google.com> · Docs: <https://developers.google.com/calendar/api>

- **Auth:** OAuth2, scopes `calendar.readonly` (sync) + `calendar.events` (write, optional).
- **Events:** `gcal:event_starting_soon`, `gcal:focus_block_start`, `gcal:meeting_ended`, `gcal:day_overbooked`
- **Payload:** `{ event_id, title, start, end, attendees_count, calendar_name }`
- **Templates:** focus-block → profile activation; day-overbooked → pre-emptive wind-down scheduling.
- **Why:** Primary time ground-truth for most users.
- **Risk:** Low.

### Apple EventKit — Official
Homepage: <https://developer.apple.com/documentation/eventkit>

- **Auth:** On-device EventKit entitlement.
- **Events:** `eventkit:event_starting_soon`, `eventkit:focus_block_start`, `eventkit:reminder_due`, `eventkit:event_ended`
- **Payload:** identical shape to gcal.
- **Why:** On-device, merges iCloud + Exchange + Google without per-account OAuth.
- **Risk:** Low.

### Outlook / MS Graph — Verified
Docs: <https://learn.microsoft.com/en-us/graph/api/resources/calendar>

- **Auth:** OAuth2, scopes `Calendars.Read`, optionally `Mail.Read`.
- **Events:** `msgraph:event_starting_soon`, `msgraph:focus_time_start`, `msgraph:teams_meeting_joined`, `msgraph:email_flagged`
- **Payload:** `{ event_id, subject, start, end, is_teams }`
- **Why:** Enterprise users live here.
- **Risk:** Low — tenant-consent can be a UX hurdle.

### Todoist — Verified
Docs: <https://developer.todoist.com/rest/v2/>

- **Auth:** OAuth2, scope `data:read` (or `data:read_write`).
- **Events:** `todoist:task_overdue`, `todoist:task_completed`, `todoist:due_today_count_high`, `todoist:priority_1_added`
- **Payload:** `{ task_id, content, project, priority, due, labels[] }`
- **Templates:** overdue P1 → social block; completed → XP reward.
- **Risk:** Low.

### TickTick — Verified
Docs: <https://developer.ticktick.com/api>

- **Auth:** OAuth2, scope `tasks:read`.
- **Events:** `ticktick:task_overdue`, `ticktick:habit_streak_at_risk`, `ticktick:pomodoro_completed`
- **Why:** Pomodoro + habit tracking → natural fit.
- **Risk:** Medium — API documentation thinner than Todoist.

### Reclaim.ai — Verified
Homepage: <https://reclaim.ai>

- **Auth:** API key.
- **Events:** `reclaim:focus_scheduled`, `reclaim:habit_at_risk`, `reclaim:task_reshuffled`
- **Why:** FocalPoint rituals + scheduler subsume Reclaim's core; this
  connector is a migration bridge for existing Reclaim users.
- **Risk:** Medium.

### Sunsama — Verified
Homepage: <https://sunsama.com>

- **Auth:** No public API. Daily-digest email export or JSON drop.
- **Events:** `sunsama:day_planned`, `sunsama:day_reviewed`, `sunsama:task_rolled_over`
- **Why:** **Sunsama's morning-planning + shutdown rituals are subsumed by
  FocalPoint's ritual engine.** Connector acts as a migration path.
- **Risk:** High — export-driven.

---

## Code & work

### GitHub — Verified
Docs: <https://docs.github.com/en/rest>

- **Auth:** OAuth2, scopes `repo`, `read:user`. PAT fallback.
- **Events:** `github:pr_opened`, `github:pr_review_requested`, `github:ci_failed`, `github:streak_at_risk`, `github:commit_pushed`
- **Payload:** `{ repo, pr_number?, sha?, status?, contributions_today }`
- **Templates:** contributions_today == 0 by 6pm → block social; ci_failed →
  nudge (no block).
- **Risk:** Low — primary rate limit is 5k/hr authenticated.

### Linear — Verified
Docs: <https://developers.linear.app/docs/graphql/working-with-the-graphql-api>

- **Auth:** OAuth2 (GraphQL), scope `read`.
- **Events:** `linear:issue_assigned`, `linear:issue_due_soon`, `linear:cycle_starting`, `linear:blocked_for_n_days`
- **Payload:** `{ issue_id, title, state, priority, cycle, due_at }`
- **Templates:** cycle-starting → study/work profile activation.
- **Risk:** Low.

---

## Wildcard: MCP-bridged

### Any MCP server — MCP-bridged
Spec: <https://modelcontextprotocol.io>

- **Tier:** MCP-bridged (see `focus-connectors::mcp_bridge::MCPBridgedConnector`).
- **Auth:** MCP session over stdio or HTTP; whatever the server demands
  (bearer, mTLS, local pipe).
- **Events:** user-defined. The user picks a tool or resource, maps its JSON
  output to an event type string + payload schema. No canonical vocabulary.
- **Example:** a hypothetical `weather-mcp` exposes `get_daily_forecast`.
  User maps its `precip_mm` field to trigger event
  `weather:rain_today_over_5mm` whenever `precip_mm >= 5`.
- **Payload sketch:** exactly whatever the MCP tool returns, optionally
  reshaped by the mapping config.
- **Templates:**
  ```yaml
  - name: "Weather: unlock gaming when rain >5mm"
    trigger: "weather:rain_today_over_5mm"
    actions: [{ kind: "unlock", profile: "gaming", duration_seconds: 7200, rigidity: "semi" }]
    explanation: "Rain day: {event.payload.precip_mm}mm forecast. Go play."
    coachy: "warm/permit"
  ```
- **Why:** Open-ended. Anything with an MCP adapter becomes a FocalPoint signal.
- **Risk:** Medium — server quality varies; health indicators must surface that.

---

## Private / self-hosted

### Home Assistant — Private
Homepage: <https://www.home-assistant.io>

- **Auth:** Local long-lived access token; FocalPoint exposes a localhost
  webhook HA posts to (automation action).
- **Events:** `homeassistant:presence_home`, `homeassistant:study_lamp_on`, `homeassistant:sleep_mode_activated`
- **Payload:** whatever HA sends; typically `{ entity_id, state, attributes }`.
- **Templates:** study_lamp_on → focus profile; presence != home → disable parental enforcement.
- **Risk:** Low — runs on LAN.

### Custom-CLI webhook — Private
- **Auth:** Localhost HTTP + per-connector shared-secret header.
- **Events:** user-defined `customcli:*`.
- **Payload:** free-form JSON matching `NormalizedEvent`.
- **Why:** Escape hatch for power users with bash/Python/Rust scripts.
- **Risk:** Low — everything stays on-device.

---

## Next steps — the 3 connectors after Canvas

Pick these in order:

1. **Google Calendar** — highest leverage, cleanest OAuth2 docs, overlaps with
   ritual/scheduler FRs already in flight. Unlocks the focus-block
   activation pattern that makes every other connector more useful. Risk low.
2. **Whoop** — demonstrates "signal outside screen → enforceable wind-down",
   which is the clearest narrative differentiator vs. Apple Screen Time /
   Opal / Foqos. Stable OAuth, small scope surface.
3. **GitHub contributions** — validates the dev/work vertical and exercises
   the streak-at-risk template shape, which will generalize to Strava, Whoop,
   and Khan Academy. Free OAuth, no per-tenant setup.

Deferred-but-easy: Todoist is a reasonable swap for GitHub if the target
audience tilts non-developer.

---

## Provenance

- Canvas: <https://canvas.instructure.com/doc/api/>
- Blackboard: <https://developer.blackboard.com/portal/displayApi>
- Khan Academy: <https://api-explorer.khanacademy.org/>
- Strava: <https://developers.strava.com/>
- Whoop: <https://developer.whoop.com/>
- Apple HealthKit: <https://developer.apple.com/documentation/healthkit>
- YNAB: <https://api.ynab.com/>
- Google Calendar: <https://developers.google.com/calendar/api>
- MS Graph Calendar: <https://learn.microsoft.com/en-us/graph/api/resources/calendar>
- Apple EventKit: <https://developer.apple.com/documentation/eventkit>
- Todoist: <https://developer.todoist.com/rest/v2/>
- TickTick: <https://developer.ticktick.com/api>
- GitHub REST: <https://docs.github.com/en/rest>
- Linear GraphQL: <https://developers.linear.app/docs/graphql/working-with-the-graphql-api>
- Model Context Protocol: <https://modelcontextprotocol.io>
- Home Assistant REST: <https://developers.home-assistant.io/docs/api/rest/>

Connectors without cited public API docs (MacroFactor, FlexAI, Copilot,
Monarch, Sunsama, Gradescope) are flagged as **High** integration risk and
assume scraping, export, or partnership paths.
