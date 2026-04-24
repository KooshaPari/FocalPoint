<script setup lang="ts">
import { useData } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import { computed } from 'vue'

const { page, frontmatter } = useData()

const readingTime = computed(() => {
  return frontmatter.value.readingTime || null
})

// Only show reading time on guide pages (not home, landing pages, etc.)
const showReadingTime = computed(() => {
  return (
    readingTime.value &&
    (page.value.relativePath.includes('/guides/') ||
      page.value.relativePath.includes('/getting-started/') ||
      page.value.relativePath.includes('/reference/') ||
      page.value.relativePath.includes('/architecture/'))
  )
})
</script>

<template>
  <DefaultTheme.Layout>
    <template #doc-top>
      <div v-if="showReadingTime" class="reading-time">
        {{ readingTime }}
      </div>
    </template>
  </DefaultTheme.Layout>
</template>

<style scoped>
.reading-time {
  font-size: 0.875rem;
  color: var(--vp-c-text-3);
  margin-bottom: 1rem;
  font-style: italic;
}
</style>
