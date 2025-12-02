# Quick Start Guide

Get up and running with the Tauri cross-origin configuration in 5 minutes.

## Prerequisites

- Node.js 16+ installed
- Rust installed (latest stable)
- Code editor

## 1. Customize Configuration (2 minutes)

Edit `src-tauri/tauri.conf.json` and replace these domains:

```json
{
  "example.com" → "your-api-domain.com",
  "login.example.com" → "your-login-domain.com",
  "auth.example.com" → "your-auth-domain.com"
}
```

Search and replace in the file:
- `example.com` → your actual domain
- `login.example.com` → your actual login page
- `auth.example.com` → your actual auth service

## 2. Install Dependencies (1 minute)

```bash
# Install frontend dependencies
npm install

# Install Rust dependencies
cd src-tauri
cargo build
cd ..
```

## 3. Run Development Server (1 minute)

```bash
npm run dev
```

The application will open with a demo interface.

## 4. Test Authentication Flow (1 minute)

1. Click "Open Login Window"
2. The login window should open and load your remote page
3. Complete the login flow
4. Token should be stored securely

## What Was Configured?

✅ **Security**: Strict CSP with whitelisted domains
✅ **Remote Access**: Login window can access your auth pages
✅ **CORS**: HTTP plugin bypasses CORS restrictions
✅ **Cookies**: Secure cookie persistence enabled
✅ **Tokens**: Encrypted token storage via tauri-plugin-store
✅ **Protocols**: Asset and data URL protocols configured

## File Overview

| File | Purpose |
|------|---------|
| `src-tauri/tauri.conf.json` | **Main configuration** (start here) |
| `src/auth.js` | Authentication module |
| `src/main.js` | Frontend application |
| `src-tauri/src/main.rs` | Rust backend |
| `README.md` | Full documentation |
| `CROSS_ORIGIN_SECURITY.md` | Security details |
| `CONFIG_EXAMPLES.md` | Configuration examples |

## Common Tasks

### Change Allowed Domains

Edit `src-tauri/tauri.conf.json`:
1. Update `dangerousRemoteDomainIpcAccess`
2. Update `capabilities[].remote.urls`
3. Update `plugins.http.scope`
4. Update CSP `connect-src`

### Add WebSocket Support

Add to CSP in `tauri.conf.json`:
```json
"connect-src": "'self' wss://your-ws-domain.com"
```

### Add CDN Support

Add to CSP in `tauri.conf.json`:
```json
"img-src": "'self' https://cdn.example.com",
"font-src": "'self' https://cdn.example.com"
```

### Store Authentication Token

```javascript
import { Store } from 'tauri-plugin-store-api';

const store = new Store('.auth.dat');
await store.set('auth_token', token);
await store.save();
```

### Make API Request

```javascript
import { invoke } from '@tauri-apps/api/core';

const data = await invoke('fetch_from_api', { 
  endpoint: 'users/profile' 
});
```

## Troubleshooting

### CORS Errors
→ Use Tauri HTTP plugin instead of fetch()

### CSP Violations
→ Add domain to appropriate CSP directive

### Login Window Not Loading
→ Check domain is in `dangerousRemoteDomainIpcAccess`

### Cookies Not Persisting
→ Verify backend sets `Secure`, `HttpOnly`, `SameSite` flags

## Next Steps

1. ✅ Customize configuration
2. ✅ Test authentication flow
3. → Implement your remote login page (see `REMOTE_PAGE_INTEGRATION.md`)
4. → Configure your backend CORS
5. → Deploy and test in production

## Need Help?

- **Security Details**: `CROSS_ORIGIN_SECURITY.md`
- **Remote Integration**: `REMOTE_PAGE_INTEGRATION.md`
- **Config Examples**: `CONFIG_EXAMPLES.md`
- **Full Guide**: `README.md`

## Production Checklist

Before deploying to production:

- [ ] Replace all `example.com` domains
- [ ] Change to HTTPS URLs (no HTTP)
- [ ] Test authentication flow
- [ ] Verify token storage
- [ ] Test API requests
- [ ] Check CSP violations in console
- [ ] Test cookie persistence
- [ ] Review security settings
- [ ] Build production version: `npm run tauri build`
- [ ] Test production build

## Build for Production

```bash
# Build
npm run tauri build

# Builds will be in:
# - src-tauri/target/release/bundle/
```

---

**That's it!** You now have a secure Tauri app with cross-origin configuration. 🎉
