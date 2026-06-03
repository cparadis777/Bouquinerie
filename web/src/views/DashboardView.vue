<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getBooks } from '../services/books'
import { getAuthors } from '../services/authors'
import { getSeries } from '../services/series'
import BookCard from '../components/BookCard.vue'
import LoadingState from '../components/LoadingState.vue'
import ErrorState from '../components/ErrorState.vue'

const bookCount = ref(0)
const authorCount = ref(0)
const seriesCount = ref(0)
const recentBooks = ref<any[]>([])
const loading = ref(true)
const error = ref<Error | null>(null)

async function fetchStats() {
  const [books, authors, series] = await Promise.all([
    getBooks({ page: 1, page_size: 10 }),
    getAuthors({ page: 1, page_size: 1 }),
    getSeries({ page: 1, page_size: 1 }),
  ])
  bookCount.value = books.total
  recentBooks.value = books.data.slice(0, 10)
  authorCount.value = authors.total
  seriesCount.value = series.total
}

onMounted(async () => {
  loading.value = true
  error.value = null
  try {
    await fetchStats()
  } catch (e) {
    error.value = e as Error
  }
  loading.value = false
})
</script>

<template>
  <div class="dashboard">
    <h1>Dashboard</h1>

    <LoadingState v-if="loading" />
    <ErrorState v-else-if="error" :error="error" @retry="fetchStats()" />

    <template v-else>
      <div class="stats">
        <div class="stat">{{ bookCount }} books</div>
        <div class="stat">{{ authorCount }} authors</div>
        <div class="stat">{{ seriesCount }} series</div>
      </div>

      <section class="section">
        <h2>Recently Added</h2>
        <div class="book-row" v-if="recentBooks.length">
          <BookCard
            v-for="entry in recentBooks"
            :key="entry.book.id"
            :entry="entry"
          />
        </div>
        <div v-else class="no-books">No books yet.</div>
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

.book-row :deep(.book-card) {
  flex-shrink: 0;
  width: 140px;
}

.no-books {
  color: var(--text-muted);
  font-size: 14px;
  padding: 24px 0;
}
</style>
