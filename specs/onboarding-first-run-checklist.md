# chore: add onboarding checklist for first-run users

## Goal
Provide a lightweight startup path for new users by documenting one-command onboarding verification using existing entry points.

## Why now
- The README explicitly marks onboarding UX as a current gap.
- No small, reversible, non-code gate exists to help QA and contributors validate first-run behavior.
- This spec introduces a minimal, low-risk quality-of-life workflow that can be removed if not useful.

## Proposed change
- Add `specs/onboarding-first-run-checklist.md` describing:
  - A minimal checklist for first-run checks (status, onboarding intent capture, dry-run mode).
  - Expected command sequence and fallback behavior on known environment blockers.
  - Explicit acceptance criteria and rollback condition.
- Keep the doc to workflow steps only; no product behavior changes yet.

## Success criteria
- New file exists and can be followed in under 10 minutes.
- Maintainers can decide whether to convert this into a tracked script in a follow-up PR.

## Revert plan
- Delete the checklist file if it causes maintenance overhead.

## UX research question
- What is the single onboarding action (or screen) users most need before they try the first rule or connector?