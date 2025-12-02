# Zuhaowan App

A Tauri application with a dedicated login window for https://zu.zuhaowan.com

## Features

- Main application window
- Separate login window for Zuhaowan authentication
- Window lifecycle management (open/close/reuse)
- Custom user agent configuration
- Navigation handling for HTTP/HTTPS redirects

## Commands

The application exposes the following Tauri commands:

### `open_login_window()`
Opens the login window. If the window is already open, it brings it to focus. If not, creates a new window pointed to https://zu.zuhaowan.com.

### `close_login_window()`
Closes the login window if it's currently open.

### `is_login_window_open()`
Returns a boolean indicating whether the login window is currently open.

## Development

```bash
npm install
npm run dev
```

## Building

```bash
npm run build
```

## Architecture

- **Frontend**: HTML/JavaScript using Tauri API
- **Backend**: Rust with Tauri 2.0
- **Window Management**: State-based window tracking using Arc<Mutex<>>
- **Navigation**: Configured to allow navigation to zu.zuhaowan.com domain

## Login Window Configuration

The login window is configured with:
- Title: "Login - Zuhaowan"
- Size: 600x700 pixels
- Resizable: Yes
- Centered on screen
- Custom user agent: Chrome 120 on Windows
- Navigation: Allows HTTP/HTTPS redirects
