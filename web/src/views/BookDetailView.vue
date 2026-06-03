<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { CollapsibleRoot as Collapsible, CollapsibleTrigger, CollapsibleContent } from 'reka-ui'
import { useBookStore } from '../stores/books'
import BackButton from '../components/primitive/BackButton.vue'
import BookCover from '../components/book/BookCover.vue'
import LoadingState from '../components/primitive/LoadingState.vue'
import { formatDate, formatIsbn, formatLanguage } from '../utils/format'

const route = useRoute()
const store = useBookStore()
const descExpanded = ref(false)
const lightboxOpen = ref(false)
const lightboxRef = ref<HTMLElement | null>(null)

const identifiers = computed(() => {
  const labels: Record<string, string> = {
    isbn_13: 'ISBN-13',
    isbn_10: 'ISBN-10',
    asin: 'ASIN',
    google_books_id: 'Google Books',
    open_library_id: 'Open Library',
    goodreads_id: 'Goodreads',
    viaf: 'VIAF',
  }
  const known: { label: string; value: string }[] = []
  const other: { source: string; value: string }[] = []
  const seenSources = new Set<string>()
  for (const id of store.currentBook?.identifiers ?? []) {
    seenSources.add(id.source)
    const label = labels[id.source]
    if (label) known.push({ label, value: id.source === 'isbn_13' ? formatIsbn(id.value) : id.value })
    else other.push({ source: id.source, value: id.value })
  }
  return { known, other }
})

onMounted(() => store.fetchBook(route.params.id as string))

watch(lightboxOpen, (open) => {
  if (open) nextTick(() => lightboxRef.value?.focus())
})
</script>

<template>
  <div class="detail">
    <BackButton to="/books" label="Books" />

    <LoadingState v-if="store.detailLoading" />

    <template v-else-if="store.currentBook">
      <div class="hero">
        <div
          v-if="store.currentBook.book.cover_path"
          class="cover-wrapper"
          @click="lightboxOpen = true"
        >
          <BookCover
            :cover-path="store.currentBook.book.cover_path"
            :title="store.currentBook.book.title"
            size="md"
          />
        </div>
        <BookCover
          v-else
          :cover-path="null"
          :title="store.currentBook.book.title"
          size="md"
        />
        <div class="hero-text">
          <h1>{{ store.currentBook.book.title }}</h1>
          <p v-if="store.currentBook.book.subtitle" class="subtitle">{{ store.currentBook.book.subtitle }}</p>
          <p class="authors">{{ store.currentBook.authors.map(a => a.name).join(', ') }}</p>
          <div v-if="store.currentBook.series.length" class="series-badge">
            {{ store.currentBook.series[0].name }}
          </div>
        </div>
      </div>

      <Teleport to="body">
        <div
          v-if="lightboxOpen"
          ref="lightboxRef"
          class="lightbox-overlay"
          tabindex="-1"
          @click.self="lightboxOpen = false"
          @keydown.escape="lightboxOpen = false"
        >
          <img
            :src="`/covers/${store.currentBook.book.cover_path}`"
            alt=""
            class="lightbox-image"
          />
          <button class="lightbox-close" @click="lightboxOpen = false">×</button>
        </div>
      </Teleport>

      <section class="section">
        <Collapsible v-model:open="descExpanded" :unmount-on-hide="false">
          <CollapsibleContent as="p" class="description">
            {{ store.currentBook.book.description || 'No description.' }}
          </CollapsibleContent>
          <CollapsibleTrigger
            v-if="store.currentBook.book.description.length > 200"
            class="expand-btn"
          >
            {{ descExpanded ? 'Show less' : 'Show more' }}
          </CollapsibleTrigger>
        </Collapsible>
      </section>

      <section class="section metadata">
        <h2>Details</h2>
        <table>
          <tr v-if="store.currentBook.book.publisher">
            <td>Publisher</td>
            <td>{{ store.currentBook.book.publisher }}</td>
          </tr>
          <tr v-if="store.currentBook.book.published_date">
            <td>Published</td>
            <td>{{ formatDate(store.currentBook.book.published_date) }}</td>
          </tr>
          <tr v-if="store.currentBook.book.page_count">
            <td>Pages</td>
            <td>{{ store.currentBook.book.page_count }}</td>
          </tr>
          <tr>
            <td>Language</td>
            <td>{{ formatLanguage(store.currentBook.book.language) }}</td>
          </tr>
        </table>
      </section>

      <section class="section" v-if="identifiers.known.length || identifiers.other.length">
        <h2>Identifiers</h2>
        <div class="id-field" v-for="field in identifiers.known" :key="field.label">
          <span class="id-label">{{ field.label }}</span>
          <span class="id-value">{{ field.value }}</span>
        </div>
        <div class="id-field" v-for="id in identifiers.other" :key="id.source">
          <span class="id-label">{{ id.source }}</span>
          <span class="id-value">{{ id.value }}</span>
        </div>
      </section>

      <section class="section" v-if="store.currentBook.authors.length">
        <h2>Authors</h2>
        <div class="chips">
          <router-link
            v-for="author in store.currentBook.authors"
            :key="author.id"
            :to="`/authors/${author.id}`"
            class="chip"
          >
            {{ author.name }}
          </router-link>
        </div>
      </section>

      <section class="section" v-if="store.currentBook.series.length">
        <h2>Series</h2>
        <div class="chips">
          <router-link
            v-for="s in store.currentBook.series"
            :key="s.id"
            :to="`/series/${s.id}`"
            class="chip"
          >
            {{ s.name }}
          </router-link>
        </div>
      </section>
    </template>

    <div v-else class="not-found">Book not found.</div>
  </div>
</template>

<style scoped>
.detail {
  display: flex;
  flex-direction: column;
  gap: 28px;
  max-width: 700px;
}

.not-found {
  color: var(--text-muted);
  padding: 48px 0;
  text-align: center;
}

.hero {
  display: flex;
  gap: 28px;
  align-items: flex-start;
}

.cover-wrapper {
  cursor: zoom-in;
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

.id-field {
  display: flex;
  gap: 12px;
  font-size: 13px;
}

.id-label {
  color: var(--text-muted);
  min-width: 100px;
}

.id-value {
  color: var(--text);
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

<style>
.lightbox-overlay {
  position: fixed;
  inset: 0;
  z-index: 100;
  background: rgba(0, 0, 0, 0.85);
  display: flex;
  align-items: center;
  justify-content: center;
}

.lightbox-image {
  max-width: 90vw;
  max-height: 90vh;
  border-radius: 8px;
  object-fit: contain;
}

.lightbox-close {
  position: fixed;
  top: 16px;
  right: 16px;
  font-size: 32px;
  color: var(--text);
  background: none;
  border: none;
  cursor: pointer;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background 0.15s;
}

.lightbox-close:hover {
  background: rgba(255, 255, 255, 0.1);
}
</style>
