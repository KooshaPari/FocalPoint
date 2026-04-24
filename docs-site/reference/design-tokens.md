---
title: Design Tokens
description: Color palette, typography, spacing, and component sizing.
---

# Design Tokens

Consistent design tokens across FocalPoint's UI.

## Color Palette

### Brand Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--color-primary` | `#FF6B3D` | Buttons, accents, highlights |
| `--color-secondary` | `#0A9396` | Secondary actions, links |
| `--color-success` | `#2D7D6B` | Confirmations, streaks |
| `--color-warning` | `#E8A87C` | Warnings, approaching deadlines |
| `--color-danger` | `#EF476F` | Errors, blocks, urgent |

### Semantic Colors

| Token | Light | Dark | Usage |
|-------|-------|------|-------|
| `--color-bg` | `#FFFFFF` | `#0F0F0F` | Background |
| `--color-surface` | `#F5F5F5` | `#1A1A1A` | Cards, panels |
| `--color-border` | `#E0E0E0` | `#333333` | Dividers, borders |
| `--color-text` | `#1A1A1A` | `#FFFFFF` | Body text |
| `--color-text-secondary` | `#666666` | `#999999` | Secondary text |

## Typography

### Font Family

- **Body**: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto' (system stack)
- **Monospace**: 'Monaco', 'Menlo', 'Ubuntu Mono' (code UI)

### Sizes

| Token | Size | Usage |
|-------|------|-------|
| `--font-xs` | 12px | Captions, micro-text |
| `--font-sm` | 14px | Labels, small text |
| `--font-base` | 16px | Body text (default) |
| `--font-lg` | 18px | Larger body, subheadings |
| `--font-xl` | 24px | Section headings |
| `--font-2xl` | 32px | Page titles |
| `--font-3xl` | 48px | Hero text |

### Weights

| Token | Weight | Usage |
|-------|--------|-------|
| `--font-light` | 300 | Decorative, subtle |
| `--font-regular` | 400 | Body text (default) |
| `--font-medium` | 500 | Medium emphasis |
| `--font-semibold` | 600 | Labels, headings |
| `--font-bold` | 700 | Strong emphasis |

## Spacing

| Token | Value | Usage |
|-------|-------|-------|
| `--spacing-xs` | 4px | Micro spacing (within buttons) |
| `--spacing-sm` | 8px | Small gaps, padding |
| `--spacing-md` | 16px | Default spacing, padding |
| `--spacing-lg` | 24px | Section spacing |
| `--spacing-xl` | 32px | Large gaps, section margins |
| `--spacing-2xl` | 48px | Hero spacing, page margins |

## Component Sizing

### Buttons

| Token | Value | Usage |
|-------|-------|-------|
| `--button-height-sm` | 32px | Small actions, secondary |
| `--button-height-md` | 44px | Standard buttons (default) |
| `--button-height-lg` | 54px | Primary actions, CTAs |

### Cards

| Token | Value |
|-------|-------|
| `--card-radius` | 12px |
| `--card-shadow` | 0 2px 8px rgba(0,0,0,0.08) |
| `--card-padding` | var(--spacing-md) |

### Focus Radius

| Component | Radius |
|-----------|--------|
| Buttons | 8px |
| Cards | 12px |
| Modal | 16px |
| App grid | 12px |

## Responsive Breakpoints

| Token | Width | Device |
|-------|-------|--------|
| `--breakpoint-sm` | 390px | iPhone SE |
| `--breakpoint-md` | 768px | iPad |
| `--breakpoint-lg` | 1024px | Desktop |

## Z-Index Stack

| Token | Value | Element |
|-------|-------|---------|
| `--z-base` | 0 | Default content |
| `--z-dropdown` | 10 | Dropdowns, popovers |
| `--z-modal` | 100 | Modals |
| `--z-toast` | 200 | Toast notifications |
| `--z-tooltip` | 300 | Tooltips |

## Dark Mode

All tokens support light/dark automatically via CSS variables:

```css
@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #0F0F0F;
    --color-text: #FFFFFF;
  }
}
```

See VitePress theme config for active implementation.
