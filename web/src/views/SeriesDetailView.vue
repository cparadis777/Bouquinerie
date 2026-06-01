<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { api } from '../api/client'
import AppButton from '../components/AppButton.vue'
import type { components } from '../types/api'

const router = useRouter()
const route = useRoute()

const seriesName = ref('')
const books = ref<components["schemas"]["BookListEntry"][]>([])
const loading = ref(true)

onMounted(async () => {
  const id = route.params.id as string

  const [seriesRes, booksRes] = await Promise.all([
    api.GET('/api/series/{id}', { params: { path: { id } } }),
    api.GET('/api/books', { params: { query: { series_id: id, page: 1, page_size: 100 } } }),
  ])

  if (seriesRes.data) {
    seriesName.value = seriesRes.data.name
  }
  if (booksRes.data) {
    books.value = booksRes.data.data
  }

  loading.value = false
})
</script>

<template>
  <div class="series-detail">
    <AppButton variant="ghost" @click="router.push('/series')">← Back to Series</AppButton>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else>
      <h1>{{ seriesName }}</h1>
      <p class="caption" v-if="books.length">{{ books.length }} books in series</p>

      <div class="list" v-if="books.length">
        <div
          v-for="(entry, _idx) in books"
          :key="entry.book.id"
          class="series-book"
          @click="router.push(`/books/${entry.book.id}`)"
        >
          <img
            v-if="entry.book.cover_path"
            :src="`/covers/${entry.book.cover_path}`"
            alt=""
            class="cover-image"
          />
          <div v-else class="cover-placeholder">
            {{ entry.book.title.charAt(0).toUpperCase() }}
          </div>
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

.cover-image {
  width: 48px;
  height: 72px;
  border-radius: 4px;
  object-fit: cover;
  flex-shrink: 0;
}

.cover-placeholder {
  width: 48px;
  height: 72px;
  border-radius: 4px;
  background: var(--placeholder-4);
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-heading);
  font-size: 18px;
  color: var(--placeholder-fg);
  flex-shrink: 0;
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
