/**
 * Reading time plugin for VitePress
 * Calculates estimated reading time (≈ N min read) based on word count / 200 wpm
 * Injects reading time metadata into page data
 */

export interface ReadingTimeData {
  minutes: number;
  words: number;
}

/**
 * Calculate reading time from markdown content
 * Formula: word_count / 200 words per minute, rounded up
 */
export function calculateReadingTime(content: string): ReadingTimeData {
  // Strip markdown syntax and HTML tags
  const cleanContent = content
    .replace(/#{1,6}\s+/gm, '') // headings
    .replace(/\*{1,3}([^\*]*)\*{1,3}/g, '$1') // bold/italic
    .replace(/\[([^\]]*)\]\([^\)]*\)/g, '$1') // links
    .replace(/```[\s\S]*?```/g, '') // code blocks
    .replace(/<[^>]*>/g, ''); // HTML tags

  const wordCount = cleanContent
    .trim()
    .split(/\s+/)
    .filter((word) => word.length > 0).length;

  const minutes = Math.max(1, Math.ceil(wordCount / 200));

  return { minutes, words: wordCount };
}

/**
 * Format reading time as human-readable string
 */
export function formatReadingTime(minutes: number): string {
  return `≈ ${minutes} min read`;
}
