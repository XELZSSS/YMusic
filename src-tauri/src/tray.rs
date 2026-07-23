use std::sync::Mutex;
use tauri::{
    menu::{CheckMenuItem, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::eq_state::{self, EqState};
use crate::i18n::I18n;

const PRESETS: &[(&str, &str)] = &[
    ("eq_preset_0", "tray.preset_flat"),
    ("eq_preset_1", "tray.preset_pop"),
    ("eq_preset_2", "tray.preset_rock"),
    ("eq_preset_3", "tray.preset_jazz"),
    ("eq_preset_4", "tray.preset_classical"),
    ("eq_preset_5", "tray.preset_bass"),
    ("eq_preset_6", "tray.preset_treble"),
];

// 10-band preset values in the same order as EQ_FREQ in equalizer.js
const PRESET_BANDS: &[([f64; 10], f64)] = &[
    ([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0.0),   // Flat
    ([-1.0, 2.0, 3.0, 4.0, 5.0, 3.0, 2.0, 1.0, 0.0, -1.0], 0.0), // Pop
    ([4.0, 3.0, 2.0, 1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0], 0.0),   // Rock
    ([3.0, 2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 3.0], 0.0),   // Jazz
    ([4.0, 3.0, 2.0, 1.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0], -2.0),  // Classical
    ([6.0, 5.0, 4.0, 2.0, 0.0, -1.0, -2.0, -3.0, -4.0, -5.0], -3.0), // Bass Booster
    ([-5.0, -4.0, -3.0, -2.0, 0.0, 2.0, 4.0, 5.0, 6.0, 6.0], -3.0),   // Treble Boost
];

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

pub fn create_tray(app: &AppHandle, saved: &EqState) {
    let i18n = I18n::new();
    let last = Mutex::new(saved.clone());

    let Ok(show_hide) = MenuItemBuilder::with_id("show_hide", i18n.t("tray.show_hide"))
        .build(app)
    else {
        eprintln!("[YMusic] Failed to create tray item 'show_hide'");
        return;
    };

    let Ok(quit) = MenuItemBuilder::with_id("quit", i18n.t("tray.quit"))
        .build(app)
    else {
        eprintln!("[YMusic] Failed to create tray item 'quit'");
        return;
    };

    let Ok(eq_reset) = MenuItemBuilder::with_id("eq_reset", i18n.t("tray.reset_eq"))
        .build(app)
    else {
        eprintln!("[YMusic] Failed to create tray item 'eq_reset'");
        return;
    };

    let mut preset_items = Vec::new();
    for (id, key) in PRESETS {
        match CheckMenuItem::with_id(app, id, i18n.t(key), true, false, None::<&str>) {
            Ok(item) => preset_items.push(item),
            Err(_) => {
                eprintln!("[YMusic] Failed to create preset '{}'", key);
                return;
            }
        }
    }
    // Restore preset check state from saved state
    for (i, item) in preset_items.iter().enumerate() {
        let _ = item.set_checked(saved.preset_index == Some(i));
    }
    let preset_clones: Vec<_> = preset_items
        .iter()
        .enumerate()
        .map(|(i, item)| (item.clone(), i))
        .collect();

    let mut pb = SubmenuBuilder::new(app, i18n.t("tray.presets"));
    for item in &preset_items {
        pb = pb.item(item);
    }
    let Ok(presets_submenu) = pb.build() else {
        eprintln!("[YMusic] Failed to build presets submenu");
        return;
    };

    let Ok(eq_toggle) = CheckMenuItem::with_id(
        app,
        "eq_toggle",
        i18n.t("tray.enable_eq"),
        true,
        saved.enabled,
        None::<&str>,
    )
    else {
        eprintln!("[YMusic] Failed to create tray item 'eq_toggle'");
        return;
    };
    let eq_toggle_ref = eq_toggle.clone();

    let Ok(eq_submenu) = SubmenuBuilder::new(app, i18n.t("tray.equalizer"))
        .item(&eq_toggle)
        .separator()
        .item(&presets_submenu)
        .separator()
        .item(&eq_reset)
        .build()
    else {
        eprintln!("[YMusic] Failed to build EQ submenu");
        return;
    };

    let Ok(menu) = MenuBuilder::new(app)
        .item(&show_hide)
        .separator()
        .item(&eq_submenu)
        .separator()
        .item(&quit)
        .build()
    else {
        eprintln!("[YMusic] Failed to build tray menu");
        return;
    };

    let Some(icon) = app.default_window_icon().cloned() else {
        eprintln!("[YMusic] No default window icon");
        return;
    };

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip(i18n.t("app.tooltip"))
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => app.exit(0),
            "show_hide" => toggle_window(app),
            "eq_toggle" => {
                let checked = eq_toggle_ref.is_checked().unwrap_or(true);
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.eval(&format!(
                        "window.__ym_eq_toggle && window.__ym_eq_toggle({})",
                        checked
                    ));
                }
                if !checked {
                    for (item, _) in &preset_clones {
                        let _ = item.set_checked(false);
                    }
                }
                // Persist: keep bands/preamp from last known state, update enabled
                let mut ls = last.lock().unwrap();
                ls.enabled = checked;
                eq_state::save(app, &ls);
            }
            id if id.starts_with("eq_preset_") => {
                let idx: usize = id.trim_start_matches("eq_preset_").parse().unwrap_or(0);
                let bands = PRESET_BANDS[idx].0;
                let preamp = PRESET_BANDS[idx].1;
                if let Some(win) = app.get_webview_window("main") {
                    let bands_str = bands.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(",");
                    let _ = win.eval(&format!(
                        "window.__ym_eq_apply_preset && window.__ym_eq_apply_preset([{}],{})",
                        bands_str, preamp
                    ));
                }
                for (item, i) in &preset_clones {
                    let _ = item.set_checked(*i == idx);
                }
                let _ = eq_toggle_ref.set_checked(true);
                // Persist
                let state = EqState {
                    enabled: true,
                    preset_index: Some(idx),
                    bands,
                    preamp,
                };
                *last.lock().unwrap() = state.clone();
                eq_state::save(app, &state);
            }
            "eq_reset" => {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.eval(
                        "window.__ym_eq_apply_preset && \
                         window.__ym_eq_apply_preset([0,0,0,0,0,0,0,0,0,0],0)",
                    );
                }
                for (item, _) in &preset_clones {
                    let _ = item.set_checked(false);
                }
                let _ = eq_toggle_ref.set_checked(true);
                // Persist
                let state = EqState {
                    enabled: true,
                    preset_index: None,
                    bands: [0.0; 10],
                    preamp: 0.0,
                };
                *last.lock().unwrap() = state.clone();
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
