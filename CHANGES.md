# 修复说明 - Tauri 登录 Token 捕获

## 修复的问题

### 1. ✅ 权限配置不足
**问题**: `event.listen not allowed` 错误

**解决方案**:
- 在 `tauri.conf.json` 中添加了完整的 capabilities 配置
- 为主窗口 (main) 添加了以下权限:
  - `core:event:allow-listen` - 监听事件
  - `core:event:allow-emit` - 发射事件
  - `core:event:allow-emit-to` - 向特定窗口发射事件
  - `core:window:allow-create` - 创建窗口
  - `core:window:allow-*` - 其他窗口操作权限
- 为登录窗口 (login) 添加了最小必需权限
- 添加了 CSP 中的 `upgrade-insecure-requests` 指令

### 2. ✅ 子窗口脚本注入错误
**问题**: `window.__TAURI_INVOKE__ is not a function` 错误

**根本原因**: 
- 外部 URL (如 https://zu.zuhaowan.com) 加载的页面无法直接访问 Tauri API
- `window.__TAURI_INVOKE__` 只在 Tauri 提供的页面中可用

**解决方案**:
1. **移除了所有 `window.__TAURI_INVOKE__` 调用**
2. **采用新的数据传递机制**:
   - JavaScript 将捕获的数据存储在 `window.__TAURI_LOGIN_CAPTURE__` 对象中
   - JavaScript 将数据序列化并存储在临时 meta 标签中
   - JavaScript 将数据编码到 `document.title` 中 (格式: `DATA:{"token":"..."}`)
   - Rust 通过 `window.title()` 读取数据
   - Rust 解析数据并通过 `emit()` 发送到主窗口

### 3. ✅ 资源加载和 Mixed Content
**解决方案**:
- 在 CSP 中添加了 `upgrade-insecure-requests` 指令
- 配置了完整的 CSP 策略，允许加载外部资源

## 技术实现细节

### 数据捕获流程

```
1. 用户点击"打开登录窗口"按钮
   ↓
2. Rust 创建登录子窗口，加载外部 URL
   ↓
3. Rust 注入 JavaScript 捕获脚本
   ↓
4. JavaScript 拦截 localStorage/sessionStorage 的 setItem 操作
   ↓
5. JavaScript 定期扫描存储和 cookies
   ↓
6. 发现 token/auth 数据时，存储在 window.__TAURI_LOGIN_CAPTURE__
   ↓
7. JavaScript 将数据编码到 document.title
   ↓
8. Rust 定期读取窗口标题
   ↓
9. Rust 解析数据并发送到主窗口
   ↓
10. 主窗口显示捕获的数据
```

### 关键技术点

1. **无需 Tauri API 的数据传递**:
   - 使用 `document.title` 作为数据通道
   - Rust 可以通过 `window.title()` 读取标题
   - 格式: `DATA:{"token":"...","cookies":"...","url":"...","timestamp":"..."}`

2. **localStorage/sessionStorage 拦截**:
   ```javascript
   const originalSetItem = localStorage.setItem;
   localStorage.setItem = function(key, value) {
       // 捕获逻辑
       return originalSetItem.apply(this, arguments);
   };
   ```

3. **定期扫描机制**:
   - JavaScript 每 2 秒扫描一次存储
   - Rust 每 2 秒读取一次窗口状态
   - 支持最多 120 次尝试 (4 分钟)

4. **去重机制**:
   - Rust 保存上一次的 token
   - 只有当 token 变化时才发送新数据

## 测试步骤

1. 安装依赖:
   ```bash
   pnpm install
   ```

2. 运行开发模式:
   ```bash
   pnpm tauri:dev
   ```

3. 测试流程:
   - 点击"打开登录窗口"按钮
   - 在登录窗口中完成登录
   - 检查主窗口是否显示捕获的 token 和 cookie
   - 检查控制台日志输出

## 配置文件变更

### tauri.conf.json
- 添加了 `app.security.capabilities` 配置
- 配置了两个 capability: `main-capability` 和 `login-window-capability`
- 更新了 CSP 策略

### src-tauri/src/main.rs
- 移除了 `window.__TAURI_INVOKE__` 调用
- 实现了基于 `document.title` 的数据传递
- 改进了错误处理和日志输出
- 添加了数据去重逻辑

## 兼容性

- ✅ Tauri 2.1+
- ✅ Vue 3.5+
- ✅ Element-Plus 2.8+
- ✅ Windows / macOS / Linux

## 注意事项

1. **外部 URL 限制**: 
   - 外部 URL 无法直接使用 Tauri API
   - 必须通过 DOM 属性 (title, meta 等) 传递数据

2. **数据大小限制**:
   - `document.title` 有长度限制 (通常几千字符)
   - 如果数据过大，可能需要使用其他方法 (如 localStorage)

3. **安全性**:
   - 捕获的数据会在窗口标题中短暂可见
   - 生产环境建议使用更安全的传输方式

4. **Cookie 限制**:
   - HTTP-only cookies 无法通过 JavaScript 访问
   - 只能捕获非 HTTP-only 的 cookies
