# First-Run Setup Checklist

## Summary

FocalPoint currently ships the core rules, wallet, audit chain, and iOS shell, but it does not yet provide a user-facing onboarding flow. New users can see the app shell and some scaffolded integration entry points, but they do not get a guided explanation of what needs to be configured first or why certain features are still unavailable.

This spec adds a small, reversible first-run checklist that appears before the main tab experience and routes the user to the right next step.

## Problem

The app currently asks users to infer setup order from scattered screens and buttons. That is a poor fit for a product that depends on permissions, account connections, and potentially entitlement-gated enforcement. The result is avoidable confusion on first launch.

## Proposed change

Add a lightweight onboarding checklist with three items:

1. Grant required device permissions or acknowledge that enforcement is pending.
2. Connect one available data source, starting with Canvas.
3. Review the first recommended rule template or leave it for later.

The checklist should be informational and dismissible. It should not block existing app areas, and it should not change core rules or enforcement logic.

## Scope

In scope:

- A new first-run screen or sheet in the iOS app shell.
- Clear copy that explains why the setup exists and what is optional.
- A single persisted flag so the checklist only appears on first launch unless the user reopens it from Settings.

Out of scope:

- New connector auth flows.
- FamilyControls behavior changes.
- Enforcement logic changes.
- Backend or sync changes.

## Acceptance criteria

- On a fresh install, the user sees the checklist before landing in the main tabs.
- The checklist can be dismissed without completing every item.
- The checklist links to the existing Canvas/connectors entry point if available.
- Returning users do not see the checklist unless they explicitly reopen it.

## Validation

- Manual walkthrough on simulator or local build.
- Verify the checklist remains purely additive and does not affect existing rule creation or audit behavior.

## Traceability

- No explicit FR/NFR or issue ID was found in the current docs or open issues for this exact gap.
