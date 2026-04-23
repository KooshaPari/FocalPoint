# Design Tokens — Phenotype / GMK-Arch palette

Canonical color tokens for FocalPoint. Inspired by the GMK Arch keycap palette
and standardized as the Phenotype-org design baseline.

## Palette

| Token | Hex | Role |
|---|---|---|
| `accent` | `#7EBAB5` | Primary accent — teal; focus/success/active states, mascot cape highlight |
| `bg-dark` | `#0F1012` | Primary background in dark mode; near-black |
| `surface` | `#353A40` | Elevated surfaces (cards, sheets, toolbars) in dark mode |
| `fg` | `#F6F5F5` | Primary text / icon on dark surfaces; also the light-mode background |

## Semantic mapping

| Semantic | Token |
|---|---|
| `app.background` | dark: `bg-dark` · light: `fg` |
| `app.foreground` | dark: `fg` · light: `bg-dark` |
| `app.surface` | dark: `surface` · light: `#E8E7E7` (derived) |
| `app.accent` | `accent` (both modes) |
| `accent.on` | `bg-dark` (contrast text on accent) |
| `mascot.cape` | `accent` — Coachy's cape |
| `mascot.body` | derived warm flame palette (see Coachy asset tokens) |

## Mascot asset tokens (Coachy)

Coachy is a flame-shaped character with a red cape + gold-star buckle. These
tokens stay *orange/warm* in our palette for brand recognition (user's approved
AI render defines them). Only UI chrome uses the Phenotype teal/dark/light set.

| Token | Hex | Role |
|---|---|---|
| `coachy.flame.core` | `#F07B3F` | Main body |
| `coachy.flame.edge` | `#F8B26A` | Rim / highlight |
| `coachy.flame.base` | `#E05A26` | Darker base |
| `coachy.cape` | `#D4462E` | Cape body |
| `coachy.buckle.gold` | `#F9C86A` | Belt buckle star |
| `coachy.eyes` | `#121212` | Near-black pupils |

## SwiftUI mapping

Defined in `apps/ios/FocalPoint/Sources/DesignSystem/Palette.swift`. Uses
dynamic color assets that resolve light/dark automatically via UITrait.

## Rust mapping

UI colors are iOS-only; Rust does not reference palette values directly.
Semantic names (e.g. `MascotPose::Celebrating`) carry across the FFI, and the
Swift layer resolves to concrete colors.

## Rationale

Phenotype-org convention: teal-on-near-black primary; warm mascot stays off-brand
on purpose, because Coachy is meant to feel *alive* against the cool UI chrome
— a warm character on cool background creates the focus-zone affordance.
