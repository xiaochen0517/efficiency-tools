use crate::pomodoro::time::PomodoroTimeMode;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::State;
use tokio::sync::oneshot::Sender;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum CountdownMode {
    Work,
    Rest,
}

/// 倒计时状态结构体
#[derive(Debug)]
pub struct PomodoroState {
    pub remaining_time: Mutex<i32>,
    pub countdown_mode: Mutex<CountdownMode>,
    pub rest_count: Mutex<i32>,
    pub time_mode: Mutex<PomodoroTimeMode>,
    pub is_running: AtomicBool,
    pub stop_signal: Mutex<Option<Sender<()>>>, // 停止信号锁
    pub should_stop: AtomicBool,                // 原子布尔标志
}

#[derive(Serialize, Deserialize)]
pub struct PomodoroStateDto {
    pub remaining_time: i32,
    pub countdown_mode: CountdownMode,
    pub rest_count: i32,
    pub time_mode: PomodoroTimeMode,
    pub started: bool,
}

impl PomodoroState {
    pub fn to_dto(&self) -> PomodoroStateDto {
        PomodoroStateDto {
            remaining_time: *self.remaining_time.lock().unwrap(),
            started: self.is_running.load(Ordering::SeqCst),
            countdown_mode: *self.countdown_mode.lock().unwrap(),
            rest_count: *self.rest_count.lock().unwrap(),
            time_mode: *self.time_mode.lock().unwrap(),
        }
    }
}

#[tauri::command]
pub fn get_pomodoro_state(state: State<'_, PomodoroState>) -> PomodoroStateDto {
    state.to_dto()
}

#[tauri::command]
pub fn set_pomodoro_time_mode(
    time_mode: PomodoroTimeMode,
    state: State<'_, PomodoroState>,
) -> Result<(), String> {
    stop_countdown(&state);
    // 等待倒计时结束
    while state.is_running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    *state.time_mode.lock().unwrap() = time_mode;
    *state.remaining_time.lock().unwrap() = time_mode.get_config().work_time;
    *state.countdown_mode.lock().unwrap() = CountdownMode::Work;
    *state.rest_count.lock().unwrap() = 0;
    Ok(())
}

pub fn stop_countdown(state: &State<'_, PomodoroState>) {
    state.should_stop.store(true, Ordering::SeqCst);
    if let Some(tx) = (*state.stop_signal.lock().unwrap()).take() {
        tx.send(()).unwrap();
    }
}
