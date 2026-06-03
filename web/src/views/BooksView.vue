<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useBookStore } from '../stores/books'
import { useResponsivePageSize } from '../composables/useResponsivePageSize'
import PageHeader from '../components/PageHeader.vue'
import BookCard from '../components/BookCard.vue'
import PaginationBar from '../components/PaginationBar.vue'
import LoadingState from '../components/LoadingState.vue'
import EmptyState from '../components/EmptyState.vue'

const viewRef = ref<HTMLElement | null>(null)
const store = useBookStore()
const { pageSize } = useResponsivePageSize({
  containerRef: viewRef,
})

watch(pageSize, (val) => { store.pageSize = val }, { immediate: true })
watch(pageSize, () => store.load())

onMounted(() => store.load())
</script>

<template>
  <div class="books-view" ref="viewRef">
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
        :page-size="store.pageSize"
      />
    </template>
    <EmptyState v-else message="No books found." />
  </div>
</template>

<style scoped>
.books-view {
  --min-card-width: 160px;
  --card-gap: 24px;

  display: flex;
  flex-direction: column;
  gap: 24px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(var(--min-card-width), 1fr));
  gap: var(--card-gap);
}
</style>
