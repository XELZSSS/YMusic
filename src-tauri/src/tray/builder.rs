use tauri::{
    menu::{CheckMenuItem, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Runtime,
};

use crate::eq_state::EqState;
use crate::i18n::{I18n, I18nKey};

pub struct EqMenuItems<R: Runtime> {
    pub eq_toggle: CheckMenuItem<R>,
    pub preset_clones: Vec<(CheckMenuItem<R>, usize)>,
    pub eq_submenu: tauri::menu::Submenu<R>,
}

pub fn build_eq_submenu<R: Runtime>(app: &AppHandle<R>, saved: &EqState) -> Result<EqMenuItems<R>, &'static str> {
    let i18n = I18n::new();

    let Ok(eq_reset) = MenuItemBuilder::with_id("eq_reset", i18n.t(I18nKey::TrayResetEq))
        .build(app)
    else {
        return Err("Failed to create tray item 'eq_reset'");
    };

    let mut preset_items = Vec::new();
    for p in crate::presets::PRESETS {
        match CheckMenuItem::with_id(app, &p.id, i18n.t(p.name_key), true, false, None::<&str>) {
            Ok(item) => preset_items.push(item),
            Err(_) => return Err("Failed to create preset item"),
        }
    }
    for (i, item) in preset_items.iter().enumerate() {
        let _ = item.set_checked(saved.preset_index == Some(i));
    }
    let preset_clones: Vec<_> = preset_items
        .iter()
        .enumerate()
        .map(|(i, item)| (item.clone(), i))
        .collect();

    let mut pb = SubmenuBuilder::new(app, i18n.t(I18nKey::TrayPresets));
    for item in &preset_items {
        pb = pb.item(item);
    }
    let Ok(presets_submenu) = pb.build() else {
        return Err("Failed to build presets submenu");
    };

    let Ok(eq_toggle) = CheckMenuItem::with_id(
        app,
        "eq_toggle",
        i18n.t(I18nKey::TrayEnableEq),
        true,
        saved.enabled,
        None::<&str>,
    ) else {
        return Err("Failed to create tray item 'eq_toggle'");
    };

    let Ok(eq_submenu) = SubmenuBuilder::new(app, i18n.t(I18nKey::TrayEqualizer))
        .item(&eq_toggle)
        .separator()
        .item(&presets_submenu)
        .separator()
        .item(&eq_reset)
        .build()
    else {
        return Err("Failed to build EQ submenu");
    };

    Ok(EqMenuItems { eq_toggle, preset_clones, eq_submenu })
}
