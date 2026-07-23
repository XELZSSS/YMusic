mod builder;

use std::sync::Mutex;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::eq_state::{self, EqState};
use crate::i18n::{I18n, I18nKey};
use crate::util;

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

    let Ok(menu) = MenuBuilder::new(app)
        .item(&show_hide)
        .separator()
        .item(&eq_submenu)
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
