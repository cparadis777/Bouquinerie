import { defineStore } from 'pinia'
import { ref } from 'vue'
import { usePaginatedList } from '../composables/usePaginatedList'
import { getBooks, getBook } from '../services/books'
import type { components } from '../types/api'

export const useBookStore = defineStore('books', () => {
  const list = usePaginatedList(
    (page, pageSize) => getBooks({ page, page_size: pageSize }),
    20,
  )

  const currentBook = ref<components['schemas']['BookResponse'] | null>(null)
  const detailLoading = ref(false)
  const detailError = ref<Error | null>(null)

  async function fetchBook(id: string) {
    detailLoading.value = true
    detailError.value = null
    try {
      currentBook.value = await getBook(id)
    } catch (e) {
      detailError.value = e as Error
    } finally {
      detailLoading.value = false
    }
  }

  return {
    ...list,
    currentBook, detailLoading, detailError, fetchBook,
  }
})
