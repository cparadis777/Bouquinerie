<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthorStore } from '../stores/authors'
import { useResponsivePageSize } from '../composables/useResponsivePageSize'
import PageHeader from '../components/primitive/PageHeader.vue'
import PaginationBar from '../components/primitive/PaginationBar.vue'
import LoadingState from '../components/primitive/LoadingState.vue'
import EmptyState from '../components/primitive/EmptyState.vue'

const router = useRouter()
const viewRef = ref<HTMLElement | null>(null)
const store = useAuthorStore()
const { pageSize } = useResponsivePageSize({
  containerRef: viewRef,
  minPageSize: 30,
})

watch(pageSize, (val) => { store.pageSize = val }, { immediate: true })
watch(pageSize, () => store.load())

onMounted(() => store.load())
</script>

<template>
  <div class="authors-view" ref="viewRef">
    <PageHeader title="Authors" :count="store.total" />
    <LoadingState v-if="store.loading" />
    <template v-else-if="store.data.length">
      <div class="list">
        <div
          v-for="author in store.data"
          :key="author.id"
          class="author-card"
          @click="router.push(`/authors/${author.id}`)"
        >
          <div class="author-icon">⊛</div>
          <div>
            <div class="author-name">{{ author.name }}</div>
          </div>
        </div>
      </div>
      <PaginationBar
        v-model:page="store.page"
        :total="store.total"
        :page-size="store.pageSize"
      />
    </template>
    <EmptyState v-else message="No authors found." />
  </div>
</template>

<style scoped>
.authors-view {
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

.author-card {
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

.author-card:hover {
  background: var(--surface-hover);
}

.author-icon {
  font-size: 22px;
  color: var(--primary);
  width: 32px;
  text-align: center;
}

.author-name {
  font-size: 14px;
  font-weight: 500;
}
</style>
