use serde::Serialize;

#[derive(Serialize)]
pub struct CpuInfo {
    pub brand: String,
    pub frequency_mhz: u64,
    pub usage_percent: f32,
}

#[derive(Serialize)]
pub struct HardwareInfo {
    pub os_name: String,
    pub os_version: String,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub cpus: Vec<CpuInfo>,
}