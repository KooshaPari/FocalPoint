# FocalPoint Marketing Landing Page

Astro 5 + Tailwind 4 marketing site for focalpoint.app.

## Stack

- **Framework**: Astro 5 (static site generation)
- **Styling**: Tailwind CSS 4 with impeccable reset baseline
- **Package Manager**: Bun
- **Type Safety**: TypeScript strict mode
- **Deployment**: Netlify or Vercel

## Structure

```
src/
  pages/              # Astro pages (index.astro)
  components/         # Reusable Astro components
    - Hero.astro      # Hero section with Coachy mascot
    - Features.astro  # Connector grid + feature grid
    - HowItWorks.astro # 3-step process animation placeholder
    - Pricing.astro   # Pricing tier cards (from entitlements)
    - Testimonials.astro # Placeholder for beta feedback
    - Footer.astro    # Legal + community links
  layouts/
    - Base.astro      # Root layout with dark mode toggle
  content/
    - copy.json       # All marketing copy (SEO, pricing, CTAs)
  types.ts            # TypeScript interfaces
  styles/
    - global.css      # Impeccable baseline + dark mode
```

## Features

- **Dark mode first** with system preference fallback
- **SEO ready**: og-image placeholder (1200x630), schema.org SoftwareApplication markup
- **Marketing-friendly**: All copy lives in `copy.json` — no code changes needed for iterations
- **Responsive**: Mobile-first, semantic HTML, accessible
- **Fast**: Static site, ~50KB gzipped HTML + CSS

## Getting Started

### Install Dependencies

```bash
cd apps/web-landing
bun install
```

### Development

```bash
bun run dev
```

Open http://localhost:3000 in your browser. Hot reload on file changes.

### Build & Preview

```bash
bun run build
bun run preview
```

## Marketing Copy

All copy is centralized in `src/content/copy.json`:

- **Hero**: Tagline, headline, CTA buttons, links
- **Features**: Connector list (10 shown), feature grid (6 items)
- **How It Works**: 3-step process description
- **Pricing**: Free / Plus ($4.99) / Pro ($9.99) / Family ($14.99) with feature lists
- **Testimonials**: Placeholder (marked "Coming — TestFlight beta testers' words")
- **Footer**: Legal links, community links, copyright

To update copy without touching components, edit `copy.json`.

## SEO & Metadata

- **og:image**: Placeholder at `/public/og-image.png` (1200x630)
- **schema.org**: SoftwareApplication markup included in Base layout (ready to configure)
- **Canonical**: https://focalpoint.app
- **Icons**: `/public/favicon.svg` (add your own)

## Deployment

### Netlify

```bash
bun run build
```

Push to GitHub; Netlify auto-deploys from `netlify.toml`.

### Vercel

```bash
bun run build
```

Push to GitHub; Vercel auto-deploys from `vercel.json`.

## Design System

- **Colors**: Dark blue (primary), dark gray (bg), white (text)
- **Typography**: System sans (headers/body), monospace (code)
- **Spacing**: Tailwind scale (4px grid)
- **Shadows**: Subtle, blue-accented on interactive elements
- **Transitions**: 200ms ease for hover states

## Type Safety

Strict TypeScript with interfaces for all content sections. Components are type-checked against `copy.json` structure.

```bash
bun run type-check
```

## Accessibility

- Semantic HTML (`<section>`, `<footer>`, proper heading hierarchy)
- Color contrast ≥4.5:1 (WCAG AA)
- ARIA labels on interactive elements
- Keyboard-navigable (no JS traps)

## License

Same as FocalPoint main project.
