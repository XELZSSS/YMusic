<div align="center">

<img src="src-tauri/icons/128x128.png" alt="YMusic" width="128" height="128">

# YMusic

A cross-platform YouTube Music desktop client built with Tauri v2

![Tauri](https://img.shields.io/badge/Tauri-v2-ffc131?logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-1.70+-dea584?logo=rust&logoColor=white)
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
├── src/                   # Frontend (injection scripts + styles)
├── src-tauri/src/         # Rust backend
│   ├── lib.rs             # App setup & command handlers
│   ├── config.rs          # Constants (URL, window size, etc.)
│   ├── window.rs          # Window creation & script injection
│   ├── tray.rs            # System tray menu & EQ control
│   ├── eq_state.rs        # EQ state persistence
│   ├── i18n.rs            # Internationalization
│   └── privacy.rs         # CSP stripping & CSS injection
├── scripts/build-all.js   # Batch build script
├── vite.config.js         # Vite dev server config
└── package.json           # npm scripts & dependencies
```

## 🔧 Injection Scripts & Equalizer & Configuration

| Script | Description |
|---|---|
| `css-injector.js` | Injects custom CSS into DOM |
| `api-interceptor.js` | Strips ad fields from JSON API responses |
| `dom-remover.js` | Removes ad DOM elements via MutationObserver |
| `audio-ad.js` | Mutes video during audio ads, auto-skips |
| `tracking-cleaner.js` | Strips tracking params from URLs |
| `innertube-tweaks.js` | Spoofs Android client for InnerTube API |
| `equalizer.js` | Web Audio API EQ core |
| `eq-ui.js` | EQ presets + Ctrl+Shift+E shortcut |

EQ state is persisted to `eq_state.json` and restored on launch Key config in `src-tauri/src/config.rs`

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
- **Dependencies**: `http`, `sys-locale`, `serde`, `serde_json`
- **CSP**: Permissive policy allowing `https:`, inline scripts, and media/blob sources

## 📄 License

MIT
