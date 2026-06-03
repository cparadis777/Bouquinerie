<script setup lang="ts">
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useSeriesStore } from '../stores/series'
import BackButton from '../components/BackButton.vue'
import BookListItem from '../components/BookListItem.vue'
import LoadingState from '../components/LoadingState.vue'
import EmptyState from '../components/EmptyState.vue'

const route = useRoute()
const store = useSeriesStore()
onMounted(() => store.fetchSeries(route.params.id as string))
</script>

<template>
  <div class="series-detail">
    <BackButton to="/series" label="Series" />

    <LoadingState v-if="store.detailLoading" />

    <template v-else-if="store.currentSeries">
      <h1>{{ store.currentSeries.name }}</h1>
      <p class="caption" v-if="store.seriesBooks.length">{{ store.seriesBooks.length }} books in series</p>

      <div class="list" v-if="store.seriesBooks.length">
        <BookListItem
          v-for="entry in store.seriesBooks"
          :key="entry.book.id"
          :entry="entry"
        />
      </div>
      <EmptyState v-else message="No books in this series." />
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

.list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
</style>
