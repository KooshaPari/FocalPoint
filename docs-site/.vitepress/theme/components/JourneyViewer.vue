<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'

interface Keyframe {
  step: number
  path: string
  timestamp: string
  caption: string
}

interface IntentStep {
  index: number
  slug: string
  intent: string
  precondition: string
  expected_visible_change: string
}

interface Verification {
  vlm_model: string | null
  description: string | null
  equivalence_score: number | null
  status: string
  verified_at: string | null
}

interface JourneyManifest {
  id: string
  title: string
  persona: string
  platform: string
  duration_seconds: number
  recording_date: string
  status: string
  intent: {
    summary: string
    steps: IntentStep[]
  }
  keyframes: Keyframe[]
  verification: Verification
}

const props = defineProps<{
  manifest: string
}>()

const manifest = ref<JourneyManifest | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)
const currentFrame = ref(0)
const showGallery = ref(false)

const statusBadge = computed(() => {
  if (!manifest.value) return { class: 'badge-neutral', text: 'Loading...' }
  switch (manifest.value.status) {
    case 'verified':
      return { class: 'badge-success', text: 'Verified' }
    case 'pending_verification':
      return { class: 'badge-warning', text: 'Pending Verification' }
    case 'planned':
      return { class: 'badge-info', text: 'Planned' }
    default:
      return { class: 'badge-neutral', text: manifest.value.status }
  }
})

const platformIcon = computed(() => {
  if (!manifest.value) return '📱'
  switch (manifest.value.platform) {
    case 'ios': return '📱'
    case 'cli': return '💻'
    case 'macos': return '🖥️'
    case 'web': return '🌐'
    default: return '📱'
  }
})

onMounted(async () => {
  try {
    const response = await fetch(props.manifest)
    if (!response.ok) throw new Error(`Failed to load manifest: ${response.status}`)
    manifest.value = await response.json()
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Unknown error'
  } finally {
    loading.value = false
  }
})

function nextFrame() {
  if (manifest.value && currentFrame.value < manifest.value.keyframes.length - 1) {
    currentFrame.value++
  }
}

function prevFrame() {
  if (currentFrame.value > 0) {
    currentFrame.value--
  }
}

function goToFrame(index: number) {
  currentFrame.value = index
  showGallery.value = false
}
</script>

<template>
  <div class="journey-viewer">
    <!-- Loading State -->
    <div v-if="loading" class="journey-loading">
      <div class="loading-spinner"></div>
      <p>Loading journey manifest...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="journey-error">
      <p>⚠️ {{ error }}</p>
      <p class="error-hint">Recording pending — manifest not yet available.</p>
    </div>

    <!-- Journey Content -->
    <div v-else-if="manifest" class="journey-content">
      <!-- Header -->
      <div class="journey-header">
        <div class="journey-meta">
          <span class="platform-badge">{{ platformIcon }} {{ manifest.platform.toUpperCase() }}</span>
          <span :class="['badge', statusBadge.class]">{{ statusBadge.text }}</span>
          <span class="duration">⏱️ {{ Math.round(manifest.duration_seconds / 60) }} min</span>
          <span class="date">📅 {{ manifest.recording_date }}</span>
        </div>

        <!-- Verification Panel -->
        <div v-if="manifest.verification.status !== 'pending'" class="verification-panel">
          <div class="verification-status" :class="manifest.verification.status.toLowerCase()">
            <span v-if="manifest.verification.equivalence_score">
              Score: {{ (manifest.verification.equivalence_score * 100).toFixed(0) }}%
            </span>
            <span v-if="manifest.verification.vlm_model">
              | Model: {{ manifest.verification.vlm_model }}
            </span>
          </div>
        </div>
      </div>

      <!-- Intent Summary -->
      <div class="intent-summary">
        <h4>Journey Intent</h4>
        <p>{{ manifest.intent.summary }}</p>
      </div>

      <!-- Steps Overview -->
      <div class="steps-overview">
        <h4>Steps ({{ manifest.intent.steps.length }})</h4>
        <div class="steps-grid">
          <div
            v-for="step in manifest.intent.steps"
            :key="step.index"
            class="step-card"
            :class="{ active: currentFrame + 1 === step.index }"
            @click="goToFrame(step.index - 1)"
          >
            <span class="step-number">{{ step.index }}</span>
            <span class="step-intent">{{ step.intent }}</span>
          </div>
        </div>
      </div>

      <!-- Keyframe Gallery (Placeholder) -->
      <div class="keyframe-section">
        <div class="keyframe-header">
          <h4>Keyframes</h4>
          <button @click="showGallery = !showGallery" class="toggle-gallery">
            {{ showGallery ? 'Hide' : 'Show' }} Gallery
          </button>
        </div>

        <div v-if="showGallery" class="keyframe-gallery">
          <div
            v-for="(kf, idx) in manifest.keyframes"
            :key="idx"
            class="keyframe-thumb"
            :class="{ active: currentFrame === idx }"
            @click="goToFrame(idx)"
          >
            <div class="thumb-placeholder">
              <span>{{ kf.timestamp }}</span>
            </div>
            <span class="kf-caption">{{ kf.caption }}</span>
          </div>
        </div>

        <!-- Current Frame Display -->
        <div class="current-frame">
          <div class="frame-placeholder">
            <span>Frame {{ currentFrame + 1 }}/{{ manifest.keyframes.length }}</span>
            <span class="kf-caption-large">
              {{ manifest.keyframes[currentFrame]?.caption || 'No caption' }}
            </span>
          </div>

          <div class="frame-nav">
            <button @click="prevFrame" :disabled="currentFrame === 0">← Previous</button>
            <span class="timestamp">
              {{ manifest.keyframes[currentFrame]?.timestamp || '0:00' }}
            </span>
            <button @click="nextFrame" :disabled="currentFrame >= manifest.keyframes.length - 1">
              Next →
            </button>
          </div>
        </div>
      </div>

      <!-- Verification Badge -->
      <div class="verification-badge" :class="manifest.verification.status.replace('_', '-')">
        <span v-if="manifest.verification.status === 'pending'">
          📋 Recording pending — VLM verification will run after recording
        </span>
        <span v-else-if="manifest.verification.status === 'PASS'">
          ✅ VLM Verified ({{ (manifest.verification.equivalence_score! * 100).toFixed(0) }}% equivalence)
        </span>
        <span v-else-if="manifest.verification.status === 'FAIL'">
          ❌ Verification Failed ({{ (manifest.verification.equivalence_score! * 100).toFixed(0) }}% equivalence)
        </span>
        <span v-else>
          📋 {{ manifest.verification.status }}
        </span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.journey-viewer {
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  padding: 1.5rem;
  margin: 1.5rem 0;
  background: var(--vp-c-bg-soft);
}

