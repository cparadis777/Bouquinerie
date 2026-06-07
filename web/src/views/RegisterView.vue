<template>

  <div class="center-fullscreen">
    <form @submit.prevent="handleSubmit">
      <div class="field">
        <label for="username">Username</label>
        <input id="username" v-model="username" type="text" required />
      </div>
      <div class="field">
        <label for="name">Name (Optional)</label>
        <input id="name" v-model="name" type="text" />
      </div>
      <div class="field">
        <label for="email">Email</label>
        <input id="email" v-model="email" type="email" required />
      </div>
      <div class="field">
        <label for="pw">Password</label>
        <input id="pw" v-model="password" type="password" required />
      </div>
      <div class="field">
        <label for="pw-confirm">Password Confirmation</label>
        <input id="pw-confirm" v-model="password_confirm" type="password" required />
      </div>
      <p style="color: var(--error)" v-if="error">{{ error }}</p>
      <button class="btn" type="submit" :disabled="loading">
        {{ loading ? "Signing Up..." : "Sign Up" }}
      </button>

    </form>
  </div>

</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import { useRouter } from "vue-router";
  import { useAuthStore } from '../stores/auth';

  const username = ref("");
  const email = ref("");
  const name = ref("");
  const password = ref("");
  const password_confirm = ref("");
  const error = ref("");
  const loading = ref(false);

  const router = useRouter();
  const authStore = useAuthStore();

  async function handleSubmit() {
    error.value = ''
    loading.value = true

    if (password.value != password_confirm.value) {
      error.value = "Password confirmation must match password";
      loading.value = false;
      return
    };

    try {
      await authStore.register({
        email: email.value,
        name: name.value,
        password: password.value,
        username: username.value,
      });

      router.push({ name: 'dashboard' })
    } catch (e: any) {
      error.value = e?.message || e?.detail || 'Sign-up failed';
    } finally {
      loading.value = false;
    }
  }

</script>
