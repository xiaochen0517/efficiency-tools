use std::sync::Mutex;
use std::time::Duration;
use tauri::{Emitter, State};
use tokio::time;

/// 倒计时状态结构体
pub struct CountdownState(pub Mutex<i32>);

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
    state: State<'_, CountdownState>,
    window: tauri::Window,
) -> Result<(), String> {
    let mut remaining = seconds;

    // 更新初始状态
    *state.0.lock().unwrap() = remaining;

    // 开始倒计时循环
    while remaining > 0 {
        // 等待1秒
        time::sleep(Duration::from_secs(1)).await;
        remaining -= 1;

        // 更新状态
        *state.0.lock().unwrap() = remaining;

        // 发送更新事件到前端
        window
            .emit("countdown-update", remaining)
            .map_err(|e| e.to_string())?;
    }

    // 发送倒计时完成事件
    window
        .emit("countdown-complete", ())
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取当前倒计时剩余时间
#[tauri::command]
pub fn get_remaining_time(state: State<'_, CountdownState>) -> i32 {
    *state.0.lock().unwrap()
}