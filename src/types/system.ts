export interface SystemInfo {
  os_name: string
  os_version: string
  total_memory_gb: string
  used_memory_gb: number
  cpu_usage: number
  cpus: {
    brand: string
    frequency_mhz: number
    usage_percent: number
  }[]
  hard_disk_memory_gb: number
  hard_disk_used_memory_gb: number
}
