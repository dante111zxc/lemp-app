# LEMP Manager - Tài liệu dự án

## Tổng quan

**LEMP Manager** là một ứng dụng desktop giúp quản lý LEMP stack (Linux, Nginx, MySQL/MariaDB, PHP-FPM) thông qua giao diện đồ họa. Ứng dụng cho phép người dùng theo dõi trạng thái hệ thống, khởi động/dừng/khởi động lại các dịch vụ, xem thông tin phần cứng (CPU, RAM, ổ cứng) và chỉnh sửa cấu hình dịch vụ ngay trên desktop.

Ứng dụng được xây dựng bằng **Tauri v2** — Rust làm backend (tương tác trực tiếp với OS) và **Vue 3 + TypeScript** làm frontend. Không có backend PHP/Laravel, toàn bộ logic phía server nằm trong Rust binary.

---

## Tech Stack

| Lớp                   | Công nghệ                 | Mô tả                                                             |
| --------------------- | ------------------------- | ----------------------------------------------------------------- |
| **Desktop Framework** | Tauri v2                  | Giải pháp nhẹ thay thế Electron, binary nhỏ (~5-10MB)             |
| **Backend**           | Rust (Edition 2021)       | Giao tiếp với hệ điều hành qua `sysinfo`, `std::process::Command` |
| **Frontend**          | Vue 3.5 + Composition API | `<script setup>`, lazy-load routes                                |
| **Ngôn ngữ**          | TypeScript 5.6            | Strict mode, shared types với Rust models                         |
| **Build tool**        | Vite 6                    | Dev server port 1420, HMR, path alias `@/`                        |
| **CSS**               | Tailwind CSS v4           | Utility-first, dùng `@tailwindcss/vite` plugin                    |
| **UI Library**        | shadcn-vue (reka-ui)      | New York style, 18 components                                     |
| **Icons**             | Lucide Vue                | Tree-shakeable SVG icons                                          |
| **Router**            | Vue Router 4              | `createWebHistory`, 6 routes                                      |
| **Package Manager**   | pnpm                      | Workspace mode                                                    |
| **State**             | Vue composables           | Mỗi composable quản lý state riêng, không dùng store toàn cục     |

---

## Cấu trúc thư mục

