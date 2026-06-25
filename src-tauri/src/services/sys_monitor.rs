use sysinfo::System;
use crate::models::hardware::{HardwareInfo, CpuInfo};

//Khai báo kiểu dữ liệu, tương tự Class trong PHP
pub struct SystemMonitor;

//(Viết hàm cho Struct)
impl SystemMonitor {

    //Lấy toàn bộ thông tin phần cứng hiện tại
    pub fn get_system_info() -> HardwareInfo {
        let mut sys = System::new_all();
        sys.refresh_all();

        // Tính toán RAM (Chuyển từ Bytes sang Gigabytes cho Frontend dễ dùng)
        let total_memory_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_memory_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        // Thu thập thông tin chi tiết từng nhân CPU
        let cpus = sys.cpus().iter().map(|cpu| {
            CpuInfo {
                brand: cpu.brand().to_string(),
                frequency_mhz: cpu.frequency(),
                usage_percent: cpu.cpu_usage(),
            }
        }).collect();

        HardwareInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown OS".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "".to_string()),
            total_memory_gb,
            used_memory_gb,
            cpus,
        }
    }
}