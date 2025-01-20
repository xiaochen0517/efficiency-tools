mod countdown;

use countdown::{get_remaining_time, start_countdown, CountdownState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        // 注册倒计时状态
        .manage(CountdownState(std::sync::Mutex::new(0)))
        // 注册命令处理函数
        .invoke_handler(tauri::generate_handler![
            start_countdown,
            get_remaining_time
        ])
        .setup(|app| {
            #[cfg(debug_assertions)] // 仅在开发模式下启用
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
