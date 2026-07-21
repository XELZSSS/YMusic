mod privacy;
mod tray;

use std::thread;
use std::time::Duration;
use tauri::webview::PageLoadEvent;
use tauri::{WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri::Manager;
use tauri_plugin_window_state::StateFlags;

const YMUSIC_URL: &str = "https://music.youtube.com";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default()
            .with_state_flags(StateFlags::SIZE | StateFlags::POSITION | StateFlags::MAXIMIZED)
            .build())
        .setup(|app| {
            // WebView2 内存和缓存限制 + 禁用无关 Chromium 功能
            std::env::set_var(
                "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
                "--disk-cache-size=104857600 --max-old-space-size=512 \
                 --disable-features=AutofillServerCommunication,TranslateUI,MediaRouter,OptimizationHints \
                 --disable-background-networking --disable-sync --no-pings \
                 --disable-breakpad --disable-component-update",
            );

            let adblock_js = include_str!("../../src/adblock.js").to_string();
            let tracking_js = include_str!("../../src/strip-tracking.js").to_string();
            let gc_js = include_str!("../../src/gc.js").to_string();

            let url = WebviewUrl::External(YMUSIC_URL.parse().expect("Invalid YMUSIC_URL"));
            let webview = WebviewWindowBuilder::new(app, "main", url)
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .resizable(true)
            .title("YMusic")
            .visible(false)
            .theme(Some(tauri::Theme::Dark))
            .on_web_resource_request(privacy::on_resource_request)
            .on_page_load(move |webview, event| {
                if let PageLoadEvent::Finished = event.event() {
                    if let Err(e) = webview.eval(&adblock_js) {
                        eprintln!("Failed to inject adblock script: {e}");
                    }
                    if let Err(e) = webview.eval(&tracking_js) {
                        eprintln!("Failed to inject tracking strip script: {e}");
                    }
                    if let Err(e) = webview.eval(&gc_js) {
                        eprintln!("Failed to inject gc helper script: {e}");
                    }
                    if let Err(e) = webview.show() {
                        eprintln!("Failed to show window on page load: {e}");
                    }
                }
            })
            .build()
            .expect("Failed to create webview window");

            // 方案 2: 窗口关闭时触发 GC 再隐藏
            let app_handle = app.handle().clone();
            webview.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.eval(
                            "if(window.gc)window.gc();\
                             var v=document.querySelector('video');\
                             if(v&&v.paused){v.src='';v.load()}",
                        );
                        let _ = win.hide();
                    }
                    api.prevent_close();
                }
            });

            // 方案 1: 每小时触发 V8 GC（不清理浏览数据，避免中断播放）
            let app_clone = app.handle().clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(3600));
                if let Some(win) = app_clone.get_webview_window("main") {
                    let _ = win.eval("if(window.gc)window.gc()");
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
