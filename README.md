<div align="center">

<img src="src-tauri/icons/128x128.png" alt="YMusic" width="128" height="128">

# YMusic

A cross-platform YouTube Music desktop client built with Tauri v2.

</div>

## Features

- **Ad blocking** — Injects scripts to block YouTube Music ads
- **Tracking param cleanup** — Automatically strips tracking parameters from URLs

## Development

```bash
npm install
npm run dev
```

## Build

```bash
npm run build
```

Windows installer output: `src-tauri/target/release/bundle/nsis/`

## Tech Stack

- **Framework**: [Tauri v2](https://v2.tauri.app) (Rust + WebView2)
- **Frontend**: Vanilla HTML + Vite (dev server only)
- **Plugin**: `tauri-plugin-window-state`

## License

MIT
