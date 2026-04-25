# Design Audit: FocalPoint/docs-site
**Date:** 2026-04-24 | **Auditor:** Claude (Haiku 4.5) | **Status:** No fixes applied

---

## Audit Health Score

| # | Dimension | Score | Key Finding |
|---|-----------|-------|-------------|
| 1 | Accessibility | 3 | Good semantic HTML; mermaid diagrams need alt text |
| 2 | Performance | 2 | Mermaid library loaded for all pages; no code splitting or lazy load detected |
| 3 | Responsive Design | 4 | VitePress responsive; mobile sidebar collapses well |
| 4 | Theming | 1 | Default VitePress theme, no FocalPoint brand colors or custom palette |
| 5 | Anti-Patterns | 2 | Mermaid diagrams are functional but lack distinctive visual styling; default VitePress look |
| **Total** | | **12/20** | **Acceptable (significant work needed)** |

---

## Anti-Patterns Verdict

**Moderate generic defaults:**
- VitePress default blue/gray theme — no personality
- Mermaid diagrams use default Mermaid styling (common, templated appearance)
- No custom color scheme or typography
- Aspirational stub pages (marked as WIP) indicate incomplete design planning

**Verdict:** Standard documentation site. Not AI-generated but lacks intentional visual direction.

---

## Executive Summary

- **Audit Health Score:** 12/20 (Acceptable — significant work needed)
- **Critical Issues:** P0 (0) | P1 (3) | P2 (2) | P3 (2)
- **Top Issues:**
  1. No FocalPoint brand visual identity (colors, fonts, icons)
  2. Mermaid diagrams lack accessibility (no alt text) and custom styling
  3. Performance: Mermaid library loads on all pages; should be lazy-loaded
- **Recommended Path:** Colorize (FocalPoint brand) → optimize (lazy-load mermaid) → harden (a11y for diagrams) → typeset

---

## Detailed Findings by Severity

### P1 Major

