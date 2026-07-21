use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

fn toggle_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            let _ = win.show();
        }
    }
}

pub fn create_tray(app: &AppHandle) {
    let show_hide = match MenuItemBuilder::with_id("show_hide", "显示/隐藏").build(app) {
        Ok(item) => item,
        Err(e) => {
            eprintln!("Failed to create show_hide menu item: {e}");
            return;
        }
    };
    let quit = match MenuItemBuilder::with_id("quit", "退出").build(app) {
        Ok(item) => item,
        Err(e) => {
            eprintln!("Failed to create quit menu item: {e}");
            return;
        }
    };

    let menu = match MenuBuilder::new(app)
        .item(&show_hide)
        .separator()
        .item(&quit)
        .build()
    {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to build tray menu: {e}");
            return;
        }
    };

    let icon = match app.default_window_icon() {
        Some(icon) => icon.clone(),
        None => {
            eprintln!("No default window icon configured for tray");
            return;
        }
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
