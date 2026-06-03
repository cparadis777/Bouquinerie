import { defineStore } from 'pinia'
import { ref } from 'vue'
import { usePaginatedList } from '../composables/usePaginatedList'
import { getSeries, getSeriesItem } from '../services/series'
import { getBooks } from '../services/books'
import type { components } from '../types/api'

export const useSeriesStore = defineStore('series', () => {
  const pageSize = ref(30)

  const list = usePaginatedList(
    (page, ps) => getSeries({ page, page_size: ps }),
    pageSize,
  )

  const currentSeries = ref<components['schemas']['Series'] | null>(null)
  const seriesBooks = ref<components['schemas']['BookListEntry'][]>([])
  const detailLoading = ref(false)
  const detailError = ref<Error | null>(null)

  async function fetchSeries(id: string) {
    detailLoading.value = true
    detailError.value = null
    try {
      const [series, booksRes] = await Promise.all([
        getSeriesItem(id),
        getBooks({ series_id: id, page: 1, page_size: 100 }),
      ])
      currentSeries.value = series
      seriesBooks.value = booksRes.data
    } catch (e) {
      detailError.value = e as Error
    } finally {
      detailLoading.value = false
    }
  }

  return {
    ...list,
    currentSeries, seriesBooks, detailLoading, detailError, fetchSeries,
  }
})
