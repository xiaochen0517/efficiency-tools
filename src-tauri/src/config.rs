use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxPusherConfig {
    pub spt_token: Option<String>,
}

impl WxPusherConfig {
    pub fn new(spt_token: Option<String>) -> Self {
        WxPusherConfig { spt_token }
    }

    pub fn default() -> Self {
        WxPusherConfig {
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
}
