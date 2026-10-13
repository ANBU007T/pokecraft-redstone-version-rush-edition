const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('gameBridge', {
  toggleFullscreen: () => ipcRenderer.invoke('toggle-fullscreen'),
  setFullscreen: (val) => ipcRenderer.invoke('set-fullscreen', val),
  isFullscreen: () => ipcRenderer.invoke('is-fullscreen'),
  setWindowSize: (w, h) => ipcRenderer.invoke('set-window-size', w, h),
  quitGame: () => ipcRenderer.send('quit-game')
});
