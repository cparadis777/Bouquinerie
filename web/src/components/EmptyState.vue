<script setup lang="ts">
defineProps<{
  message: string
}>()
</script>

<template>
  <div class="empty-state">{{ message }}</div>
</template>

<style scoped>
.empty-state {
  color: var(--text-muted);
  padding: 48px 0;
  text-align: center;
}
</style>
