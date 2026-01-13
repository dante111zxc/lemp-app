export interface Service {
  id: string
  name: string
  displayName: string
  description: string
  status: number
  port?: number
  autoStart: boolean
  version?: string
}

export interface ServiceStats {
  cpu: number
  memory: number
  uptime: string
}
