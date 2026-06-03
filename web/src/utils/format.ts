export function formatDate(iso: string): string {
  return new Date(iso + 'T00:00:00').toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

const isbn13 = /^(\d{3})(\d{1})(\d{4})(\d{4})(\d)$/

export function formatIsbn(isbn: string): string {
  const m = isbn.match(isbn13)
  return m ? `${m[1]}-${m[2]}-${m[3]}-${m[4]}-${m[5]}` : isbn
}

const languageNames: Record<string, string> = {
  en: 'English',
  fr: 'French',
  de: 'German',
  es: 'Spanish',
  it: 'Italian',
  pt: 'Portuguese',
  nl: 'Dutch',
  ru: 'Russian',
  ja: 'Japanese',
  zh: 'Chinese',
  ko: 'Korean',
  ar: 'Arabic',
}

export function formatLanguage(code: string): string {
  return languageNames[code] ?? code
}

export function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB']
  let i = 0
  let size = bytes
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024
    i++
  }
  return `${size.toFixed(i === 0 ? 0 : 1)} ${units[i]}`
}
