export interface NginxWebsite {
  name: string
  root: string
  enabled: boolean
  file_path: string
}

export interface NginxResponse {
  message: string
  data: {
    status: number
    version: string
  }
}
