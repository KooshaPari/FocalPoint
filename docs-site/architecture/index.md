---
title: Architecture
description: System design, FFI topology, connector framework, and architectural decisions.
---

# Architecture Overview

FocalPoint is a **connector-first, rule-driven screen-time engine** built on Rust core with native iOS and Android frontends.

## System Design

- **Rust Core** — Domain logic, rule evaluation, state mutations, audit chain
- **SQLite** — Local append-only ledger (tamper-evident with HMAC-SHA256)
- **UniFFI/JNI** — Cross-platform bindings to native UI frameworks
- **Connectors** — Pluggable event sources (Canvas, Calendar, Health, GitHub, etc.)

## Key Subsystems

| Subsystem | Purpose | Language |
|-----------|---------|----------|
| **Rule Engine** | Condition → Action evaluation, guard clauses | Rust |
| **Event Store** | Append-only log with signature chain | Rust + SQLite |
| **Connector Framework** | Plugin protocol for external data sources | Rust traits + WASM/FFI |
| **Wallet / Ledger** | Reward/penalty mutations + audit trail | Rust + SQLite |
| **iOS Enforcement** | FamilyControls, Accessibility, Keychain bridges | Swift + UniFFI |
| **Android Enforcement** | DevicePolicyManager, AccessibilityService, Keystore | Kotlin + JNI |

## Documentation

- **[System Diagram](./system-diagram)** — Visual architecture (mermaid)
- **[Connector Framework](./connector-framework)** — How to build a connector
- **[FFI Topology](./ffi-topology)** — UniFFI and JNI binding architecture
- **[ADRs](./adrs)** — Architectural Decision Records and design debates

## Design Principles

1. **Local-first**: All state lives in SQLite. Cloud sync is optional (user-initiated).
2. **Audit-append**: Every mutation produces an `AuditRecord`. Chain is tamper-evident.
3. **Trait-stable**: Public APIs use Rust traits (`Connector`, `EventStore`, `RuleStore`). Break only via ADR.
4. **Domain-agnostic**: Crates are generic and reusable (error handling, config, caching, FSM).
5. **Connector-first**: New features start as connector templates, not core logic.

## Open Questions

See [Research: Open Questions](../docs/research/open_questions.md) for unresolved design issues (Foqos URLs, entitlement strategy, coaching loop).
