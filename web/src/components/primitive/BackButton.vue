<script setup lang="ts">
import { useRouter } from 'vue-router'
import AppButton from './AppButton.vue'

const router = useRouter()

defineProps<{
  to: string
  label: string
}>()
</script>

<template>
  <AppButton variant="ghost" @click="router.push(to)">
    ← Back to {{ label }}
  </AppButton>
</template>
