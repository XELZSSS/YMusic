use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager};

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

fn state_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("eq_state.json");
    Ok(path)
}

pub fn load(app: &AppHandle) -> EqState {
    let path = match state_path(app) {
        Ok(p) => p,
        Err(_) => return EqState::default(),
    };
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(app: &AppHandle, state: &EqState) {
    let path = match state_path(app) {
        Ok(p) => p,
        Err(_) => return,
    };
    if let Ok(json) = serde_json::to_string_pretty(state) {
        let _ = fs::write(&path, &json);
    }
}


