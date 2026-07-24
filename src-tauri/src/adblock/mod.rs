    pub fn get_init_scripts() -> &'static [&'static str] {
        &[
            include_str!("../../../src/scripts/core/css-injector.js"),
            include_str!("../../../src/scripts/adblock/api-interceptor.js"),
            include_str!("../../../src/scripts/adblock/dom-remover.js"),
            include_str!("../../../src/scripts/adblock/audio-ad.js"),
        ]
    }
