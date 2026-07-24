<div align="center">

<img src="src-tauri/icons/128x128.png" alt="YMusic" width="128" height="128">

# YMusic

基于 Tauri v2 构建的跨平台 YouTube Music 桌面客户端

![Tauri](https://img.shields.io/badge/Tauri-v2-ffc131?logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-1.77.2+-dea584?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-22a6f0)
![Platform](https://img.shields.io/badge/Platform-Windows-0078d4)
[![GitHub](https://img.shields.io/badge/GitHub-XELZSSS/YMusic-181717?logo=github)](https://github.com/XELZSSS/YMusic)

[**English**](./README.md) | [**中文**](./README.zh.md)

</div>

## ✨ 功能特性

- **广告拦截** — 多层广告移除
- **均衡器** — 内置 7 种预设，支持托盘控制
- **隐私增强** — 剥离 CSP 头，清除跟踪参数
- **InnerTube 伪装** — 伪装为 Android 客户端请求
- **系统托盘** — 最小化到托盘，集成均衡器菜单
- **布局优化** — 更简洁的界面
- **国际化** — 自动检测中文/英文
- **优化 WebView2** — 禁用不必要的后台功能

## 🏗️ 项目架构

```
YMusic/
├── src/                   # 注入脚本 + 样式
│   └── scripts/
│       ├── core/          # CSS 注入
│       ├── adblock/       # API/DOM/音频广告拦截
│       ├── privacy/       # 追踪清理、InnerTube 伪装
│       ├── equalizer/     # Web Audio API 10 段均衡器
│       └── audio/         # 纯音频模式、可视化
├── src-tauri/             # Rust 后端
│   └── src/
│       ├── lib.rs         # 应用启动、插件、命令
│       ├── app/           # 配置、工具函数
│       ├── i18n/          # 国际化
│       ├── window/        # 窗口管理
│       ├── privacy/       # CSP 剥离
│       ├── adblock/       # 广告拦截脚本
│       ├── equalizer/     # EQ 状态、预设
│       └── tray/          # 系统托盘
├── scripts/
├── vite.config.js
└── package.json
```

## 🔧 注入脚本 & 均衡器 & 配置

| 模块 | 脚本 | 说明 |
|---|---|---|
| `core/` | `css-injector.js` | 将自定义 CSS 注入 DOM |
| `adblock/` | `api-interceptor.js` | 从 API 响应中清除广告字段 |
| `adblock/` | `dom-remover.js` | 通过 MutationObserver 删除广告元素 |
| `adblock/` | `audio-ad.js` | 音频广告期间静音并自动跳过 |
| `privacy/` | `unified-fetch.js` | 合并的 fetch 拦截器（追踪参数清理 + InnerTube 伪装） |
| `privacy/` | `ytcfg-injector.js` | 修改 ytcfg 开启增强功能 |
| `equalizer/` | `equalizer.js` | Web Audio API 10 段均衡器（`window.__ym.eq.*`） |
| `equalizer/` | `eq-ui.js` | 均衡器预设 UI |
| `audio/` | `audio-only.js` | 纯音频模式（禁用视频流） |
| `audio/` | `visualizer.js` | 音频可视化（`window.__ym.viz.*`） |

均衡器状态通过 `tauri-plugin-store` 持久化，启动时自动恢复。全局快捷键 `Ctrl+Shift+E` 由 Rust 端注册（`tauri-plugin-global-shortcut`），窗口隐藏时也可切换。

## 💻 开发

```bash
npm install
npm run dev
```

启动 Vite 开发服务器（端口 1420）并运行 Tauri 应用

## 📦 构建

```bash
# 构建当前架构（x64）
npm run build

# 构建 x86
npm run build:x86

# 依次构建 x64 和 x86（仅 Windows）
npm run build:all
```

Windows 安装包输出：`src-tauri/target/release/bundle/nsis/`

## 🛠️ 技术栈

- **框架**：[Tauri v2](https://v2.tauri.app)（Rust + WebView2）
- **前端**：纯 JS 注入脚本 + Vite（仅开发服务器）
- **Tauri 插件**：`window-state`、`global-shortcut`、`single-instance`、`store`、`opener`、`notification`、`updater`
- **依赖**：`http`、`sys-locale`、`serde`、`serde_json`、`log`、`env_logger`
- **CSP**：宽松策略，允许 `https:`、内联脚本及媒体/blob 资源

## 📄 许可证

MIT
