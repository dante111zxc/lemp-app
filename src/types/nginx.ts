export interface NginxResponse {
  message: string
  data: {
    status: number
    version: string
  }
}
