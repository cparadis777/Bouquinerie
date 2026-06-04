import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { login as loginApi, logout as logoutApi, getMe, register as registerApi } from '../services/auth'
import type { components } from '../types/api'

type User = components['schemas']['UserResponse']

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const loading = ref(true)

  const isAuthenticated = computed(() => user.value !== null)

  async function checkAuth() {
    loading.value = true
    try {
      user.value = await getMe()
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  async function login(username: string, password: string) {
    await loginApi({ username, password })
    user.value = await getMe()
  }

  async function register(body: components['schemas']['RegisterRequest']) {
    user.value = await registerApi(body)
  }

  async function logout() {
    await logoutApi()
    user.value = null
  }

  return { user, loading, isAuthenticated, checkAuth, login, register, logout }
})
