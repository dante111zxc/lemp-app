use std::process::Command;
use std::env;
use crate::models::nginx::Nginx;
use crate::models::nginx::NginxData;

pub struct NginxService;

impl NginxService {
    /// Khởi động Nginx
    pub fn start() -> Result<String, String> {
        Self::run_elevated_command("start")
    }

    pub fn stop()  -> Result<String, String> {
        Self::run_elevated_command("stop")
    }

    pub fn restart () -> Result<String, String> {
        Self::run_elevated_command("restart")
    }

    /// Hàm cốt lõi tự động kích hoạt hộp thoại xin quyền Admin dựa trên Hệ điều hành
    fn run_elevated_command(action: &str) -> Result<String, String> {
        let os = env::consts::OS;

        let output = if os == "macos" {
            // === Trên macOS ===
            // Sử dụng AppleScript thông qua osascript để ép hệ thống bật hộp thoại Touch ID / Password.
            // Câu lệnh mẫu: osascript -e 'do shell script "brew services start nginx" with administrator privileges'
            let script = format!("do shell script \"brew services {} nginx\" with administrator privileges", action);
            
            Command::new("osascript")
                .arg("-e")
                .arg(script)
                .output()

        } else if os == "linux" {
            // === Trên Linux ===
            // Sử dụng `pkexec` (PolicyKit). Công cụ này sẽ tự động bật hộp thoại nhập mật khẩu dạng GUI 
            // (như của Ubuntu/Fedora) lên để chạy lệnh systemctl dưới quyền root.
            Command::new("pkexec")
                .arg("systemctl")
                .arg(action)
                .arg("nginx")
                .output()

        } else {
            return Err(String::from("Hệ điều hành không được hỗ trợ"));
        };

        // Xử lý kết quả trả về
        match output {
            Ok(out) => {
                if out.status.success() {
                    Ok(format!("Thực hiện hành động '{}' Nginx thành công!", action))
                } else {
                    let err_msg = String::from_utf8_lossy(&out.stderr);
                    let clean_err = err_msg.trim();
                    
                    if clean_err.contains("canceled") || clean_err.contains("Authorization Denied") {
                        Err(String::from("Người dùng đã hủy hoặc từ chối cấp quyền Admin."))
                    } else if clean_err.is_empty() {
                        Err(String::from("Thực thi thất bại (Sai mật khẩu hoặc lỗi hệ thống)."))
                    } else {
                        Err(clean_err.to_string())
                    }
                }
            }
            Err(e) => Err(format!("Không thể gọi lệnh hệ thống: {}", e)),
        }
    }

    pub fn get_nginx_status() -> Nginx {
        let installed = Self::is_installed();
        let running = Self::is_running();

        let (status, message) = if !installed {
            (0, String::from("Nginx chưa được cài"))
        } else if running {
            (1, String::from("Nginx đang hoạt động"))
        } else {
            (2, String::from("Nginx đã cài đặt nhưng không hoạt động"))
        };

        let version = if installed { Self::get_version() } else { String::from("N/A") };

        Nginx {
            message,
            data: NginxData {
                status,
                version,
            },
        }
    }

    /// Hàm lấy phiên bản hiện tại của Nginx
    pub fn get_version() -> String {
        let output = Command::new("nginx")
            .arg("-v") // Lệnh lấy phiên bản ngắn gọn (ví dụ: nginx version: nginx/1.25.3)
            .output();

        match output {
            Ok(out) => {
                // RẤT QUAN TRỌNG: Nginx in version ra `stderr`, không phải `stdout`
                let raw_err = String::from_utf8_lossy(&out.stderr);
                
                if raw_err.is_empty() {
                    // Dự phòng nếu ở một số OS cũ nó lại in ra stdout
                    let raw_out = String::from_utf8_lossy(&out.stdout);
                    Self::clean_version_string(&raw_out)
                } else {
                    Self::clean_version_string(&raw_err)
                }
            }
            Err(_) => String::from("Không thể lấy phiên bản"),
        }
    }

    /// Hàm phụ trợ để cắt gọt chuỗi "nginx version: nginx/1.24.0" thành "1.24.0"
    fn clean_version_string(raw: &str) -> String {
        // Chuỗi thô thường có dạng: "nginx version: nginx/1.25.3\n"
        // Ta bóc tách lấy phần sau dấu gạch chéo `/`
        raw.split('/')
            .nth(1) // Lấy phần tử thứ 2 sau dấu '/'
            .unwrap_or(raw) // Nếu không chia được thì giữ nguyên chuỗi thô
            .trim() // Xóa bỏ khoảng trắng và dấu xuống dòng \n
            .to_string()
    }

    pub fn is_installed () -> bool {
        Command::new("which").arg("nginx").output().map(|out| out.status.success()).unwrap_or(false)
    }

    pub fn is_running () -> bool {
        let os = env::consts::OS;
        if os == "linux" {
            if let Ok(st) = Command::new("systemctl").args(&["is-active", "--quiet", "nginx"]).status() {
                if st.success() { return true; }
            }
        }
        // Fallback pgrep cho cả macos và linux
        Command::new("pgrep").args(&["-x", "nginx"]).output().map(|out| out.status.success()).unwrap_or(false)
    }
}