pub mod presets;
pub mod state;

use tauri::AppHandle;

pub use state::EqState;

#[tauri::command]
pub fn get_eq_presets() -> Vec<presets::EqPreset> {
    presets::PRESETS.to_vec()
}

#[tauri::command]
pub fn save_eq_state(app: tauri::AppHandle, enabled: bool, preset_index: Option<usize>, bands: [f64; 10], preamp: f64) {
    state::save(&app, &state::EqState { enabled, preset_index, bands, preamp });
}

pub fn load_eq_state(app: &AppHandle) -> state::EqState {
    state::load(app)
}

pub fn save_eq_state_impl(app: &AppHandle, state_val: &state::EqState) {
    state::save(app, state_val);
}
