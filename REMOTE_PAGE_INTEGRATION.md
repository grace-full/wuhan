# Remote Login Page Integration Guide

This document explains how to integrate your remote login page with the Tauri application's cross-origin configuration.

## Overview

The Tauri application opens a login window that loads your remote authentication page (e.g., `https://login.example.com`). This page needs to communicate back to the Tauri app after successful authentication.

## Prerequisites

Your remote domain must be configured in `tauri.conf.json`:

```json
{
  "app": {
    "security": {
      "capabilities": [
        {
          "identifier": "login-capability",
          "windows": ["login"],
          "remote": {
            "urls": ["https://login.example.com/*", "https://auth.example.com/*"]
          }
        }
      ]
    }
  },
  "tauri": {
    "security": {
      "dangerousRemoteDomainIpcAccess": [
        {
          "domain": "login.example.com",
          "windows": ["login"],
          "enableTauriAPI": true
        }
      ]
    }
  }
}
```

## Integration Methods

### Method 1: Using postMessage (Recommended)

This method works without requiring Tauri API access and is more secure.

#### Remote Login Page Code

```html
<!DOCTYPE html>
<html>
<head>
    <title>Login</title>
</head>
<body>
    <form id="loginForm">
        <input type="email" id="email" placeholder="Email" required />
        <input type="password" id="password" placeholder="Password" required />
        <button type="submit">Login</button>
    </form>

    <script>
        document.getElementById('loginForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;
            
            try {
                // Authenticate with your backend
                const response = await fetch('https://api.example.com/auth/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ email, password }),
                    credentials: 'include' // Important for cookies
                });
                
                const data = await response.json();
                
                if (data.success) {
                    // Send authentication token to Tauri app
                    if (window.opener) {
                        window.opener.postMessage({
                            type: 'auth_success',
                            token: data.token,
                            user: data.user
                        }, '*');
                    } else {
                        // For iframe or webview context
                        window.parent.postMessage({
                            type: 'auth_success',
                            token: data.token,
                            user: data.user
                        }, '*');
                    }
                    
                    // Close the window (optional)
                    setTimeout(() => window.close(), 1000);
                } else {
                    alert('Login failed: ' + data.message);
                }
            } catch (error) {
                console.error('Login error:', error);
                alert('Login failed. Please try again.');
            }
        });
    </script>
</body>
</html>
```

### Method 2: Using Tauri IPC API

This method requires `enableTauriAPI: true` in the configuration.

#### Remote Login Page Code

```html
<!DOCTYPE html>
<html>
<head>
    <title>Login</title>
    <!-- Include Tauri API -->
    <script type="module">
        // Check if Tauri API is available
        if (window.__TAURI__) {
            console.log('Tauri API is available');
        }
    </script>
</head>
<body>
    <form id="loginForm">
        <input type="email" id="email" placeholder="Email" required />
        <input type="password" id="password" placeholder="Password" required />
        <button type="submit">Login</button>
    </form>

    <script type="module">
        // Import Tauri invoke function
        const { invoke } = window.__TAURI__.core;
        
        document.getElementById('loginForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;
            
            try {
                // Authenticate with your backend
                const response = await fetch('https://api.example.com/auth/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ email, password }),
                    credentials: 'include'
                });
                
                const data = await response.json();
                
                if (data.success) {
                    // Store token via Tauri command
                    await invoke('store_auth_token', { token: data.token });
                    
                    // Close login window
                    await invoke('close_login_window');
                } else {
                    alert('Login failed: ' + data.message);
                }
            } catch (error) {
                console.error('Login error:', error);
                alert('Login failed. Please try again.');
            }
        });
    </script>
</body>
</html>
```

### Method 3: Using Redirect URLs

This method uses OAuth-style redirect URLs.

#### Remote Login Page Code

```javascript
// After successful authentication
const token = data.token;
const callbackUrl = new URL(window.location.href).searchParams.get('callback');

if (callbackUrl) {
    // Redirect back to the app with token
    window.location.href = `${callbackUrl}?token=${encodeURIComponent(token)}`;
} else {
    // Fallback to postMessage
    window.opener.postMessage({
        type: 'auth_success',
        token: token
    }, '*');
}
```

#### Tauri App Code

```javascript
// Open login window with callback URL
const callbackUrl = 'tauri://localhost/auth/callback';
const loginUrl = `https://login.example.com?callback=${encodeURIComponent(callbackUrl)}`;

await invoke('open_login_window', { url: loginUrl });
```

## Backend Configuration

Your authentication backend needs to support cross-origin requests from Tauri.

### CORS Headers

```javascript
// Express.js example
const cors = require('cors');

