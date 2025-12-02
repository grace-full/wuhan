# Cross-Origin Configuration and Security

This document explains the cross-origin configuration implemented in `tauri.conf.json` and how various security concerns are mitigated.

## Overview

The application is configured to allow secure communication with specific remote domains while maintaining strict security for all other origins. This is particularly important for the login window which needs to access external authentication services.

## Configuration Details

### 1. Content Security Policy (CSP)

The CSP is configured to allow specific domains while blocking all others:

```json
{
  "csp": {
    "default-src": "'self' tauri:",
    "connect-src": "'self' https://api.example.com wss://api.example.com tauri: ipc: http://ipc.localhost",
    "img-src": "'self' https://api.example.com data: asset: blob:",
    "style-src": "'self' 'unsafe-inline'",
    "script-src": "'self' 'unsafe-inline' 'wasm-unsafe-eval'",
    "font-src": "'self' data:",
    "media-src": "'self' https://api.example.com"
  }
}
```

**What this does:**
- `default-src`: Only allows resources from the app itself and Tauri's custom protocol
- `connect-src`: Allows network connections to the specified API domain and WebSocket connections
- `img-src`: Allows images from the app, API domain, and data URIs
- `style-src`: Allows inline styles (required for many UI frameworks)
- `script-src`: Allows inline scripts and WebAssembly (required for modern web frameworks)
- `font-src`: Allows fonts from the app and data URIs
- `media-src`: Allows media from the app and API domain

### 2. Remote Domain IPC Access

The `dangerousRemoteDomainIpcAccess` setting allows specific remote domains to access Tauri's IPC (Inter-Process Communication) API:

```json
{
  "dangerousRemoteDomainIpcAccess": [
    {
      "domain": "login.example.com",
      "windows": ["login"],
      "enableTauriAPI": true
    },
    {
      "domain": "auth.example.com",
      "windows": ["login"],
      "enableTauriAPI": true
    }
  ]
}
```

**Security measures:**
- Only the `login` window can access these domains
- Only specific domains are whitelisted
- The Tauri API is explicitly enabled only for these domains
- All other domains are blocked by default

### 3. Window Capabilities

Different windows have different capability profiles:

#### Main Window
- Standard application permissions
- No remote domain access
- Full control over window management

#### Login Window
- Limited to specific remote URLs: `https://login.example.com/*` and `https://auth.example.com/*`
- Can only close, show, and hide the window
- Cannot create new windows or access file system

### 4. HTTP Plugin Configuration

The HTTP plugin is scoped to specific domains:

```json
{
  "plugins": {
    "http": {
      "scope": [
        "https://api.example.com/*",
        "https://login.example.com/*",
        "https://auth.example.com/*"
      ]
    }
  }
}
```

This ensures that HTTP requests can only be made to whitelisted domains.

### 5. Protocol Allowlists

Asset and custom protocols are configured with strict scopes:

```json
{
  "assetProtocol": {
    "enable": true,
    "scope": ["$RESOURCE/**"]
  }
}
```

This allows loading of local assets while preventing access to arbitrary file system locations.

### 6. Cookie and Session Persistence

Cookies are handled securely through the WebView's native cookie storage:

- **HTTP-only cookies**: Backend should set cookies with `HttpOnly` flag
- **Secure flag**: All authentication cookies should use `Secure` flag
- **SameSite**: Use `SameSite=Strict` or `SameSite=Lax` for CSRF protection
- **Persistent storage**: WebView automatically persists cookies in the app's data directory

## Cross-Origin Issue Mitigation

### 1. CORS (Cross-Origin Resource Sharing)

**Problem:** Browsers block requests to different origins by default.

**Solution:**
- Backend API must send appropriate CORS headers
- Use Tauri's HTTP plugin for backend requests (bypasses browser CORS)
- Configure the backend to allow requests from `tauri://localhost` and other Tauri origins

**Example backend CORS configuration (Express.js):**
```javascript
app.use(cors({
  origin: [
    'tauri://localhost',
    'http://tauri.localhost',
    'https://tauri.localhost'
  ],
  credentials: true
}));
```

### 2. Mixed Content

**Problem:** Loading HTTP resources from an HTTPS page is blocked.

**Solution:**
- All remote resources should use HTTPS
- Local development can use HTTP for `localhost`
- CSP enforces secure connections for production domains

