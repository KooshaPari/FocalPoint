/**
 * VitePress Theme Configuration
 * 
 * @see https://vitepress.dev/guide/custom-theme
 */

import { h } from 'vue'
import DefaultTheme from 'vitepress/theme'
import './custom.css'

// Import custom components
import JourneyViewer from './components/JourneyViewer.vue'

/** @type {import('vitepress').Theme} */
export default {
  extends: DefaultTheme,
  
  Layout: () => {
    return h(DefaultTheme.Layout, null, {
      // ...
    })
  },

  enhanceApp({ app, router, siteData }) {
    // Register custom components
    app.component('JourneyViewer', JourneyViewer)
  }
}
