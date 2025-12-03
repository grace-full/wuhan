# 租号玩登录捕获 Demo

使用 **Tauri 2.8 + Vue 3 + Element Plus** 构建的桌面端示例，点击按钮会弹出一个加载
[https://zu.zuhaowan.com](https://zu.zuhaowan.com) 的登录窗口，自动监听登录成功后的 token 及 cookie，
再通过事件回传到主窗口进行展示与复制。

## 技术要点

- **Tauri 2.8**：使用自定义 Rust 命令在运行时创建/管理登录 WebView，并注入脚本抓取凭证。
- **安全通信**：远程页面中的注入脚本只负责收集数据，通过 `invoke -> event` 的链路把结果安全传回主窗口，
  主窗口监听 `login-info` 事件即可获得 token / cookie。
- **Vue 3 + Element Plus**：提供现代化的 UI，包含状态提示、结果展示、历史记录与复制能力。

## 本地运行

1. 安装依赖
   ```bash
   npm install
   ```
2. 安装 Rust 及 Tauri 所需的系统依赖（参见 [官方文档](https://tauri.app/start/prerequisites/)）。
3. 启动开发环境
   ```bash
   npm run tauri dev
   ```

> 可选：运行 `npm run typecheck` 进行前端类型检查。

## 目录结构

```
├── src                # Vue 3 + Element Plus 前端
├── src-tauri          # Tauri 2.8 后端 (Rust)
└── README.md
```

## 工作原理

1. 主窗口点击按钮后，通过 `open_login_window` 命令创建一个加载租号玩官网的 WebView。
2. Rust 端在该 WebView 中注入一段脚本，定时读取 token / cookie 并调用 `report_login_state` 命令。
3. 命令会把信息广播为 `login-info` 事件，同时关闭登录窗口；主窗口监听事件并在 UI 上展示结果。

这样就能覆盖页面跳转、SPA、刷新等各种登录情况，确保能够获取到有效的 token 与 cookie。
