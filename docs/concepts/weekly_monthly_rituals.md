# Weekly & Monthly Rituals: Cadence for Sustained Productivity

FocalPoint extends the daily Morning Brief and Evening Shutdown with weekly and monthly review cadences, turning daily discipline into compounding growth. Every serious productivity system needs these horizons — they're where you notice patterns, celebrate wins, and course-correct.

## The Weekly Review

Every Sunday (or on-demand), the `WeeklyReviewEngine` synthesizes a week's worth of focus time, task completion, rule firings, and streak data into a narrative that's both motivating and honest.

**What It Includes**

- **Focus Hours & Session Count**: Total hours in focus blocks + count of distinct sessions. Stub data shows ~12.5h/week across 8 sessions (your real data comes from the event store).
- **Credits Earned/Spent**: Wallet deltas. Rewards from rules firing + penalties applied during the week.
- **Top Rules**: The 2-3 rule-firing patterns that dominated the week (e.g., "Deep Work (4h+) fired 5 times").
- **Streaks Extended**: Which streaks grew this week. The focus streak at 12 days means you shipped focus time 12 consecutive days.
- **Tasks Shipped/Slipped**: The scorecard. 18 shipped, 3 slipped. Both the wins and the gaps.
- **Wins Summary**: LLM-synthesized narrative of the week's wins. If coaching is disabled (`FOCALPOINT_DISABLE_COACHING=1`), a static template: "12.5h focused, 18 tasks shipped. Solid week."
- **Growth Area**: One specific improvement target for next week, derived from slip patterns. If zero slipped, it suggests "Increase task complexity — you're ready for bigger challenges." If 3+ slipped, it says "Reduce slip rate — focus on estimation or scope."
- **Coachy Closing**: A short, encouraging line (≤80 chars). Stub: "Strong week ahead. Keep the streak alive."

## The Monthly Retrospective

On the last day of the month (or on-demand), the `MonthlyRetrospectiveEngine` aggregates the month's weeks into a macro view: trend detection, theme identification, and reflection.

**What It Includes**

- **Month & Total Focus Hours**: "2026-05", 52.0 hours total. Stub shows 4 weeks of breakdowns: [11.5, 12.0, 13.5, 15.0] — an upward trend.
- **Weekly Breakdown**: A Vec of weekly totals, so you can see the tempo change across the month. Week 4 jumped to 15h — why? Growth? Deadline sprint? The breakdown surface lets you ask the right questions.
- **Theme**: A one-word or two-word summary (e.g., "Momentum", "Execution", "Growth"). Derived from focus totals and task completion counts. Real logic: if >50h + >60 tasks, it's "Momentum"; if >80 tasks, "Execution"; else "Growth".
- **Top Accomplishments**: Bullet-list of the month's key wins. Stub shows 3: "Shipped 68 tasks across 4 weeks", "Extended focus streak to 20 days", "Maintained 100% morning ritual compliance".
- **Compared to Prior Month**: Month-over-month deltas. Focus hours (+4.5h), tasks completed (+12), credits earned (+60), and a trend direction ("up", "down", "stable"). This is where you see real growth.
- **Streak Peak**: The highest count any streak reached. "Focus: 20 consecutive days (Apr 11–May 1)".
- **Coachy Reflection**: A 100-char coached reflection on the theme and next month's focus. Stub: "You built real momentum this month. Keep the streak alive—compound wins."

## Local-First LLM Integration

Both engines route all text generation through `complete_guarded`, which:

1. Calls the coaching provider (if configured and FOCALPOINT_DISABLE_COACHING is not set).
2. Falls back to static templates if the provider returns None or coaching is disabled.
3. Enforces token limits (80 chars for closings, 120 chars for summaries).

**Why deterministic fallback?** Because you should never see a blank "coachy" field. If the LLM is unavailable (no internet, API overloaded, kill switch on), the app still ships a coherent, motivating review. The UX doesn't degrade to "waiting for Claude..."

## Privacy & Audit Trail

- All aggregations (focus hours, task counts, rule firings, streak growth) are derived from your local audit chain and event store.
- No data leaves the device unless you explicitly sync or backup.
- The weekly/monthly narrative uses only your own statistics — no cross-user benchmarking.
- If you later want to understand why a growth area was suggested, the audit record is tamper-evident and immutable.

## Triggering the Rituals

**iOS UI**: The "Today" tab extends to show:

- **Weekly Review Card** (every Sunday, or tap "Weekly review" button): Full-screen card with the week's stats, wins summary, growth area, and a "Share summary" button (copies markdown to clipboard for Slack, email, etc.).
- **Monthly Retro Card** (last day of month, or tap "Monthly retro" button): Similar layout but with weekly breakdown as a mini-chart, theme badge, top accomplishments list, and month-over-month delta callout.

**Cron Integration**: The always-on engine emits a synthetic `ritual:weekly_review_ready` event every Sunday at 20:00 UTC (configurable). When this event fires, the NotificationDispatcher sends an iOS notification: "Your week is ready — tap to see." The app opens to the Weekly Review card.

**Fallback to Manual**: Both cards are always available in the UI. Tap the button anytime to generate an ad-hoc review (not just on schedule).

## Design & Polish

Coachy appears in both cards with a `pose=.confident, emotion=.proud` stance when celebrating wins, or `pose=.encouraging, emotion=.warm` when acknowledging slips. The wins summary and growth area are highlighted with callout UI (bold, maybe a left border or background). The stats grid uses tabular layout for clarity.

Static copy is localized-string-ready per the i18n agent's convention (all fallback text wraps in `String(localized:)` for easy extraction to Localizable.strings).

## Notes for Future Work

1. **Real Event Store Integration**: Replace stub data aggregation with actual queries to the event store, grouped by week/month and summed.
2. **Wallet History**: Track earning/spending deltas per week so the monthly report shows real "credits earned" trends.
3. **Rule Analytics**: Index rule firings in the audit chain so you can answer "which rules drove this week's success?"
4. **Streak Analysis**: Extend the wallet's streak tracking to record peak counts per month so the monthly retro can call out personal records.
5. **Custom Themes**: Let power users define their own theme criteria (e.g., "Consistency" if slip rate < 5%).
6. **Export & Share**: Add PDF export so you can save or share monthly retrospectives with coaches or accountability partners.

---

**Traces to:** FR-RITUAL-003 (Weekly Review), FR-RITUAL-004 (Monthly Retrospective).