**[P1] No FocalPoint brand visual identity in theme**
- **Category:** Theming / Branding
- **Location:** .vitepress/theme/* (using defaults)
- **Impact:** Site could be any VitePress docs. Doesn't communicate FocalPoint's visual identity or product positioning.
- **Recommendation:** Create `custom.css` or theme override with FocalPoint colors (blue, black from brand_playbook.md). Consider custom logo or accent color.
- **Suggested command:** `/colorize` (to integrate FocalPoint brand colors)

**[P1] Mermaid diagrams lack alt text and accessibility**
- **Category:** Accessibility
- **Location:** All .md files with mermaid blocks
- **Impact:** Diagrams are rendered as SVG but have no text alternative. Screen reader users cannot access diagram content.
- **Recommendation:** Add descriptive text before/after each diagram (e.g., "The rules engine processes input from connectors and outputs to the dual ledger...").
- **Suggested command:** `/harden` (to add alt text and accessible descriptions)

**[P1] Mermaid library loads on every page (performance)**
- **Category:** Performance
- **Location:** vitepress-plugin-mermaid in config
- **Impact:** Mermaid JS library (~500KB+) loads on all pages, even those without diagrams. Bloats bundle for documentation pages that are text-only.
- **Recommendation:** Use lazy-loading wrapper or conditional loading (only load when diagram detected on page).
- **Suggested command:** `/optimize` (to lazy-load Mermaid on demand)

### P2 Minor

**[P2] Default Mermaid diagram styling lacks visual polish**
- **Category:** Visual Detail / Design Direction
- **Location:** Mermaid diagrams throughout (flowcharts, sequence diagrams, entity models)
- **Impact:** Diagrams use Mermaid defaults (gray nodes, basic styling, generic connectors). Could be more visually aligned with product aesthetic.
- **Recommendation:** Create custom Mermaid theme (via `%%{init: {...}}%%` in diagram code) to match FocalPoint brand colors and typography.
- **Suggested command:** `/bolder` (to stylize Mermaid diagrams with custom theme)

**[P2] Stub pages are not clearly marked**
- **Category:** Content / UX Clarity
- **Location:** /connectors/*, /rules/templates/*, /rituals/* (marked in config as aspirational)
- **Impact:** Users may start reading an incomplete page without realizing it's WIP. "Coming soon" banner or grayed-out styling would clarify intent.
- **Recommendation:** Add prominent WIP badge or banner to stub pages with expected completion date.
- **Suggested command:** `/clarify` (to add clear labeling for incomplete sections)

### P3 Polish

**[P3] Missing custom logo or visual mark**
- **Category:** Branding
- **Location:** .vitepress/theme/index.ts (siteTitle is text-only)
- **Impact:** No visual icon to represent FocalPoint. Text-only feels less premium.
- **Recommendation:** Create or add FocalPoint logo/icon (SVG recommended). Consider favicon as well.
- **Suggested command:** `/delight` (to add visual brand mark)

**[P3] Heading typography could be more distinctive**
- **Category:** Typography
- **Location:** .vitepress theme
- **Impact:** Headings use default VitePress sizing. Could be larger, bolder, or use distinctive font.
- **Recommendation:** Increase heading sizes (h1 > h2 > h3) or apply custom font weight. Current is functional but unremarkable.
- **Suggested command:** `/typeset` (to enhance heading hierarchy)

---

## Patterns & Systemic Issues

### VitePress Default Theme Not Customized
**Systemic Issue:** Zero custom theming or branding applied.
- **Pattern:** All visual styling is VitePress default
- **Impact:** Site blends into thousands of other VitePress docs
- **Recommendation:** Create `.vitepress/theme/custom.css` or `index.ts` override with FocalPoint brand colors, fonts, logos

### Mermaid as Default Include
**Systemic Issue:** Mermaid plugin loaded globally; not conditionally loaded.
- **Pattern:** vitepress-plugin-mermaid wraps entire config
- **Impact:** ~500KB JS loaded on every page, even text-only docs
- **Recommendation:** Consider lazy-loading or extracting Mermaid to specific page types

### Incomplete Documentation Structure
**Systemic Issue:** Many pages are stubs/aspirational (ignoreDeadLinks regex allows /connectors/, /rules/templates/, /rituals/*)
- **Pattern:** Section structure is planned but content is missing
- **Impact:** Users hit incomplete pages; affects perceived quality
- **Recommendation:** (1) Clearly mark WIP pages; (2) Prioritize core content before launch; (3) Consider hidden sidenav items for incomplete sections

---

## Positive Findings

✓ **Excellent semantic structure** — Proper heading hierarchy, clean HTML, good link organization.

✓ **Smart OpenGraph/Twitter meta tags** — SEO setup is thorough; proper social sharing cards.

✓ **Mermaid integration is working** — Plugin correctly renders diagrams. Just needs styling + performance optimization.

✓ **Navigation is clear** — Home, Getting Started, FAQ, Troubleshooting are all accessible in nav. Good UX.

✓ **Responsive mobile-first** — Sidebar collapses, content adapts. No horizontal scroll detected.

---

## Recommended Actions

1. **[P1] `/colorize`** — Apply FocalPoint brand colors to VitePress theme (blue, black, accent from brand_playbook.md)
2. **[P1] `/harden`** — Add alt text and descriptions for all Mermaid diagrams (screen reader accessibility)
3. **[P1] `/optimize`** — Lazy-load Mermaid library (only load when diagram detected on page)
4. **[P2] `/bolder`** — Customize Mermaid diagram styling with brand colors and typography
5. **[P2] `/clarify`** — Add WIP badges to stub pages and mark expected completion dates
6. **[P3] `/delight`** — Design and add FocalPoint logo/icon to site header
7. **[P3] `/typeset`** — Enhance heading hierarchy (size, weight, or custom font)
8. **Final step:** `/audit` to verify score improvement

---

## Notes

- **FocalPoint brand colors:** Likely in BRAND.md or brand_playbook.md (blue + black mentioned in description)
- **Mermaid theming:** Use `%%{init: {theme: 'custom', primaryColor: '...'}}%%` syntax or custom config
- **Lazy loading Mermaid:** Consider conditional plugin loading or wrapper component
- **Build impact:** No build-time issues detected; all .md parsing is clean