.journey-loading {
  text-align: center;
  padding: 2rem;
  color: var(--vp-c-text-2);
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--vp-c-divider);
  border-top-color: var(--vp-c-brand-1);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.journey-error {
  text-align: center;
  padding: 2rem;
  color: var(--vp-c-yellow-1);
}

.error-hint {
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
  margin-top: 0.5rem;
}

.journey-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 1rem;
  margin-bottom: 1rem;
}

.journey-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  align-items: center;
}

.platform-badge {
  background: var(--vp-c-brand-1);
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 600;
}

.badge {
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
  font-weight: 600;
}

.badge-success { background: #22c55e; color: white; }
.badge-warning { background: #f59e0b; color: white; }
.badge-info { background: #3b82f6; color: white; }
.badge-neutral { background: #6b7280; color: white; }

.duration, .date {
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
}

.intent-summary {
  background: var(--vp-c-bg);
  padding: 1rem;
  border-radius: 6px;
  margin-bottom: 1rem;
}

.intent-summary h4 {
  margin: 0 0 0.5rem;
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
}

.intent-summary p {
  margin: 0;
  font-size: 0.9375rem;
}

.steps-overview {
  margin-bottom: 1rem;
}

.steps-overview h4 {
  margin: 0 0 0.75rem;
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
}

.steps-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 0.5rem;
}

.step-card {
  display: flex;
  gap: 0.5rem;
  padding: 0.5rem;
  background: var(--vp-c-bg);
  border: 1px solid var(--vp-c-divider);
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8125rem;
  transition: all 0.2s;
}

.step-card:hover {
  border-color: var(--vp-c-brand-1);
}

.step-card.active {
  border-color: var(--vp-c-brand-1);
  background: var(--vp-c-brand-1);
  color: white;
}

.step-number {
  font-weight: 700;
  min-width: 1.5rem;
}

.step-intent {
  color: inherit;
  opacity: 0.9;
}

.keyframe-section {
  margin-bottom: 1rem;
}

.keyframe-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.keyframe-header h4 {
  margin: 0;
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
}

.toggle-gallery {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  background: var(--vp-c-bg);
  border: 1px solid var(--vp-c-divider);
  border-radius: 4px;
  cursor: pointer;
}

.toggle-gallery:hover {
  border-color: var(--vp-c-brand-1);
}

.keyframe-gallery {
  display: flex;
  gap: 0.5rem;
  overflow-x: auto;
  padding: 0.5rem 0;
  margin-bottom: 1rem;
}

.keyframe-thumb {
  flex-shrink: 0;
  cursor: pointer;
  text-align: center;
}

.thumb-placeholder {
  width: 80px;
  height: 50px;
  background: var(--vp-c-divider);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.625rem;
  color: var(--vp-c-text-2);
  margin-bottom: 0.25rem;
}

.keyframe-thumb.active .thumb-placeholder {
  border: 2px solid var(--vp-c-brand-1);
}

.kf-caption {
  font-size: 0.6875rem;
  color: var(--vp-c-text-2);
  display: block;
  max-width: 80px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.current-frame {
  background: var(--vp-c-bg);
  border-radius: 6px;
  overflow: hidden;
}

.frame-placeholder {
  height: 200px;
  background: linear-gradient(135deg, var(--vp-c-bg-soft) 0%, var(--vp-c-divider) 100%);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  color: var(--vp-c-text-2);
}

.kf-caption-large {
  font-size: 0.9375rem;
  color: var(--vp-c-text-1);
  padding: 0 1rem;
  text-align: center;
}

.frame-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  border-top: 1px solid var(--vp-c-divider);
}

.frame-nav button {
  font-size: 0.8125rem;
  padding: 0.375rem 0.75rem;
  background: var(--vp-c-brand-1);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.frame-nav button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.timestamp {
  font-size: 0.875rem;
  color: var(--vp-c-text-2);
  font-family: monospace;
}

.verification-badge {
  text-align: center;
  padding: 0.75rem;
  border-radius: 6px;
  font-size: 0.875rem;
  background: var(--vp-c-bg);
  border: 1px dashed var(--vp-c-divider);
  color: var(--vp-c-text-2);
}

.verification-badge.pending {
  border-color: var(--vp-c-yellow-1);
  color: var(--vp-c-yellow-1);
}

.verification-badge.pass {
  border-color: var(--vp-c-green-1);
  background: rgba(34, 197, 94, 0.1);
  color: var(--vp-c-green-1);
}

.verification-badge.fail {
  border-color: var(--vp-c-red-1);
  background: rgba(239, 68, 68, 0.1);
  color: var(--vp-c-red-1);
}
</style>