```
lemp-app/
│
├── index.html                    # Vite entry HTML
├── package.json                  # Node dependencies & scripts
├── pnpm-lock.yaml                # pnpm lockfile
├── vite.config.ts                # Vite config (port 1420, Vue + Tailwind)
├── tsconfig.json                 # TypeScript base config
├── tsconfig.app.json             # TypeScript app config
├── tsconfig.node.json            # TypeScript Node tooling config
├── components.json               # shadcn-vue config (New York style)
│
├── src/                          # Vue 3 Frontend
│   ├── main.ts                   # Entry: createApp → router → mount
│   ├── App.vue                   # Root layout: SidebarProvider + <router-view>
│   ├── router/
│   │   └── index.ts              # 6 routes (Dashboard + 5 services)
│   ├── views/
│   │   ├── Dashboard.vue         # CPU, RAM, Disk overview
│   │   ├── Nginx.vue             # Nginx status + virtual hosts
│   │   ├── Mysql.vue             # MySQL status + database list
│   │   ├── Php.vue               # PHP-FPM status + php.ini editor
│   │   ├── Redis.vue             # Redis status + metrics
│   │   └── Mailpit.vue           # Mailpit SMTP/Web UI config
│   ├── components/
│   │   ├── ui/                   # 18 shadcn-vue components
│   │   ├── AppHeader.vue
│   │   ├── ServiceCard.vue       # Card hiển thị Start/Stop/Restart cho mỗi service
│   │   ├── ServiceStatus.vue     # Đèn trạng thái (xanh/đỏ) có animation
│   │   ├── ServiceList.vue
│   │   └── ...
│   ├── composables/              # Vue composables (reusable logic)
│   │   ├── useSystemInfo.ts      # Gọi get_hardware_info, auto-refresh 5s
│   │   ├── useNginx.ts           # State cho Nginx
│   │   └── useTheme.ts           # Dark/light/system theme
│   ├── types/
│   │   ├── system.ts             # SystemInfo interface (mirror Rust HardwareInfo)
│   │   └── service.ts            # Service, ServiceStats interfaces
│   ├── enums/
│   │   ├── BaseEnum.ts           # Abstract enum với getLabel, list, toObject
│   │   └── EnumServiceStatus.ts  # RUNNING=1, STOPPED=0, NOT_INSTALLED=2
│   ├── lib/
│   │   └── utils.ts              # cn() = clsx + tailwind-merge
│   ├── utils/
│   │   └── format.ts             # Format functions (formatCpuFrequency...)
│   └── layouts/
│       └── AppSidebar.vue        # Sidebar nav: System + Services groups
│
├── src-tauri/                    # Rust Backend
│   ├── Cargo.toml                # Dependencies: tauri 2, sysinfo 0.32, serde
│   ├── build.rs                  # Tauri build script
│   ├── tauri.conf.json           # App config, window settings, bundle targets
│   ├── capabilities/
│   │   └── default.json          # Permissions: core:default, opener:default
│   ├── icons/                    # App icons (PNG, ICO, ICNS)
│   └── src/
│       ├── main.rs               # Entry: disable Windows console, call lib::run()
│       ├── lib.rs                # Tauri builder: register plugins + commands
│       ├── commands/
│       │   ├── mod.rs            # Module declarations
│       │   ├── hardware_control.rs  # get_hardware_info → SystemMonitor
│       │   ├── nginx_control.rs     # get_nginx_status → NginxService
│       │   ├── service_control.rs   # (Stub) Start/Stop/Restart services
│       │   └── config_manager.rs    # (Stub) Đọc/Ghi file config
│       ├── services/
│       │   ├── mod.rs            # Module declarations
│       │   ├── sys_monitor.rs    # SystemMonitor: CPU, RAM, Disk via sysinfo crate
│       │   ├── nginx_service.rs  # NginxService: kiểm tra cài đặt + trạng thái
│       │   ├── os_command.rs     # (Stub) Chạy bash script, systemctl, brew services
│       │   └── file_system.rs    # (Stub) Đọc/Ghi file hệ thống, phân quyền
│       └── models/
│           ├── mod.rs
│           ├── hardware.rs       # HardwareInfo, CpuInfo structs (Serialize)
│           └── nginx.rs          # Nginx, NginxData structs (Serialize)
│
├── .shared/                      # Design reference data
├── public/                       # Static assets
└── dist/                         # Built frontend output
```

---

## Kiến trúc giao tiếp Frontend ↔ Backend

### Mô hình Invoke Pattern

Frontend (Vue/TypeScript) gọi Rust backend thông qua Tauri command invocation:

```
┌─────────────────────────┐         invoke('command_name')        ┌──────────────────────────┐
│   Vue 3 Frontend         │ ──────────────────────────────────→  │   Rust Backend (Tauri)    │
│                          │                                       │                          │
│   composables/           │                                       │   commands/               │
│   useSystemInfo.ts       │                                       │   hardware_control.rs     │
│   useNginx.ts            │ ←──── JSON response (Serialize) ──── │   nginx_control.rs        │
└─────────────────────────┘                                       └──────────┬───────────────┘
                                                                              │
                                                                     ┌────────▼───────────────┐
                                                                     │   services/             │
                                                                     │   sys_monitor.rs        │
                                                                     │   nginx_service.rs      │
                                                                     └──────────┬───────────────┘
                                                                                │
                                                                     ┌──────────▼───────────────┐
                                                                     │   OS System Calls        │
                                                                     │   sysinfo crate          │
                                                                     │   std::process::Command  │
                                                                     └──────────────────────────┘
```

**Chi tiết data flow:**

