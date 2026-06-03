import { ref, onMounted, onUnmounted, type Ref } from 'vue'

export function useResponsivePageSize(options: {
  containerRef: Ref<HTMLElement | null>
  targetRows?: number
  minPageSize?: number
}) {
  const { containerRef, targetRows = 4, minPageSize = 20 } = options
  const pageSize = ref(minPageSize)

  let resizeTimer: ReturnType<typeof setTimeout> | null = null

  function recalculate() {
    const el = containerRef.value
    if (!el) return

    const style = getComputedStyle(el)
    const minCardWidth = parseInt(style.getPropertyValue('--min-card-width')) || 160
    const gap = parseInt(style.getPropertyValue('--card-gap')) || 24

    const sidebarWidth = window.innerWidth >= 800 ? 72 : 0
    const padding = window.innerWidth >= 800 ? 64 : 32
    const contentWidth = window.innerWidth - sidebarWidth - padding
    const columns = Math.max(1, Math.floor(contentWidth / (minCardWidth + gap)))
    pageSize.value = Math.max(minPageSize, columns * targetRows)
  }

  function onResize() {
    if (resizeTimer) clearTimeout(resizeTimer)
    resizeTimer = setTimeout(recalculate, 150)
  }

  onMounted(recalculate)
  onMounted(() => window.addEventListener('resize', onResize))
  onUnmounted(() => {
    window.removeEventListener('resize', onResize)
    if (resizeTimer) clearTimeout(resizeTimer)
  })

  return { pageSize }
}
