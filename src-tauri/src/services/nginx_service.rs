use std::process::Command;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use crate::models::nginx::Nginx;
use crate::models::nginx::NginxData;
use crate::models::nginx::Website;

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

    pub fn read_config(file_path: &str) -> Result<String, String> {
        fs::read_to_string(file_path)
            .map_err(|e| format!("Không thể đọc file {}: {}", file_path, e))
    }

    pub fn test_config(content: &str) -> Result<String, String> {
        let tmp_path = format!("/tmp/nginx_test_{}.conf", std::process::id());

        fs::write(&tmp_path, content)
            .map_err(|e| format!("Không thể ghi file tạm: {}", e))?;

        let output = Command::new("nginx")
            .arg("-t")
            .arg("-c")
            .arg(&tmp_path)
            .output()
            .map_err(|e| format!("Không thể chạy nginx -t: {}", e))?;

        let _ = fs::remove_file(&tmp_path);

        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(stderr.trim().to_string())
        } else {
            let err_lines: Vec<&str> = stderr
                .lines()
                .filter(|l| l.contains("emerg") || l.contains("error") || l.contains("test failed"))
                .collect();

            if err_lines.is_empty() {
                Err(stderr.trim().to_string())
            } else {
                Err(err_lines.join("\n"))
            }
        }
    }

    pub fn write_config(file_path: &str, content: &str) -> Result<String, String> {
        Self::test_config(content)?;
        Self::write_with_elevated_privileges(file_path, content)
    }

    fn write_with_elevated_privileges(file_path: &str, content: &str) -> Result<String, String> {
        let os = env::consts::OS;

        if os == "macos" {
            let tmp = format!("/tmp/nginx_config_{}.tmp", std::process::id());
            fs::write(&tmp, content)
                .map_err(|e| format!("Không thể ghi file tạm: {}", e))?;

            let script = format!(
                "do shell script \"cp {} {}\" with administrator privileges",
                tmp, file_path
            );
            let out = Command::new("osascript")
                .arg("-e")
                .arg(script)
                .output()
                .map_err(|e| format!("Không thể gọi osascript: {}", e))?;

            let _ = fs::remove_file(&tmp);

            if out.status.success() {
                Ok(format!("Đã lưu cấu hình {}", file_path))
            } else {
                let err_msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
                if err_msg.contains("canceled") || err_msg.contains("Authorization Denied") {
                    Err(String::from(
                        "Người dùng đã hủy hoặc từ chối cấp quyền Admin.",
                    ))
                } else {
                    Err(err_msg)
                }
            }
        } else if os == "linux" {
            let mut child = Command::new("pkexec")
                .arg("tee")
                .arg(file_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .spawn()
                .map_err(|e| format!("Không thể chạy pkexec: {}", e))?;

            {
                let mut stdin = child
                    .stdin
                    .take()
                    .ok_or("Không thể mở stdin của pkexec")?;
                stdin
                    .write_all(content.as_bytes())
                    .map_err(|e| format!("Lỗi ghi stdin: {}", e))?;
            }

            let out = child
                .wait_with_output()
                .map_err(|e| format!("Lỗi chờ pkexec: {}", e))?;

            if out.status.success() {
                Ok(format!("Đã lưu cấu hình {}", file_path))
            } else {
                Err(String::from("Thực thi thất bại (Sai mật khẩu hoặc lỗi hệ thống)."))
            }
        } else {
            Err(String::from("Hệ điều hành không được hỗ trợ"))
        }
    }

    /// Hàm cốt lõi tự động kích hoạt hộp thoại xin quyền Admin dựa trên Hệ điều hành
    fn run_elevated_command(action: &str) -> Result<String, String> {
        let os = env::consts::OS;

        match action {
            "start"|"stop"|"restart" => {}
            _ => return Err("Invalid action".into())
        }

        let output = if os == "macos" {
            Command::new("brew")
            .args(["services", action, "nginx"])
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

    pub fn get_list_websites() -> Vec<Website> {
        let mut websites: Vec<Website> = Vec::new();

        let common_roots = [
            "/etc/nginx/sites-enabled",
            "/etc/nginx/conf.d",
            "/usr/local/etc/nginx/servers",
            "/opt/homebrew/etc/nginx/servers",
        ];

        let mut config_files: Vec<String> = Vec::new();

        for root in common_roots {
            if Path::new(root).exists() {
                if let Ok(entries) = std::fs::read_dir(root) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "conf")
                            || path.file_name().map_or(false, |n| !n.to_string_lossy().starts_with('.'))
                        {
                            config_files.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        for file_path in &config_files {
            let name = Path::new(file_path)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| file_path.clone());

            let content = std::fs::read_to_string(file_path).unwrap_or_default();

            let server_name = Self::extract_nginx_value(&content, "server_name");
            let root = Self::extract_nginx_value(&content, "root");

            let domain = if !server_name.is_empty() && server_name != "_" {
                server_name
            } else {
                name.trim_end_matches(".conf").to_string()
            };

            let doc_root = if !root.is_empty() {
                root
            } else {
                String::from("/var/www/html")
            };

            let enabled = file_path.contains("sites-enabled") || file_path.contains("conf.d") || file_path.contains("servers");

            websites.push(Website {
                name: domain,
                root: doc_root,
                enabled,
                file_path: file_path.clone(),
            });
        }

        if websites.is_empty() {
            websites.push(Website {
                name: String::from("default"),
                root: String::from("/var/www/html"),
                enabled: true,
                file_path: String::from("/etc/nginx/sites-available/default"),
            });
        }

        websites
    }

    fn extract_nginx_value(content: &str, directive: &str) -> String {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(directive) {
                let value = trimmed
                    .strip_prefix(directive)
                    .unwrap_or("")
                    .trim()
                    .trim_end_matches(';')
                    .trim()
                    .to_string();

                if !value.is_empty() {
                    return value.split_whitespace().next().unwrap_or("").to_string();
                }
            }
        }
        String::new()
    }

    pub fn is_installed () -> bool {
        Command::new("which").arg("nginx").output().map(|out| out.status.success()).unwrap_or(false)
    }

    pub fn is_running () -> bool {
        let os: &str = env::consts::OS;
        if os == "linux" {
            if let Ok(st) = Command::new("systemctl").args(&["is-active", "--quiet", "nginx"]).status() {
                if st.success() { return true; }
            }
        }
        // Fallback pgrep cho cả macos và linux
        Command::new("pgrep")
            .arg("nginx")
            .stdout(std::process::Stdio::null()) 
            .stderr(std::process::Stdio::null())
            .status()                          
            .map(|status| status.success())
            .unwrap_or(false)
    }
}