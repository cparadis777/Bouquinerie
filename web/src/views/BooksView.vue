<script setup lang="ts">
import { onMounted } from 'vue'
import { useBookStore } from '../stores/books'
import PageHeader from '../components/PageHeader.vue'
import BookCard from '../components/BookCard.vue'
import PaginationBar from '../components/PaginationBar.vue'
import LoadingState from '../components/LoadingState.vue'
import EmptyState from '../components/EmptyState.vue'

const store = useBookStore()
onMounted(() => store.load())
</script>

<template>
  <div class="books-view">
    <PageHeader title="Books" :count="store.total" />
    <LoadingState v-if="store.loading" />
    <template v-else-if="store.data.length">
      <div class="grid">
        <BookCard
          v-for="(entry, idx) in store.data"
          :key="entry.book.id"
          :entry="entry"
          :tint-index="idx"
        />
      </div>
      <PaginationBar
        v-model:page="store.page"
        :total="store.total"
        :page-size="20"
      />
    </template>
    <EmptyState v-else message="No books found." />
  </div>
</template>

<style scoped>
.books-view {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 24px;
}
</style>
