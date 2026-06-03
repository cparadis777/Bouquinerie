import { defineStore } from 'pinia'
import { ref } from 'vue'
import { usePaginatedList } from '../composables/usePaginatedList'
import { getAuthors, getAuthor } from '../services/authors'
import { getBooks } from '../services/books'
import type { components } from '../types/api'

export const useAuthorStore = defineStore('authors', () => {
  const list = usePaginatedList(
    (page, pageSize) => getAuthors({ page, page_size: pageSize }),
    30,
  )

  const currentAuthor = ref<components['schemas']['Author'] | null>(null)
  const authorBooks = ref<components['schemas']['BookListEntry'][]>([])
  const detailLoading = ref(false)
  const detailError = ref<Error | null>(null)

  async function fetchAuthor(id: string) {
    detailLoading.value = true
    detailError.value = null
    try {
      const [author, booksRes] = await Promise.all([
        getAuthor(id),
        getBooks({ author_id: id, page: 1, page_size: 100 }),
      ])
      currentAuthor.value = author
      authorBooks.value = booksRes.data
    } catch (e) {
      detailError.value = e as Error
    } finally {
      detailLoading.value = false
    }
  }

  return {
    ...list,
    currentAuthor, authorBooks, detailLoading, detailError, fetchAuthor,
  }
})
