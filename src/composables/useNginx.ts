import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import type { NginxResponse, NginxWebsite } from '@/types/nginx'

export const useNginx = () => {
  const nginxRes = ref<NginxResponse | null>(null)
  const websites = ref<NginxWebsite[]>([])
  const configContent = ref('')
  const editingWebsite = ref<NginxWebsite | null>(null)
  const loading = ref(true)
  const error = ref<string | null>(null)
  const acting = ref(false)

  const start = async () => {
    try {
      loading.value = true
      acting.value = true
      error.value = null
      await invoke('start_nginx')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to start Nginx'
    } finally {
      acting.value = false
      loading.value = false
    }
  }

  const stop = async () => {
    try {
      loading.value = true
      acting.value = true
      error.value = null
      await invoke('stop_nginx')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to stop Nginx'
    } finally {
      acting.value = false
      loading.value = false
    }
  }

  const restart = async () => {
    try {
      loading.value = true
      acting.value = true
      error.value = null
      await invoke('restart_nginx')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to restart Nginx'
    } finally {
      acting.value = false
      loading.value = false
    }
  }

  const getListWebsites = async () => {
    try {
      websites.value = await invoke<NginxWebsite[]>('get_list_websites')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch websites'
    }
  }

  const show = async (website: NginxWebsite) => {
    try {
      error.value = null
      editingWebsite.value = website
      configContent.value = await invoke<string>('read_nginx_config', {
        filePath: website.file_path,
      })
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to read config'
    }
  }

  const update = async (content: string) => {
    if (!editingWebsite.value) return
    try {
      acting.value = true
      error.value = null
      await invoke('write_nginx_config', {
        filePath: editingWebsite.value.file_path,
        content,
      })
      configContent.value = content
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to save config'
    } finally {
      acting.value = false
    }
  }

  const closeEditor = () => {
    editingWebsite.value = null
    configContent.value = ''
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

    getListWebsites()
  })

  onUnmounted(() => {
    if (unlisten) {
      unlisten()
      unlisten = null
    }
  })

  return {
    nginxRes,
    websites,
    configContent,
    editingWebsite,
    loading,
    error,
    acting,
    start,
    stop,
    restart,
    getListWebsites,
    show,
    update,
    closeEditor,
  }
}
