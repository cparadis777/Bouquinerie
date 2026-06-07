<template>

  <div class="center-fullscreen">
    <form @submit.prevent="handleSubmit">
      <div class="field">
        <label for="username">Username</label>
        <input id="username" v-model="username" type="text" required />
      </div>
      <div class="field">
        <label for="pw">Password</label>
        <input id="pw" v-model="password" type="password" required />
      </div>
      <p style="color: var(--error)" v-if="error">{{ error }}</p>
      <button class="btn" type="submit" :disabled="loading">
        {{ loading ? "Logging in..." : "Login" }}
      </button>

    </form>
  </div>

</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import { useRouter } from "vue-router";
  import { useAuthStore } from '../stores/auth';

  const username = ref("");
  const password = ref("");
  const error = ref("");
  const loading = ref(false);

  const router = useRouter();
  const authStore = useAuthStore();

  async function handleSubmit() {
    error.value = ''
    loading.value = true
    try {
      await authStore.login(username.value, password.value)
      router.push({ name: 'dashboard' })
    } catch (e: any) {
      error.value = e?.message || e?.detail || 'Login failed'
    } finally {
      loading.value = false
    }
  }




</script>
