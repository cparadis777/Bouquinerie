<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthorStore } from '../stores/authors'
import PageHeader from '../components/PageHeader.vue'
import PaginationBar from '../components/PaginationBar.vue'
import LoadingState from '../components/LoadingState.vue'
import EmptyState from '../components/EmptyState.vue'

const router = useRouter()
const store = useAuthorStore()
onMounted(() => store.load())
</script>

<template>
  <div class="authors-view">
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
        :page-size="30"
      />
    </template>
    <EmptyState v-else message="No authors found." />
  </div>
</template>

<style scoped>
.authors-view {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
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
