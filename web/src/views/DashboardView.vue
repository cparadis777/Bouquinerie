<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '../api/client'
import { useRouter } from 'vue-router'
import type { components } from '../types/api'

const router = useRouter()

const bookCount = ref(0)
const authorCount = ref(0)
const seriesCount = ref(0)
const recentBooks = ref<components["schemas"]["BookListEntry"][]>([])
const surpriseBook = ref<components["schemas"]["BookListEntry"] | null>(null)
const loading = ref(true)

async function fetchStats() {
  const [booksRes, authorsRes, seriesRes] = await Promise.all([
    api.GET('/api/books', { params: { query: { page: 1, page_size: 10 } } }),
    api.GET('/api/authors', { params: { query: { page: 1, page_size: 1 } } }),
    api.GET('/api/series', { params: { query: { page: 1, page_size: 1 } } }),
  ])

  if (booksRes.data) {
    bookCount.value = booksRes.data.total
    recentBooks.value = booksRes.data.data.slice(0, 10)
  }
  if (authorsRes.data) {
    authorCount.value = authorsRes.data.total
  }
  if (seriesRes.data) {
    seriesCount.value = seriesRes.data.total
  }
}

async function fetchSurprise() {
  const res = await api.GET('/api/books', {
    params: { query: { page: 1, page_size: 100 } },
  })
  if (res.data && res.data.data.length > 0) {
    const idx = Math.floor(Math.random() * res.data.data.length)
    surpriseBook.value = res.data.data[idx]
  }
}

onMounted(async () => {
  loading.value = true
  await Promise.all([fetchStats(), fetchSurprise()])
  loading.value = false
})
</script>

<template>
  <div class="dashboard">
    <h1>Dashboard</h1>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else>
      <div class="stats">
        <div class="stat">{{ bookCount }} books</div>
        <div class="stat">{{ authorCount }} authors</div>
        <div class="stat">{{ seriesCount }} series</div>
      </div>

      <section class="section">
        <h2>Recently Added</h2>
        <div class="book-row" v-if="recentBooks.length">
          <div
            v-for="entry in recentBooks"
            :key="entry.book.id"
            class="book-card"
            @click="router.push(`/books/${entry.book.id}`)"
          >
            <div class="cover-placeholder">{{ entry.book.title.charAt(0).toUpperCase() }}</div>
            <div class="book-info">
              <div class="book-title">{{ entry.book.title }}</div>
              <div class="book-author">{{ entry.author_names.join(', ') }}</div>
            </div>
          </div>
        </div>
        <div v-else class="empty">No books yet.</div>
      </section>

      <section class="section">
        <h2>Surprise Me</h2>
        <div v-if="surpriseBook" class="surprise-card" @click="router.push(`/books/${surpriseBook.book.id}`)">
          <div class="cover-placeholder large">{{ surpriseBook.book.title.charAt(0).toUpperCase() }}</div>
          <div>
            <div class="book-title">{{ surpriseBook.book.title }}</div>
            <div class="book-author">{{ surpriseBook.author_names.join(', ') }}</div>
          </div>
        </div>
        <div v-else class="empty">No books yet.</div>
      </section>
    </template>
  </div>
</template>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: 32px;
}

.loading {
  color: var(--text-muted);
  padding: 48px 0;
  text-align: center;
}

.stats {
  display: flex;
  gap: 24px;
  flex-wrap: wrap;
}

.stat {
  font-size: 14px;
  color: var(--text-muted);
  padding: 8px 16px;
  background: var(--surface-elevated);
  border-radius: 8px;
  border: 1px solid var(--border);
}

.section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.book-row {
  display: flex;
  gap: 16px;
  overflow-x: auto;
  padding-bottom: 8px;
}

.book-card {
  flex-shrink: 0;
  width: 140px;
  cursor: pointer;
  transition: transform 0.15s;
}

.book-card:hover {
  transform: translateY(-2px);
}

.cover-placeholder {
  width: 100%;
  aspect-ratio: 2 / 3;
  background: var(--placeholder-1);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-heading);
  font-size: 32px;
  color: var(--placeholder-fg);
  margin-bottom: 8px;
}

.cover-placeholder.large {
  width: 120px;
  font-size: 40px;
}

.book-info {
  padding: 0 2px;
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
  font-size: 11px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.empty {
  color: var(--text-muted);
  font-size: 14px;
  padding: 24px 0;
}

.surprise-card {
  display: flex;
  gap: 20px;
  align-items: flex-start;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 20px;
  cursor: pointer;
  max-width: 400px;
  transition: background 0.15s;
}

.surprise-card:hover {
  background: var(--surface-hover);
}
</style>
