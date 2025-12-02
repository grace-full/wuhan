# Tauri Cross-Origin Configuration

This project demonstrates a secure Tauri application with properly configured cross-origin settings that allow remote domain access for authentication while maintaining strict security for all other resources.

## Features

- ✅ Secure cross-origin configuration
- ✅ Remote domain access for login window
- ✅ Content Security Policy (CSP) protection
- ✅ Protocol allowlists and scope restrictions
- ✅ Secure cookie and token persistence
- ✅ CORS mitigation via Tauri HTTP plugin
- ✅ Comprehensive security documentation

## Project Structure

```
.
├── src/
│   └── auth.js                    # Frontend authentication module
├── src-tauri/
│   ├── src/
│   │   └── main.rs               # Rust backend with secure commands
│   ├── Cargo.toml                # Rust dependencies
│   ├── build.rs                  # Build script
│   └── tauri.conf.json           # Main configuration file
├── CROSS_ORIGIN_SECURITY.md      # Detailed security documentation
└── README.md                     # This file
```

## Configuration Overview

### Security Settings

The `tauri.conf.json` file contains comprehensive security settings:

1. **Content Security Policy (CSP)**
   - Restricts resource loading to trusted sources
   - Allows specific API domains for network requests
   - Enables data URIs for images and fonts
   - Permits inline styles and scripts (required for modern frameworks)

2. **Remote Domain IPC Access**
   - Whitelists `login.example.com` and `auth.example.com`
   - Restricted to the `login` window only
   - Enables Tauri API access for authenticated flows

3. **Window Capabilities**
   - Main window: Full application permissions, no remote access
   - Login window: Limited permissions, remote domain access only

4. **Protocol Allowlists**
   - Asset protocol enabled for local resources
   - Scoped to application resources only
   - Prevents unauthorized file system access

5. **HTTP Plugin Scope**
   - Limited to specific API endpoints
   - Bypasses CORS restrictions
   - Secure communication with backend services

## Getting Started

### Prerequisites

- Node.js (v16 or higher)
- Rust (latest stable)
- Tauri CLI

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Install Rust dependencies:
   ```bash
   cd src-tauri
   cargo build
   ```

### Configuration

Before running the application, you need to customize the configuration for your domains:

1. Open `src-tauri/tauri.conf.json`
2. Replace the following placeholders with your actual domains:
   - `example.com` → Your API domain
   - `login.example.com` → Your login page domain
   - `auth.example.com` → Your authentication service domain

3. Update CSP rules in both:
   - `app.security.csp` (Tauri v2 format)
   - `tauri.security.csp` (Tauri v1 compatibility)

4. Adjust HTTP plugin scope to match your API endpoints

### Running the Application

#### Development Mode

```bash
npm run dev
```

Or:

```bash
cargo tauri dev
```

#### Production Build

```bash
npm run build
```

Or:

```bash
cargo tauri build
```

## Usage

### Opening the Login Window

```javascript
import { openLoginWindow } from './src/auth.js';

// Opens a new window loading the remote login page
await openLoginWindow();
```

### Handling Authentication

The login window loads the remote authentication page. After successful login, the remote page should:

1. **Option 1: Use postMessage**
   ```javascript
   // From remote login page (login.example.com)
   window.parent.postMessage(
     { type: 'auth_success', token: 'your-auth-token' },
     '*'
   );
   ```

2. **Option 2: Use Tauri IPC (if enableTauriAPI is true)**
   ```javascript
   // From remote login page
   import { invoke } from '@tauri-apps/api/core';
   await invoke('store_auth_token', { token: 'your-auth-token' });
   ```

### Making API Requests

```javascript
import { fetchFromAPI } from './src/auth.js';

// Makes a request through Tauri HTTP plugin (no CORS issues)
const data = await fetchFromAPI('users/profile');
```

### Storing Tokens Securely

```javascript
import { Store } from 'tauri-plugin-store-api';

const store = new Store('.auth.dat');
await store.set('auth_token', token);
await store.save();
```

