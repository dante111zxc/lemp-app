# Hướng dẫn Build và Cài đặt Tauri App trên Ubuntu

## 📋 Yêu cầu hệ thống

- Ubuntu 20.04 LTS trở lên
- Rust (phiên bản mới nhất)
- Node.js 18+ và pnpm
- Dependencies GTK và WebKit

## 🛠️ Cài đặt Dependencies

### 1. Cài đặt Rust (nếu chưa có)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2. Cài đặt dependencies Linux cho Tauri

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config
```

### 3. Cài đặt Node.js và pnpm (nếu chưa có)

```bash
# Cài đặt Node.js 20 LTS
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Cài đặt pnpm
npm install -g pnpm
```

## 🚀 Build ứng dụng

### 1. Clone hoặc vào thư mục dự án

```bash
cd /var/www/html/lemp-app
```

### 2. Cài đặt dependencies

```bash
pnpm install
```

### 3. Build ứng dụng

```bash
pnpm build
```

Lệnh này sẽ:

- Type-check Vue code với TypeScript
- Build frontend với Vite
- Compile Rust backend
- Tạo file cài đặt trong thư mục `src-tauri/target/release/bundle/`

### 4. Tìm file cài đặt

Sau khi build xong, bạn sẽ tìm thấy các file sau:

#### Cho Ubuntu/Debian (file .deb):

```bash
src-tauri/target/release/bundle/deb/tauri-app_0.1.0_amd64.deb
```

#### Cho Linux thông thường (AppImage):

```bash
src-tauri/target/release/bundle/appimage/tauri-app_0.1.0_amd64.AppImage
```

## 📦 Cài đặt ứng dụng

### Cách 1: Cài đặt file .deb (khuyên dùng cho Ubuntu/Debian)

```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/tauri-app_0.1.0_amd64.deb

# Nếu có lỗi dependencies, chạy:
sudo apt-get install -f
```

### Cách 2: Chạy AppImage (không cần cài đặt)

```bash
# Cấp quyền thực thi
chmod +x src-tauri/target/release/bundle/appimage/tauri-app_0.1.0_amd64.AppImage

# Chạy ứng dụng
./src-tauri/target/release/bundle/appimage/tauri-app_0.1.0_amd64.AppImage
```

### Cách 3: Copy AppImage vào thư mục ứng dụng

```bash
# Copy vào thư mục cá nhân
cp src-tauri/target/release/bundle/appimage/tauri-app_0.1.0_amd64.AppImage ~/Applications/

# Hoặc vào thư mục hệ thống (cần sudo)
sudo cp src-tauri/target/release/bundle/appimage/tauri-app_0.1.0_amd64.AppImage /opt/
```

## 🎯 Chạy ứng dụng

### Sau khi cài đặt .deb:

- Tìm "tauri-app" trong menu ứng dụng
- Hoặc chạy từ terminal: `tauri-app`

### Với AppImage:

```bash
./tauri-app_0.1.0_amd64.AppImage
```

## 🧪 Test trong chế độ Development

Nếu muốn test mà không cần build:

```bash
pnpm tauri dev
```

Lệnh này sẽ:

- Khởi động Vite dev server (port 1420)
- Compile và chạy Rust backend
- Mở cửa sổ ứng dụng với hot-reload

## ✨ Tính năng đã thêm

### Function `hello_world`

Trong ứng dụng này, đã có function Rust `hello_world()` được kết nối với frontend:

**Backend (Rust)** - `src-tauri/src/lib.rs`:

```rust
#[tauri::command]
fn hello_world() -> String {
    "Hello World from Rust!".to_string()
}
```

**Frontend (Vue)** - `src/App.vue`:

```typescript
async function showHelloWorld() {
  helloWorldMsg.value = await invoke('hello_world')
}
```

Khi bạn click nút "Show Hello World" trong ứng dụng, nó sẽ gọi function Rust và hiển thị "Hello World from Rust!" lên màn hình!

## 🔧 Gỡ lỗi

### Nếu gặp lỗi khi build:

1. **Lỗi pkg-config**: Cài đặt lại dependencies

   ```bash
   sudo apt install pkg-config
   ```

2. **Lỗi webkit**: Cài đặt webkit2gtk

   ```bash
   sudo apt install libwebkit2gtk-4.1-dev
   ```

3. **Lỗi Rust**: Cập nhật Rust

   ```bash
   rustup update
   ```

4. **Clean build nếu cần**:
   ```bash
   cargo clean
   rm -rf node_modules
   pnpm install
   pnpm build
   ```

## 📝 Ghi chú

- File .deb chỉ hoạt động trên Ubuntu/Debian và các distro dựa trên Debian
- AppImage hoạt động trên hầu hết các distro Linux
- Kích thước file build khoảng 50-100MB
- Lần build đầu tiên sẽ mất nhiều thời gian (5-15 phút) do cần download và compile dependencies
- Các lần build sau sẽ nhanh hơn nhiều

## 🎨 Tùy chỉnh

Để thay đổi tên, icon hoặc metadata của ứng dụng, chỉnh sửa:

- `src-tauri/tauri.conf.json` - Cấu hình Tauri
- `src-tauri/icons/` - Thay thế icon
- `package.json` - Metadata dự án

## 📚 Tài liệu tham khảo

- [Tauri Documentation](https://tauri.app)
- [Vue 3 Documentation](https://vuejs.org)
- [Rust Documentation](https://www.rust-lang.org/learn)
