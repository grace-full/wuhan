# Tauri Cross-Origin Configuration Examples

This document provides configuration examples for different cross-origin scenarios.

## Table of Contents

1. [Basic Remote Authentication](#basic-remote-authentication)
2. [Multiple Remote Domains](#multiple-remote-domains)
3. [Development vs Production](#development-vs-production)
4. [OAuth Flow](#oauth-flow)
5. [WebSocket Support](#websocket-support)
6. [Strict Security (No Remote Access)](#strict-security-no-remote-access)

## Basic Remote Authentication

Single login page with API access.

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://api.example.com data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'"
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default"]
        },
        {
          "identifier": "login-capability",
          "windows": ["login"],
          "permissions": ["core:default"],
          "remote": {
            "urls": ["https://login.example.com/*"]
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
  },
  "plugins": {
    "http": {
      "scope": ["https://api.example.com/*"]
    }
  }
}
```

## Multiple Remote Domains

Support for multiple authentication providers.

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api.example.com https://auth.google.com https://login.microsoftonline.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://api.example.com https://accounts.google.com https://login.microsoftonline.com data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'"
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default"]
        },
        {
          "identifier": "login-capability",
          "windows": ["login"],
          "permissions": ["core:default"],
          "remote": {
            "urls": [
              "https://login.example.com/*",
              "https://accounts.google.com/*",
              "https://login.microsoftonline.com/*"
            ]
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
        },
        {
          "domain": "accounts.google.com",
          "windows": ["login"],
          "enableTauriAPI": true
        },
        {
          "domain": "login.microsoftonline.com",
          "windows": ["login"],
          "enableTauriAPI": true
        }
      ]
    }
  },
  "plugins": {
    "http": {
      "scope": [
        "https://api.example.com/*",
        "https://oauth2.googleapis.com/*",
        "https://graph.microsoft.com/*"
      ]
    }
  }
}
```

## Development vs Production

Use environment-specific configuration.

### Development Configuration

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' http://localhost:3000 https://api-dev.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' http://localhost:3000 data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline' 'unsafe-eval'"
      },
      "capabilities": [
        {
          "identifier": "login-capability",
          "windows": ["login"],
          "permissions": ["core:default"],
          "remote": {
            "urls": [
              "http://localhost:3000/*",
              "https://login-dev.example.com/*"
            ]
          }
        }
      ]
    }
  },
  "tauri": {
    "security": {
      "dangerousRemoteDomainIpcAccess": [
        {
          "domain": "localhost",
          "windows": ["login"],
          "enableTauriAPI": true
        }
      ]
    }
  },
  "plugins": {
    "http": {
      "scope": [
        "http://localhost:3000/*",
        "https://api-dev.example.com/*"
      ]
    }
  }
}
```

### Production Configuration

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api.example.com wss://api.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://api.example.com data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'"
      },
      "capabilities": [
        {
          "identifier": "login-capability",
          "windows": ["login"],
          "permissions": ["core:default"],
          "remote": {
            "urls": ["https://login.example.com/*"]
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
      ],
      "freezePrototype": true
    }
  },
  "plugins": {
    "http": {
      "scope": ["https://api.example.com/*"]
    }
  }
}
```

## OAuth Flow

Configuration for OAuth 2.0 authentication flow.

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api.example.com https://oauth.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://api.example.com https://oauth.example.com data: asset:",
        "style-src": "'self' https://oauth.example.com 'unsafe-inline'",
        "script-src": "'self' https://oauth.example.com 'unsafe-inline'",
        "frame-src": "https://oauth.example.com"
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default", "core:window:allow-create"]
        },
        {
          "identifier": "oauth-capability",
          "windows": ["oauth"],
          "permissions": ["core:default"],
          "remote": {
            "urls": [
              "https://oauth.example.com/*",
              "https://accounts.example.com/*"
            ]
          }
        }
      ]
    }
  },
  "tauri": {
    "security": {
      "dangerousRemoteDomainIpcAccess": [
        {
          "domain": "oauth.example.com",
          "windows": ["oauth"],
          "enableTauriAPI": true
        }
      ]
    }
  },
  "plugins": {
    "http": {
      "scope": [
        "https://api.example.com/*",
        "https://oauth.example.com/token"
      ]
    }
  }
}
```

### Rust Command for OAuth

```rust
#[tauri::command]
async fn start_oauth_flow(app: tauri::AppHandle) -> Result<String, String> {
    // Generate OAuth state
    let state = generate_random_state();
    
    // Build OAuth URL
    let oauth_url = format!(
        "https://oauth.example.com/authorize?client_id={}&redirect_uri={}&state={}&scope={}",
        "YOUR_CLIENT_ID",
        "tauri://oauth/callback",
        state,
        "read write"
    );
    
    // Open OAuth window
    let window = tauri::WebviewWindowBuilder::new(
        &app,
        "oauth",
        tauri::WebviewUrl::External(oauth_url.parse().unwrap())
    )
    .title("Sign In")
    .inner_size(600.0, 700.0)
    .center()
    .build()
    .map_err(|e| e.to_string())?;
    
    Ok(state)
}
```

## WebSocket Support

Configuration for real-time communication.

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api.example.com wss://ws.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://api.example.com data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'"
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default"]
        }
      ]
    }
  },
  "plugins": {
    "http": {
      "scope": [
        "https://api.example.com/*",
        "wss://ws.example.com/*"
      ]
    }
  }
}
```

