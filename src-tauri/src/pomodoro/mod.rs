use crate::config::WxPusherConfig;
use crate::pomodoro::countdown::{stop_countdown, CountdownMode, PomodoroState};
use reqwest::{Error, Response};
use serde_json::json;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_store::StoreExt;

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

    let mut countdown_mode_guard = state.countdown_mode.lock().unwrap();
    let countdown_mode_clone = (*countdown_mode_guard).clone();
    tokio::spawn(async move {
        send_wx_message(app_handle.clone(), countdown_mode_clone).await;
    });

    // 更新状态
    let time_mode_config = (*state.time_mode.lock().unwrap()).get_config();
    let mut next_round_time: i32;
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

async fn send_wx_message(app_handle: AppHandle, countdown_mode: CountdownMode) {
    let wx_pusher_config = WxPusherConfig::get_config(&app_handle);
    if wx_pusher_config.spt_token.is_none() {
        log::warn!("wx pusher spt_token config is not set");
        return;
    }
    log::debug!(
        "wx pusher config spt_token: {:?}",
        wx_pusher_config.spt_token
    );

    let content = match countdown_mode {
        CountdownMode::Work => "工作时间结束，休息一下吧",
        CountdownMode::Rest => "休息时间结束，继续工作吧",
    };
    let client = reqwest::Client::new();
    let res = client
        .post("https://wxpusher.zjiecode.com/api/send/message/simple-push")
        .json(&json!({
          "content": content,
          "summary": content,
          "contentType": 1,
          "spt": wx_pusher_config.spt_token
        }))
        .send()
        .await;

    match res {
        Ok(resp) => {
            log::debug!("Response:\n{}", resp.text().await.unwrap());
        }
        Err(e) => {
            log::error!("Error: {}", e);
        }
    }
}

#[tauri::command]
pub async fn stop_pomodoro(state: State<'_, PomodoroState>) -> Result<(), String> {
    stop_countdown(&state);
    Ok(())
}
