import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';

export default defineConfig({
  site: 'https://focalpoint.app',
  integrations: [tailwind()],
  output: 'static',
  vite: {
    ssr: {
      external: ['@astrojs/tailwind'],
    },
  },
});
