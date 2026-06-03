<script setup lang="ts">
withDefaults(defineProps<{
  variant?: 'default' | 'ghost'
}>(), { variant: 'default' })
</script>

<template>
  <button :class="['btn', `btn--${variant}`]">
    <slot />
  </button>
</template>

<style scoped>
.btn {
  font-family: var(--font-body);
  font-size: 13px;
  cursor: pointer;
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  transition: background 0.15s;
}
.btn--default {
  background: var(--surface-elevated);
  border: 1px solid var(--border);
  color: var(--text);
}
.btn--default:hover:not(:disabled) {
  background: var(--surface-hover);
}
.btn--default:disabled {
  opacity: 0.4;
  cursor: default;
}
.btn--ghost {
  background: none;
  color: var(--primary);
  padding: 4px 0;
  align-self: flex-start;
}
.btn--ghost:hover {
  color: var(--primary-hover);
}
</style>
