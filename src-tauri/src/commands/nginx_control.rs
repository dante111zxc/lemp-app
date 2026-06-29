use crate::models::nginx::Nginx;
use crate::models::nginx::Website;
use crate::services::nginx_service::NginxService;

#[tauri::command]
pub fn get_nginx_status() -> Nginx {
    NginxService::get_nginx_status()
}

#[tauri::command]
pub fn get_list_websites() -> Vec<Website> {
    NginxService::get_list_websites()
}

#[tauri::command]
pub fn read_nginx_config(file_path: String) -> Result<String, String> {
    NginxService::read_config(&file_path)
}

#[tauri::command]
pub fn write_nginx_config(file_path: String, content: String) -> Result<String, String> {
    NginxService::write_config(&file_path, &content)
}

#[tauri::command]
pub fn test_nginx_config(content: String) -> Result<String, String> {
    NginxService::test_config(&content)
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
