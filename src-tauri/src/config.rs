pub const YMUSIC_URL: &str = "https://music.youtube.com";
pub const WINDOW_TITLE: &str = "YMusic";
pub const WINDOW_WIDTH: f64 = 1200.0;
pub const WINDOW_HEIGHT: f64 = 800.0;
pub const WINDOW_MIN_WIDTH: f64 = 800.0;
pub const WINDOW_MIN_HEIGHT: f64 = 600.0;

pub const INJECTED_CSS: &str = include_str!("../../src/assets/injected.css");

pub const WEBVIEW2_ARGS: &str = concat!(
    "--disable-features=AutofillServerCommunication,TranslateUI,MediaRouter,OptimizationHints ",
    "--disable-background-networking --disable-sync --no-pings ",
    "--disable-breakpad --disable-component-update",
);
