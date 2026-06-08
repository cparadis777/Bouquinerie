<template>
  <nav class="breadcrumb">
    <button class="back-btn" @click="goBack">
      <Icon icon="mdi-arrow-left" class="back-icon" />
      Back
    </button>
    <span class="breadcrumb-separator">/</span>
    <RouterLink to="/books" class="breadcrumb-link">Books</RouterLink>
    <span class="breadcrumb-separator">/</span>
    <span class="breadcrumb-current">{{ entry.book.title }}</span>
  </nav>

  <article class="book-detail">
    <aside class="cover-column">
      <img
        v-if="entry.book.cover_path && !coverError"
        :src="`/covers/${entry.book.cover_path}`"
        :alt="`Cover for ${entry.book.title}`"
        class="cover-image"
        @error="coverError = true"
      />
      <div
        v-else
        class="cover-placeholder"
        :style="`background-color: var(${color});`"
      >
        {{ initials }}
      </div>

      <div class="actions">
        <a
          v-if="entry.book.book_path"
          :href="`/library/${entry.book.book_path}`"
          class="btn-primary"
          download
        >
          Download
        </a>
        <button class="btn-secondary" @click="goEdit">Edit metadata</button>
      </div>
    </aside>

    <section class="content-column">
      <div class="title-block">
        <h1 class="book-title">{{ entry.book.title }}</h1>
        <p v-if="entry.book.subtitle" class="book-subtitle">
          {{ entry.book.subtitle }}
        </p>

        <div v-if="entry.authors.length" class="authors">
          <span class="by-label">by</span>
          <RouterLink
            v-for="(author, idx) in entry.authors"
            :key="author.id"
            :to="`/authors/${author.id}`"
            class="author-name"
          >
            {{ author.name }}<span v-if="idx < entry.authors.length - 1" class="comma">,</span>
          </RouterLink>
        </div>

        <div v-if="entry.series.length" class="series-badges">
          <RouterLink
            v-for="s in entry.series"
            :key="s.id"
            :to="`/series/${s.id}`"
            class="series-badge"
          >
            {{ s.name }}
          </RouterLink>
        </div>
      </div>

      <div class="section">
        <h2 class="section-title">Description</h2>
        <div
          v-if="entry.book.description"
          class="description"
          :class="{ collapsed: !descriptionExpanded }"
          v-html="entry.book.description"
        />
        <p v-else class="description empty">No description available.</p>
        <button
          v-if="entry.book.description && entry.book.description.length > 300"
          class="text-btn"
          :aria-expanded="descriptionExpanded"
          @click="descriptionExpanded = !descriptionExpanded"
        >
          {{ descriptionExpanded ? 'Show less' : 'Read more' }}
        </button>
      </div>

      <div class="section">
        <h2 class="section-title">Details</h2>
        <dl v-if="hasMetadata" class="metadata-grid">
          <div class="meta-row" v-if="entry.book.publisher">
            <dt class="metadata-label">Publisher</dt>
            <dd class="metadata-value">{{ entry.book.publisher }}</dd>
          </div>
          <div class="meta-row" v-if="entry.book.published_date">
            <dt class="metadata-label">Published</dt>
            <dd class="metadata-value">{{ formatDate(entry.book.published_date) }}</dd>
          </div>
          <div class="meta-row" v-if="isbn">
            <dt class="metadata-label">ISBN</dt>
            <dd class="metadata-value isbn-value" @click="copyIsbn" :title="copyFeedback || 'Click to copy'">
              {{ formatIsbn(isbn) }}
              <span v-if="copyFeedback" class="copy-toast">{{ copyFeedback }}</span>
            </dd>
          </div>
          <div class="meta-row" v-if="entry.book.page_count">
            <dt class="metadata-label">Pages</dt>
            <dd class="metadata-value">{{ entry.book.page_count }}</dd>
          </div>
          <div class="meta-row" v-if="entry.book.language">
            <dt class="metadata-label">Language</dt>
            <dd class="metadata-value">{{ formatLanguage(entry.book.language) }}</dd>
          </div>
        </dl>
        <p v-else class="metadata-empty">No details available.</p>
      </div>

      <div class="section" v-if="parsedIdentifiers.length">
        <h2 class="section-title">Identifiers</h2>
        <div class="identifiers">
          <span
            v-for="ident in parsedIdentifiers"
            :key="ident.id"
            class="identifier-chip"
          >
            {{ ident.label }}: {{ ident.value }}
          </span>
        </div>
      </div>
    </section>
  </article>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { Icon } from '@iconify/vue'
