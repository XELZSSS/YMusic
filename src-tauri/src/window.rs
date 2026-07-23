use tauri::webview::{NewWindowResponse, PageLoadEvent};
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tauri::Manager;

use crate::config;
use crate::i18n::{I18n, I18nKey};
use crate::util;

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
        include_str!("../../src/scripts/unified-fetch.js"),
        include_str!("../../src/scripts/audio-ad.js"),
        include_str!("../../src/scripts/ytcfg-injector.js"),
        include_str!("../../src/scripts/equalizer.js"),
        include_str!("../../src/scripts/visualizer.js"),
        include_str!("../../src/scripts/audio-only.js"),
    ];
    format!(
        "window.__YM_CSS={};(function(){{if(window.__ym_adblock)return;window.__ym_adblock=true;{}}})();",
        js_template_literal(config::INJECTED_CSS),
        parts.join("\n"),
    )
}

fn show_window(webview: &WebviewWindow) {
    let was_max = webview.is_maximized().unwrap_or(false);
    if was_max {
        let _ = webview.unmaximize();
    }
    let _ = webview.show();
    let _ = webview.set_focus();
    if was_max {
        let w = webview.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            let _ = w.maximize();
        });
    }
}

fn restore_eq_state(webview: &WebviewWindow, bands: &[f64; 10], preamp: f64, enabled: bool) {
    let mut bands_js = String::new();
    for (i, b) in bands.iter().enumerate() {
        if i > 0 { bands_js.push(','); }
        bands_js.push_str(&b.to_string());
    }
    util::eval(webview, &format!(
        "window.__ym.eq.toggle({});window.__ym.eq.applyPreset([{}],{})",
        enabled, bands_js, preamp
    ));
}

pub fn create_main_window(app: &tauri::AppHandle, i18n: &I18n) -> Result<WebviewWindow, Box<dyn std::error::Error>> {
    let url = WebviewUrl::External(config::YMUSIC_URL.parse()?);
    let combined_script = build_initialization_script();

    let webview = WebviewWindowBuilder::new(app, "main", url)
        .inner_size(config::WINDOW_WIDTH, config::WINDOW_HEIGHT)
        .min_inner_size(config::WINDOW_MIN_WIDTH, config::WINDOW_MIN_HEIGHT)
        .resizable(true)
        .title(i18n.t(I18nKey::AppWindowTitle))
        .visible(false)
        .theme(Some(tauri::Theme::Dark))
        .on_web_resource_request(crate::privacy::on_resource_request)
        .on_new_window(|_url, _features| NewWindowResponse::Allow)
        .initialization_script(&combined_script)
        .on_page_load(move |webview, event| {
            if let PageLoadEvent::Finished = event.event() {
                let app_handle = webview.app_handle();
                let current = crate::eq_state::load(&app_handle);
                restore_eq_state(&webview, &current.bands, current.preamp, current.enabled);
                show_window(&webview);
            }
        })
        .build()?;

    Ok(webview)
}
