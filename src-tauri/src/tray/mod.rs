mod builder;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{CheckMenuItem, MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewUrl,
};
use tauri_plugin_notification::NotificationExt;

use crate::eq_state::{self, EqState};
use crate::i18n::{I18n, I18nKey};
use crate::util;

static SLEEP_CANCEL: Mutex<Option<Arc<AtomicBool>>> = Mutex::new(None);

fn toggle_window(app: &AppHandle) {
    let Some(win) = app.get_webview_window("main") else {
        return;
    };
    if win.is_visible().unwrap_or(false) {
        let _ = win.hide();
    } else {
        let _ = win.show();
    }
}

fn apply_preset(app: &AppHandle, idx: usize) {
    let Some(preset) = crate::presets::PRESETS.get(idx) else {
        log::error!("Invalid preset index: {}", idx);
        return;
    };
    let mut bands_str = String::new();
    for (i, b) in preset.bands.iter().enumerate() {
        if i > 0 { bands_str.push(','); }
        bands_str.push_str(&b.to_string());
    }
    if let Some(win) = app.get_webview_window("main") {
        util::eval(&win, &format!(
            "window.__ym.eq.applyPreset([{}],{})",
            bands_str, preset.preamp
        ));
    }
}

fn reset_eq(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        util::eval(&win, "window.__ym.eq.applyPreset([0,0,0,0,0,0,0,0,0,0],0)");
    }
}

fn toggle_eq(app: &AppHandle, checked: bool) {
    if let Some(win) = app.get_webview_window("main") {
        util::eval(&win, &format!("window.__ym.eq.toggle({})", checked));
    }
}

fn send_sleep_notification(app: &AppHandle, minutes: u64) {
    let _ = app.notification()
        .builder()
        .title("Sleep Timer")
        .body(format!("Timer set for {} minutes", minutes))
        .show();
}

