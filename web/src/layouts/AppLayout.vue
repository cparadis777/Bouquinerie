<template>
  <div>
    <aside>
      <NavRail />
    </aside>
    <main>
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
  import NavRail from './NavRail.vue';

</script>

<style lang="css" scoped>

  div {
    display: flex;
  }

  aside {
    max-width: 25%;
  }

  main {
    width: 100%;
  }

</style>
