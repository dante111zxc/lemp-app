use std::env;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[tauri::command]
pub fn read_nginx_config(file_path: String) -> Result<String, String> {
    match fs::read_to_string(&file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Không thể đọc file {}: {}", file_path, e)),
    }
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

#[tauri::command]
pub fn write_nginx_config(file_path: String, content: String) -> Result<String, String> {
    write_with_elevated_privileges(&file_path, &content)
}
