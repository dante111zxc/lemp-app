// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use sysinfo::{Disks, System};

#[derive(Serialize)]
pub struct SystemInfo {
    os_name: String,
    os_version: String,
    cpu_name: String,
    cpu_cores: usize,
    cpu_usage: f32,
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
    total_disk: u64,
    used_disk: u64,
    free_disk: u64,
}

#[tauri::command]
fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    // OS info
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());

    // CPU info
    let cpu_name = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let cpu_cores = sys.cpus().len();
    let cpu_usage = sys.global_cpu_usage();

    // Memory info (in bytes)
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = total_memory - used_memory;

    // Disk info (sum all disks)
    let disks = Disks::new_with_refreshed_list();
    let mut total_disk: u64 = 0;
    let mut free_disk: u64 = 0;

    for disk in disks.list() {
        total_disk += disk.total_space();
        free_disk += disk.available_space();
    }
    let used_disk = total_disk - free_disk;

    SystemInfo {
        os_name,
        os_version,
        cpu_name,
        cpu_cores,
        cpu_usage,
        total_memory,
        used_memory,
        free_memory,
        total_disk,
        used_disk,
        free_disk,
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn hello_world() -> String {
    "Hello World from Rust!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, hello_world, get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
