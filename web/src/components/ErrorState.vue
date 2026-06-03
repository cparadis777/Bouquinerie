<script setup lang="ts">
defineProps<{
  error: Error | null
}>()

const emit = defineEmits<{
  retry: []
}>()
</script>

<template>
  <div v-if="error" class="error-state">
    <p>{{ error.message }}</p>
    <button class="retry-btn" @click="emit('retry')">Try again</button>
  </div>
</template>

<style scoped>
.error-state {
  color: var(--error);
  padding: 48px 0;
  text-align: center;
}

.retry-btn {
  margin-top: 12px;
  font-family: var(--font-body);
  font-size: 13px;
  color: var(--primary);
  background: none;
  border: 1px solid var(--primary);
  border-radius: 6px;
  padding: 8px 16px;
  cursor: pointer;
  transition: background 0.15s;
}

.retry-btn:hover {
  background: var(--primary);
  color: var(--surface);
}
</style>
