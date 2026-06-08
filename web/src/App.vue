<script setup lang="ts">
  import { useRoute } from 'vue-router';
  import AppLayout from "./layouts/AppLayout.vue";

  const route = useRoute();
</script>

<template>
  <AppLayout v-if="route.meta.layout !== 'blank'">
    <suspense>
      <template #default>
        <router-view />
      </template>
      <template #fallback>
        <div class="loading-fallback">
          <div class="spinner" />
          <p class="loading-text">Loading...</p>
        </div>
      </template>
    </suspense>
  </AppLayout>
</template>

<style scoped>
.loading-fallback {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
  gap: var(--space-3);
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--border);
  border-top-color: var(--primary);
  border-radius: var(--radius-full);
  animation: spin 800ms linear infinite;
}

.loading-text {
  font-size: var(--text-sm);
  color: var(--text-muted);
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
