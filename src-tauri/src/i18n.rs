use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum I18nKey {
    AppWindowTitle,
    AppTooltip,
    TrayShowHide,
    TrayEqualizer,
    TrayEnableEq,
    TrayPresets,
    TrayResetEq,
    TrayQuit,
    TrayPresetFlat,
    TrayPresetPop,
    TrayPresetRock,
    TrayPresetJazz,
    TrayPresetClassical,
    TrayPresetBass,
    TrayPresetTreble,
}

pub struct I18n {
    lang: String,
    strings: HashMap<I18nKey, &'static str>,
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

    pub fn t(&self, key: I18nKey) -> &str {
        self.strings.get(&key).copied().unwrap_or(match key {
            I18nKey::AppWindowTitle => "YMusic",
            I18nKey::AppTooltip => "YMusic",
            I18nKey::TrayShowHide => "Show/Hide",
            I18nKey::TrayEqualizer => "Equalizer",
            I18nKey::TrayEnableEq => "Enable EQ",
            I18nKey::TrayPresets => "Presets",
            I18nKey::TrayResetEq => "Reset EQ",
            I18nKey::TrayQuit => "Quit",
            I18nKey::TrayPresetFlat => "Flat",
            I18nKey::TrayPresetPop => "Pop",
            I18nKey::TrayPresetRock => "Rock",
            I18nKey::TrayPresetJazz => "Jazz",
            I18nKey::TrayPresetClassical => "Classical",
            I18nKey::TrayPresetBass => "Bass Booster",
            I18nKey::TrayPresetTreble => "Treble Boost",
        })
    }
}

fn en_map() -> HashMap<I18nKey, &'static str> {
    let mut m = HashMap::new();
    m.insert(I18nKey::AppWindowTitle, "YMusic");
    m.insert(I18nKey::AppTooltip, "YMusic");
    m.insert(I18nKey::TrayShowHide, "Show/Hide");
    m.insert(I18nKey::TrayEqualizer, "Equalizer");
    m.insert(I18nKey::TrayEnableEq, "Enable EQ");
    m.insert(I18nKey::TrayPresets, "Presets");
    m.insert(I18nKey::TrayResetEq, "Reset EQ");
    m.insert(I18nKey::TrayQuit, "Quit");
    m.insert(I18nKey::TrayPresetFlat, "Flat");
    m.insert(I18nKey::TrayPresetPop, "Pop");
    m.insert(I18nKey::TrayPresetRock, "Rock");
    m.insert(I18nKey::TrayPresetJazz, "Jazz");
    m.insert(I18nKey::TrayPresetClassical, "Classical");
    m.insert(I18nKey::TrayPresetBass, "Bass Booster");
    m.insert(I18nKey::TrayPresetTreble, "Treble Boost");
    m
}

fn zh_map() -> HashMap<I18nKey, &'static str> {
    let mut m = HashMap::new();
    m.insert(I18nKey::AppWindowTitle, "YMusic");
    m.insert(I18nKey::AppTooltip, "YMusic");
    m.insert(I18nKey::TrayShowHide, "显示/隐藏");
    m.insert(I18nKey::TrayEqualizer, "均衡器");
    m.insert(I18nKey::TrayEnableEq, "开启均衡器");
    m.insert(I18nKey::TrayPresets, "预设");
    m.insert(I18nKey::TrayResetEq, "重置均衡器");
    m.insert(I18nKey::TrayQuit, "退出");
    m.insert(I18nKey::TrayPresetFlat, "平坦");
    m.insert(I18nKey::TrayPresetPop, "流行");
    m.insert(I18nKey::TrayPresetRock, "摇滚");
    m.insert(I18nKey::TrayPresetJazz, "爵士");
    m.insert(I18nKey::TrayPresetClassical, "古典");
    m.insert(I18nKey::TrayPresetBass, "低音增强");
    m.insert(I18nKey::TrayPresetTreble, "高音增强");
    m
}
