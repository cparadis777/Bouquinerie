<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { Icon } from "@iconify/vue";
import { useAuthStore } from '../../stores/auth'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

async function handleLogout() {
  await auth.logout()
  router.push('/login')
}

const tabs = [
  { name: 'dashboard', label: 'Dashboard', path: '/', icon: 'material-symbols:home' },
  { name: 'books', label: 'Books', path: '/books', icon: 'material-symbols-light:book' },
  { name: 'authors', label: 'Authors', path: '/authors', icon: 'material-symbols:person-rounded' },
  { name: 'series', label: 'Series', path: '/series', icon: 'mdi:books' },
]
</script>

<template>
  <nav class="rail">
    <div class="brand">B</div>
    <div class="tabs">
      <router-link
        v-for="tab in tabs"
        :key="tab.name"
        :to="tab.path"
        class="tab"
        :class="{ active: route.name === tab.name }"
      >
        <Icon :icon="tab.icon" />
        <span class="label">{{ tab.label }}</span>
      </router-link>
    </div>
    <div v-if="auth.user" class="user-section">
      <span class="avatar">{{ (auth.user.name || auth.user.username).charAt(0).toUpperCase() }}</span>
      <button class="logout-btn" @click="handleLogout()">Log out</button>
    </div>
  </nav>
</template>

<style scoped>
.rail {
  width: 72px;
  background: var(--surface-elevated);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 0;
  gap: 24px;
  position: sticky;
  top: 0;
  height: 100vh;
}

.brand {
  font-family: var(--font-heading);
  font-size: 24px;
  font-weight: 700;
  color: var(--primary);
}

.tabs {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
  padding: 0 8px;
}

.tab {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  padding: 10px 4px;
  border-radius: 8px;
  color: var(--text-muted);
  text-decoration: none;
  transition: background 0.15s, color 0.15s;
}

.tab:hover {
  background: var(--surface-hover);
  color: var(--text);
}

.tab.active {
  background: var(--surface-hover);
  color: var(--primary);
}

.icon {
  font-size: 20px;
  line-height: 1;
}

.label {
  font-size: 11px;
  font-weight: 500;
}

.user-section {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px 8px;
  width: 100%;
  border-top: 1px solid var(--border);
}

.avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--primary);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: 600;
  flex-shrink: 0;
}

.logout-btn {
  font-size: 11px;
  color: var(--primary);
  cursor: pointer;
  background: none;
  border: none;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background 0.15s;
}

.logout-btn:hover {
  background: var(--surface-hover);
}
</style>
