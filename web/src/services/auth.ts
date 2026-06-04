import { api } from '../api/client'
import type { components } from '../types/api'

export async function login(body: components['schemas']['Credentials']) {
  const { data, error } = await api.POST('/api/login', { body })
  if (error) throw error
  return data
}

export async function logout() {
  const { error } = await api.POST('/api/logout')
  if (error) throw error
}

export async function getMe() {
  const { data, error } = await api.GET('/api/me')
  if (error) throw error
  return data!
}

export async function register(body: components['schemas']['RegisterRequest']) {
  const { data, error } = await api.POST('/api/register', { body })
  if (error) throw error
  return data!
}
