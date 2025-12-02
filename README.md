# Tauri + Vue 3 + Element-Plus

A modern desktop application built with Tauri 2.8, Vue 3, TypeScript, and Element-Plus UI components.

## Features

- ⚡️ **Tauri 2.8** - Build smaller, faster, and more secure desktop applications
- 🎨 **Vue 3** - Progressive JavaScript framework with Composition API
- 🎭 **TypeScript** - Type safety and better development experience
- 🧩 **Element-Plus** - Vue 3 UI component library with global styling
- ⚙️ **Vite** - Lightning-fast frontend build tool

## Prerequisites

Before getting started, ensure you have the following installed:

### General Requirements

- **Node.js** (v16 or higher) - [Download](https://nodejs.org/)
- **pnpm** (recommended package manager) - Install via `npm install -g pnpm`
- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)

### Platform-specific Dependencies

#### Linux
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

#### macOS
```bash
# Xcode Command Line Tools
xcode-select --install
```

#### Windows
- Microsoft Visual Studio C++ Build Tools
- WebView2 (usually pre-installed on Windows 11)

## Getting Started

### 1. Install Dependencies

```bash
pnpm install
```

This will install all npm dependencies including Vue 3, Element-Plus, and Tauri CLI.

### 2. Run Development Server

```bash
pnpm dev
```

This command will:
- Start the Vite development server for Vue 3 frontend
- Launch the Tauri desktop application
- Enable hot-reload for both frontend and Rust backend changes

The application window should open automatically with the Vue 3 + Element-Plus interface.

### 3. Build for Production

```bash
pnpm build
```

This will:
- Type-check TypeScript files with `vue-tsc`
- Build the Vue frontend with Vite
- Compile the Rust backend and bundle the application

The final application bundle will be available in `src-tauri/target/release/bundle/`.

## Project Structure

```
.
├── src/                    # Vue 3 frontend source code
│   ├── assets/            # Static assets (images, fonts, etc.)
│   ├── App.vue            # Root Vue component with Element-Plus examples
│   └── main.ts            # Application entry point with Element-Plus setup
├── src-tauri/             # Tauri Rust backend
│   ├── src/               # Rust source code
│   ├── icons/             # Application icons
│   ├── Cargo.toml         # Rust dependencies and project metadata
│   └── tauri.conf.json    # Tauri configuration
├── public/                # Public static files
├── index.html             # HTML entry point
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # TypeScript configuration
└── package.json           # npm dependencies and scripts
```

## Available Scripts

- `pnpm dev` - Start development server with Tauri desktop app
- `pnpm build` - Build production-ready application
- `pnpm preview` - Preview Vite build locally (frontend only)
- `pnpm tauri` - Direct access to Tauri CLI commands

## Element-Plus Integration

Element-Plus is globally configured in `src/main.ts`:

```typescript
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";

const app = createApp(App);
app.use(ElementPlus);
app.mount("#app");
```

This enables you to use any Element-Plus component throughout your application without additional imports.

## Configuration

### Tauri Configuration (`src-tauri/tauri.conf.json`)

Key settings:
- `productName`: Application display name
- `identifier`: Unique app identifier (reverse domain notation)
- `bundle.targets`: Build targets (all platforms by default)
- `app.windows`: Default window size and properties

### Vite Configuration (`vite.config.ts`)

Includes Vue plugin and Tauri-specific optimizations.

## Development Tips

1. **Hot Reload**: Changes to Vue files are reflected instantly without restarting
2. **Rust Changes**: Rust backend changes require a rebuild (automatic in dev mode)
3. **Console Logs**: Use browser DevTools (F12) for frontend debugging
4. **Tauri Commands**: Define Rust functions and call them from Vue via `invoke()`

## Learn More

- [Tauri Documentation](https://tauri.app/)
- [Vue 3 Documentation](https://vuejs.org/)
- [Element-Plus Documentation](https://element-plus.org/)
- [Vite Documentation](https://vitejs.dev/)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

MIT
