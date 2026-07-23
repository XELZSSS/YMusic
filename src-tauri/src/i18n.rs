use std::collections::HashMap;

pub struct I18n {
    lang: String,
    strings: HashMap<&'static str, &'static str>,
}

impl I18n {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().unwrap_or_default();
        let lang = if locale.starts_with("zh") { "zh" } else { "en" };
        let strings = match lang {
            "zh" => zh_map(),
            _ => en_map(),
        };
        I18n { lang: lang.to_string(), strings }
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn t(&self, key: &'static str) -> &str {
        self.strings.get(key).copied().unwrap_or(key)
    }
}

fn en_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("app.window_title", "YMusic");
    m.insert("app.tooltip", "YMusic");
    m.insert("tray.show_hide", "Show/Hide");
    m.insert("tray.equalizer", "Equalizer");
    m.insert("tray.enable_eq", "Enable EQ");
    m.insert("tray.presets", "Presets");
    m.insert("tray.reset_eq", "Reset EQ");
    m.insert("tray.quit", "Quit");
    m.insert("tray.preset_flat", "Flat");
    m.insert("tray.preset_pop", "Pop");
    m.insert("tray.preset_rock", "Rock");
    m.insert("tray.preset_jazz", "Jazz");
    m.insert("tray.preset_classical", "Classical");
    m.insert("tray.preset_bass", "Bass Booster");
    m.insert("tray.preset_treble", "Treble Boost");
    m
}

fn zh_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("app.window_title", "YMusic");
    m.insert("app.tooltip", "YMusic");
    m.insert("tray.show_hide", "显示/隐藏");
    m.insert("tray.equalizer", "均衡器");
    m.insert("tray.enable_eq", "开启均衡器");
    m.insert("tray.presets", "预设");
    m.insert("tray.reset_eq", "重置均衡器");
    m.insert("tray.quit", "退出");
    m.insert("tray.preset_flat", "平坦");
    m.insert("tray.preset_pop", "流行");
    m.insert("tray.preset_rock", "摇滚");
    m.insert("tray.preset_jazz", "爵士");
    m.insert("tray.preset_classical", "古典");
    m.insert("tray.preset_bass", "低音增强");
    m.insert("tray.preset_treble", "高音增强");
    m
}
