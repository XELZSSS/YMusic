use tauri::WebviewWindow;

pub fn eval(win: &WebviewWindow, js: &str) {
    if let Err(e) = win.eval(js) {
        log::error!("eval failed: {}", e);
    }
}
