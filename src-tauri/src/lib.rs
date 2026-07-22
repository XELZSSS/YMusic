mod config;
mod privacy;
mod tray;
mod window;

use tauri::WindowEvent;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                config::WEBVIEW2_ARGS,
            );

            let webview = window::create_main_window(app.handle())
                .expect("Failed to create main window");

            let app_handle = app.handle().clone();
            webview.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.eval(
                            "var v=document.querySelector('video');\
                             if(v&&v.paused){v.src='';v.load()}",
                        );
                        let _ = win.hide();
                    }
                    api.prevent_close();
                }
            });

            tray::create_tray(app.handle());

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
