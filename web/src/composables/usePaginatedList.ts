import { ref, computed, watch, type Ref } from 'vue'

type ListResponse<T> = { data: T[]; total: number; page: number }

export function usePaginatedList<T>(
  fetchFn: (page: number, pageSize: number) => Promise<ListResponse<T>>,
  pageSizeRef: Ref<number>,
) {
  const data = ref<T[]>([])
  const total = ref(0)
  const page = ref(1)
  const loading = ref(false)
  const error = ref<Error | null>(null)
  const pages = computed(() => Math.ceil(total.value / pageSizeRef.value))

  async function load() {
    loading.value = true
    error.value = null
    try {
      const result = await fetchFn(page.value, pageSizeRef.value)
      data.value = result.data
      total.value = result.total
      page.value = result.page
    } catch (e) {
      error.value = e as Error
    } finally {
      loading.value = false
    }
  }

  watch(page, load)

  return { data, total, page, pages, loading, error, load, pageSize: pageSizeRef }
}
