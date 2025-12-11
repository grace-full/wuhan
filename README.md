# Electron + Vue3 + Element-Plus 登录捕获演示

该示例演示了如何在 Electron 应用中整合 Vue3 与 Element-Plus，打开第三方登录页面并捕获登录成功后的 token 与 cookie 信息。应用会实时监听导航与 Cookie 变化，并把结果回传到主窗口的 UI 与控制台中。

## 功能亮点

- 🖥️ 主窗口使用 Vue3 + Element-Plus 搭建，提供友好的交互界面
- 🔐 通过 Electron `BrowserWindow` 打开第三方登录页（https://zu.zuhaowan.com）
- 📡 监听 `will-redirect`、`did-navigate`、`did-navigate-in-page`、`did-frame-navigate` 等事件，适配各种跳转方式
- 🍪 通过 `session.cookies` 实时收集 Cookie，自动识别常见 token 字段
- 🔁 捕获的 token、Cookie 列表与最近跳转 URL 会同步打印到 UI 与控制台

## 快速开始

```bash
# 安装依赖
npm install

# 开发模式：启动 Vite 与 Electron
npm run dev

# 构建渲染进程静态资源
npm run build

# 生产模式启动 Electron（需先执行 npm run build）
npm start
```

> 默认开发端口为 `5173`。`npm run dev` 会自动并行启动 Vite 与 Electron，并在渲染进程可用后打开桌面窗口。

## 目录结构

```
├── electron
│   ├── main.cjs        # Electron 主进程，负责窗口与登录捕获逻辑
│   └── preload.cjs     # 通过 contextBridge 暴露 IPC API 给渲染进程
├── src
│   ├── App.vue         # Element-Plus UI，展示 token/cookie 结果
│   ├── main.js         # Vue3 入口
│   └── assets
│       └── main.css    # 全局样式
├── index.html          # Vite 入口文件
├── package.json
└── vite.config.js
```

## 登录捕获说明

1. 在主窗口点击“打开登录窗口”，Electron 会以独立分区打开第三方登录页。
2. 主进程会同时监听：
   - URL 跳转（HTTP 重定向、JS 跳转、Hash 变化等）
   - Session Cookie 变化
3. 只要检测到 URL 或 Cookie 中包含常见 token 字段，即视为成功并回传给渲染进程。
4. 同时会把该域名下的 Cookie 全量输出，便于二次调用接口或调试。

## 注意事项

- 由于示例直接加载外部登录页，首次运行可能需要较长时间完成用户交互。
- 捕获逻辑基于 URL 参数与 Cookie 名称的启发式匹配，如需适配特殊站点可在 `electron/main.cjs` 中扩展 `tokenKeys` 或自定义解析策略。
- 生产模式下请根据实际需求补充代码签名、打包流程等。

祝使用愉快 🎉
