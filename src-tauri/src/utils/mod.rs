pub mod time_util;

#[tauri::command]
pub fn is_dev_mode() -> bool {
    tauri::is_dev() // 直接使用 Tauri 的 is_dev() 函数
}