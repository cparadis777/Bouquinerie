<template>

  <img v-if="entry.book.cover_path" :src="`/covers/${entry.book.cover_path}`" :alt="`Cover for ${entry.book.title}`" />

  <div v-else class="cover-placeholder" :style="`background-color: var(${color});`">
    {{ entry.book.sort_title[0] }}
  </div>

</template>
<script setup lang="ts">
  import { getBook } from '../services/books';
  import { useRoute } from 'vue-router';
  import { usePlaceholderCover } from "../composables/usePlaceholderCover";

  const route = useRoute()
  const id = route.params.id as string

  const entry = await getBook(id);
  const color = usePlaceholderCover(entry.book.id);

</script>
