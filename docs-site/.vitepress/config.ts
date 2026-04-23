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

  ignoreDeadLinks: [
    'localhostLinks',
    // Many pages are aspirational stubs; allow internal cross-references to
    // resolve as they are filled in. Tighten once v1 ships.
    /^\/connectors\/(?!canvas|index).*$/,
    /^\/rules\/templates\//,
    /^\/rituals\/.*$/,
  ],

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
      { text: 'GitHub', link: 'https://github.com/KooshaPari/FocalPoint' },
    ],

    sidebar: {
      '/getting-started/': [
        { text: 'Overview', link: '/getting-started/' },
        { text: 'Install on iOS', link: '/getting-started/install-ios' },
        { text: 'First rule walkthrough', link: '/getting-started/first-rule' },
      ],

      '/architecture/': [
        { text: 'Overview', link: '/architecture/' },
        { text: 'System diagram', link: '/architecture/system-diagram' },
        { text: 'Connector framework', link: '/architecture/connector-framework' },
        { text: 'FFI topology', link: '/architecture/ffi-topology' },
        { text: 'ADRs', link: '/architecture/adrs' },
      ],

      '/connectors/': [
        { text: 'Overview', link: '/connectors/' },
        {
          text: 'Shipping',
          collapsed: false,
          items: [
            { text: 'Canvas LMS', link: '/connectors/canvas' },
          ],
        },
        {
          text: 'Aspirational',
          collapsed: false,
          items: [
            { text: 'MacroFactor', link: '/connectors/macrofactor' },
            { text: 'YNAB', link: '/connectors/ynab' },
            { text: 'Google Calendar', link: '/connectors/google-calendar' },
            { text: 'Apple Health', link: '/connectors/apple-health' },
            { text: 'Todoist', link: '/connectors/todoist' },
          ],
        },
      ],

      '/connector-sdk/': [
        { text: 'SDK Spec', link: '/connector-sdk/' },
        { text: 'Manifest format', link: '/connector-sdk/manifest' },
        { text: 'Event schema', link: '/connector-sdk/events' },
        { text: 'Auth flows', link: '/connector-sdk/auth' },
        { text: 'Testing', link: '/connector-sdk/testing' },
      ],

      '/ecosystem/': [
        { text: 'Marketplace strategy', link: '/ecosystem/' },
        { text: 'Verification tiers', link: '/ecosystem/verification-tiers' },
        { text: 'Rule-template format', link: '/ecosystem/rule-template-format' },
      ],

      '/rules/': [
        { text: 'Overview', link: '/rules/' },
        { text: 'DSL reference', link: '/rules/dsl' },
        { text: 'Condition built-ins', link: '/rules/conditions' },
        { text: 'Action catalogue', link: '/rules/actions' },
        { text: 'Sample rule packs', link: '/rules/samples' },
      ],

      '/mascot/': [
        { text: 'Coachy', link: '/mascot/' },
        { text: 'Character sheet', link: '/mascot/character-sheet' },
        { text: 'Personality guide', link: '/mascot/personality' },
      ],

      '/rituals/': [
        { text: 'Overview', link: '/rituals/' },
        { text: 'Morning brief', link: '/rituals/morning-brief' },
        { text: 'Evening shutdown', link: '/rituals/evening-shutdown' },
      ],

      '/reference/': [
        { text: 'Reference', link: '/reference/' },
        { text: 'Design tokens', link: '/reference/design-tokens' },
        { text: 'Traceability', link: '/reference/traceability' },
        { text: 'Honest coverage', link: '/reference/honest-coverage' },
      ],

      '/governance/': [
        { text: 'Governance', link: '/governance/' },
        { text: 'Contribution guide', link: '/governance/contributing' },
        { text: 'Verification criteria', link: '/governance/verification' },
        { text: 'Code of Conduct', link: '/governance/coc' },
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