app.use(cors({
    origin: [
        'tauri://localhost',
        'http://tauri.localhost',
        'https://tauri.localhost',
        'http://localhost:5173' // Development
    ],
    credentials: true,
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Content-Type', 'Authorization']
}));
```

### Cookie Configuration

```javascript
// Set cookies with appropriate flags
res.cookie('auth_token', token, {
    httpOnly: true,      // Prevent JavaScript access
    secure: true,        // HTTPS only
    sameSite: 'none',    // Allow cross-site (required for Tauri)
    maxAge: 7 * 24 * 60 * 60 * 1000 // 7 days
});
```

## Security Considerations

### 1. Origin Validation

Always validate the origin of postMessage events:

```javascript
window.addEventListener('message', (event) => {
    // Whitelist of allowed origins
    const allowedOrigins = [
        'https://login.example.com',
        'https://auth.example.com'
    ];
    
    if (!allowedOrigins.includes(event.origin)) {
        console.warn('Unauthorized origin:', event.origin);
        return;
    }
    
    // Process the message
    if (event.data.type === 'auth_success') {
        handleAuthSuccess(event.data.token);
    }
});
```

### 2. Token Validation

Always validate tokens on the backend:

```javascript
// Backend token validation
app.get('/api/protected', authenticateToken, (req, res) => {
    res.json({ user: req.user });
});

function authenticateToken(req, res, next) {
    const token = req.headers['authorization']?.split(' ')[1];
    
    if (!token) {
        return res.status(401).json({ error: 'No token provided' });
    }
    
    jwt.verify(token, process.env.JWT_SECRET, (err, user) => {
        if (err) {
            return res.status(403).json({ error: 'Invalid token' });
        }
        req.user = user;
        next();
    });
}
```

### 3. HTTPS Only

Always use HTTPS for production domains:

```json
{
  "remote": {
    "urls": ["https://login.example.com/*"]
  }
}
```

Never use `http://` for production.

### 4. Token Storage

Store tokens securely using Tauri's store plugin:

```javascript
import { Store } from 'tauri-plugin-store-api';

const store = new Store('.auth.dat');

// Store token
await store.set('auth_token', token);
await store.save();

// Retrieve token
const token = await store.get('auth_token');
```

Never use `localStorage` or `sessionStorage` for sensitive data.

## Testing

### Local Development

For local development, you can test with a local server:

1. Update `tauri.conf.json` to allow localhost:
   ```json
   {
     "remote": {
       "urls": ["http://localhost:3000/*"]
     }
   }
   ```

2. Run your login page locally:
   ```bash
   cd remote-login-page
   npm run dev
   ```

3. Test the authentication flow

### Production Testing

1. Deploy your login page to the production domain
2. Update `tauri.conf.json` with production URLs
3. Build the Tauri app: `npm run tauri build`
4. Test the authentication flow in the production build

## Troubleshooting

### postMessage Not Working

**Problem**: Messages not being received in the main app.

**Solution**:
- Verify the window relationship (window.opener vs window.parent)
- Check browser console for errors
- Ensure origin validation is not too strict during development

### Tauri API Not Available

**Problem**: `window.__TAURI__` is undefined on remote page.

**Solution**:
- Verify domain is in `dangerousRemoteDomainIpcAccess`
- Check that `enableTauriAPI` is set to `true`
- Ensure window label matches
- Verify domain matches exactly (no trailing slash)

### CORS Errors

**Problem**: Network requests failing with CORS errors.

**Solution**:
- Configure backend CORS to allow Tauri origins
- Use Tauri's HTTP plugin for API requests
- Check that credentials are included in requests

### Cookies Not Being Set

**Problem**: Authentication cookies not persisting.

**Solution**:
- Set `sameSite: 'none'` on cookies
- Ensure `secure: true` flag is set
- Use `credentials: 'include'` in fetch requests
- Check that domain and path are correct

## Example Projects

### Complete Remote Login Page

See the `examples/remote-login-page` directory for a complete example of a remote login page with:
- Email/password authentication
- OAuth integration
- Token management
- Error handling
- Loading states

### Complete Tauri Integration

See the `src/auth.js` file for a complete example of integrating with the remote login page from the Tauri side.

## Additional Resources

- [Tauri Security Documentation](https://tauri.app/v1/references/architecture/security/)
- [Content Security Policy Guide](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
- [postMessage API](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)
- [OAuth 2.0 Best Practices](https://oauth.net/2/)
