import { ref, onMounted, onUnmounted, computed } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { SystemInfo } from '@/types/system'

export const useSystemInfo = () => {
  const systemInfo = ref<SystemInfo | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)

  let unlisten: (() => void) | null = null

  const cpuMhz = computed(() => {
    if (!systemInfo.value?.cpus) {
      return 0
    }
    return (
      systemInfo.value.cpus.reduce(
        (accumulator, currentItem) => accumulator + currentItem.frequency_mhz,
        0
      ) / systemInfo.value.total_core
    )
  })

  onMounted(() => {
    listen<SystemInfo>('system-info', (event) => {
      systemInfo.value = event.payload
      loading.value = false
      error.value = null
    }).then((fn) => {
      unlisten = fn
    }).catch((err) => {
      error.value = err instanceof Error ? err.message : 'Failed to listen system-info event'
      loading.value = false
    })
  })

  onUnmounted(() => {
    if (unlisten) {
      unlisten()
      unlisten = null
    }
  })

  return {
    systemInfo,
    loading,
    error,
    cpuMhz,
  }
}
