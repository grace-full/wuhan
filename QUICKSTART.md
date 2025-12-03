# 快速启动指南

## 一键运行

```bash
# 1. 安装依赖
pnpm install

# 2. 运行开发模式
pnpm tauri dev
```

就这么简单！🎉

## 使用说明

1. 应用启动后，你会看到主窗口
2. 点击 **"打开登录窗口"** 按钮
3. 在弹出的登录窗口中访问 https://zu.zuhaowan.com
4. 完成登录操作
5. Token 和 Cookie 会自动捕获并显示在主窗口

## 查看捕获的数据

- **UI 显示**: 主窗口会实时显示捕获的 Token、Cookie 和完整数据
- **控制台输出**: 按 F12 打开开发者工具，查看详细日志

## 常见命令

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build

# 只构建前端
pnpm build

# 验证项目
./verify-project.sh
```

## 如果 pnpm 不可用

```bash
# 安装 pnpm
npm install -g pnpm

# 或使用 npm 代替
npm install
npm run tauri:dev
```

## 需要帮助？

- 📖 查看 [README.md](README.md) 了解项目详情
- 🔧 查看 [SETUP.md](SETUP.md) 了解完整安装指南
- 📋 查看 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解技术实现

## 快速定制

### 修改登录 URL

编辑 `src-tauri/src/main.rs` 第 22 行:

```rust
WebviewUrl::External("https://your-url.com".parse().unwrap())
```

### 调整捕获间隔

编辑 `src-tauri/src/main.rs` 第 39 行:

```rust
let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));
```

### 修改主窗口标题

编辑 `src-tauri/tauri.conf.json` 第 10 行:

```json
"title": "你的应用标题"
```

---

**祝你使用愉快！** 🚀
