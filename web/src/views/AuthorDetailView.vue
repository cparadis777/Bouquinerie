<script setup lang="ts">
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthorStore } from '../stores/authors'
import BackButton from '../components/primitive/BackButton.vue'
import BookCard from '../components/book/BookCard.vue'
import LoadingState from '../components/primitive/LoadingState.vue'
import EmptyState from '../components/primitive/EmptyState.vue'

const route = useRoute()
const store = useAuthorStore()
onMounted(() => store.fetchAuthor(route.params.id as string))
</script>

<template>
  <div class="author-detail">
    <BackButton to="/authors" label="Authors" />

    <LoadingState v-if="store.detailLoading" />

    <template v-else-if="store.currentAuthor">
      <h1>{{ store.currentAuthor.name }}</h1>
      <p class="caption" v-if="store.authorBooks.length">{{ store.authorBooks.length }} books</p>

      <div class="grid" v-if="store.authorBooks.length">
        <BookCard
          v-for="entry in store.authorBooks"
          :key="entry.book.id"
          :entry="entry"
        />
      </div>
      <EmptyState v-else message="No books found for this author." />
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

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 20px;
}
</style>
