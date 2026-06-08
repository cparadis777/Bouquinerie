  <template>
    <section ref="gridRef" class="grid-wrapper">
      <BookGrid :entries="data" :loading="loading" :error="error" />

      <Paginator :page="page" :total="total" :pages="pages" :pageSize="pageSize" @update:page="page = $event" />
    </section>

  </template>

<script setup lang="ts">
  import { onMounted, ref } from 'vue';
  import BookGrid from "../components/BookGrid.vue";
  import { useResponsivePageSize } from "../composables/useResponsivePageSize.ts";
  import { getBooks } from "../services/books"
  import { usePaginatedList } from '../composables/usePaginatedList.ts';
  import Paginator from '../components/Paginator.vue';

  const gridRef = ref<HTMLElement | null>(null)

  const { pageSize } = useResponsivePageSize({
    containerRef: gridRef,
    targetRows: 4,
    minPageSize: 20,
  })


  const { data, total, page, pages, loading, error, load } = usePaginatedList(
    (p, ps) => getBooks({ page: p, page_size: ps }),
    pageSize,
  )
  onMounted(load)


</script>

<style lang="css" scoped>
  .grid-wrapper {
    --min-card-width: 160px;
    --card-gap: 24px;
    padding: var(--space-3);
  }
</style>
