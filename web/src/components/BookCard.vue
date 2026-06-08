<template>

  <article>
    <a :href="`/books/${entry.book.id}`">
      <img v-if="entry.book.cover_path" :src="`/covers/${entry.book.cover_path}`"
        :alt="`Cover for ${entry.book.title}`" />

      <div v-else class="cover-placeholder" :style="`background-color: var(${color});`">
        {{ initials }}
      </div>
    </a>
    <h3 :title="entry.book.title">{{ entry.book.title }}</h3>
    <p>{{ entry.author_names.join(', ') }}</p>
  </article>

</template>

<script setup lang="ts">
  import { usePlaceholderCover } from "../composables/usePlaceholderCover";
  import type { components } from "../types/api";

  const props = defineProps<{ entry: components["schemas"]["BookListEntry"] }>()

  const initials = props.entry.book.sort_title[0].toUpperCase();

  const color = usePlaceholderCover(props.entry.book.id);

</script>

<style lang="css" scoped>
  img {
    width: 100%;
    aspect-ratio: 1/1.5;
  }

  h3 {
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .cover-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    aspect-ratio: 1/1.5;
    font-family: 'Crimson Pro';
    font-size: 72px;

  }
</style>
