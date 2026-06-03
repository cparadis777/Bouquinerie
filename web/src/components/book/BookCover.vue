<script setup lang="ts">
withDefaults(defineProps<{
  coverPath: string | null
  title: string
  size?: 'sm' | 'md' | 'lg'
  tintIndex?: number
}>(), { size: 'lg', tintIndex: 0 })

const tints = [
  'linear-gradient(135deg, #7A3B3B, #8B3A3A)',
  'linear-gradient(135deg, #7A6B4B, #8B7A5A)',
  'linear-gradient(135deg, #4A6741, #5A7751)',
  'linear-gradient(135deg, #8E5B3A, #9E6B4A)',
  'linear-gradient(135deg, #3A4A5A, #4A5A6A)',
]

function placeholderTint(index: number): string {
  return tints[index % tints.length]
}
</script>

<template>
  <img
    v-if="coverPath"
    :src="`/covers/${coverPath}`"
    alt=""
    :class="['cover-image', `cover--${size}`]"
  />
  <div
    v-else
    :class="['cover-placeholder', `cover--${size}`]"
    :style="{ background: placeholderTint(tintIndex) }"
  >
    {{ title.charAt(0).toUpperCase() }}
  </div>
</template>

<style scoped>
.cover-image {
  object-fit: cover;
  border-radius: 4px;
}

.cover-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-heading);
  color: var(--placeholder-fg);
  border-radius: 4px;
}

.cover--sm {
  width: 48px;
  height: 72px;
  flex-shrink: 0;
}

.cover--sm.cover-placeholder {
  font-size: 18px;
}

.cover--md {
  width: 140px;
  aspect-ratio: 2 / 3;
  border-radius: 6px;
  flex-shrink: 0;
}

.cover--md.cover-placeholder {
  font-size: 48px;
}

.cover--lg {
  width: 100%;
  aspect-ratio: 2 / 3;
}

.cover--lg.cover-placeholder {
  font-size: 32px;
}
</style>
