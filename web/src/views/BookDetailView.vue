<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { api } from '../api/client'
import type { components } from '../types/api'

const route = useRoute()
const router = useRouter()

const bookData = ref<components["schemas"]["BookResponse"] | null>(null)
const loading = ref(true)
const descExpanded = ref(false)

async function loadBook() {
  loading.value = true
  const id = route.params.id as string
  const res = await api.GET('/api/books/{id}', {
    params: { path: { id } },
  })
  if (res.data) {
    bookData.value = res.data
  }
  loading.value = false
}

onMounted(loadBook)
</script>

<template>
  <div class="detail">
    <button class="back" @click="router.push('/books')">← Back to Books</button>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else-if="bookData">
      <div class="hero">
        <img
          v-if="bookData.book.cover_path"
          :src="`/covers/${bookData.book.cover_path}`"
          alt=""
          class="cover-image"
        />
        <div v-else class="cover-placeholder">
          {{ bookData.book.title.charAt(0).toUpperCase() }}
        </div>
        <div class="hero-text">
          <h1>{{ bookData.book.title }}</h1>
          <p v-if="bookData.book.subtitle" class="subtitle">{{ bookData.book.subtitle }}</p>
          <p class="authors">{{ bookData.authors.map(a => a.name).join(', ') }}</p>
          <div v-if="bookData.series.length" class="series-badge">
            {{ bookData.series[0].name }}
          </div>
        </div>
      </div>

      <section class="section">
        <p class="description" :class="{ expanded: descExpanded }">
          {{ bookData.book.description || 'No description.' }}
        </p>
        <button
          v-if="bookData.book.description.length > 200"
          class="expand-btn"
          @click="descExpanded = !descExpanded"
        >
          {{ descExpanded ? 'Show less' : 'Show more' }}
        </button>
      </section>

      <section class="section metadata">
        <h2>Details</h2>
        <table>
          <tr v-if="bookData.book.publisher">
            <td>Publisher</td>
            <td>{{ bookData.book.publisher }}</td>
          </tr>
          <tr v-if="bookData.book.isbn">
            <td>ISBN</td>
            <td>{{ bookData.book.isbn }}</td>
          </tr>
          <tr v-if="bookData.book.published_date">
            <td>Published</td>
            <td>{{ bookData.book.published_date }}</td>
          </tr>
          <tr v-if="bookData.book.page_count">
            <td>Pages</td>
            <td>{{ bookData.book.page_count }}</td>
          </tr>
          <tr>
            <td>Language</td>
            <td>{{ bookData.book.language }}</td>
          </tr>
        </table>
      </section>

      <section class="section" v-if="bookData.authors.length">
        <h2>Authors</h2>
        <div class="chips">
          <router-link
            v-for="author in bookData.authors"
            :key="author.id"
            :to="`/authors/${author.id}`"
            class="chip"
          >
            {{ author.name }}
          </router-link>
        </div>
      </section>

      <section class="section" v-if="bookData.series.length">
        <h2>Series</h2>
        <div class="chips">
          <router-link
            v-for="s in bookData.series"
            :key="s.id"
            :to="`/series/${s.id}`"
            class="chip"
          >
            {{ s.name }}
          </router-link>
        </div>
      </section>
    </template>

    <div v-else class="loading">Book not found.</div>
  </div>
</template>

<style scoped>
.detail {
  display: flex;
  flex-direction: column;
  gap: 28px;
  max-width: 700px;
}

.back {
  align-self: flex-start;
  color: var(--primary);
  font-size: 13px;
  padding: 4px 0;
}

.loading {
  color: var(--text-muted);
  padding: 48px 0;
  text-align: center;
}

.hero {
  display: flex;
  gap: 28px;
  align-items: flex-start;
}

.cover-image {
  width: 140px;
  aspect-ratio: 2 / 3;
  border-radius: 6px;
  object-fit: cover;
  flex-shrink: 0;
}

.cover-placeholder {
  width: 140px;
  aspect-ratio: 2 / 3;
  background: var(--placeholder-2);
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-heading);
  font-size: 48px;
  color: var(--placeholder-fg);
  flex-shrink: 0;
}

.hero-text {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-top: 4px;
}

.subtitle {
  font-size: 15px;
  color: var(--text-muted);
}

.authors {
  font-size: 14px;
  color: var(--primary);
}

.series-badge {
  display: inline-flex;
  padding: 4px 10px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-muted);
  align-self: flex-start;
}

.section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.description {
  line-height: 1.7;
  max-height: 4.8em;
  overflow: hidden;
  transition: max-height 0.2s;
}

.description.expanded {
  max-height: none;
}

.expand-btn {
  align-self: flex-start;
  color: var(--primary);
  font-size: 13px;
  padding: 4px 0;
}

.metadata table {
  width: 100%;
  border-collapse: collapse;
}

.metadata td {
  padding: 6px 0;
  border-bottom: 1px solid var(--border);
  font-size: 14px;
}

.metadata td:first-child {
  color: var(--text-muted);
  width: 120px;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.chip {
  padding: 6px 14px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 13px;
  color: var(--primary);
  text-decoration: none;
  transition: background 0.15s;
}

.chip:hover {
  background: var(--surface-hover);
}
</style>
