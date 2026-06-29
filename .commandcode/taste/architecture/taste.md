# architecture
- Use brew services commands via Rust std::process::Command for service management. Confidence: 0.85
- Manage multi-version PHP by linking/unlinking brew formulae (brew link --overwrite --force php@x.y). Confidence: 0.85
- Use osascript in Rust to request sudo permissions for editing /etc/hosts or Nginx configs. Confidence: 0.85
- Focus on tauri::SystemTray for the native macOS menu bar experience. Confidence: 0.85
- Use Tauri's event system (AppHandle::emit) for real-time monitoring instead of frontend polling with setInterval + invoke. Confidence: 0.65
- For macOS Homebrew Nginx installations, include /opt/homebrew/etc/nginx/servers in common_roots path lookups. Confidence: 0.70
