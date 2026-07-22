use std::thread;
use std::time::Duration;

use tauri::webview::PageLoadEvent;
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri::Manager;

use crate::config;

fn js_template_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('`');
    for c in s.chars() {
        match c {
            '`' => out.push_str("\\`"),
            '\\' => out.push_str("\\\\"),
            '$' => out.push_str("\\$"),
            c => out.push(c),
        }
    }
    out.push('`');
    out
}

pub fn create_main_window(app: &tauri::AppHandle) -> Result<WebviewWindow, Box<dyn std::error::Error>> {
    let url = WebviewUrl::External(config::YMUSIC_URL.parse()?);

    let combined_script = format!(
        "var __YM_CSS={css};{adblock}",
        css = js_template_literal(config::INJECTED_CSS),
        adblock = include_str!("../../src/adblock.js"),
    );

    let ah = app.app_handle().clone();
    let webview = WebviewWindowBuilder::new(app, "main", url)
        .inner_size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .min_inner_size(config::WINDOW_MIN_WIDTH, config::WINDOW_MIN_HEIGHT)
        .resizable(true)
        .title(config::WINDOW_TITLE)
        .visible(false)
        .theme(Some(tauri::Theme::Dark))
        .on_web_resource_request(crate::privacy::on_resource_request)
        .initialization_script(&combined_script)
        .on_page_load(move |_webview, event| {
            if let PageLoadEvent::Finished = event.event() {
                let a = ah.clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(80));
                    if let Some(win) = a.get_webview_window("main") {
                        let _ = win.show();
                    }
                });
            }
        })
        .build()?;

    Ok(webview)
}
