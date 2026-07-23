use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

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
        eprintln!("Failed to create tray menu item 'show_hide'");
        return;
    };
    let Ok(quit) = MenuItemBuilder::with_id("quit", "退出").build(app) else {
        eprintln!("Failed to create tray menu item 'quit'");
        return;
    };
    let Ok(menu) = MenuBuilder::new(app)
        .item(&show_hide)
        .separator()
        .item(&quit)
        .build()
    else {
        eprintln!("Failed to build tray menu");
        return;
    };
    let Some(icon) = app.default_window_icon().cloned() else {
        eprintln!("No default window icon configured for tray");
        return;
    };

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip("YMusic")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => app.exit(0),
            "show_hide" => toggle_window(app),
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
