import { api } from '../api/client'

export async function getSeries(params: { page?: number; page_size?: number }) {
  const { data, error } = await api.GET('/api/series', { params: { query: params } })
  if (error) throw error
  return data!
}

export async function getSeriesItem(id: string) {
  const { data, error } = await api.GET('/api/series/{id}', { params: { path: { id } } })
  if (error) throw error
  return data!
}