1. **Rust** định nghĩa struct với `#[derive(Serialize)]` → tự động convert sang JSON
2. **Rust** đăng ký command qua `#[tauri::command]` trong `lib.rs`
3. **TypeScript** import `invoke` từ `@tauri-apps/api/core`
4. **TypeScript** gọi `invoke<SystemInfo>('get_hardware_info')` với type parameter để deserialize JSON

### Ví dụ: Lấy thông tin phần cứng

**Rust Model** (`src-tauri/src/models/hardware.rs`):

```rust
#[derive(Serialize)]
pub struct HardwareInfo {
    pub os_name: String,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub cpus: Vec<CpuInfo>,
    // ...
}
```

**Rust Service** (`src-tauri/src/services/sys_monitor.rs`) dùng `sysinfo` crate để đọc dữ liệu thực từ OS, tính toán RAM/disk theo GB.

**TypeScript Type** (`src/types/system.ts`) mirror cấu trúc Rust:

```typescript
export interface SystemInfo {
  os_name: string
  total_memory_gb: number
  used_memory_gb: number
  cpus: { frequency_mhz: number; usage_percent: number }[]
  // ...
}
```

**Vue Composable** (`src/composables/useSystemInfo.ts`) gọi `invoke<SystemInfo>('get_hardware_info')` mỗi 5 giây để auto-refresh.

---

## Routes & Views

| Route      | Component       | Chức năng                                                 |
| ---------- | --------------- | --------------------------------------------------------- |
| `/`        | `Dashboard.vue` | Tổng quan hệ thống: CPU, RAM, Disk (dữ liệu thực từ Rust) |
| `/nginx`   | `Nginx.vue`     | Trạng thái Nginx, danh sách website, start/stop/restart   |
| `/mysql`   | `Mysql.vue`     | Trạng thái MySQL, danh sách database, start/stop/restart  |
| `/redis`   | `Redis.vue`     | Trạng thái Redis, uptime, clients, memory usage           |
| `/mailpit` | `Mailpit.vue`   | Trạng thái Mailpit, SMTP port, Web UI port                |
| `/php`     | `Php.vue`       | Trạng thái PHP-FPM, editor php.ini, quản lý extensions    |

Sidebar chia thành 2 nhóm:

- **System**: Dashboard
- **Services**: Nginx, MySQL, Redis, Mailpit, PHP

---

## Trạng thái triển khai

### Đã hoàn thiện

- ✅ `get_hardware_info` — Trả về CPU, RAM, Disk, OS info từ sysinfo crate
- ✅ `get_nginx_status` — Kiểm tra Nginx đã cài (`which nginx`) và đang chạy (`systemctl` / `pgrep`)
- ✅ Dashboard view hiển thị thông tin hệ thống
- ✅ UI components: sidebar, header, service cards, status indicators
- ✅ Dark/Light/System theme toggle

### Đang phát triển (Stub)

- ⚠️ `service_control.rs` — Start/Stop/Restart Nginx, PHP, Redis
- ⚠️ `config_manager.rs` — Đọc/Ghi file config (nginx.conf, php.ini)
- ⚠️ `os_command.rs` — Cross-platform service management (systemctl, brew services)
- ⚠️ `file_system.rs` — Phân quyền, đọc/ghi file hệ thống
- ⚠️ Các view Nginx, MySQL, Redis, Mailpit, PHP — UI đã có nhưng chưa kết nối dữ liệu thực

---

## Scripts

| Lệnh               | Mô tả                                        |
| ------------------ | -------------------------------------------- |
| `pnpm dev`         | Chạy Vite dev server (port 1420)             |
| `pnpm build`       | Type-check + build production                |
| `pnpm tauri dev`   | Build Rust + chạy app desktop với hot-reload |
| `pnpm tauri build` | Build production binary (.deb, .AppImage)    |

---

## Cấu hình Tauri

- **Product name**: `tauri-app`
- **Identifier**: `com.hongha.tauri-app`
- **Window**: 800×600 (min 700×500), resizable, maximizable
- **Dev URL**: `http://localhost:1420`
- **Bundle**: Tất cả định dạng (deb, AppImage, dmg, msi)
- **Permissions**: `core:default`, `opener:default`
