use tauri::{
    menu::{CheckMenuItem, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

const PRESETS: &[(&str, &str)] = &[
    ("eq_preset_0", "Flat"),
    ("eq_preset_1", "Pop"),
    ("eq_preset_2", "Rock"),
    ("eq_preset_3", "Jazz"),
    ("eq_preset_4", "Classical"),
    ("eq_preset_5", "Bass Booster"),
    ("eq_preset_6", "Treble Boost"),
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

pub fn create_tray(app: &AppHandle) {
    let Ok(show_hide) = MenuItemBuilder::with_id("show_hide", "显示/隐藏").build(app) else {
        eprintln!("[YMusic] Failed to create tray item 'show_hide'");
        return;
    };

    let Ok(quit) = MenuItemBuilder::with_id("quit", "退出").build(app) else {
        eprintln!("[YMusic] Failed to create tray item 'quit'");
        return;
    };

    let Ok(eq_reset) = MenuItemBuilder::with_id("eq_reset", "重置均衡器").build(app) else {
        eprintln!("[YMusic] Failed to create tray item 'eq_reset'");
        return;
    };

    // -- preset check items --
    let mut preset_items = Vec::new();
    for (id, label) in PRESETS {
        match CheckMenuItem::with_id(app, id, *label, true, false, None::<&str>) {
            Ok(item) => preset_items.push(item),
            Err(_) => {
                eprintln!("[YMusic] Failed to create preset '{}'", label);
                return;
            }
        }
    }
    // clones for event handler: (clone, index)
    let preset_clones: Vec<(CheckMenuItem<tauri::Wry>, usize)> = preset_items
        .iter()
        .enumerate()
        .map(|(i, item)| (item.clone(), i))
        .collect();

    let mut pb = SubmenuBuilder::new(app, "预设");
    for item in &preset_items {
        pb = pb.item(item);
    }
    let Ok(presets_submenu) = pb.build() else {
        eprintln!("[YMusic] Failed to build presets submenu");
        return;
    };

    // -- EQ toggle --
    let Ok(eq_toggle) =
        CheckMenuItem::with_id(app, "eq_toggle", "开启均衡器", true, true, None::<&str>)
    else {
        eprintln!("[YMusic] Failed to create tray item 'eq_toggle'");
        return;
    };
    let eq_toggle_ref = eq_toggle.clone();

    let Ok(eq_submenu) = SubmenuBuilder::new(app, "均衡器")
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
        .tooltip("YMusic")
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
                // clear all preset checks when toggling EQ off
                if !checked {
                    for (item, _) in &preset_clones {
                        let _ = item.set_checked(false);
                    }
                }
            }
            id if id.starts_with("eq_preset_") => {
                let idx: usize = id.trim_start_matches("eq_preset_").parse().unwrap_or(0);
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.eval(&format!(
                        "var p=window.__ym_eq_presets[{}];\
                         p&&window.__ym_eq_apply_preset&&window.__ym_eq_apply_preset(p[1],p[2])",
                        idx
                    ));
                }
                // check selected preset, uncheck others
                for (item, i) in &preset_clones {
                    let _ = item.set_checked(*i == idx);
                }
                // ensure EQ is on
                let _ = eq_toggle_ref.set_checked(true);
            }
            "eq_reset" => {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.eval(
                        "window.__ym_eq_apply_preset && \
                         window.__ym_eq_apply_preset([0,0,0,0,0,0,0,0,0,0],0)",
                    );
                }
                // clear all preset checks
                for (item, _) in &preset_clones {
                    let _ = item.set_checked(false);
                }
                let _ = eq_toggle_ref.set_checked(true);
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
