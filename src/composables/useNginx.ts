import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { NginxResponse } from '@/types/nginx'

export const useNginx = () => {
  const nginxRes = ref<NginxResponse | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)
  const acting = ref(false)

  const start = async () => {
    try {
      acting.value = true
      error.value = null
      await invoke('start_nginx')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to start Nginx'
    } finally {
      acting.value = false
    }
  }

  const stop = async () => {
    try {
      acting.value = true
      error.value = null
      await invoke('stop_nginx')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to stop Nginx'
    } finally {
      acting.value = false
    }
  }

  const restart = async () => {
    try {
      acting.value = true
      error.value = null
      await invoke('restart_nginx')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to restart Nginx'
    } finally {
      acting.value = false
    }
  }

  let unlisten: (() => void) | null = null

  onMounted(() => {
    listen<NginxResponse>('nginx-status', event => {
      nginxRes.value = event.payload
      loading.value = false
      error.value = null
    })
      .then(fn => {
        unlisten = fn
      })
      .catch(err => {
        error.value = err instanceof Error ? err.message : 'Failed to listen nginx-status event'
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
    nginxRes,
    loading,
    error,
    acting,
    start,
    stop,
    restart,
  }
}
