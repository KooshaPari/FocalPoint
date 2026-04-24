import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid(defineConfig({
  title: 'FocalPoint',
  description: 'Connector-first screen-time management. Rules engine + dual ledger + iOS enforcement.',
  base: process.env.GITHUB_ACTIONS ? '/FocalPoint/' : '/',
  lang: 'en-US',
  cleanUrls: true,
  srcDir: '.',

  markdown: {
    math: false,
    lineNumbers: false,
  },

  transformHead({ pageData }) {
    const head: any[] = []
    const url = `https://focalpoint.app${pageData.relativePath === 'index.md' ? '/' : '/' + pageData.relativePath.replace(/\.md$/, '').replace(/index$/, '')}`
    const title = pageData.title || 'FocalPoint'
    const description = pageData.description || 'Connector-first screen-time management. Rules engine + dual ledger + iOS enforcement.'

    head.push(['meta', { property: 'og:title', content: title }])
    head.push(['meta', { property: 'og:description', content: description }])
    head.push(['meta', { property: 'og:type', content: 'website' }])
    head.push(['meta', { property: 'og:url', content: url }])
    head.push(['meta', { property: 'og:image', content: '/og-default.png' }])
    head.push(['meta', { name: 'twitter:card', content: 'summary_large_image' }])
    head.push(['meta', { name: 'twitter:title', content: title }])
    head.push(['meta', { name: 'twitter:description', content: description }])
    head.push(['meta', { name: 'twitter:image', content: '/og-default.png' }])

    return head
  },

  ignoreDeadLinks: [
    'localhostLinks',
    // Many pages are aspirational stubs; allow internal cross-references to
    // resolve as they are filled in. Tighten once v1 ships.
    /^\/connectors\/(?!canvas|index).*$/,
    /^\/rules\/templates\//,
    /^\/rituals\/.*$/,
  ],

  sitemap: {
    hostname: 'https://focalpoint.app',
  },

  themeConfig: {
    siteTitle: 'FocalPoint',
    logo: undefined,

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started/' },
      { text: 'Architecture', link: '/architecture/' },
      { text: 'Connectors', link: '/connectors/' },
      { text: 'Rules', link: '/rules/' },
      { text: 'Coachy', link: '/mascot/' },
      { text: 'Ecosystem', link: '/ecosystem/' },
      { text: 'Journeys', link: '/journeys/' },
      { text: 'Reference', link: '/reference/' },
      { text: 'GitHub', link: 'https://github.com/KooshaPari/FocalPoint' },
    ],

    sidebar: {
      '/': [
        { text: 'Home', link: '/' },

        {
          text: 'Quick Start',
          collapsed: false,
          items: [
            { text: 'Five-minute tour', link: '/guides/five_minute_tour' },
            { text: 'Getting started guide', link: '/guides/getting-started' },
            { text: 'Your first rule (3 surfaces)', link: '/guides/your_first_rule' },
            { text: 'Install on iOS', link: '/getting-started/install-ios' },
            { text: 'First rule walkthrough', link: '/getting-started/first-rule' },
          ],
        },

        {
          text: 'User Guides',
          collapsed: false,
          items: [
            { text: 'Write a rule', link: '/rules/' },
            { text: 'Install a template pack', link: '/ecosystem/' },
            { text: 'Focus mode', link: '/guides/focus-mode' },
            { text: 'Rewards & penalties', link: '/guides/rewards-penalties' },
            { text: 'Backup & restore', link: '/guides/backup-restore' },
            { text: 'Feedback', link: '/guides/feedback' },
          ],
        },

        {
          text: 'Concepts',
          collapsed: false,
          items: [
            { text: 'Core loop', link: '/guides/core-loop' },
            { text: 'Audit chain', link: '/architecture/audit-chain' },
            { text: 'Rule DSL', link: '/rules/dsl' },
            { text: 'Connectors', link: '/connectors/' },
            { text: 'Coachy mascot', link: '/mascot/' },
          ],
        },

        {
          text: 'Plugin SDK',
          collapsed: false,
          items: [
            { text: 'SDK overview', link: '/connector-sdk/' },
            { text: 'Manifest format', link: '/connector-sdk/manifest' },
            { text: 'Event schema', link: '/connector-sdk/events' },
            { text: 'Auth flows', link: '/connector-sdk/auth' },
            { text: 'Testing', link: '/connector-sdk/testing' },
            { text: 'Verification tiers', link: '/ecosystem/verification-tiers' },
          ],
        },

        {
          text: 'Architecture',
          collapsed: false,
          items: [
            { text: 'System diagram', link: '/architecture/' },
            { text: 'FFI topology', link: '/architecture/ffi-topology' },
            { text: 'Connector framework', link: '/architecture/connector-framework' },
            { text: 'ADRs', link: '/architecture/adrs' },
            { text: 'Design system', link: '/architecture/design_system_tokens' },
            { text: 'Testing strategy', link: '/architecture/testing_strategy' },
          ],
        },

        {
          text: 'Reports',
          collapsed: true,
          items: [
            { text: 'Design audit', link: '/reports/ios_design_audit_2026_04' },
            { text: 'Performance baselines', link: '/reports/performance_baselines_2026_04' },
            { text: 'Accessibility audit', link: '/reports/accessibility_audit_2026_04' },
            { text: 'Docs site audit', link: '/reports/docs_site_audit_2026_04' },
          ],
        },

        {
          text: 'Release',
          collapsed: true,
          items: [
            { text: 'Release loop', link: '/guides/release_loop' },
            { text: 'Self-hosted CI', link: '/deployment/self_hosted_ci_2026_04' },
            { text: 'Webhook server', link: '/deployment/webhook_server' },
          ],
        },

        {
          text: 'Community',
          collapsed: true,
          items: [
            { text: 'Discord playbook', link: '/community/discord_launch_playbook' },
            { text: 'Contributing', link: '/governance/contributing' },
            { text: 'Code of Conduct', link: '/governance/coc' },
            { text: 'Verification criteria', link: '/governance/verification' },
          ],
        },

        {
          text: 'Reference',
          collapsed: true,
          items: [
            { text: 'CLI reference', link: '/reference/cli_reference' },
            { text: 'Dual surface matrix', link: '/reference/dual_surface_matrix' },
            { text: 'Design tokens', link: '/reference/design-tokens' },
            { text: 'Traceability', link: '/reference/traceability' },
            { text: 'Coverage', link: '/reference/honest-coverage' },
          ],
        },

        {
          text: 'Journeys',
          collapsed: true,
          items: [
            { text: 'Student on Canvas', link: '/journeys/student-canvas' },
            { text: 'Developer with GitHub', link: '/journeys/developer-github' },
            { text: 'Connector SDK author', link: '/journeys/connector-sdk-author' },
          ],
        },

        { text: 'Status Dashboard', link: '/status' },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/FocalPoint' },
    ],

    editLink: {
      pattern: 'https://github.com/KooshaPari/FocalPoint/edit/main/docs-site/:path',
      text: 'Edit this page on GitHub',
    },

    lastUpdated: {
      text: 'Last updated',
      formatOptions: { dateStyle: 'short', timeStyle: 'short' },
    },

    footer: {
      message: 'Released under MIT OR Apache-2.0.',
      copyright: 'Copyright © 2026 FocalPoint contributors',
    },

    outline: 'deep',

    search: {
      provider: 'local',
      options: {
        miniSearch: {
          options: { processTerm: (t: string) => t.toLowerCase() },
        },
      },
    },
  },

  head: [
    ['meta', { name: 'theme-color', content: '#ff6b3d' }],
    ['meta', { name: 'color-scheme', content: 'dark light' }],
  ],
}))
