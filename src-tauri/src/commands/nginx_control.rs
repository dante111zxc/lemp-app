use crate::models::nginx::Nginx;
use crate::services::nginx_service::NginxService;

#[tauri::command]
pub fn get_nginx_status()->Nginx {
    NginxService::get_nginx_status()
}

#[tauri::command]
pub async fn start_nginx() -> Result<String, String> {
    NginxService::start()
}

#[tauri::command]
pub async fn stop_nginx() -> Result<String, String> {
    NginxService::stop()
}

#[tauri::command]
pub async fn restart_nginx() -> Result<String, String> {
    NginxService::restart()
}