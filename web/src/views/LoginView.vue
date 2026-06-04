<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import AppButton from '../components/primitive/AppButton.vue'

const router = useRouter()
const auth = useAuthStore()

const username = ref('')
const password = ref('')
const error = ref('')
const submitting = ref(false)

async function handleSubmit() {
  error.value = ''
  submitting.value = true
  try {
    await auth.login(username.value, password.value)
    router.push('/')
  } catch (e: any) {
    error.value = e?.error || 'Invalid credentials'
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="login-view">
    <div class="card">
      <h1>Bouquinerie</h1>
      <p class="subtitle">Sign in to your account</p>

      <form @submit.prevent="handleSubmit">
        <div class="field">
          <label for="username">Username</label>
          <input id="username" v-model="username" type="text" required autocomplete="username" />
        </div>

        <div class="field">
          <label for="password">Password</label>
          <input id="password" v-model="password" type="password" required autocomplete="current-password" />
        </div>

        <p v-if="error" class="error">{{ error }}</p>

        <AppButton type="submit" :disabled="submitting" variant="default">
          {{ submitting ? 'Signing in...' : 'Sign in' }}
        </AppButton>
      </form>

      <p class="footer">
        No account?
        <router-link :to="{ name: 'register' }">Register</router-link>
      </p>
    </div>
  </div>
</template>

<style scoped>
.login-view {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 100vh;
  padding: 24px;
}

.card {
  width: 100%;
  max-width: 360px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

h1 {
  font-size: 32px;
  text-align: center;
}

.subtitle {
  text-align: center;
  margin-top: -16px;
}

form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-muted);
}

.error {
  color: var(--error);
  font-size: 13px;
}

.footer {
  text-align: center;
  font-size: 13px;
}
</style>
