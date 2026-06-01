<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { PaginationRoot, PaginationPrev, PaginationNext, PaginationList, PaginationListItem, PaginationEllipsis } from 'reka-ui'
import { api } from '../api/client'
import { useRouter } from 'vue-router'
import type { components } from '../types/api'

const router = useRouter()

const books = ref<components["schemas"]["BookListEntry"][]>([])
const total = ref(0)
const page = ref(1)
const loading = ref(true)
const pageSize = 20
const pages = computed(() => Math.ceil(total.value / pageSize))

async function loadBooks() {
  loading.value = true
  const res = await api.GET('/api/books', {
    params: { query: { page: page.value, page_size: pageSize } },
  })
  if (res.data) {
    books.value = res.data.data
    total.value = res.data.total
    page.value = res.data.page
  }
  loading.value = false
}

watch(page, loadBooks)
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
          <img
            v-if="entry.book.cover_path"
            :src="`/covers/${entry.book.cover_path}`"
            alt=""
            class="cover-image"
          />
          <div
            v-else
            class="cover-placeholder"
            :style="{ background: placeholderTint(idx) }"
          >
            {{ entry.book.title.charAt(0).toUpperCase() }}
          </div>
          <div class="book-title">{{ entry.book.title }}</div>
          <div class="book-author caption">{{ entry.author_names.join(', ') }}</div>
        </div>
      </div>

      <PaginationRoot
        v-if="pages > 1"
        v-model:page="page"
        :items-per-page="pageSize"
        :total="total"
        :sibling-count="2"
        class="pagination"
      >
        <PaginationPrev class="pagination-btn">Previous</PaginationPrev>
        <PaginationList v-slot="{ items }">
          <template v-for="item in items">
            <PaginationListItem
              v-if="item.type === 'page'"
              :key="item.value"
              :value="item.value"
              class="pagination-btn"
            />
            <PaginationEllipsis v-else :key="item.type" />
          </template>
        </PaginationList>
        <span class="caption">Page {{ page }} of {{ pages }}</span>
        <PaginationNext class="pagination-btn">Next</PaginationNext>
      </PaginationRoot>
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
  gap: 8px;
  padding: 16px 0;
}

.pagination-btn {
  padding: 8px 12px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-family: var(--font-body);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.15s;
}

.pagination-btn:hover:not(:disabled) {
  background: var(--surface-hover);
}

.pagination-btn:disabled {
  opacity: 0.4;
  cursor: default;
}

.pagination-btn[data-selected="true"] {
  background: var(--primary);
  border-color: var(--primary);
  color: var(--surface);
}
</style>
