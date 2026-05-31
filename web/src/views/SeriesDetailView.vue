<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { api } from '../api/client'
import type { components } from '../types/api'

const router = useRouter()

const seriesName = ref('')
const books = ref<components["schemas"]["BookListEntry"][]>([])
const loading = ref(true)

onMounted(async () => {
  // Stub: fetch all books until the backend has GET /api/series/{id}/books
  const res = await api.GET('/api/books', {
    params: { query: { page: 1, page_size: 100 } },
  })

  if (res.data) {
    books.value = res.data.data
    seriesName.value = 'Series'
  }

  loading.value = false
})
</script>

<template>
  <div class="series-detail">
    <button class="back" @click="router.push('/series')">← Back to Series</button>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else>
      <h1>{{ seriesName }}</h1>
      <p class="caption" v-if="books.length">{{ books.length }} books in series</p>

      <div class="list" v-if="books.length">
        <div
          v-for="(entry, idx) in books"
          :key="entry.book.id"
          class="series-book"
          @click="router.push(`/books/${entry.book.id}`)"
        >
          <span class="position">#{{ idx + 1 }}</span>
          <div class="book-info">
            <div class="book-title">{{ entry.book.title }}</div>
            <div class="caption">{{ entry.author_names.join(', ') }}</div>
          </div>
        </div>
      </div>

      <div v-else class="empty">No books in this series.</div>
    </template>
  </div>
</template>

<style scoped>
.series-detail {
  display: flex;
  flex-direction: column;
  gap: 20px;
  max-width: 700px;
}

.back {
  align-self: flex-start;
  color: var(--primary);
  font-size: 13px;
  padding: 4px 0;
}

.loading, .empty {
  color: var(--text-muted);
  padding: 48px 0;
  text-align: center;
}

.list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.series-book {
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

.series-book:hover {
  background: var(--surface-hover);
}

.position {
  font-size: 14px;
  font-weight: 500;
  color: var(--primary);
  width: 36px;
  flex-shrink: 0;
}

.book-title {
  font-size: 14px;
  font-weight: 500;
}
</style>
