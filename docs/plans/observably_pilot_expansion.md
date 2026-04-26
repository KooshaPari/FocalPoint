# Observably Macro Expansion Plan

## Pilot Results (W-72/73)
Successfully applied `#[observably]` to 3 inherent async fns in focus-sync-core (c040a49). Lessons: macro works well for Result-returning async fns; skip async_trait trait impls entirely.

## Top 10 Expansion Candidates

Ranked by inherent async fn density (non-trait):

| Rank | Crate | Inherent Async Fns | async_trait Blocks | Example Fn | Priority |
|------|-------|-------------------|-------------------|-----------|----------|
| 1 | connector-gcal | 36 | 4 | `refresh_client_token() -> Result<()>` | High |
| 2 | focus-eval | 33 | 4 | `dispatch_actions() -> Result<...>` | High |
| 3 | connector-github | 30 | 3 | `fetch_events() -> Result<Vec<...>>` | High |
| 4 | connector-canvas | 29 | 5 | `sync_assignments() -> Result<()>` | High |
| 5 | focus-rituals | 23 | 6 | `check_ritual_state() -> Result<...>` | Medium |
| 6 | connector-strava | 21 | 2 | `upload_activity() -> Result<String>` | Medium |
| 7 | connector-fitbit | 13 | 3 | `query_heart_rate() -> Result<Data>` | Medium |
| 8 | focus-events | 12 | 2 | `normalize_event() -> Result<Event>` | Medium |
| 9 | connector-readwise | 11 | 1 | `fetch_highlights() -> Result<Vec<...>>` | Low |
| 10 | connector-notion | 11 | 2 | `query_database() -> Result<Vec<...>>` | Low |

## Risk Mitigation

**Before applying to each crate:**
1. Audit `src/lib.rs` and `src/main.rs` for `#[async_trait]` usage
2. Exclude all trait method impls (mock-friendly; trait inflation risk)
3. Apply only to inherent impl blocks with multiple Result-returning async fns

## Suggested Wave Ordering

- **Wave 1 (next 1 week):** connector-gcal, focus-eval (high return, low risk)
- **Wave 2 (next 2 weeks):** connector-github, connector-canvas (high async density, 3-5 trait blocks to skip)
- **Wave 3 (ongoing):** Remaining 6 per quarterly velocity

## Measurable Success

Track per-crate: observability coverage % = (macro-applied fns / total inherent async fns).
Target: 80% Wave 1, 60% Wave 2, 40% backlog.
