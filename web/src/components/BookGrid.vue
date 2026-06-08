<template>


  <p v-if="loading">Loading...</p>
  <p v-else-if="error" style="color: var(--error)">{{ error }}</p>
  <section v-else class="book-grid">
    <BookCard v-for="entry in entries" :entry="entry" />
  </section>


</template>
<script setup lang="ts">
  import BookCard from './BookCard.vue';
  import type { components } from "../types/api";

  defineProps<{ entries: components["schemas"]["BookListEntry"][], loading: boolean, error: Error | null }>();

</script>
<style lang="css" scoped>
  .book-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: var(--card-gap);
  }

</style>
