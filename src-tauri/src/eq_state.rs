use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EqState {
    pub enabled: bool,
    pub preset_index: Option<usize>,
    pub bands: [f64; 10],
    pub preamp: f64,
}

impl Default for EqState {
    fn default() -> Self {
        Self {
            enabled: true,
            preset_index: None,
            bands: [0.0; 10],
            preamp: 0.0,
        }
    }
}

const STORE_FILE: &str = "eq_state.json";
const STORE_KEY: &str = "eq_state";

pub fn load(app: &AppHandle) -> EqState {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(STORE_KEY))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

pub fn save(app: &AppHandle, state: &EqState) {
    if let Ok(store) = app.store(STORE_FILE) {
        let _ = store.set(STORE_KEY, serde_json::to_value(state).unwrap());
        let _ = store.save();
    }
}


