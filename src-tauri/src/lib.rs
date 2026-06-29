pub mod commands;
pub mod models;
pub mod services;

use std::time::Duration;
use tauri::Emitter;
use crate::services::sys_monitor::SystemMonitor;
use crate::services::nginx_service::NginxService;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::hardware_control::get_hardware_info,
            commands::nginx_control::get_nginx_status,
            commands::nginx_control::start_nginx,
            commands::nginx_control::stop_nginx,
            commands::nginx_control::restart_nginx,
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            std::thread::spawn(move || {
                loop {
                    let info = SystemMonitor::get_system_info();
                    let _ = handle.emit("system-info", info);
                    std::thread::sleep(Duration::from_secs(1));
                }
            });

            let handle = app.handle().clone();

            std::thread::spawn(move || {
                loop {
                    let status = NginxService::get_nginx_status();
                    let _ = handle.emit("nginx-status", status);
                    std::thread::sleep(Duration::from_secs(1));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
