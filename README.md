# Tauri Auth Capture App

A Tauri + Vue application that demonstrates capturing authentication tokens and cookies from third-party authentication pages using IPC communication.

## Features

- 🔐 **Authentication Window**: Opens a separate window for third-party authentication
- 🎯 **Token Detection**: Automatically detects tokens in URLs (query params and hash fragments)
- 🍪 **Cookie Capture**: Extracts cookies from authenticated pages using JavaScript injection
- 📦 **LocalStorage/SessionStorage**: Captures data from browser storage
- 🔄 **IPC Communication**: Uses Tauri events to send captured data from Rust to Vue
- 📊 **Live Display**: Shows captured tokens and cookies in the main window UI
- 🖥️ **Console Logging**: Logs all captured data to the browser console

## Architecture

### Rust Backend (`src-tauri/src/lib.rs`)

The Rust backend handles:

1. **Window Management**: Creates authentication windows using `WebviewWindowBuilder`
2. **Navigation Monitoring**: Uses `on_navigation` callback to detect URL changes
3. **Token Extraction**: Parses URLs to extract tokens from query parameters and hash fragments
4. **JavaScript Injection**: Executes JavaScript in the webview to capture cookies and storage data
5. **IPC Events**: Emits `auth-data-captured` events to communicate with the Vue frontend

Key components:
- `open_auth_window`: Command to open authentication window
- `setup_navigation_listener`: Sets up URL monitoring
- `extract_token_from_url`: Parses tokens from URLs
- `extract_auth_data_from_window`: Executes JS to capture cookies and storage

### Vue Frontend (`src/App.vue`)

The Vue frontend:

1. **Event Listeners**: Listens for `auth-data-captured` events from Rust
2. **State Management**: Stores captured tokens, cookies, and URLs in reactive refs
3. **UI Display**: Shows captured authentication data in a clean interface
4. **Console Logging**: Logs all captured data for debugging

## How It Works

1. User clicks "Open Authentication Window"
2. Tauri creates a new window pointing to the authentication URL
3. User completes the authentication flow
4. Rust monitors navigation events via `on_navigation`
5. When a success URL is detected (containing "token", "success", "callback"), Rust:
   - Extracts tokens from the URL
   - Injects JavaScript to capture cookies and storage
   - Emits IPC event with captured data
6. Vue receives the event and updates the UI
7. All data is logged to the console

## Setup

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Platform-specific dependencies (see [Tauri Prerequisites](https://tauri.app/start/prerequisites/))

### Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Customization

### Change Authentication URL

Edit `src-tauri/src/lib.rs`, line 29:

```rust
tauri::WebviewUrl::External("https://your-auth-provider.com/login".parse().unwrap()),
```

### Customize Token Detection

Edit the `setup_navigation_listener` function in `src-tauri/src/lib.rs` to match your authentication flow:

```rust
if url_str.contains("your-success-indicator") {
    // Custom logic here
}
```

### Add Custom Token Extraction

Modify the `extract_token_from_url` function to handle your specific token format.

## Security Considerations

⚠️ **Important**: This demo uses `dangerousRemoteDomainIpcAccess` for demonstration purposes. In production:

1. Restrict IPC access to specific domains
2. Validate all captured data
3. Use secure token storage
4. Implement proper CORS policies
5. Never log sensitive tokens in production

## Technologies Used

- **Tauri 2.0**: Native app framework
- **Vue 3**: Frontend framework
- **TypeScript**: Type-safe JavaScript
- **Vite**: Build tool
- **tauri-plugin-http**: HTTP client for Tauri

## API Reference

### Tauri Commands

#### `open_auth_window()`
Opens a new authentication window.

**Returns**: `Promise<void>`

#### `extract_auth_data_from_window(window)`
Manually extracts authentication data from a specific window.

**Parameters**:
- `window: WebviewWindow` - The window to extract data from

**Returns**: `Promise<String>`

### Events

#### `auth-data-captured`
Emitted when authentication data is captured.

**Payload**:
```typescript
{
  token: string | null;
  cookies: Array<{ name: string; value: string }>;
  url: string;
}
```

## Troubleshooting

### Tokens not captured
- Check browser console for errors
- Verify the authentication URL is correct
- Ensure the success URL matches the detection patterns

### Cookies not appearing
- Some auth flows use tokens instead of cookies
- Check if the site uses HttpOnly cookies (not accessible via JavaScript)
- Verify CORS and security policies

### Window doesn't open
- Check Rust console for errors
- Ensure the URL is valid and accessible
- Verify network connectivity

## License

MIT

## Contributing

Contributions welcome! Please open an issue or PR.
