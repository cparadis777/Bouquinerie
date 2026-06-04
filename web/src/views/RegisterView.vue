<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import AppButton from '../components/primitive/AppButton.vue'

const router = useRouter()
const auth = useAuthStore()

const username = ref('')
const email = ref('')
const password = ref('')
const name = ref('')
const error = ref('')
const submitting = ref(false)

// TODO: Add validation
// - username: min length, allowed characters
// - email: valid email format
// - password: min length, complexity requirements

async function handleSubmit() {
  error.value = ''
  submitting.value = true
  try {
    await auth.register({
      username: username.value,
      email: email.value,
      password: password.value,
      name: name.value || undefined,
    })
    router.push('/')
  } catch (e: any) {
    // TODO: differentiate between 409 (duplicate) and other errors
    error.value = typeof e?.error === 'string' ? e.error : 'Registration failed'
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="register-view">
    <div class="card">
      <h1>Bouquinerie</h1>
      <p class="subtitle">Create your account</p>

      <form @submit.prevent="handleSubmit">
        <div class="field">
          <label for="username">Username</label>
          <input id="username" v-model="username" type="text" required autocomplete="username" />
        </div>

        <div class="field">
          <label for="email">Email</label>
          <input id="email" v-model="email" type="email" required autocomplete="email" />
        </div>

        <div class="field">
          <label for="password">Password</label>
          <input id="password" v-model="password" type="password" required autocomplete="new-password" />
        </div>

        <div class="field">
          <label for="name">Display Name <span class="optional">(optional)</span></label>
          <input id="name" v-model="name" type="text" autocomplete="name" />
        </div>

        <p v-if="error" class="error">{{ error }}</p>

        <AppButton type="submit" :disabled="submitting" variant="default">
          {{ submitting ? 'Creating account...' : 'Create account' }}
        </AppButton>
      </form>

      <p class="footer">
        Already have an account?
        <router-link :to="{ name: 'login' }">Sign in</router-link>
      </p>
    </div>
  </div>
</template>

<style scoped>
.register-view {
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

.optional {
  font-weight: 400;
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
