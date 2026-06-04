import { api } from '../api/client'
import type { components } from '../types/api'

export async function login(body: components['schemas']['Credentials']) {
  const { data, error } = await api.POST('/login', { body })
  if (error) throw error
  return data
}

export async function logout() {
  const { error } = await api.POST('/logout')
  if (error) throw error
}

export async function getMe() {
  const { data, error } = await api.GET('/me')
  if (error) throw error
  return data!
}

export async function register(body: components['schemas']['RegisterRequest']) {
  const { data, error } = await api.POST('/register', { body })
  if (error) throw error
  return data!
}
