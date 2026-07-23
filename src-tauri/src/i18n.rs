#[derive(Clone, Copy)]
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
}

impl I18n {
    pub fn new() -> Self {
        let locale = sys_locale::get_locale().unwrap_or_default();
        let lang = if locale.starts_with("zh") { "zh" } else { "en" };
        I18n { lang: lang.to_string() }
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn t(&self, key: I18nKey) -> &'static str {
        use I18nKey::*;
        let is_zh = self.lang == "zh";
        match key {
            AppWindowTitle | AppTooltip => "YMusic",
            TrayShowHide => if is_zh { "显示/隐藏" } else { "Show/Hide" },
            TrayEqualizer => if is_zh { "均衡器" } else { "Equalizer" },
            TrayEnableEq => if is_zh { "开启均衡器" } else { "Enable EQ" },
            TrayPresets => if is_zh { "预设" } else { "Presets" },
            TrayResetEq => if is_zh { "重置均衡器" } else { "Reset EQ" },
            TrayQuit => if is_zh { "退出" } else { "Quit" },
            TrayPresetFlat => if is_zh { "平坦" } else { "Flat" },
            TrayPresetPop => if is_zh { "流行" } else { "Pop" },
            TrayPresetRock => if is_zh { "摇滚" } else { "Rock" },
            TrayPresetJazz => if is_zh { "爵士" } else { "Jazz" },
            TrayPresetClassical => if is_zh { "古典" } else { "Classical" },
            TrayPresetBass => if is_zh { "低音增强" } else { "Bass Booster" },
            TrayPresetTreble => if is_zh { "高音增强" } else { "Treble Boost" },
        }
    }
}
