'use strict';

const { contextBridge, ipcRenderer } = require('electron');

const api = {
  startLoginFlow: () => ipcRenderer.invoke('auth:start-login'),
  getAuthState: () => ipcRenderer.invoke('auth:get-state'),
  onAuthData: (callback) => {
    if (typeof callback !== 'function') {
      return () => undefined;
    }
    const subscription = (_event, payload) => callback(payload);
    ipcRenderer.on('auth:data', subscription);
    return () => ipcRenderer.removeListener('auth:data', subscription);
  }
};

contextBridge.exposeInMainWorld('electronAPI', api);
