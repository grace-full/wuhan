import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface CookieData {
  name: string
  value: string
}

export interface AuthData {
  token: string | null
  cookies: CookieData[]
  url: string
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(null)
  const cookies = ref<CookieData[]>([])
  const url = ref<string>('')
  const isAuthenticated = computed(() => token.value !== null)
  const hasData = computed(() => token.value !== null || cookies.value.length > 0)

  function setAuthData(data: AuthData) {
    token.value = data.token
    cookies.value = data.cookies
    url.value = data.url

    console.log('Auth store updated:')
    console.log('Token:', token.value)
    console.log('Cookies:', cookies.value)
    console.log('URL:', url.value)
  }

  function clearAuthData() {
    token.value = null
    cookies.value = []
    url.value = ''
    console.log('Auth store cleared')
  }

  function updateToken(newToken: string | null) {
    token.value = newToken
    console.log('Token updated:', token.value)
  }

  function addCookie(cookie: CookieData) {
    cookies.value.push(cookie)
    console.log('Cookie added:', cookie)
  }

  return {
    token,
    cookies,
    url,
    isAuthenticated,
    hasData,
    setAuthData,
    clearAuthData,
    updateToken,
    addCookie
  }
})
