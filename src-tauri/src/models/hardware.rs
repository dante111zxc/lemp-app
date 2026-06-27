use serde::Serialize;

#[derive(Serialize)]
pub struct CpuInfo {
    pub frequency_mhz: u64,
    pub usage_percent: f32,
}

#[derive(Serialize)]
pub struct HardwareInfo {
    pub os_name: String,
    pub os_version: String,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub hard_disk_memory_gb: f64,
    pub hard_disk_used_memory_gb: f64,
    pub cpus: Vec<CpuInfo>,
    pub logical_cores: usize,
    pub physical_cores: usize,
    pub total_core: usize,
    pub cpu_brand: String,
    pub cpu_architecture: String,
    pub total_cpu_usage: f32
}