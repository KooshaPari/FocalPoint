# FocalPoint — User Journeys

> Source: arch doc lines 60–72 + 546–568.

## J1 — First-run onboarding

1. Install iOS app → grant FamilyControls entitlement
2. Pick 1 template rule ("lock Instagram during class hours")
3. Authenticate Canvas (or skip)
4. Set wake time → first enforcement window scheduled
5. Home screen shows active rule + explanation

## J2 — Assignment-driven focus

1. Canvas sync detects upcoming assignment due
2. Rule "lock social when assignment due < 24h" fires
3. User opens Instagram → blocked with explanation
4. Submit assignment → Canvas webhook or next poll detects submission
5. Rule evaluates new state → unlock; streak incremented; credit granted

## J3 — Sleep-debt penalty escalation

1. Health app reports sleep < 6h
2. `PenaltyMutation::Escalate(Warning)` applied; audit record appended
3. Morning: strict_mode_until is next noon → enforcement policy tightens
4. User sees tier + reason on home surface; bypass cost displayed
5. User either earns way out (study rule fires → credit) or pays bypass budget

## J4 — Rule template install + explanation

1. User browses rule template marketplace (Phase 3)
2. Installs "pomodoro" template → rule scaffolded into wallet
3. First firing → explanation_template rendered with actual event_ids + state snapshot
4. User taps explanation → full `RuleEvaluation` record visible (audit trail)

## J5 — Unlock via QR/NFC proof (Phase 1.5)

1. User approaches physical "focus zone" with NFC tag or QR sticker
2. Scan triggers `UnlockSession` with `proof_type = Nfc | Qr`
3. Proof validated against rule (e.g. "at library → study mode on")
4. Rule fires → streak incremented; specific unlock balances updated
