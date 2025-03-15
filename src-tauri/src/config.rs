use crate::pomodoro::time::PomodoroTimeMode;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxPusherConfig {
    pub time_mode: Option<PomodoroTimeMode>,
    pub spt_token: Option<String>,
}

impl WxPusherConfig {
    pub fn new(time_mode: Option<PomodoroTimeMode>, spt_token: Option<String>) -> Self {
        WxPusherConfig {
            time_mode: Some(time_mode.unwrap_or(PomodoroTimeMode::Medium)),
            spt_token,
        }
    }

    pub fn default() -> Self {
        WxPusherConfig {
            time_mode: Some(PomodoroTimeMode::Medium),
            spt_token: None,
        }
    }

    pub fn get_config(app: &AppHandle) -> Self {
        match app.store("store.json") {
            Ok(store) => match store.get("wxpusher_config") {
                Some(config) => serde_json::from_value(config).unwrap_or(WxPusherConfig::default()),
                None => WxPusherConfig::default(),
            },
            Err(_) => WxPusherConfig::default(),
        }
    }

    pub fn get_time_mode(&self) -> PomodoroTimeMode {
        self.time_mode.unwrap_or(PomodoroTimeMode::Medium)
    }

    pub fn get_spt_token(&self) -> Option<String> {
        self.spt_token.clone()
    }
}
