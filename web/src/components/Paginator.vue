<template>
  <PaginationRoot class="root" v-if="pages > 1" :page="page" :total="total" :items-per-page="pageSize"
    :sibling-count="1" @update:page="$emit('update:page', $event)">
    <PaginationList class="list" v-slot="{ items }">
      <PaginationFirst> {{ '<<' }} </PaginationFirst>
          <PaginationPrev> {{ '<' }} </PaginationPrev>

              <template v-for="item in items">
                <PaginationListItem v-if="item.type === 'page'" :key="item.value" :value="item.value">
                  {{ item.value }}

                </PaginationListItem>
                <PaginationEllipsis v-else :key="item.type" />
              </template>

              <PaginationNext> {{ '>' }} </PaginationNext>
              <PaginationLast> {{ '>>' }} </PaginationLast>
    </PaginationList>
  </PaginationRoot>
</template>

<script setup lang="ts">
  import { PaginationEllipsis, PaginationFirst, PaginationLast, PaginationList, PaginationListItem, PaginationNext, PaginationPrev, PaginationRoot } from 'reka-ui'

  defineProps<{ page: number, pages: number, pageSize: number, total: number }>()
</script>

<style lang="css" scoped>

  .root {
    padding-bottom: var(--space-2);
    padding-top: var(--space-2);
  }

  .list {
    display: flex;
    justify-content: center;
    gap: var(--space-5);
  }

  :deep([ data-selected="true"]) {
    color: var(--accent);
    border-radius: var(--radius-sm);
  }

  :deep([disabled]) {
    opacity: 0.4;
    cursor: not-allowed;
  }
</style>
