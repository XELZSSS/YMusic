mod config;
mod eq_state;
mod i18n;
mod presets;
mod privacy;
mod tray;
mod util;
mod window;

use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_window_state::StateFlags;

fn send_eq_notification(app: &tauri::AppHandle, enabled: bool) {
    let status = if enabled { "Enabled" } else { "Disabled" };
    let _ = app.notification()
        .builder()
        .title("YMusic Equalizer")
        .body(format!("Equalizer {}", status))
        .show();
}

fn on_window_close_requested(app_handle: &tauri::AppHandle) {
    let Some(win) = app_handle.get_webview_window("main") else {
        return;
    };
    if !win.is_visible().unwrap_or(false) {
        return;
    }
    util::eval(&win,
        "var v=document.querySelector('video');\
         if(v&&v.paused){v.src='';v.load()}",
    );
    let _ = win.hide();
}

#[tauri::command]
fn get_locale() -> String {
    i18n::I18n::new().lang().to_string()
}

#[tauri::command]
fn get_eq_presets() -> Vec<presets::EqPreset> {
    presets::PRESETS.to_vec()
}

#[tauri::command]
fn save_eq_state(app: tauri::AppHandle, enabled: bool, preset_index: Option<usize>, bands: [f64; 10], preamp: f64) {
    eq_state::save(&app, &eq_state::EqState { enabled, preset_index, bands, preamp });
}

fn toggle_eq_global(app: &tauri::AppHandle) {
    let saved = eq_state::load(app);
    let new_state = !saved.enabled;
    if let Some(win) = app.get_webview_window("main") {
        util::eval(&win, &format!("window.__ym.eq.toggle({})", new_state));
    }
    eq_state::save(app, &eq_state::EqState {
        enabled: new_state,
        preset_index: saved.preset_index,
        bands: saved.bands,
        preamp: saved.preamp,
    });
    send_eq_notification(app, new_state);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("ymusic=warn")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main").map(|w| w.show());
        }))
        .plugin(tauri_plugin_window_state::Builder::default()
            .with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED)
            .build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_locale,
            get_eq_presets,
            save_eq_state,
        ])
        .setup(|app| {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                config::WEBVIEW2_ARGS,
            );

            let eq_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyE);
            app.global_shortcut().on_shortcut(eq_shortcut, move |app, _shortcut, event| {
                if event.state() == ShortcutState::Pressed {
                    toggle_eq_global(app);
                }
            }).map_err(|e| log::error!("Failed to setup shortcut handler: {}", e)).ok();

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
