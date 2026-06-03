<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { CollapsibleRoot as Collapsible, CollapsibleTrigger, CollapsibleContent } from 'reka-ui'
import { useBookStore } from '../stores/books'
import BackButton from '../components/BackButton.vue'
import BookCover from '../components/BookCover.vue'
import LoadingState from '../components/LoadingState.vue'

const route = useRoute()
const store = useBookStore()
const descExpanded = ref(false)

onMounted(() => store.fetchBook(route.params.id as string))
</script>

<template>
  <div class="detail">
    <BackButton to="/books" label="Books" />

    <LoadingState v-if="store.detailLoading" />

    <template v-else-if="store.currentBook">
      <div class="hero">
        <BookCover
          :cover-path="store.currentBook.book.cover_path"
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
          <tr v-if="store.currentBook.book.isbn">
            <td>ISBN</td>
            <td>{{ store.currentBook.book.isbn }}</td>
          </tr>
          <tr v-if="store.currentBook.book.published_date">
            <td>Published</td>
            <td>{{ store.currentBook.book.published_date }}</td>
          </tr>
          <tr v-if="store.currentBook.book.page_count">
            <td>Pages</td>
            <td>{{ store.currentBook.book.page_count }}</td>
          </tr>
          <tr>
            <td>Language</td>
            <td>{{ store.currentBook.book.language }}</td>
          </tr>
        </table>
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
