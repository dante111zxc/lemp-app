// Vô hiệu hóa cửa sổ dòng lệnh (Terminal) chạy ngầm trên Windows khi app mở lên
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Gọi hàm `run()` được định nghĩa bên file lib.rs
    tauri_app_lib::run(); 
}