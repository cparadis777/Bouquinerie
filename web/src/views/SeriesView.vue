<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '../api/client'
import { useRouter } from 'vue-router'
import type { components } from '../types/api'

const router = useRouter()

const seriesList = ref<components["schemas"]["Series"][]>([])
const total = ref(0)
const page = ref(1)
const pages = ref(0)
const loading = ref(true)

async function loadSeries(p: number = 1) {
  loading.value = true
  const res = await api.GET('/api/series', {
    params: { query: { page: p, page_size: 30 } },
  })
  if (res.data) {
    seriesList.value = res.data.data
    total.value = res.data.total
    page.value = res.data.page
    pages.value = res.data.pages
  }
  loading.value = false
}

onMounted(() => loadSeries())
</script>

<template>
  <div class="series-view">
    <div class="header">
      <h1>Series</h1>
      <span class="count caption">{{ total }} total</span>
    </div>

    <div v-if="loading" class="loading">Loading...</div>

    <template v-else-if="seriesList.length">
      <div class="list">
        <div
          v-for="s in seriesList"
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

      <div class="pagination" v-if="pages > 1">
        <button :disabled="page <= 1" @click="loadSeries(page - 1)">Previous</button>
        <span class="caption">Page {{ page }} of {{ pages }}</span>
        <button :disabled="page >= pages" @click="loadSeries(page + 1)">Next</button>
      </div>
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
  gap: 16px;
  padding: 16px 0;
}

.pagination button {
  padding: 8px 16px;
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 13px;
}

.pagination button:hover:not(:disabled) {
  background: var(--surface-hover);
}

.pagination button:disabled {
  opacity: 0.4;
  cursor: default;
}
</style>
