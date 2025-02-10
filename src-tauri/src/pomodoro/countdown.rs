use crate::pomodoro::time::PomodoroTimeMode;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tokio::time;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum CountdownMode {
    Work,
    Rest,
}

/// 倒计时状态结构体
pub struct PomodoroState {
    pub remaining_time: Mutex<i32>,
    pub started: Mutex<bool>,
    pub countdown_mode: Mutex<CountdownMode>,
    pub rest_count: Mutex<i32>,
    pub time_mode: Mutex<PomodoroTimeMode>,
}

/// 开始倒计时的命令处理函数
///
/// # 参数
/// * `seconds` - 倒计时的秒数
/// * `state` - 倒计时状态
/// * `window` - Tauri窗口实例，用于向前端发送事件
///
/// # 返回值
/// * `Result<(), String>` - 成功返回 Ok(()), 失败返回错误信息
#[tauri::command]
pub async fn start_countdown(
    seconds: i32,
    state: State<'_, PomodoroState>,
    window: tauri::Window,
    app_handle: AppHandle,
) -> Result<(), String> {
    let mut remaining = seconds;

    log::debug!(
        "Starting countdown for {} seconds",
        *state.remaining_time.lock().unwrap()
    );
    log::debug!(
        "Countdown is already running: {}",
        *state.started.lock().unwrap()
    );

    // 如果已经有倒计时在进行中，则直接返回错误信息
    if *state.started.lock().unwrap() {
        return Err("Countdown is already running".to_string());
    }

    // 更新初始状态
    *state.remaining_time.lock().unwrap() = remaining;
    *state.started.lock().unwrap() = true;

    // 开始倒计时循环
    while remaining > 0 {
        // 等待1秒
        time::sleep(Duration::from_secs(1)).await;
        remaining -= 1;

        // 更新状态
        *state.remaining_time.lock().unwrap() = remaining;

        // 发送更新事件到前端
        window
            .emit("countdown-update", remaining)
            .map_err(|e| e.to_string())?;
    }

    app_handle
        .dialog()
        .message("计时结束")
        .kind(MessageDialogKind::Info)
        .title("提示")
        .show(|_| {});

    // 更新状态
    *state.started.lock().unwrap() = false;
    let countdown_mode = *state.countdown_mode.lock().unwrap();
    if countdown_mode == CountdownMode::Rest {
        *state.rest_count.lock().unwrap() += 1;
        // 如果当前已经休息2次，则进入长休息周期
    }

    // 发送倒计时完成事件
    window
        .emit("countdown-complete", countdown_mode)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 获取当前倒计时剩余时间
#[tauri::command]
pub fn get_remaining_time(state: State<'_, PomodoroState>) -> i32 {
    *state.remaining_time.lock().unwrap()
}

/// 获取当前倒计时状态
#[tauri::command]
pub fn get_countdown_state(state: State<'_, PomodoroState>) -> bool {
    *state.started.lock().unwrap()
}

#[tauri::command]
pub fn set_countdown_mode(mode: CountdownMode, state: State<'_, PomodoroState>) {
    *state.countdown_mode.lock().unwrap() = mode;
}

#[tauri::command]
pub fn get_countdown_mode(state: State<'_, PomodoroState>) -> CountdownMode {
    *state.countdown_mode.lock().unwrap()
}