### WebSocket Usage Example

```javascript
// Frontend WebSocket connection
const ws = new WebSocket('wss://ws.example.com');

ws.onopen = () => {
    console.log('WebSocket connected');
    ws.send(JSON.stringify({ type: 'auth', token: authToken }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Message received:', data);
};

ws.onerror = (error) => {
    console.error('WebSocket error:', error);
};

ws.onclose = () => {
    console.log('WebSocket disconnected');
};
```

## Strict Security (No Remote Access)

Configuration with no remote domain access (most secure).

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' tauri: ipc: http://ipc.localhost",
        "img-src": "'self' data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'"
      },
      "freezePrototype": true,
      "assetProtocol": {
        "enable": true,
        "scope": ["$RESOURCE/**"]
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default"]
        }
      ]
    }
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "scope": ["$APPDATA/**"]
      },
      "http": {
        "all": false,
        "scope": []
      },
      "protocol": {
        "all": false,
        "asset": true,
        "assetScope": ["$RESOURCE/**"]
      },
      "window": {
        "all": false,
        "create": false,
        "center": true,
        "requestUserAttention": true,
        "setResizable": true,
        "setTitle": true
      }
    },
    "security": {
      "csp": "default-src 'self' tauri:; connect-src 'self' tauri: ipc: http://ipc.localhost; img-src 'self' data: asset:; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'",
      "dangerousRemoteDomainIpcAccess": [],
      "freezePrototype": true
    }
  },
  "plugins": {
    "http": {
      "scope": []
    },
    "fs": {
      "scope": ["$APPDATA/**"]
    }
  }
}
```

## CDN and External Resources

Allow loading resources from CDN while maintaining security.

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://cdn.example.com https://api.example.com data: asset: blob:",
        "style-src": "'self' https://cdn.example.com 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'",
        "font-src": "'self' https://cdn.example.com data:",
        "media-src": "'self' https://cdn.example.com https://media.example.com"
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default"]
        }
      ]
    }
  }
}
```

## API with Multiple Endpoints

Configuration for microservices architecture.

```json
{
  "app": {
    "security": {
      "csp": {
        "default-src": "'self' tauri:",
        "connect-src": "'self' https://api-auth.example.com https://api-data.example.com https://api-files.example.com wss://api-ws.example.com tauri: ipc: http://ipc.localhost",
        "img-src": "'self' https://api-files.example.com data: asset:",
        "style-src": "'self' 'unsafe-inline'",
        "script-src": "'self' 'unsafe-inline'"
      },
      "capabilities": [
        {
          "identifier": "main-capability",
          "windows": ["main"],
          "permissions": ["core:default"]
        }
      ]
    }
  },
  "plugins": {
    "http": {
      "scope": [
        "https://api-auth.example.com/*",
        "https://api-data.example.com/*",
        "https://api-files.example.com/*",
        "wss://api-ws.example.com/*"
      ]
    }
  }
}
```

## Tips for Configuration

### 1. Start Restrictive

Begin with the most restrictive configuration and only add permissions as needed:

```json
{
  "csp": {
    "default-src": "'self' tauri:"
  }
}
```

Then add specific directives only when required.

### 2. Use Environment Variables

Use environment variables for different environments:

```rust
let api_url = if cfg!(debug_assertions) {
    "http://localhost:3000"
} else {
    "https://api.example.com"
};
```

### 3. Test Thoroughly

Always test your configuration in both development and production:

```bash
# Development
npm run tauri dev

# Production build
npm run tauri build
```

### 4. Monitor CSP Violations

Add CSP violation reporting:

```json
{
  "csp": {
    "default-src": "'self' tauri:",
    "report-uri": "https://api.example.com/csp-report"
  }
}
```

### 5. Regular Security Audits

Regularly review your configuration:
- Remove unused domains
- Update to stricter CSP rules
- Check for deprecated settings
- Verify plugin scopes

## Common Mistakes to Avoid

### ❌ Too Permissive CSP

```json
{
  "csp": "default-src *"
}
```

### ✅ Specific CSP

```json
{
  "csp": {
    "default-src": "'self' tauri:",
    "connect-src": "'self' https://api.example.com"
  }
}
```

### ❌ Enabling All Allowlist Features

```json
{
  "allowlist": {
    "all": true
  }
}
```

### ✅ Specific Permissions

```json
{
  "allowlist": {
    "all": false,
    "fs": {
      "scope": ["$APPDATA/**"]
    }
  }
}
```

### ❌ Wildcard HTTP Scope

```json
{
  "http": {
    "scope": ["https://*"]
  }
}
```

### ✅ Specific Domains

```json
{
  "http": {
    "scope": ["https://api.example.com/*"]
  }
}
```

## Additional Resources

- [Tauri Configuration Reference](https://tauri.app/v1/api/config/)
- [CSP Validator](https://csp-evaluator.withgoogle.com/)
- [Security Headers](https://securityheaders.com/)
- [Mozilla Observatory](https://observatory.mozilla.org/)
