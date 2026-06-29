export interface SystemInfo {
  os_name: string
  os_version: string
  total_memory_gb: number
  used_memory_gb: number
  cpu_usage: number
  cpus: {
    frequency_mhz: number
    usage_percent: number
  }[]
  hard_disk_memory_gb: number
  hard_disk_used_memory_gb: number
  cpu_architecture: string
  logical_cores: number
  physical_cores: number
  cpu_brand: string
  total_core: number
  total_cpu_usage: number
}
