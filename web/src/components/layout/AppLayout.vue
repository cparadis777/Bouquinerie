<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute } from 'vue-router'
import NavigationRail from './NavigationRail.vue'
import BottomNav from './BottomNav.vue'

const route = useRoute()
const windowWidth = ref(window.innerWidth)

function onResize() {
  windowWidth.value = window.innerWidth
}

onMounted(() => window.addEventListener('resize', onResize))
onUnmounted(() => window.removeEventListener('resize', onResize))

const isMobile = computed(() => windowWidth.value < 800)
const isBlank = computed(() => route.meta.blank === true)
</script>

<template>
  <div class="layout">
    <NavigationRail v-if="!isMobile && !isBlank" />
    <main class="main" :class="{ blank: isBlank }">
      <router-view />
    </main>
    <BottomNav v-if="isMobile && !isBlank" />
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  min-height: 100vh;
}

.main {
  flex: 1;
  padding: 32px;
  padding-bottom: 80px;
  /* max-width: 1200px; */
  width: 100%;
}

@media (max-width: 800px) {
  .main {
    padding: 16px;
    padding-bottom: 72px;
  }
}

.main.blank {
  padding: 0;
  padding-bottom: 0;
}
</style>
