/**
 * VitePress plugin: Reading time widget
 * Injects reading time below page title on guide pages
 */

import type { PluginOption } from 'vite'
import { calculateReadingTime, formatReadingTime } from '../theme/reading-time'

export function readingTimePlugin(): PluginOption {
  return {
    name: 'vitepress-reading-time',

    resolveId(id) {
      if (id === 'virtual-reading-time') {
        return id
      }
    },

    load(id) {
      if (id === 'virtual-reading-time') {
        return `export const readingTime = {}`
      }
    },

    transform(code, id) {
      // Transform markdown files to inject reading time metadata
      if (id.endsWith('.md')) {
        const readingData = calculateReadingTime(code)
        const readingTimeStr = formatReadingTime(readingData.minutes)

        // Add reading time as frontmatter if not already present
        if (!code.startsWith('---')) {
          return {
            code: `---\nreadingTime: "${readingTimeStr}"\nwordCount: ${readingData.words}\n---\n${code}`,
            map: null
          }
        }

        // Insert after first frontmatter block
        const frontmatterEnd = code.indexOf('---', 3)
        if (frontmatterEnd > 0) {
          const before = code.substring(0, frontmatterEnd)
          const after = code.substring(frontmatterEnd)

          if (!code.includes('readingTime:')) {
            return {
              code: `${before}\nreadingTime: "${readingTimeStr}"\nwordCount: ${readingData.words}\n${after}`,
              map: null
            }
          }
        }
      }

      return null
    }
  }
}

export default readingTimePlugin
