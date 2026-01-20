export interface SystemInfo {
  os_name: string
  os_version: string
  cpu_name: string
  cpu_cores: number
  cpu_usage: number
  total_memory: number
  used_memory: number
  free_memory: number
  total_disk: number
  used_disk: number
  free_disk: number
}

// Helper functions to format bytes
export function formatBytes(bytes: number, decimals = 2): string {
  if (bytes === 0) return '0 Bytes'

  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB']

  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i]
}
