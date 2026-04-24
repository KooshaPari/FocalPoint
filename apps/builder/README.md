# FocalPoint Rule Builder

A visual ReactFlow-based node editor for authoring FocalPoint Rules without writing code.

## Quick Start

```bash
cd apps/builder
bun install
bun run dev
```

Then open [http://localhost:5182](http://localhost:5182) in your browser.

## Overview

The Rule Builder is a first slice of the FocalPoint node builder, supporting **Rules only**:

- **TriggerNode** — Event, Schedule, StateChange triggers with string parameters
- **ConditionNode** — 12 condition primitives (time_in_range, day_of_week, user_attribute, event_property, streak_above, credit_below, policy_active, consecutive_days, weekend_hour, and/or/not)
- **ActionNode** — 7 primary actions (GrantCredit, DeductCredit, Block, Unblock, StreakIncrement, StreakReset, Notify)
- **RuleMetaNode** — Rule name, priority, cooldown, duration, explanation template, enabled flag

## Features

- **Canvas**: ReactFlow 12+ with snap-to-grid, pan/zoom, undo/redo
- **Left Sidebar**: Node insertion palette with drag-drop support
- **Top Bar**:
  - Show/Hide DSL panel (live FPL compilation)
  - Save/Load graph to/from localStorage
  - Load Sample (deep-work-starter)
  - Export FPL (download as .fpl file)
  - Clear (reset canvas)
- **Right Panel**: Live Starlark FPL source code with copy-to-clipboard
- **Data Persistence**: Automatic localStorage save on graph changes

## Build & Test

```bash
# Development
bun run dev

# Production build
bun run build

# Preview build
bun run preview

# Type-check
bun run type-check

# Run unit tests
bun run test

# Run smoke tests (Playwright)
bun run test:smoke

# Lint
bun run lint
```

## Stack

- **React** 19.x + TypeScript 5.3
- **@xyflow/react** 12.0+ (ReactFlow successor)
- **Tailwind CSS** 4.0+ via @tailwindcss/vite
- **Vite** 7.0+ build tool
- **Vitest** + **Playwright** for testing

## Project Layout

```
src/
  main.tsx                 # Entry point
  App.tsx                  # Main app component
  index.css                # Tailwind imports
  types/
    graph.ts              # Type definitions
  lib/
    graphToFpl.ts         # Graph → FPL compilation
    graphToFpl.test.ts    # Unit tests
    sampleRule.ts         # Sample rule data
    utils.ts              # Utilities
  components/
    Canvas.tsx            # ReactFlow canvas
    FplPanel.tsx          # DSL side panel
    NodePalette.tsx       # Node insertion sidebar
    nodes/
      TriggerNode.tsx     # Trigger node component
      ConditionNode.tsx   # Condition node component
      ActionNode.tsx      # Action node component
      RuleMetaNode.tsx    # Metadata node component
e2e/
  smoke.spec.ts          # Playwright smoke tests
public/
  index.html             # HTML template
```

## Graph → FPL Compilation

The canvas graph is compiled to Starlark FPL:

```fpl
rule("deep-work-starter") {
  @layout { x = 50, y = 50 }
  priority = 10
  cooldown_seconds = 300
  duration_seconds = 3600
  enabled = true
  
  trigger {
    type = "Event"
    value = "focus_session_start"
  }
  
  when {
    time_in_range(start = 8, end = 18)
  }
  
  then {
    Block(policy = "social-media-block")
    Notify(message = "Focus mode activated")
  }
}
```

Template strings are used for v1; future phases will wire against focus-lang WASM compiler.

## Future Enhancements

- Real FPL round-trip via focus-lang WASM
- Undo/Redo history UI
- Minimap + search palette (Cmd+K)
- Copy/paste subgraphs
- Comment boxes & group framing
- All 12 FPL primitives (Task, Schedule, Pose, Connector, etc.)

## Constraints

- No external state manager (React hooks only for v1)
- No animation libs beyond Tailwind transitions
- <2500 LOC of TypeScript/React
- Runnable with `bun install && bun run dev`; no env vars required
