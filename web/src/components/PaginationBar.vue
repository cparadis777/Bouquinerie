<script setup lang="ts">
import { computed } from 'vue'
import { PaginationRoot, PaginationPrev, PaginationNext, PaginationList, PaginationListItem, PaginationEllipsis } from 'reka-ui'

const props = defineProps<{
  page: number
  total: number
  pageSize: number
}>()

const emit = defineEmits<{
  'update:page': [value: number]
}>()

const pages = computed(() => Math.ceil(props.total / props.pageSize))

const currentPage = computed({
  get: () => props.page,
  set: (val: number) => emit('update:page', val),
})
</script>

<template>
  <PaginationRoot
    v-if="pages > 1"
    v-model:page="currentPage"
    :items-per-page="pageSize"
    :total="total"
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
    <span class="caption">Page {{ currentPage }} of {{ pages }}</span>
    <PaginationNext class="pagination-btn">Next</PaginationNext>
  </PaginationRoot>
</template>

<style scoped>
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