import { getBook } from '../services/books'
import { usePlaceholderCover } from '../composables/usePlaceholderCover'
import {
  formatDate,
  formatIsbn,
  formatLanguage,
} from '../utils/format'


const route = useRoute()
const router = useRouter()
const id = route.params.id as string

const entry = await getBook(id)
const color = usePlaceholderCover(entry.book.id)

const coverError = ref(false)
const descriptionExpanded = ref(false)
const copyFeedback = ref('')

const initials = computed(() => {
  const first = entry.book.sort_title?.[0]
  return first ? first.toUpperCase() : '?'
})

const isbn = computed(() =>
  entry.identifiers.find((i) => i.source.toLowerCase() === 'isbn')?.value
)

const hasMetadata = computed(() =>
  entry.book.publisher ||
  entry.book.published_date ||
  isbn.value ||
  entry.book.page_count ||
  entry.book.language
)

const parsedIdentifiers = computed(() =>
  entry.identifiers
    .filter((i) => i.source.toLowerCase() !== 'isbn')
    .map((i) => {
      const val = i.value
      const idx = val.indexOf(':')
      if (idx === -1) {
        return { id: i.id, label: 'ID', value: val }
      }
      return {
        id: i.id,
        label: val.slice(0, idx).toUpperCase(),
        value: val.slice(idx + 1),
      }
    })
)

function goEdit() {
  router.push(`/books/${id}/edit`)
}

function goBack() {
  if (window.history.length > 1) {
    router.back()
  } else {
    router.push('/books')
  }
}

async function copyIsbn() {
  if (!isbn.value) return
  try {
    await navigator.clipboard.writeText(isbn.value)
    copyFeedback.value = 'Copied!'
    setTimeout(() => { copyFeedback.value = '' }, 2000)
  } catch {
    copyFeedback.value = 'Copy failed'
    setTimeout(() => { copyFeedback.value = '' }, 2000)
  }
}
</script>

<style lang="css" scoped>
.breadcrumb {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3) var(--space-5);
  font-size: var(--text-sm);
  color: var(--text-muted);
  max-width: 1200px;
  margin: 0 auto;
}

.back-btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  color: var(--primary);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  background: none;
  border: none;
  padding: var(--space-1) var(--space-2);
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: background var(--transition-fast);
}

.back-btn:hover {
  background: var(--surface-hover);
  color: var(--primary-hover);
}

.back-btn:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}

.back-icon {
  font-size: 16px;
}

.breadcrumb-separator {
  color: var(--border);
  user-select: none;
}

.breadcrumb-link {
  color: var(--primary);
  text-decoration: none;
  transition: color var(--transition-fast);
}

.breadcrumb-link:hover {
  color: var(--primary-hover);
  text-decoration: underline;
}

.breadcrumb-current {
  color: var(--text-muted);
  font-weight: var(--weight-medium);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 400px;
}

.book-detail {
  display: grid;
  grid-template-columns: 280px 1fr;
  gap: var(--space-6);
  padding: var(--space-5);
  max-width: 1200px;
  margin: 0 auto;
}

/* ── Cover column ────────────────────────────────────────── */
.cover-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  position: sticky;
  top: var(--space-4);
  align-self: start;
}

.cover-image {
  width: 100%;
  aspect-ratio: 2 / 3;
  object-fit: cover;
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-lg);
}

.cover-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  aspect-ratio: 2 / 3;
  border-radius: var(--radius-lg);
  font-family: var(--font-heading);
  font-size: 96px;
  color: var(--placeholder-fg);
  box-shadow: var(--shadow-lg);
}

.actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.btn-primary,
.btn-secondary {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  font-weight: var(--weight-medium);
  font-size: var(--text-sm);
  cursor: pointer;
  transition: background var(--transition-fast);
  text-decoration: none;
}

.btn-primary {
  background: var(--accent);
  color: var(--text);
  border: none;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-secondary {
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  color: var(--text);
}

.btn-secondary:hover {
  background: var(--surface-hover);
}

.btn-primary:focus-visible,
.btn-secondary:focus-visible,
.text-btn:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}

/* ── Content column ──────────────────────────────────────── */
.content-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.title-block {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.book-title {
  font-family: var(--font-heading);
  font-size: var(--text-2xl);
  font-weight: var(--weight-bold);
  color: var(--text);
  line-height: 1.2;
  margin: 0;
  overflow-wrap: break-word;
}

.book-subtitle {
  font-family: var(--font-heading);
  font-size: var(--text-lg);
  font-weight: var(--weight-medium);
  color: var(--text-muted);
  margin: 0;
}

.authors {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
  margin-top: var(--space-1);
}

.by-label {
  font-size: var(--text-sm);
  color: var(--text-muted);
  font-style: italic;
}

.author-name {
  font-size: var(--text-sm);
  color: var(--text);
  font-weight: var(--weight-medium);
  text-decoration: none;
  transition: color var(--transition-fast);
}

.author-name:hover {
  color: var(--primary);
  text-decoration: underline;
}

.author-name:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
  border-radius: 2px;
}

.comma {
  color: var(--text-muted);
  margin-right: var(--space-1);
}

.series-badges {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: var(--space-1);
}

.series-badge {
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-full);
  font-size: var(--text-sm);
  color: var(--text-muted);
  text-decoration: none;
  transition: background var(--transition-fast);
}

.series-badge:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.series-badge:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
  border-radius: var(--radius-full);
}

/* ── Sections ────────────────────────────────────────────── */
.section {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.section-title {
  font-family: var(--font-heading);
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text);
  margin: 0;
}

.description {
  color: var(--text-muted);
  line-height: 1.7;
  font-size: var(--text-base);
  margin: 0;
}

.description.collapsed {
  display: -webkit-box;
  -webkit-line-clamp: 4;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.description.empty {
  font-style: italic;
  opacity: 0.7;
}

.text-btn {
  align-self: flex-start;
  color: var(--primary);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  margin-top: var(--space-1);
}

.text-btn:hover {
  color: var(--primary-hover);
  text-decoration: underline;
}

/* ── Metadata ──────────────────────────────────────────────── */
.metadata-grid {
  display: grid;
  grid-template-columns: 140px 1fr;
  gap: var(--space-2) var(--space-4);
  align-items: baseline;
}

.meta-row {
  display: contents;
}

.metadata-label {
  font-size: var(--text-sm);
  color: var(--text-muted);
  font-weight: var(--weight-medium);
}

.metadata-value {
  font-size: var(--text-sm);
  color: var(--text);
  margin: 0;
}

.isbn-value {
  cursor: pointer;
  position: relative;
  transition: color var(--transition-fast);
}

.isbn-value:hover {
  color: var(--primary);
}

.copy-toast {
  margin-left: var(--space-2);
  font-size: var(--text-xs);
  color: var(--success);
  font-weight: var(--weight-medium);
  animation: fadeIn 150ms ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.metadata-empty {
  font-size: var(--text-sm);
  color: var(--text-muted);
  font-style: italic;
  opacity: 0.7;
  margin: 0;
}

/* ── Identifiers ───────────────────────────────────────────── */
.identifiers {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.identifier-chip {
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-md);
  font-size: var(--text-xs);
  color: var(--text-muted);
  font-family: monospace;
}

/* ── Responsive ───────────────────────────────────────────── */
@media (max-width: 800px) {
  .book-detail {
    grid-template-columns: 1fr;
  }

  .cover-column {
    position: static;
    max-width: 240px;
    margin: 0 auto;
    align-self: center;
  }

  .metadata-grid {
    grid-template-columns: 120px 1fr;
  }
}
</style>
