use serde::Serialize;

use crate::i18n::I18nKey;

#[derive(Clone, Serialize)]
pub struct EqPreset {
    pub id: &'static str,
    #[serde(skip)]
    pub name_key: I18nKey,
    pub bands: [f64; 10],
    pub preamp: f64,
}

pub const PRESETS: &[EqPreset] = &[
    EqPreset { id: "eq_preset_0", name_key: I18nKey::TrayPresetFlat,      bands: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], preamp: 0.0 },
    EqPreset { id: "eq_preset_1", name_key: I18nKey::TrayPresetPop,       bands: [-1.0, 3.0, 5.0, 5.0, 3.0, 1.0, 0.0, 1.0, 2.0, 2.0], preamp: 0.0 },
    EqPreset { id: "eq_preset_2", name_key: I18nKey::TrayPresetRock,      bands: [5.0, 4.0, 1.0, -2.0, -3.0, -1.0, 2.0, 4.0, 5.0, 5.0], preamp: -2.0 },
    EqPreset { id: "eq_preset_3", name_key: I18nKey::TrayPresetJazz,      bands: [3.0, 2.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0], preamp: 0.0 },
    EqPreset { id: "eq_preset_4", name_key: I18nKey::TrayPresetClassical, bands: [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, -2.0, -2.0, -3.0], preamp: 0.0 },
    EqPreset { id: "eq_preset_5", name_key: I18nKey::TrayPresetBass,      bands: [7.0, 6.0, 4.0, 2.0, 0.0, -1.0, -3.0, -4.0, -5.0, -6.0], preamp: -4.0 },
    EqPreset { id: "eq_preset_6", name_key: I18nKey::TrayPresetTreble,    bands: [-5.0, -4.0, -3.0, -1.0, 0.0, 2.0, 4.0, 6.0, 7.0, 8.0], preamp: -3.0 },
];
