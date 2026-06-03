<script setup lang="ts">
import { useRouter } from 'vue-router'
import BookCover from './BookCover.vue'
import type { components } from '../../types/api'

const router = useRouter()

defineProps<{
  entry: components['schemas']['BookListEntry']
  tintIndex?: number
}>()
</script>

<template>
  <div class="book-card" @click="router.push(`/books/${entry.book.id}`)">
    <BookCover
      :cover-path="entry.book.cover_path"
      :title="entry.book.title"
      size="lg"
      :tint-index="tintIndex"
    />
    <div class="book-title">{{ entry.book.title }}</div>
    <div v-if="entry.author_names.length" class="book-author caption">
      {{ entry.author_names.join(', ') }}
    </div>
  </div>
</template>

<style scoped>
.book-card {
  cursor: pointer;
  transition: transform 0.15s;
}

.book-card:hover {
  transform: translateY(-2px);
}

.book-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.book-author {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
