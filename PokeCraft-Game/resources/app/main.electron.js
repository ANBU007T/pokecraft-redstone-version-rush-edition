const { app, BrowserWindow, protocol, net, ipcMain, shell } = require('electron');
const path = require('path');
const url = require('url');

// Enable GPU hardware acceleration flags
app.commandLine.appendSwitch('enable-gpu-rasterization');
app.commandLine.appendSwitch('enable-zero-copy');
app.commandLine.appendSwitch('ignore-gpu-blocklist');

// Offline Performance & Fast-Boot Flags (Disable background polling, telemetry & sync)
app.commandLine.appendSwitch('disable-background-networking');
app.commandLine.appendSwitch('disable-component-update');
app.commandLine.appendSwitch('disable-default-apps');
app.commandLine.appendSwitch('disable-domain-reliability');
app.commandLine.appendSwitch('disable-sync');
app.commandLine.appendSwitch('disable-breakpad');
app.commandLine.appendSwitch('disable-features', 'AutofillServerCommunication,CalculateNativeWinOcclusion');

protocol.registerSchemesAsPrivileged([
  {
    scheme: 'app',
    privileges: {
      standard: true,
      secure: true,
      supportFetchAPI: true,
      corsEnabled: true,
      stream: true
    }
  }
]);

let mainWindow = null;

function createWindow() {
  const iconPath = path.join(__dirname, 'PokeCraft.ico');

  mainWindow = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 800,
    minHeight: 533,
    title: 'PokéCRAFT - Pokemon Redstone PC Edition v1.0.1',
    icon: iconPath,
    backgroundColor: '#151515',
    autoHideMenuBar: true,
    show: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  });

  mainWindow.setMenuBarVisibility(false);

  mainWindow.webContents.on('console-message', (event, level, message, line, sourceId) => {
    console.log(`[Renderer] [${level}] ${message} (${sourceId}:${line})`);
  });

  mainWindow.webContents.setWindowOpenHandler(({ url: targetUrl }) => {
    if (targetUrl.startsWith('http:') || targetUrl.startsWith('https:')) {
      shell.openExternal(targetUrl);
      return { action: 'deny' };
    }
    return { action: 'allow' };
  });

  mainWindow.once('ready-to-show', () => {
    mainWindow.focus();
  });

  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  mainWindow.loadURL('app://game/index.html');
}

app.whenReady().then(() => {
  // Ultra-fast C++ native file streamer protocol
  protocol.handle('app', (request) => {
    const reqUrl = new URL(request.url);
    let pathname = decodeURIComponent(reqUrl.pathname);
    if (pathname === '/' || pathname === '') {
      pathname = '/index.html';
    }
    const cleanPath = pathname.replace(/^\//, '');
    const filePath = path.join(__dirname, cleanPath);
    return net.fetch(url.pathToFileURL(filePath).toString());
  });

  // IPC Handlers
  ipcMain.handle('toggle-fullscreen', () => {
    if (!mainWindow) return false;
    const nextState = !mainWindow.isFullScreen();
    mainWindow.setFullScreen(nextState);
    return mainWindow.isFullScreen();
  });

  ipcMain.handle('set-fullscreen', (event, val) => {
    if (!mainWindow) return false;
    mainWindow.setFullScreen(Boolean(val));
    return mainWindow.isFullScreen();
  });

  ipcMain.handle('is-fullscreen', () => {
    if (!mainWindow) return false;
    return mainWindow.isFullScreen();
  });

  ipcMain.handle('set-window-size', (event, w, h) => {
    if (!mainWindow) return false;
    if (mainWindow.isFullScreen()) {
      mainWindow.setFullScreen(false);
    }
    mainWindow.setSize(Number(w), Number(h));
    mainWindow.center();
    return true;
  });

  ipcMain.on('quit-game', () => {
    app.quit();
  });

  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
