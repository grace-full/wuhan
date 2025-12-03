# Tauri 登录 Token/Cookie 捕获演示

这是一个完整的 Tauri 2.8 + Vue3 + Element-Plus 应用，展示如何创建登录窗口并捕获第三方登录页面的 token 和 cookie。

## 功能特性

- ✅ 使用 Element-Plus 构建的现代化 UI 界面
- ✅ 点击按钮弹出登录子窗口
- ✅ 加载第三方登录页面 (https://zu.zuhaowan.com)
- ✅ 自动捕获 localStorage、sessionStorage 和 cookies
- ✅ 拦截登录成功后的 token
- ✅ 通过 IPC 事件将数据传回主窗口
- ✅ 在 UI 和控制台中显示捕获的数据
- ✅ 支持多种登录跳转方式（JS 跳转、HTTP 重定向等）

## 技术栈

- **前端**: Vue 3.5 + Element-Plus 2.8
- **桌面框架**: Tauri 2.1
- **构建工具**: Vite 5.4
- **后端**: Rust

## 安装依赖

确保您已安装以下工具：

- Node.js (推荐 18+)
- pnpm (或 npm/yarn)
- Rust (推荐最新稳定版)

```bash
# 安装前端依赖
pnpm install
```

## 运行项目

### 开发模式

```bash
pnpm tauri dev
```

### 构建生产版本

```bash
pnpm tauri build
```

## 使用说明

1. 启动应用后，会看到一个带有"打开登录窗口"按钮的主界面
2. 点击按钮，会弹出一个新窗口加载第三方登录页面
3. 在登录窗口中完成登录操作
4. 应用会自动捕获登录过程中的 token、cookie 等数据
5. 捕获的数据会实时显示在主窗口的 UI 中，同时也会在控制台打印

## 工作原理

### Cookie/Token 捕获机制

应用使用多种技术来确保可靠地捕获登录数据：

1. **JavaScript 注入**: 在登录窗口中注入脚本，拦截 localStorage、sessionStorage 的写入操作
2. **定时轮询**: 每 2-3 秒检查一次存储和 cookie 的变化
3. **导航监听**: 监听页面导航事件，在每次页面跳转时尝试捕获数据
4. **网络拦截**: 拦截 fetch 和 XMLHttpRequest 请求，记录响应数据

### IPC 通信

- 主窗口通过 `invoke('open_login_window')` 创建登录窗口
- 登录窗口通过 `invoke('capture_login_data')` 将捕获的数据发送到 Rust 后端
- Rust 后端通过 `emit('login-data-captured')` 事件将数据发送回主窗口
- 主窗口监听该事件并更新 UI

### 跨域和安全配置

- CSP (Content Security Policy) 配置允许加载外部资源
- 登录窗口可以正常访问第三方网站
- Tauri 的安全策略确保只有授权的 IPC 调用可以执行

## 项目结构

```
.
├── src/                    # 前端源代码
│   ├── App.vue            # 主应用组件
│   └── main.js            # 入口文件
├── src-tauri/             # Tauri 后端代码
│   ├── src/
│   │   └── main.rs        # Rust 主程序
│   ├── icons/             # 应用图标
│   ├── Cargo.toml         # Rust 依赖配置
│   ├── tauri.conf.json    # Tauri 配置
│   └── build.rs           # 构建脚本
├── index.html             # HTML 模板
├── vite.config.js         # Vite 配置
├── package.json           # Node.js 依赖
└── README.md             # 项目文档
```

## 自定义配置

### 修改登录 URL

编辑 `src-tauri/src/main.rs` 文件，找到 `open_login_window` 函数，修改 URL：

```rust
WebviewUrl::External("https://your-login-url.com".parse().unwrap())
```

### 调整捕获策略

在 `src-tauri/src/main.rs` 中可以调整：

- 轮询间隔时间（默认 2 秒）
- 最大检查次数（默认 60 次）
- Token 关键字匹配规则

## 常见问题

### Q: 为什么捕获不到数据？

A: 可能的原因：
- 登录页面使用了特殊的存储方式
- Token 存储在 HTTP-only cookie 中（无法通过 JavaScript 访问）
- 需要调整关键字匹配规则

### Q: 如何查看详细的调试信息？

A: 在开发模式下运行时，打开浏览器开发者工具（主窗口和登录窗口都可以），查看控制台输出。

### Q: 支持哪些平台？

A: 支持 Windows、macOS 和 Linux。

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
