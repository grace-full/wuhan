# Tauri 登录捕获 - 安装和配置指南

## 前置要求

### 1. Node.js 和包管理器
```bash
# 检查 Node.js 版本（需要 18+ ）
node --version

# 安装 pnpm（推荐）
npm install -g pnpm

# 或使用 npm（也可以）
npm --version
```

### 2. Rust
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 加载 Rust 环境
source "$HOME/.cargo/env"

# 验证安装
rustc --version
cargo --version
```

### 3. 系统依赖（Linux）
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 快速开始

### 1. 安装项目依赖
```bash
# 使用 pnpm（推荐）
pnpm install

# 或使用 npm
npm install
```

### 2. 运行开发模式
```bash
# 使用 pnpm
pnpm tauri dev

# 或使用 npm
npm run tauri:dev
```

### 3. 构建生产版本
```bash
# 使用 pnpm
pnpm tauri build

# 或使用 npm
npm run tauri:build
```

## 项目文件说明

### 核心文件
- `src/App.vue` - 主界面组件
- `src/main.js` - Vue 应用入口
- `src-tauri/src/main.rs` - Rust 后端逻辑
- `src-tauri/tauri.conf.json` - Tauri 配置

### 配置文件
- `package.json` - Node.js 依赖和脚本
- `vite.config.js` - Vite 构建配置
- `src-tauri/Cargo.toml` - Rust 依赖

### 资源文件
- `src-tauri/icons/` - 应用图标（PNG、ICO、ICNS）

## 常见问题排查

### 问题 1: 找不到 pnpm 命令
```bash
npm install -g pnpm
```

### 问题 2: Rust 命令找不到
```bash
source "$HOME/.cargo/env"
```

### 问题 3: 系统库缺失
参考上方"系统依赖"部分，安装所需的库。

### 问题 4: 图标错误
确保 `src-tauri/icons/` 目录下的 PNG 文件是 RGBA 格式。

### 问题 5: 端口被占用
Vite 默认使用端口 1420。如果被占用，可以修改 `vite.config.js` 中的端口配置。

## 开发提示

### 查看日志
- 开发模式下，Rust 日志会在终端显示
- 浏览器控制台显示前端日志
- 打开开发者工具: 主窗口和登录窗口都可以右键 -> 检查

### 调试登录捕获
1. 打开主窗口的开发者工具（控制台）
2. 点击"打开登录窗口"
3. 在登录窗口中完成登录
4. 观察控制台输出的捕获数据
5. 数据会自动显示在主窗口 UI 中

### 修改登录 URL
编辑 `src-tauri/src/main.rs`，找到第 22 行:
```rust
WebviewUrl::External("https://zu.zuhaowan.com".parse().unwrap())
```
替换为你需要的 URL。

### 调整捕获策略
在 `src-tauri/src/main.rs` 中:
- 第 39 行: 轮询间隔（当前 3 秒）
- 第 41 行: 最大尝试次数（当前 60 次）
- 第 69-72、84-87 行: Token 关键字匹配规则

## 技术架构

### 窗口通信流程
1. 主窗口调用 `invoke('open_login_window')` 创建登录窗口
2. 登录窗口注入 JavaScript 拦截脚本
3. 定时轮询检查 localStorage、sessionStorage 和 cookies
4. 发现 Token/Cookie 后调用 `invoke('capture_login_data')`
5. Rust 后端通过 `emit('login-data-captured')` 发送到主窗口
6. 主窗口监听事件并更新 UI

### 安全配置
- CSP 允许加载外部资源（用于登录页）
- IPC 通信使用 Tauri 的安全机制
- 仅授权的命令可以被前端调用

## 下一步

- 根据实际需求调整 Token 捕获规则
- 自定义 UI 样式和交互
- 添加数据持久化存储
- 实现更复杂的登录流程处理

## 支持

如有问题，请查看:
- [Tauri 官方文档](https://tauri.app)
- [Vue 3 文档](https://vuejs.org)
- [Element-Plus 文档](https://element-plus.org)
