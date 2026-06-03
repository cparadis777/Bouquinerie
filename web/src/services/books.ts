import { api } from '../api/client'

export async function getBooks(params: {
  page?: number
  page_size?: number
  author_id?: string
  series_id?: string
}) {
  const { data, error } = await api.GET('/api/books', { params: { query: params } })
  if (error) throw error
  return data!
}

export async function getBook(id: string) {
  const { data, error } = await api.GET('/api/books/{id}', { params: { path: { id } } })
  if (error) throw error
  return data!
}
