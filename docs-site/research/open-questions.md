---
title: Open Questions
description: Unresolved design and implementation questions for FocalPoint.
---

# Open Questions

These are the design decisions currently under discussion or blocked on external dependencies.

## Q1: Mascot Name (BLOCKED)

**Status**: Blocked on design review

What do we call the coaching mascot? Current candidates:
- Coachy (warm, approachable)
- Reef (nod to wellbeing, nautical theme)
- Foqos-inspired naming

## Q2: Entitlement Strategy (BLOCKED)

**Status**: Blocked on Apple entitlement approval

- **Apple entitlement**: `com.apple.developer.family-controls` — submitted; awaiting review (1-4 weeks)
- Once approved, unlocks ManagedSettings + DeviceActivity enforcement on iOS

## Q3: Multi-Device Sync (PLANNED)

**Status**: Phase 6 (deferred)

- Single-device (iPhone) MVP ships first
- Multi-device sync (iPad, Mac, Watch) planned for v2.0

## Q4: Android Strategy (PLANNED)

**Status**: Phase 5 (deferred)

- UsageStats + Accessibility Service approach designed
- Implementation deferred until iOS v1.0 ships

## Q5: Coaching Loop Personalization (IN PROGRESS)

**Status**: Research in progress

- How do we personalize Coachy's messages without being invasive?
- Mood tracking, energy levels, learning style?
- User privacy constraints?

## Q6: Connector Marketplace Trust Model (DESIGN PHASE)

**Status**: Specification in progress

- How do we verify community connectors without bottlenecking on manual review?
- Automated security scanning (SAST)?
- User reputation system?

See [Verification Tiers](/ecosystem/verification-tiers) for current approach.

## Q7: Rule DSL Syntax (DESIGNED)

**Status**: Specification complete; implementation pending

Current syntax uses YAML with condition/action keywords. See [Rules DSL](/rules/dsl) for details.

## Q8: Foqos & Reef Integration (BLOCKED)

**Status**: Blocked on URL clarification

Awaiting confirmation on:
- Foqos GitHub URL (donor codebase for FamilyControls pattern)
- Reef Canvas API testing environment

Once clarified, integration tests can begin.

---

**Questions?** Open an issue on [GitHub](https://github.com/KooshaPari/FocalPoint/issues) or discuss in [Discussions](https://github.com/KooshaPari/FocalPoint/discussions).
