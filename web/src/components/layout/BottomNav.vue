<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../../stores/auth'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

async function handleLogout() {
  await auth.logout()
  router.push('/login')
}

const tabs = [
  { name: 'dashboard', label: 'Dashboard', path: '/', icon: '⊞' },
  { name: 'books', label: 'Books', path: '/books', icon: '⊡' },
  { name: 'authors', label: 'Authors', path: '/authors', icon: '⊛' },
  { name: 'series', label: 'Series', path: '/series', icon: '⊟' },
]
</script>

<template>
  <nav class="bottom-nav">
    <router-link
      v-for="tab in tabs"
      :key="tab.name"
      :to="tab.path"
      class="tab"
      :class="{ active: route.name === tab.name }"
    >
      <span class="icon">{{ tab.icon }}</span>
      <span class="label">{{ tab.label }}</span>
    </router-link>
    <button v-if="auth.user" class="tab logout-tab" @click="handleLogout()">
      <span class="icon">⊘</span>
      <span class="label">Log out</span>
    </button>
  </nav>
</template>

<style scoped>
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: var(--surface-elevated);
  border-top: 1px solid var(--border);
  display: flex;
  justify-content: space-around;
  padding: 6px 0 env(safe-area-inset-bottom, 6px);
  z-index: 100;
}

.tab {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: 6px 12px;
  color: var(--text-muted);
  text-decoration: none;
  transition: color 0.15s;
}

.tab.active {
  color: var(--primary);
}

.icon {
  font-size: 18px;
  line-height: 1;
}

.label {
  font-size: 11px;
  font-weight: 500;
}
</style>
