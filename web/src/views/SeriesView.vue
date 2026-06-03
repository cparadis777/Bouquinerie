<script setup lang="ts">
import { onMounted } from 'vue'
import { PaginationRoot, PaginationPrev, PaginationNext, PaginationList, PaginationListItem, PaginationEllipsis } from 'reka-ui'
import { useRouter } from 'vue-router'
import { useSeriesStore } from '../stores/series'

const router = useRouter()
const store = useSeriesStore()
onMounted(() => store.load())
</script>

<template>
  <div class="series-view">
    <div class="header">
      <h1>Series</h1>
      <span class="count caption">{{ store.total }} total</span>
    </div>

    <div v-if="store.loading" class="loading">Loading...</div>

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

      <PaginationRoot
        v-if="store.pages > 1"
        v-model:page="store.page"
        :items-per-page="30"
        :total="store.total"
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
        <span class="caption">Page {{ store.page }} of {{ store.pages }}</span>
        <PaginationNext class="pagination-btn">Next</PaginationNext>
      </PaginationRoot>
    </template>

    <div v-else class="empty">No series found.</div>
  </div>
</template>

<style scoped>
.series-view {
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

.list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
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
