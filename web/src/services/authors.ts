import { api } from '../api/client'

export async function getAuthors(params: { page?: number; page_size?: number }) {
  const { data, error } = await api.GET('/api/authors', { params: { query: params } })
  if (error) throw error
  return data!
}

export async function getAuthor(id: string) {
  const { data, error } = await api.GET('/api/authors/{id}', { params: { path: { id } } })
  if (error) throw error
  return data!
}