fn sleep_timer(app: &AppHandle, minutes: u64) {
    if let Ok(mut guard) = SLEEP_CANCEL.lock() {
        if let Some(cancel) = guard.take() {
            cancel.store(true, Ordering::Relaxed);
        }
        let cancel = Arc::new(AtomicBool::new(false));
        *guard = Some(cancel.clone());
        let app = app.clone();
        std::thread::spawn(move || {
            for _ in 0..(minutes * 60) {
                if cancel.load(Ordering::Relaxed) {
                    return;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.eval("var v=document.querySelector('video');if(v)v.pause()");
            }
        });
    }
}

pub fn create_tray(app: &AppHandle, saved: &EqState, i18n: &I18n) {
    let last = Mutex::new(saved.clone());

    let Ok(show_hide) = MenuItemBuilder::with_id("show_hide", i18n.t(I18nKey::TrayShowHide))
        .build(app)
    else {
        log::error!("Failed to create tray item 'show_hide'");
        return;
    };

    let Ok(quit) = MenuItemBuilder::with_id("quit", i18n.t(I18nKey::TrayQuit))
        .build(app)
    else {
        log::error!("Failed to create tray item 'quit'");
        return;
    };

    let eq_items = match builder::build_eq_submenu(app, saved, i18n) {
        Ok(v) => v,
        Err(msg) => { log::error!("{}", msg); return; }
    };
    let eq_toggle = eq_items.eq_toggle;
    let preset_clones = eq_items.preset_clones;
    let eq_submenu = eq_items.eq_submenu;
    let eq_toggle_ref = eq_toggle.clone();

    let Ok(aot) = CheckMenuItem::with_id(app, "always_on_top", i18n.t(I18nKey::TrayAlwaysOnTop), true, false, None::<&str>)
    else {
        log::error!("Failed to create tray item 'always_on_top'");
        return;
    };
    let aot_ref = aot.clone();

    let Ok(pip) = MenuItemBuilder::with_id("picture_in_picture", i18n.t(I18nKey::TrayPictureInPicture))
        .build(app)
    else {
        log::error!("Failed to create tray item 'picture_in_picture'");
        return;
    };

    let Ok(audio_only) = CheckMenuItem::with_id(app, "audio_only", i18n.t(I18nKey::TrayAudioOnly), true, false, None::<&str>)
    else {
        log::error!("Failed to create tray item 'audio_only'");
        return;
    };
    let audio_only_ref = audio_only.clone();

    let sleep_submenu = match builder::build_sleep_submenu(app, i18n) {
        Ok(v) => v,
        Err(msg) => { log::error!("{}", msg); return; }
    };

    let Ok(menu) = MenuBuilder::new(app)
        .item(&show_hide)
        .separator()
        .item(&aot)
        .item(&pip)
        .item(&audio_only)
        .separator()
        .item(&eq_submenu)
        .separator()
        .item(&sleep_submenu)
        .separator()
        .item(&quit)
        .build()
    else {
        log::error!("Failed to build tray menu");
        return;
    };

    let Some(icon) = app.default_window_icon().cloned() else {
        log::error!("No default window icon");
        return;
    };

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip(i18n.t(I18nKey::AppTooltip))
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => app.exit(0),
            "show_hide" => toggle_window(app),
            "eq_toggle" => {
                let checked = eq_toggle_ref.is_checked().unwrap_or(true);
                toggle_eq(app, checked);
                if !checked {
                    for (item, _) in &preset_clones {
                        let _ = item.set_checked(false);
                    }
                }
                let mut ls = match last.lock() {
                    Ok(guard) => guard,
                    Err(poisoned) => {
                        log::error!("EQ state mutex poisoned, recovering");
                        poisoned.into_inner()
                    }
                };
                ls.enabled = checked;
                eq_state::save(app, &ls);
            }
            id if id.starts_with("eq_preset_") => {
                let idx: usize = id.trim_start_matches("eq_preset_").parse().unwrap_or(0);
                let idx = idx.min(crate::presets::PRESETS.len().saturating_sub(1));
                let preset = &crate::presets::PRESETS[idx];
                apply_preset(app, idx);
                for (item, i) in &preset_clones {
                    let _ = item.set_checked(*i == idx);
                }
                let _ = eq_toggle_ref.set_checked(true);
                let state = EqState {
                    enabled: true,
                    preset_index: Some(idx),
                    bands: preset.bands,
                    preamp: preset.preamp,
                };
                match last.lock() {
                    Ok(mut guard) => *guard = state.clone(),
                    Err(poisoned) => {
                        log::error!("EQ state mutex poisoned, recovering");
                        *poisoned.into_inner() = state.clone();
                    }
                }
                eq_state::save(app, &state);
            }
            "eq_reset" => {
                reset_eq(app);
                for (item, _) in &preset_clones {
                    let _ = item.set_checked(false);
                }
                let _ = eq_toggle_ref.set_checked(true);
                let state = EqState {
                    enabled: true,
                    preset_index: None,
                    bands: [0.0; 10],
                    preamp: 0.0,
                };
                match last.lock() {
                    Ok(mut guard) => *guard = state.clone(),
                    Err(poisoned) => {
                        log::error!("EQ state mutex poisoned, recovering");
                        *poisoned.into_inner() = state.clone();
                    }
                }
                eq_state::save(app, &state);
            }
            "always_on_top" => {
                if let Some(win) = app.get_webview_window("main") {
                    let on = win.is_always_on_top().unwrap_or(false);
                    let _ = win.set_always_on_top(!on);
                    let _ = aot_ref.set_checked(!on);
                }
            }
            "picture_in_picture" => {
                if let Some(win) = app.get_webview_window("main") {
                    if let Ok(url) = win.url() {
                        let _ = tauri::webview::WebviewWindowBuilder::new(
                            app,
                            "pip",
                            WebviewUrl::External(url),
                        )
                        .title("YMusic PiP")
                        .inner_size(480.0, 360.0)
                        .always_on_top(true)
                        .build();
                    } else {
                        let _ = app.notification()
                            .builder()
                            .title("YMusic")
                            .body("No page URL available for PiP")
                            .show();
                    }
                }
            }
            "audio_only" => {
                if let Some(win) = app.get_webview_window("main") {
                    util::eval(&win, "window.__ym.audioOnly.toggle()");
                    let checked = audio_only_ref.is_checked().unwrap_or(false);
                    let _ = audio_only_ref.set_checked(!checked);
                    let status = if !checked { "ON" } else { "OFF" };
                    let _ = app.notification()
                        .builder()
                        .title("Audio Only")
                        .body(format!("Audio only mode {}", status))
                        .show();
                }
            }
            "sleep_15" => { sleep_timer(app, 15); send_sleep_notification(app, 15); }
            "sleep_30" => { sleep_timer(app, 30); send_sleep_notification(app, 30); }
            "sleep_60" => { sleep_timer(app, 60); send_sleep_notification(app, 60); }
            "sleep_off" => {
                if let Ok(mut guard) = SLEEP_CANCEL.lock() {
                    if let Some(cancel) = guard.take() {
                        cancel.store(true, Ordering::Relaxed);
                    }
                }
                let _ = app.notification()
                    .builder()
                    .title("Sleep Timer")
                    .body("Sleep timer cancelled")
                    .show();
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_window(tray.app_handle());
            }
        })
        .build(app);
}
