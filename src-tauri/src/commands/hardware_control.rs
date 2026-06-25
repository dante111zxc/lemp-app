use crate::models::hardware::HardwareInfo;
use crate::services::sys_monitor::SystemMonitor;

#[tauri::command]
pub fn get_hardware_info() -> Result<HardwareInfo, String> {
    let info = SystemMonitor::get_system_info();
    Ok(info)
}