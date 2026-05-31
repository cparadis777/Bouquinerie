<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '../api/client'
import { useRouter } from 'vue-router'
import type { components } from '../types/api'

const router = useRouter()

const books = ref<components["schemas"]["BookListEntry"][]>([])
const total = ref(0)
const page = ref(1)
const pages = ref(0)
const loading = ref(true)

async function loadBooks(p: number = 1) {
  loading.value = true
  const res = await api.GET('/api/books', {
    params: { query: { page: p, page_size: 20 } },
  })
  if (res.data) {
    books.value = res.data.data
    total.value = res.data.total
    page.value = res.data.page
    pages.value = res.data.pages
  }
  loading.value = false
}

onMounted(() => loadBooks())

function placeholderTint(index: number): string {
  const tints = [
    'var(--placeholder-1)',
    'var(--placeholder-2)',
    'var(--placeholder-3)',
    'var(--placeholder-4)',
    'var(--placeholder-5)',
  ]
  return tints[index % tints.length]
}
</script>

<template>
  <div class="books-view">
    <div class="header">
      <h1>Books</h1>
      <span class="count caption">{{ total }} total</span>
    </div>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else-if="books.length">
      <div class="grid">
        <div
          v-for="(entry, idx) in books"
          :key="entry.book.id"
          class="book-card"
          @click="router.push(`/books/${entry.book.id}`)"
        >
          <div
            class="cover-placeholder"
            :style="{ background: placeholderTint(idx) }"
          >
            {{ entry.book.title.charAt(0).toUpperCase() }}
          </div>
          <div class="book-title">{{ entry.book.title }}</div>
          <div class="book-author caption">{{ entry.author_names.join(', ') }}</div>
        </div>
      </div>

      <div class="pagination" v-if="pages > 1">
        <button
          :disabled="page <= 1"
          @click="loadBooks(page - 1)"
        >
          Previous
        </button>
        <span class="caption">Page {{ page }} of {{ pages }}</span>
        <button
          :disabled="page >= pages"
          @click="loadBooks(page + 1)"
        >
          Next
        </button>
      </div>
    </template>

    <div v-else class="empty">No books found.</div>
  </div>
</template>

<style scoped>
.books-view {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.header {
  display: flex;
  align-items: baseline;
  gap: 12px;
}

.loading, .empty {
  color: var(--text-muted);
  padding: 48px 0;
  text-align: center;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 24px;
}

.book-card {
  cursor: pointer;
  transition: transform 0.15s;
}

.book-card:hover {
  transform: translateY(-2px);
}

.cover-placeholder {
  width: 100%;
  aspect-ratio: 2 / 3;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-heading);
  font-size: 36px;
  color: var(--placeholder-fg);
  margin-bottom: 8px;
}

.book-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 16px 0;
}

.pagination button {
  padding: 8px 16px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 13px;
  transition: background 0.15s;
}

.pagination button:hover:not(:disabled) {
  background: var(--surface-hover);
}

.pagination button:disabled {
  opacity: 0.4;
  cursor: default;
}
</style>
