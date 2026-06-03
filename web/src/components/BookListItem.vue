<script setup lang="ts">
import { useRouter } from 'vue-router'
import BookCover from './BookCover.vue'
import type { components } from '../types/api'

const router = useRouter()

defineProps<{
  entry: components['schemas']['BookListEntry']
  tintIndex?: number
}>()
</script>

<template>
  <div class="book-list-item" @click="router.push(`/books/${entry.book.id}`)">
    <BookCover
      :cover-path="entry.book.cover_path"
      :title="entry.book.title"
      size="sm"
      :tint-index="tintIndex"
    />
    <div class="item-info">
      <div class="item-title">{{ entry.book.title }}</div>
      <div v-if="entry.author_names.length" class="item-author caption">
        {{ entry.author_names.join(', ') }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.book-list-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 16px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.book-list-item:hover {
  background: var(--surface-hover);
}

.item-title {
  font-size: 14px;
  font-weight: 500;
}

.item-author {
  margin-top: 2px;
}
</style>
