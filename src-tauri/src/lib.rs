mod config;
mod eq_state;
mod i18n;
mod privacy;
mod tray;
mod window;

use tauri::Manager;

fn on_window_close_requested(app_handle: &tauri::AppHandle) {
    let Some(win) = app_handle.get_webview_window("main") else {
        return;
    };
    if !win.is_visible().unwrap_or(false) {
        return;
    }
    let _ = win.eval(
        "var v=document.querySelector('video');\
         if(v&&v.paused){v.src='';v.load()}",
    );
    let _ = win.hide();
}

#[tauri::command]
fn set_eq_band(app: tauri::AppHandle, index: usize, gain: f64) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.eval(&format!("window.__ym_eq_set_band && window.__ym_eq_set_band({}, {})", index, gain));
    }
}

#[tauri::command]
fn set_eq_preamp(app: tauri::AppHandle, gain: f64) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.eval(&format!("window.__ym_eq_set_preamp && window.__ym_eq_set_preamp({})", gain));
    }
}

#[tauri::command]
fn toggle_eq(app: tauri::AppHandle, enabled: bool) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.eval(&format!("window.__ym_eq_toggle && window.__ym_eq_toggle({})", enabled));
    }
}

#[tauri::command]
fn get_locale() -> String {
    i18n::I18n::new().lang().to_string()
}

#[tauri::command]
fn save_eq_state(app: tauri::AppHandle, enabled: bool, preset_index: Option<usize>, bands: [f64; 10], preamp: f64) {
    eq_state::save(&app, &eq_state::EqState { enabled, preset_index, bands, preamp });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_locale,
            save_eq_state,
            set_eq_band,
            set_eq_preamp,
            toggle_eq,
        ])
        .setup(|app| {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                config::WEBVIEW2_ARGS,
            );

            let saved = eq_state::load(app.handle());

            let webview = window::create_main_window(app.handle())
                .expect("Failed to create main window");

            let app_handle = app.handle().clone();
            webview.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    on_window_close_requested(&app_handle);
                    api.prevent_close();
                }
            });

            tray::create_tray(app.handle(), &saved);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
