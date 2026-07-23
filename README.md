<div align="center">

<img src="src-tauri/icons/128x128.png" alt="YMusic" width="128" height="128">

# YMusic

A cross-platform YouTube Music desktop client built with Tauri v2

![Tauri](https://img.shields.io/badge/Tauri-v2-ffc131?logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-1.77.2+-dea584?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-22a6f0)
![Platform](https://img.shields.io/badge/Platform-Windows-0078d4)
[![GitHub](https://img.shields.io/badge/GitHub-XELZSSS/YMusic-181717?logo=github)](https://github.com/XELZSSS/YMusic)

[**English**](./README.md) | [**中文**](./README.zh.md)

</div>

## ✨ Features

- **Ad Blocking** — Multi-layer ad removal
- **Equalizer** — Built-in equalizer with 7 presets and tray control
- **Privacy Enhancements** — Strips CSP headers and removes tracking parameters
- **InnerTube Tweaks** — Masks API requests as Android client
- **System Tray** — Minimizes to tray with full EQ menu
- **Layout Optimization** — Cleaner look with hidden scrollbars
- **Internationalization** — Auto-detects Chinese / English
- **Optimized WebView2** — Disables unnecessary background features

## 🏗️ Architecture

```
YMusic/
├── src/                   # Injection scripts + styles
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Entry point & plugin registration
│   │   ├── window.rs      # Window & script injection
│   │   ├── tray/          # System tray
│   │   ├── eq_state.rs    # EQ persistence
│   │   ├── presets.rs     # EQ presets
│   │   ├── i18n.rs        # i18n
│   │   ├── privacy.rs     # Privacy enhancements
│   │   └── util.rs        # Utilities
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/
├── scripts/build-all.js
├── vite.config.js
└── package.json
```

## 🔧 Injection Scripts & Equalizer & Configuration

| Script | Description |
|---|---|---|
| `css-injector.js` | Injects custom CSS into DOM |
| `api-interceptor.js` | Strips ad fields from JSON API responses |
| `dom-remover.js` | Removes ad DOM elements via MutationObserver |
| `audio-ad.js` | Mutes video during audio ads, auto-skips |
| `unified-fetch.js` | Merged fetch interceptor (tracking cleanup + InnerTube spoofing) |
| `equalizer.js` | Web Audio API 10-band EQ (`window.__ym.eq.*`) |
| `eq-ui.js` | EQ preset UI |

EQ state is persisted via `tauri-plugin-store` and restored on launch. Global shortcut `Ctrl+Shift+E` is registered in Rust via `tauri-plugin-global-shortcut`, works even when the window is hidden.

## 💻 Development

```bash
npm install
npm run dev
```

This starts the Vite dev server on port 1420 and launches the Tauri application

## 📦 Build

```bash
# Build for current architecture (x64)
npm run build

# Build for x86
npm run build:x86

# Build both x64 and x86 in sequence (Windows only)
npm run build:all
```

Windows installer output: `src-tauri/target/release/bundle/nsis/`

## 🛠️ Tech Stack

- **Framework**: [Tauri v2](https://v2.tauri.app) (Rust + WebView2)
- **Frontend**: Vanilla JS injection scripts + Vite (dev server only)
- **Tauri Plugins**: `window-state`, `global-shortcut`, `single-instance`, `store`, `opener`, `notification`, `updater`
- **Dependencies**: `http`, `sys-locale`, `serde`, `serde_json`, `log`, `env_logger`
- **CSP**: Permissive policy allowing `https:`, inline scripts, and media/blob sources

## 📄 License

MIT
