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

fn build_initialization_script() -> String {
    let parts = [
        include_str!("../../src/scripts/css-injector.js"),
        include_str!("../../src/scripts/api-interceptor.js"),
        include_str!("../../src/scripts/dom-remover.js"),
        include_str!("../../src/scripts/audio-ad.js"),
        include_str!("../../src/scripts/tracking-cleaner.js"),
        include_str!("../../src/scripts/innertube-tweaks.js"),
        include_str!("../../src/scripts/equalizer.js"),
        include_str!("../../src/scripts/eq-ui.js"),
    ];
    format!(
        "window.__YM_CSS={};(function(){{if(window.__ym_adblock)return;window.__ym_adblock=true;{}}})();",
        js_template_literal(config::INJECTED_CSS),
        parts.join("\n"),
    )
}

fn show_window_later(ah: tauri::AppHandle) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(80));
        if let Some(win) = ah.get_webview_window("main") {
            let _ = win.show();
        }
    });
}

pub fn create_main_window(app: &tauri::AppHandle) -> Result<WebviewWindow, Box<dyn std::error::Error>> {
    let url = WebviewUrl::External(config::YMUSIC_URL.parse()?);
    let combined_script = build_initialization_script();

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
                show_window_later(ah.clone());
            }
        })
        .build()?;

    Ok(webview)
}