## Security Considerations

### What's Protected

1. **File System Access**: Limited to app data directory only
2. **Network Requests**: Scoped to whitelisted domains
3. **IPC Access**: Remote domains can only access Tauri API from login window
4. **Window Creation**: Remote domains cannot create new windows
5. **Protocol Access**: Asset protocol limited to app resources

### What's Allowed

1. **Login Window**: Can load and interact with whitelisted remote domains
2. **API Communication**: Can make HTTP requests to specified API domains
3. **Cookie Storage**: WebView persists cookies securely
4. **Token Storage**: Encrypted local storage via tauri-plugin-store

### Best Practices

1. ✅ Always use HTTPS for remote domains in production
2. ✅ Minimize the number of whitelisted domains
3. ✅ Use capability-based permissions (principle of least privilege)
4. ✅ Validate all data from remote sources
5. ✅ Store sensitive data using tauri-plugin-store, not localStorage
6. ✅ Implement token refresh mechanisms
7. ✅ Set appropriate cookie flags (HttpOnly, Secure, SameSite)
8. ✅ Regular security audits of allowed domains and permissions

## Troubleshooting

### CORS Errors

**Problem**: Getting CORS errors when making API requests.

**Solution**:
- Use Tauri's HTTP plugin via `invoke('fetch_from_api')` instead of `fetch()`
- Ensure the API domain is in the HTTP plugin scope
- Configure backend to allow Tauri origins

### CSP Violations

**Problem**: Resources blocked by Content Security Policy.

**Solution**:
- Check browser console for specific violation
- Add the domain to appropriate CSP directive in `tauri.conf.json`
- Verify the resource URL matches the allowed patterns

### Login Window Not Loading

**Problem**: Login window shows blank page or error.

**Solution**:
- Verify the remote URL is accessible
- Check that the domain is in `dangerousRemoteDomainIpcAccess`
- Ensure the domain is in the login capability's `remote.urls`
- Check network connectivity

### Cookies Not Persisting

**Problem**: Cookies are lost after app restart.

**Solution**:
- Verify backend sets cookies with appropriate flags
- Check cookie domain and path settings
- Ensure WebView cookie storage is not being cleared
- Use developer tools to inspect cookie storage

## Documentation

For detailed security documentation, see [CROSS_ORIGIN_SECURITY.md](./CROSS_ORIGIN_SECURITY.md).

This document covers:
- Detailed CSP configuration
- CORS mitigation strategies
- Mixed content handling
- Cookie security
- Plugin integration examples
- Testing procedures
- Security best practices

## Testing

### Manual Testing

1. Start the development server
2. Click to open login window
3. Verify the remote page loads without CORS errors
4. Complete the login flow
5. Check that authentication token is stored
6. Verify cookies persist after app restart
7. Test API requests work without CORS issues

### Security Testing

1. Try loading an unauthorized domain → should be blocked
2. Attempt file system access from remote page → should fail
3. Try creating windows from remote page → should be denied
4. Verify main window cannot load remote domains → should be blocked
5. Check that only login window has remote access → verified

## Plugins Used

- **tauri-plugin-http**: HTTP client that bypasses CORS restrictions
- **tauri-plugin-store**: Encrypted key-value storage for tokens
- **tauri-plugin-fs**: File system access with strict scoping
- **tauri-plugin-autostart**: Auto-start application on system boot

## License

[Your License Here]

## Contributing

[Your Contributing Guidelines Here]

## Support

For issues or questions:
- Check [CROSS_ORIGIN_SECURITY.md](./CROSS_ORIGIN_SECURITY.md) for detailed documentation
- Review the troubleshooting section above
- Open an issue on GitHub

## Changelog

### v0.1.0 (Initial Release)
- Secure cross-origin configuration
- Remote domain access for login
- CSP protection
- HTTP plugin integration
- Secure token storage
- Comprehensive documentation
