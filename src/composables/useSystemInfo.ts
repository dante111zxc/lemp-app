import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { SystemInfo } from '@/types/system'

export function useSystemInfo(refreshInterval = 5000) {
  const systemInfo = ref<SystemInfo | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)

  let intervalId: ReturnType<typeof setInterval> | null = null

  const fetchSystemInfo = async () => {
    try {
      const info = await invoke<SystemInfo>('get_system_info')
      systemInfo.value = info
      error.value = null
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch system info'
      console.error('Error fetching system info:', err)
    } finally {
      loading.value = false
    }
  }

  const startAutoRefresh = () => {
    if (intervalId) return
    intervalId = setInterval(fetchSystemInfo, refreshInterval)
  }

  const stopAutoRefresh = () => {
    if (intervalId) {
      clearInterval(intervalId)
      intervalId = null
    }
  }

  onMounted(() => {
    fetchSystemInfo()
    startAutoRefresh()
  })

  onUnmounted(() => {
    stopAutoRefresh()
  })

  return {
    systemInfo,
    loading,
    error,
    refresh: fetchSystemInfo,
    startAutoRefresh,
    stopAutoRefresh,
  }
}
