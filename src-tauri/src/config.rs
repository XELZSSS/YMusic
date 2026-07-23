pub const YMUSIC_URL: &str = "https://music.youtube.com";
pub const WINDOW_WIDTH: f64 = 1280.0;
pub const WINDOW_HEIGHT: f64 = 720.0;
pub const WINDOW_MIN_WIDTH: f64 = 854.0;
pub const WINDOW_MIN_HEIGHT: f64 = 480.0;

pub const INJECTED_CSS: &str = include_str!("../../src/assets/injected.css");

pub const WEBVIEW2_ARGS: &str = concat!(
    "--disable-features=AutofillServerCommunication,TranslateUI,MediaRouter,OptimizationHints,CalculateNativeWinOcclusion,EdgeFeedback ",
    "--disable-background-networking --disable-sync --no-pings ",
    "--disable-breakpad --disable-component-update ",
    "--disable-background-timer-throttling --disable-renderer-backgrounding ",
    "--disable-domain-reliability --metrics-recording-only ",
    "--autoplay-policy=no-user-gesture-required ",
    "--js-flags=--scavenger_max_new_space_capacity_mb=8",
);
