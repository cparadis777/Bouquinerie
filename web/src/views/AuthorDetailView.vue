<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import type { components } from '../types/api'
import { api } from '../api/client'

const router = useRouter()
const route = useRoute()

const authorName = ref('')
const books = ref<components["schemas"]["BookListEntry"][]>([])
const loading = ref(true)

onMounted(async () => {
  const id = route.params.id as string

  const [authorRes, booksRes] = await Promise.all([
    api.GET('/api/authors/{id}', { params: { path: { id } } }),
    api.GET('/api/books', { params: { query: { author_id: id, page: 1, page_size: 100 } } }),
  ])

  if (authorRes.data) {
    authorName.value = authorRes.data.name
  }
  if (booksRes.data) {
    books.value = booksRes.data.data
  }

  loading.value = false
})
</script>

<template>
  <div class="author-detail">
    <button class="back" @click="router.push('/authors')">← Back to Authors</button>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else>
      <h1>{{ authorName }}</h1>
      <p class="caption" v-if="books.length">{{ books.length }} books</p>

      <div class="grid" v-if="books.length">
        <div
          v-for="entry in books"
          :key="entry.book.id"
          class="book-card"
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
          <div class="book-title">{{ entry.book.title }}</div>
        </div>
      </div>

      <div v-else class="empty">No books found for this author.</div>
    </template>
  </div>
</template>

<style scoped>
.author-detail {
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

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 20px;
}

.book-card {
  cursor: pointer;
  transition: transform 0.15s;
}

.book-card:hover {
  transform: translateY(-2px);
}

.cover-image {
  width: 100%;
  aspect-ratio: 2 / 3;
  border-radius: 4px;
  object-fit: cover;
  margin-bottom: 8px;
}

.cover-placeholder {
  width: 100%;
  aspect-ratio: 2 / 3;
  background: var(--placeholder-4);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-heading);
  font-size: 32px;
  color: var(--placeholder-fg);
  margin-bottom: 8px;
}

.book-title {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>
