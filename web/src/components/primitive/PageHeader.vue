<script setup lang="ts">
defineProps<{
  title: string
  count?: number
}>()
</script>

<template>
  <div class="page-header">
    <h1>{{ title }}</h1>
    <span v-if="count !== undefined" class="count caption">{{ count }} total</span>
  </div>
</template>

<style scoped>
.page-header {
  display: flex;
  align-items: baseline;
  gap: 12px;
}
</style>
