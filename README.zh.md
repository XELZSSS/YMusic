<div align="center">

<img src="src-tauri/icons/128x128.png" alt="YMusic" width="128" height="128">

# YMusic

基于 Tauri v2 构建的跨平台 YouTube Music 桌面客户端

![Tauri](https://img.shields.io/badge/Tauri-v2-ffc131?logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-1.70+-dea584?logo=rust&logoColor=white)
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
├── src/                   # 前端（注入脚本 + 样式）
├── src-tauri/src/         # Rust 后端
│   ├── lib.rs             # 应用设置 & 命令处理器
│   ├── config.rs          # 常量配置（URL、窗口大小等）
│   ├── window.rs          # 窗口创建 & 脚本注入
│   ├── tray.rs            # 系统托盘 & 均衡器控制
│   ├── eq_state.rs        # EQ 状态持久化
│   ├── i18n.rs            # 国际化
│   └── privacy.rs         # 剥离 CSP & 注入 CSS
├── scripts/build-all.js   # 批量构建脚本
├── vite.config.js         # Vite 开发服务器配置
└── package.json           # npm 脚本 & 依赖
```

## 🔧 注入脚本 & 均衡器 & 配置

| 脚本 | 说明 |
|---|---|
| `css-injector.js` | 将自定义 CSS 注入 DOM |
| `api-interceptor.js` | 从 API 响应中清除广告字段 |
| `dom-remover.js` | 通过 MutationObserver 删除广告元素 |
| `audio-ad.js` | 音频广告期间静音并自动跳过 |
| `tracking-cleaner.js` | 从 URL 中清除跟踪参数 |
| `innertube-tweaks.js` | 伪装为 Android 客户端请求 |
| `equalizer.js` | Web Audio API 均衡器 |
| `eq-ui.js` | 预设定义 + Ctrl+Shift+E 快捷键 |

均衡器状态持久化到 `eq_state.json`，启动自动恢复关键配置在 `src-tauri/src/config.rs`

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
- **依赖**：`http`、`sys-locale`、`serde`、`serde_json`
- **CSP**：宽松策略，允许 `https:`、内联脚本及媒体/blob 资源

## 📄 许可证

MIT
