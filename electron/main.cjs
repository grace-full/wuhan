'use strict';

const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('node:path');

const LOGIN_URL = 'https://zu.zuhaowan.com';
const AUTH_CHANNEL = 'auth:data';

let mainWindow = null;
let loginWindow = null;

const authState = {
  token: '',
  rawCookieHeader: '',
  cookies: [],
  lastUrl: ''
};

const tokenKeys = ['token', 'auth_token', 'access_token', 'ticket', 'authorization', 'jwt'];

const getAuthSnapshot = () => ({
  ...authState,
  cookies: authState.cookies.map((cookie) => ({ ...cookie }))
});

const sendAuthUpdate = () => {
  if (mainWindow && !mainWindow.isDestroyed()) {
    mainWindow.webContents.send(AUTH_CHANNEL, getAuthSnapshot());
  }
  console.log('[Auth Monitor] 当前状态', authState);
};

const parseTokenFromUrl = (targetUrl) => {
  try {
    const parsed = new URL(targetUrl);
    for (const key of tokenKeys) {
      const value = parsed.searchParams.get(key);
      if (value) {
        return value;
      }
    }

    if (parsed.hash) {
      const hashParams = new URLSearchParams(parsed.hash.replace(/^#/, ''));
      for (const key of tokenKeys) {
        const value = hashParams.get(key);
        if (value) {
          return value;
        }
      }
    }
  } catch (error) {
    console.error('[Auth Monitor] 解析 URL token 失败', error);
  }
  return null;
};

const parseTokenFromCookies = (cookies = []) => {
  for (const cookie of cookies) {
    if (tokenKeys.some((key) => cookie.name.toLowerCase().includes(key.toLowerCase()))) {
      return cookie.value;
    }
  }
  return null;
};

const isZuhaowanCookie = (domain = '') => domain.includes('zuhaowan.com');

const collectCookies = async (loginSession) => {
  try {
    const cookies = await loginSession.cookies.get({});
    const filtered = cookies.filter((cookie) => isZuhaowanCookie(cookie.domain));

    authState.cookies = filtered.map((cookie) => ({
      name: cookie.name,
      value: cookie.value,
      domain: cookie.domain,
      path: cookie.path,
      secure: cookie.secure,
      httpOnly: cookie.httpOnly,
      expirationDate: cookie.expirationDate
    }));

    authState.rawCookieHeader = authState.cookies.map((cookie) => `${cookie.name}=${cookie.value}`).join('; ');

    const tokenFromCookie = parseTokenFromCookies(filtered);
    if (tokenFromCookie && tokenFromCookie !== authState.token) {
      authState.token = tokenFromCookie;
    }

    sendAuthUpdate();
  } catch (error) {
    console.error('[Auth Monitor] 获取 Cookie 失败', error);
  }
};

const maybeCloseLoginWindow = () => {
  if (loginWindow && !loginWindow.isDestroyed() && authState.token && authState.rawCookieHeader) {
    loginWindow.close();
  }
};

const registerLoginWindowEvents = (targetWindow) => {
  const contents = targetWindow.webContents;
  const loginSession = contents.session;

  const handleUrl = (navigatedUrl) => {
    if (!navigatedUrl) {
      return;
    }
    authState.lastUrl = navigatedUrl;
    const token = parseTokenFromUrl(navigatedUrl);
    if (token) {
      authState.token = token;
    }
    collectCookies(loginSession).finally(() => {
      maybeCloseLoginWindow();
    });
  };

  const navHandlers = {
    'will-redirect': (_event, url) => handleUrl(url),
    'did-navigate': (_event, url) => handleUrl(url),
    'did-navigate-in-page': (_event, url) => handleUrl(url),
    'did-frame-navigate': (_event, url) => handleUrl(url)
  };

  Object.entries(navHandlers).forEach(([event, handler]) => {
    contents.on(event, handler);
  });

  const cookieListener = (_event, cookie, _cause, removed) => {
    if (removed || !isZuhaowanCookie(cookie.domain)) {
      return;
    }
    collectCookies(loginSession).finally(() => {
      maybeCloseLoginWindow();
    });
  };

  loginSession.cookies.on('changed', cookieListener);

  targetWindow.once('closed', () => {
    Object.entries(navHandlers).forEach(([event, handler]) => {
      contents.removeListener(event, handler);
    });
    loginSession.cookies.removeListener('changed', cookieListener);
    loginWindow = null;
  });
};

const createLoginWindow = () => {
  if (loginWindow && !loginWindow.isDestroyed()) {
    loginWindow.focus();
    return;
  }

  loginWindow = new BrowserWindow({
    parent: mainWindow,
    width: 1100,
    height: 760,
    title: '租号玩登录',
    autoHideMenuBar: true,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      partition: 'persist:zuhaowan-session'
    }
  });

  registerLoginWindowEvents(loginWindow);

  loginWindow.loadURL(LOGIN_URL).catch((error) => {
    console.error('[Auth Monitor] 加载登录页失败', error);
  });
};

const createMainWindow = () => {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    minWidth: 900,
    minHeight: 600,
    title: 'Electron + Vue3 登录演示',
    autoHideMenuBar: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.cjs'),
      nodeIntegration: false,
      contextIsolation: true,
      sandbox: false
    }
  });

  const devServerUrl = process.env.VITE_DEV_SERVER_URL || 'http://localhost:5173';

  if (!app.isPackaged) {
    mainWindow.loadURL(devServerUrl).catch((error) => {
      console.error('[Main] 加载 Vite 开发服务器失败', error);
    });
    mainWindow.webContents.openDevTools({ mode: 'detach' });
  } else {
    const indexPath = path.join(app.getAppPath(), 'dist/renderer/index.html');
    mainWindow.loadFile(indexPath).catch((error) => {
      console.error('[Main] 加载构建后的页面失败', error);
    });
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
};

app.whenReady().then(() => {
  createMainWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createMainWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

ipcMain.handle('auth:start-login', async () => {
  createLoginWindow();
  return getAuthSnapshot();
});

ipcMain.handle('auth:get-state', async () => getAuthSnapshot());
