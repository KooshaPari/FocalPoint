# Fonts Directory

This directory contains web font files for FocalPoint docs-site.

## Font Files

- **Inter-Regular.woff2** — Inter regular weight (400) for body text
- **Inter-SemiBold.woff2** — Inter semi-bold weight (600) for emphasis
- **Inter-Bold.woff2** — Inter bold weight (700) for headings
- **JetBrainsMono-Regular.woff2** — JetBrains Mono regular weight (400) for code blocks

## Obtaining Font Files

1. **Inter:** Download from https://fonts.google.com/specimen/Inter
   - Select regular (400), semi-bold (600), bold (700)
   - Export as WOFF2 format

2. **JetBrains Mono:** Download from https://www.jetbrains.com/lp/mono/
   - Select regular weight (400)
   - Export as WOFF2 format

## Installation

Place the `.woff2` files in this directory. They will be loaded via `@font-face` declarations in `../.vitepress/theme/custom.css` with `font-display: swap` for optimal performance.
