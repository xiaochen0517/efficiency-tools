mod pomodoro;
mod utils;

use crate::pomodoro::countdown::{
    get_countdown_mode, get_countdown_state, get_remaining_time, set_countdown_mode,
    start_countdown, CountdownMode, PomodoroState,
};
use std::error::Error;
use std::sync::Mutex;
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Manager, WindowEvent};

use crate::pomodoro::time::PomodoroTimeMode;
use utils::time_util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册插件
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        // 注册倒计时状态
        .manage(PomodoroState {
            remaining_time: Mutex::new(0),
            started: Mutex::new(false),
            countdown_mode: Mutex::new(CountdownMode::Work),
            rest_count: Mutex::new(0),
            time_mode: Mutex::new(PomodoroTimeMode::Medium),
        })
        // 注册命令处理函数
        .invoke_handler(tauri::generate_handler![
            start_countdown,
            get_remaining_time,
            get_countdown_state,
            set_countdown_mode,
            get_countdown_mode
        ])
        .setup(|app| {
            create_tray(app)?;

            update_tray_tooltip(app);

            #[cfg(debug_assertions)] // 仅在开发模式下启用
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                log::debug!("Close requested");
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn update_tray_tooltip(app: &mut App) {
    // 定时更新托盘提示
    let app_handle = app.handle().clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        let countdown_state = app_handle.state::<PomodoroState>();
        let total_secs = countdown_state.remaining_time.lock().unwrap().clone();
        let hms_data = time_util::sec_to_hms(total_secs);
        let tooltip = format!(
            "当前剩余 {}小时{}分钟{}秒",
            hms_data.hours, hms_data.minutes, hms_data.seconds
        );

        let app_clone = app_handle.clone();
        app_handle
            .run_on_main_thread(move || {
                app_clone
                    .tray_by_id("main_tray")
                    .unwrap()
                    .set_tooltip(Some(tooltip.as_str()))
                    .unwrap();
            })
            .unwrap();
    });
}

fn create_tray(app: &mut App) -> Result<(), Box<dyn Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    TrayIconBuilder::with_id("main_tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app: &AppHandle, event| match event.id.as_ref() {
            "quit" => {
                log::debug!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                log::debug!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::DoubleClick { .. } => {
                log::debug!("Show and focus main window");
                // in this example, let's show and focus the main window when the tray is clicked
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}
