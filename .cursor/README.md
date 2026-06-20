# LEMP Stack Manager for macOS (Tauri + Vue)

## Project Overview

An application to manage Nginx, PHP (Multi-version), MySQL, PostgreSQL, Redis, and more using Homebrew.

## Key Technical Rules

1. **Service Management**: Use `brew services` commands via Rust `std::process::Command`.
2. **Multi-version PHP**: Manage versions by linking/unlinking brew formulae (e.g., `brew link --overwrite --force php@8.2`).
3. **Privileges**: Use `osascript` in Rust to request sudo permissions when editing `/etc/hosts` or Nginx configs.
4. **Menu Bar**: Implementation must focus on `tauri::SystemTray` for the native macOS menu bar experience.

## Folder Map

- `src-tauri/src/services`: Contains the core logic for each stack component.
- `src-tauri/src/commands.rs`: Bridging functions between Vue and Rust.
- `src/views`: Dashboard and configuration screens.

## Important Paths (macOS)

- Nginx Config: `/usr/local/etc/nginx/` (Intel) or `/opt/homebrew/etc/nginx/` (Apple Silicon).
- Hosts file: `/etc/hosts`.

### project directory structure

lemp-manager/
├── src-tauri/ # Backend (Rust) - Xử lý logic hệ thống
│ ├── src/
│ │ ├── commands/ # Các hàm Rust gọi từ Frontend (invokes)
│ │ ├── services/ # Logic quản lý từng dịch vụ (Nginx, PHP,...)
│ │ ├── utils/ # Helper về file system, shell execution
│ │ ├── menu.rs # Cấu hình Native Icon Menu Bar
│ │ └── main.rs # Entry point
│ ├── scripts/ # Chứa các shell script phức tạp cho Homebrew/Nginx
│ ├── icons/ # Icon cho Menu bar và App (.ico, .png, .icns)
│ └── tauri.conf.json # Cấu hình quyền truy cập hệ thống của Tauri
├── src/ # Frontend (Vue.js + Vite)
│ ├── assets/ # Styles, images
│ ├── components/ # UI Components (ServiceCard, StatusBadge...)
│ ├── views/ # Pages (Dashboard, PHP Config, Nginx Settings...)
│ ├── store/ # Quản lý state (Pinia) - lưu trạng thái các service
│ └── App.vue
├── public/
└── README.md # Bản đồ hướng dẫn cho AI Agent

❯ hãy sửa lại giao diện FE thành 2 cột, bên trái là sidebar, bên phải là content.
Toàn bộ các tính năng đề phải có nút start, stop, restart