### 3. Cookie Domain Restrictions

**Problem:** Cookies from remote domains might not be accessible.

**Solution:**
- Remote authentication domains set their own cookies
- Session tokens are passed back to the app via redirect URLs or postMessage
- App stores tokens securely using `tauri-plugin-store`

### 4. WebSocket Connections

**Problem:** WebSocket connections might be blocked by CSP.

**Solution:**
- `wss://` protocol is explicitly allowed in CSP's `connect-src`
- WebSocket scope is limited to whitelisted domains

## Plugin Integration

### tauri-plugin-autostart

Used for starting the application on system boot. No cross-origin concerns as it's a native capability.

**Configuration:**
```rust
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### tauri-plugin-http

Custom HTTP plugin that bypasses browser CORS restrictions:

**Configuration:**
```rust
use tauri_plugin_http::reqwest;

// In your Tauri command
#[tauri::command]
async fn fetch_data() -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.example.com/data")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    response.text().await.map_err(|e| e.to_string())
}
```

### tauri-plugin-store

Securely stores authentication tokens and session data:

**Configuration:**
```javascript
import { Store } from 'tauri-plugin-store-api';

const store = new Store('.settings.dat');

// Store authentication token
await store.set('auth_token', token);
await store.save();

// Retrieve token
const token = await store.get('auth_token');
```

## Security Best Practices

1. **Always use HTTPS** for remote domains in production
2. **Minimize remote domain access** - only allow what's absolutely necessary
3. **Use capability-based permissions** - give each window only the permissions it needs
4. **Validate all IPC messages** - never trust data from remote domains
5. **Keep CSP strict** - only relax rules when absolutely necessary
6. **Regular security audits** - review allowed domains and permissions regularly
7. **Token storage** - use `tauri-plugin-store` for secure token storage, never localStorage for sensitive data
8. **Token transmission** - use secure channels (HTTPS, WSS) for token transmission
9. **Token expiration** - implement token refresh mechanisms
10. **Error handling** - don't expose sensitive information in error messages

## Testing Cross-Origin Access

### Testing the Login Window

1. **Start the development server:**
   ```bash
   npm run dev
   ```

2. **Open the login window** - should load without CORS errors
3. **Check browser console** - no CSP violations should appear
4. **Verify authentication flow** - login should complete successfully
5. **Check cookie persistence** - cookies should persist across app restarts

### Testing Security Restrictions

1. **Try loading unauthorized domain** - should be blocked by CSP
2. **Attempt to access file system from remote domain** - should fail
3. **Try to create windows from remote domain** - should be denied
4. **Verify IPC access** - only login window should access Tauri API from remote domains

## Troubleshooting

### CORS Errors

If you see CORS errors:
1. Check that the backend is sending correct CORS headers
2. Verify the domain is in the HTTP plugin scope
3. Consider using Tauri's HTTP plugin instead of fetch API

### CSP Violations

If you see CSP violations:
1. Check the CSP configuration in `tauri.conf.json`
2. Verify the resource URL is whitelisted
3. Add the domain to the appropriate CSP directive

### Cookie Not Persisting

If cookies aren't persisting:
1. Verify the backend sets cookies with appropriate flags
2. Check that the domain is whitelisted
3. Ensure the cookie path is correct
4. Use developer tools to inspect cookie storage

### Remote Domain IPC Access Not Working

If remote domains can't access Tauri API:
1. Verify the domain is in `dangerousRemoteDomainIpcAccess`
2. Check that the window label matches
3. Ensure `enableTauriAPI` is set to `true`
4. Verify the domain matches exactly (no protocol or trailing slash)

## Configuration Customization

To customize for your specific domains:

1. Replace `example.com`, `login.example.com`, and `auth.example.com` with your actual domains
2. Update the HTTP plugin scope with your API endpoints
3. Adjust CSP rules based on your resource requirements
4. Add or remove window capabilities as needed

## References

- [Tauri Security Best Practices](https://tauri.app/v1/references/architecture/security/)
- [Content Security Policy (MDN)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [Tauri Configuration](https://tauri.app/v1/api/config/)
- [tauri-plugin-http](https://github.com/tauri-apps/tauri-plugin-http)
- [tauri-plugin-store](https://github.com/tauri-apps/tauri-plugin-store)
- [tauri-plugin-autostart](https://github.com/tauri-apps/tauri-plugin-autostart)
