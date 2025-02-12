use crate::pomodoro::countdown::{stop_countdown, CountdownMode, PomodoroState};
use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

pub mod countdown;
pub mod time;

#[tauri::command]
pub async fn start_pomodoro(
    state: State<'_, PomodoroState>,
    window: tauri::Window,
    app_handle: AppHandle,
) -> Result<(), String> {
    // 如果已经有倒计时在进行中，则直接返回错误信息
    if state.is_running.load(Ordering::SeqCst) {
        return Err("Countdown is already running".to_string());
    }

    // 更新初始状态
    state.is_running.store(true, Ordering::SeqCst);
    state.should_stop.store(false, Ordering::SeqCst);

    // 创建停止信号
    let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();
    *state.stop_signal.lock().unwrap() = Some(stop_tx);

    // 发送开始倒计时事件
    window.emit("countdown-started", ()).unwrap();

    // 运行倒计时
    let mut remaining = *state.remaining_time.lock().unwrap();

    while remaining > 0 && !state.should_stop.load(Ordering::SeqCst) {
        // 使用可中断的等待
        let sleep_future = tokio::time::sleep(Duration::from_secs(1));
        tokio::select! {
            _ = sleep_future => {
                remaining -= 1;
                *state.remaining_time.lock().unwrap() = remaining;
                window.emit("countdown-update", remaining).unwrap();
            }
            _ = &mut stop_rx => {
                break;
            }
        }
    }
    // 清理状态
    *state.stop_signal.lock().unwrap() = None;
    if state.should_stop.load(Ordering::SeqCst) {
        // 处理手动停止逻辑
        window.emit("countdown-stopped", ()).unwrap();
    } else {
        // 正常结束逻辑
        countdown_complete(&state, app_handle, window);
    }
    state.is_running.store(false, Ordering::SeqCst);
    Ok(())
}

fn countdown_complete(
    state: &State<'_, PomodoroState>,
    app_handle: AppHandle,
    window: tauri::Window,
) {
    app_handle
        .dialog()
        .message("计时结束")
        .kind(MessageDialogKind::Info)
        .title("提示")
        .show(|_| {});

    // 更新状态
    let time_mode_config = (*state.time_mode.lock().unwrap()).get_config();
    let mut next_round_time: i32;
    let mut countdown_mode_guard = state.countdown_mode.lock().unwrap();
    let mut rest_count_guard = state.rest_count.lock().unwrap();
    match *countdown_mode_guard {
        CountdownMode::Work => {
            // 如果当前已经休息2次，并且当前结束了工作模式，则进入长休息周期
            if *rest_count_guard % 2 == 0 && *rest_count_guard > 0 {
                next_round_time = time_mode_config.long_rest_time;
            } else {
                next_round_time = time_mode_config.short_rest_time;
            }
            *countdown_mode_guard = CountdownMode::Rest;
        }
        CountdownMode::Rest => {
            // 一个工作对应一个休息，当前休息状态结束后，将休息次数+1
            *rest_count_guard += 1;
            // 设置下一次倒计时时间
            next_round_time = time_mode_config.work_time;
            *countdown_mode_guard = CountdownMode::Work;
        }
    }
    *state.remaining_time.lock().unwrap() = next_round_time;
    window.emit("countdown-complete", ()).unwrap();
    window.show().unwrap();
    window.set_focus().unwrap();
}

#[tauri::command]
pub async fn stop_pomodoro(state: State<'_, PomodoroState>) -> Result<(), String> {
    stop_countdown(&state);
    Ok(())
}
