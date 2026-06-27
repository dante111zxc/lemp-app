use std::env::consts::ARCH;
use sysinfo::{System, Disks};
use crate::models::hardware::{HardwareInfo, CpuInfo};

//Khai báo kiểu dữ liệu, tương tự Class trong PHP
pub struct SystemMonitor;

//(Viết hàm cho Struct)
impl SystemMonitor {

    //Lấy toàn bộ thông tin phần cứng hiện tại
    pub fn get_system_info() -> HardwareInfo {
        let mut sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();
        sys.refresh_all();

        // Tính toán RAM (Chuyển từ Bytes sang Gigabytes cho Frontend dễ dùng)
        let total_memory_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_memory_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let mut total_disk_bytes: u64 = 0;
        let mut available_disk_bytes: u64 = 0;

        for disk in disks.list() {
            total_disk_bytes += disk.total_space();
            available_disk_bytes += disk.available_space();
        }

        let used_disk_bytes = total_disk_bytes - available_disk_bytes;

        // Đổi ổ cứng từ Bytes sang GB
        let total_disk_gb = total_disk_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_disk_gb = used_disk_bytes as f64 / 1024.0 / 1024.0 / 1024.0;

        // Thu thập thông tin chi tiết từng nhân CPU
        let cpus = sys.cpus().iter().map(|cpu| {
            CpuInfo {
                frequency_mhz: cpu.frequency(),
                usage_percent: cpu.cpu_usage(),
            }
        }).collect();

        // 1. Lấy kiến trúc CPU (Trả về chuỗi như "x86_64", "aarch64", "arm", v.v.)
        let cpu_architecture = ARCH.to_string();
        let logical_cores = sys.cpus().len(); // Số nhân logic (Luồng)
        let physical_cores = sys.physical_core_count().unwrap_or(logical_cores); // Số nhân vật lý
        let cpu_brand = sys.cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let total_core = logical_cores + physical_cores;
        let total_cpu_usage = sys.global_cpu_usage();
        
        HardwareInfo {
            os_name: System::name().unwrap_or_else(|| "Unknown OS".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "".to_string()),
            total_memory_gb,
            used_memory_gb,
            cpus,
            total_cpu_usage,
            hard_disk_memory_gb: total_disk_gb,
            hard_disk_used_memory_gb: used_disk_gb,
            cpu_architecture,
            physical_cores,
            logical_cores,
            total_core,
            cpu_brand,
        }
    }
}