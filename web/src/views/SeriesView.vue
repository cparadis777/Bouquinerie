<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useSeriesStore } from '../stores/series'
import { useResponsivePageSize } from '../composables/useResponsivePageSize'
import PageHeader from '../components/PageHeader.vue'
import PaginationBar from '../components/PaginationBar.vue'
import LoadingState from '../components/LoadingState.vue'
import EmptyState from '../components/EmptyState.vue'

const router = useRouter()
const viewRef = ref<HTMLElement | null>(null)
const store = useSeriesStore()
const { pageSize } = useResponsivePageSize({
  containerRef: viewRef,
  minPageSize: 30,
})

watch(pageSize, (val) => { store.pageSize = val }, { immediate: true })
watch(pageSize, () => store.load())

onMounted(() => store.load())
</script>

<template>
  <div class="series-view" ref="viewRef">
    <PageHeader title="Series" :count="store.total" />
    <LoadingState v-if="store.loading" />
    <template v-else-if="store.data.length">
      <div class="list">
        <div
          v-for="s in store.data"
          :key="s.id"
          class="series-card"
          @click="router.push(`/series/${s.id}`)"
        >
          <div class="series-icon">⊟</div>
          <div>
            <div class="series-name">{{ s.name }}</div>
          </div>
        </div>
      </div>
      <PaginationBar
        v-model:page="store.page"
        :total="store.total"
        :page-size="store.pageSize"
      />
    </template>
    <EmptyState v-else message="No series found." />
  </div>
</template>

<style scoped>
.series-view {
  --min-card-width: 200px;
  --card-gap: 12px;

  display: flex;
  flex-direction: column;
  gap: 24px;
}

.list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(var(--min-card-width), 1fr));
  gap: var(--card-gap);
}

.series-card {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 16px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s;
}

.series-card:hover {
  background: var(--surface-hover);
}

.series-icon {
  font-size: 22px;
  color: var(--primary);
  width: 32px;
  text-align: center;
}

.series-name {
  font-size: 14px;
  font-weight: 500;
}
</style>
