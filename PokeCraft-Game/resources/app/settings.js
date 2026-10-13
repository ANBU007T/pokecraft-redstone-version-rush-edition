// PokéCRAFT PC Edition - Game Settings & Native Window Manager

(() => {
  const modal = document.getElementById('settings-modal');
  const btnSettings = document.getElementById('hud-settings-btn');
  const btnFullscreen = document.getElementById('hud-fullscreen-btn');
  const btnCloseModal = document.getElementById('settings-close-btn');
  const btnResume = document.getElementById('btn-resume-game');
  const btnQuit = document.getElementById('btn-quit-game');

  // Display elements
  const btnModeFs = document.getElementById('opt-mode-fullscreen');
  const btnModeWin = document.getElementById('opt-mode-windowed');
  const scaleBtns = document.querySelectorAll('.opt-scale-btn');
  const resBtns = document.querySelectorAll('.opt-res-btn');

  // Audio elements
  const masterSlider = document.getElementById('slider-master-vol');
  const masterVal = document.getElementById('val-master-vol');
  const musicSlider = document.getElementById('slider-music-vol');
  const musicVal = document.getElementById('val-music-vol');
  const sfxSlider = document.getElementById('slider-sfx-vol');
  const sfxVal = document.getElementById('val-sfx-vol');
  const musicMuteCheck = document.getElementById('check-music-mute');

  // Tab elements
  const tabBtns = document.querySelectorAll('.tab-btn');
  const tabPanes = document.querySelectorAll('.tab-pane');

  let isFullscreen = false;

  // Initialize tabs
  tabBtns.forEach((btn) => {
    btn.addEventListener('click', () => {
      tabBtns.forEach((b) => b.classList.remove('active'));
      tabPanes.forEach((p) => p.classList.remove('active'));
      btn.classList.add('active');
      const targetPane = document.getElementById(btn.dataset.tab);
      if (targetPane) targetPane.classList.add('active');
    });
  });

  // Modal open / close
  function openSettings() {
    window.__settingsModalOpen = true;
    modal.classList.add('active');
    syncSettingsUI();
  }

  function closeSettings() {
    window.__settingsModalOpen = false;
    modal.classList.remove('active');
    const gameCanvas = document.getElementById('game');
    if (gameCanvas) gameCanvas.focus();
    if (window.__refitGame) window.__refitGame();
  }

  btnSettings.addEventListener('click', openSettings);
  if (btnCloseModal) btnCloseModal.addEventListener('click', closeSettings);
  if (btnResume) btnResume.addEventListener('click', closeSettings);

  // Fullscreen management
  async function setFullscreenState(enable) {
    isFullscreen = !!enable;
    window.__pywebviewFullscreen = isFullscreen;

    if (window.gameBridge && window.gameBridge.setFullscreen) {
      try {
        await window.gameBridge.setFullscreen(isFullscreen);
      } catch (err) {
        console.warn('Native gameBridge fullscreen error:', err);
      }
    } else if (window.pywebview && window.pywebview.api && window.pywebview.api.set_fullscreen) {
      try {
        await window.pywebview.api.set_fullscreen(isFullscreen);
      } catch (err) {
        console.warn('Native fullscreen error:', err);
      }
    } else {
      // Browser fallback
      if (isFullscreen) {
        if (!document.fullscreenElement && document.documentElement.requestFullscreen) {
          try { await document.documentElement.requestFullscreen(); } catch (e) {}
        }
      } else {
        if (document.fullscreenElement && document.exitFullscreen) {
          try { await document.exitFullscreen(); } catch (e) {}
        }
      }
    }

    if (isFullscreen) {
      document.body.classList.add('game-fullscreen');
      btnFullscreen.innerHTML = '🗗 Window [F11]';
    } else {
      document.body.classList.remove('game-fullscreen');
      btnFullscreen.innerHTML = '⛶ Fullscreen [F11]';
    }

    updateModeBtns();
    try { localStorage.setItem('pokecraft-fullscreen', isFullscreen ? '1' : '0'); } catch(e){}
    setTimeout(() => { if (window.__refitGame) window.__refitGame(); }, 50);
  }

  async function toggleFullscreen() {
    await setFullscreenState(!isFullscreen);
  }

  btnFullscreen.addEventListener('click', toggleFullscreen);
  if (btnModeFs) btnModeFs.addEventListener('click', () => setFullscreenState(true));
  if (btnModeWin) btnModeWin.addEventListener('click', () => setFullscreenState(false));

  function updateModeBtns() {
    if (btnModeFs && btnModeWin) {
      if (isFullscreen) {
        btnModeFs.classList.add('active');
        btnModeWin.classList.remove('active');
      } else {
        btnModeFs.classList.remove('active');
        btnModeWin.classList.add('active');
      }
    }
  }

  // Scaling modes
  scaleBtns.forEach((btn) => {
    btn.addEventListener('click', () => {
      scaleBtns.forEach((b) => b.classList.remove('active'));
      btn.classList.add('active');
      const mode = btn.dataset.scale;
      if (window.__setGameScaleMode) window.__setGameScaleMode(mode);
    });
  });

  // Window resolution presets
  resBtns.forEach((btn) => {
    btn.addEventListener('click', async () => {
      const [w, h] = btn.dataset.res.split('x').map(Number);
      if (isFullscreen) await setFullscreenState(false);
      if (window.gameBridge && window.gameBridge.setWindowSize) {
        try {
          await window.gameBridge.setWindowSize(w, h);
        } catch (e) {
          console.warn(e);
        }
      } else if (window.pywebview && window.pywebview.api && window.pywebview.api.set_window_size) {
        try {
          await window.pywebview.api.set_window_size(w, h);
        } catch (e) {
          console.warn(e);
        }
      }
      setTimeout(() => { if (window.__refitGame) window.__refitGame(); }, 100);
    });
  });

  // Audio controls
  function updateAudioUI() {
    if (!window.__pokeAudio) return;
    const mVol = Math.round(window.__pokeAudio.getMasterVolume() * 100);
    const muVol = Math.round(window.__pokeAudio.getMusicVolume() * 100);
    const sVol = Math.round(window.__pokeAudio.getSfxVolume() * 100);
    const muted = window.__pokeAudio.isMusicMuted();

    if (masterSlider) { masterSlider.value = mVol; masterVal.textContent = mVol + '%'; }
    if (musicSlider) { musicSlider.value = muVol; musicVal.textContent = muVol + '%'; }
    if (sfxSlider) { sfxSlider.value = sVol; sfxVal.textContent = sVol + '%'; }
    if (musicMuteCheck) musicMuteCheck.checked = muted;
  }

  if (masterSlider) {
    masterSlider.addEventListener('input', (e) => {
      const v = Number(e.target.value) / 100;
      masterVal.textContent = e.target.value + '%';
      if (window.__pokeAudio) window.__pokeAudio.setMasterVolume(v);
    });
  }

  if (musicSlider) {
    musicSlider.addEventListener('input', (e) => {
      const v = Number(e.target.value) / 100;
      musicVal.textContent = e.target.value + '%';
      if (window.__pokeAudio) window.__pokeAudio.setMusicVolume(v);
    });
  }

  if (sfxSlider) {
    sfxSlider.addEventListener('input', (e) => {
      const v = Number(e.target.value) / 100;
      sfxVal.textContent = e.target.value + '%';
      if (window.__pokeAudio) window.__pokeAudio.setSfxVolume(v);
    });
  }

  if (musicMuteCheck) {
    musicMuteCheck.addEventListener('change', (e) => {
      if (window.__pokeAudio) window.__pokeAudio.setMusicMuted(e.target.checked);
    });
  }

  // Quit Game button
  if (btnQuit) {
    btnQuit.addEventListener('click', () => {
      if (confirm('Quit PokéCRAFT to Desktop?')) {
        if (window.gameBridge && window.gameBridge.quitGame) {
          window.gameBridge.quitGame();
        } else if (window.pywebview && window.pywebview.api && window.pywebview.api.quit_game) {
          window.pywebview.api.quit_game();
        } else {
          window.close();
        }
      }
    });
  }

  // Sprint Speed & Fast Movement wiring
  const sprintBtns = document.querySelectorAll('.opt-sprint-btn');
  const checkAlwaysSprint = document.getElementById('check-always-sprint');
  const checkToggleSprint = document.getElementById('check-toggle-sprint');

  function setSprintSpeedMode(mode) {
    window.__sprintSpeedMode = mode;
    try { localStorage.setItem('pokecraft-sprint-speed', mode); } catch(e){}
    sprintBtns.forEach(b => {
      if (b.dataset.sprint === mode) b.classList.add('active');
      else b.classList.remove('active');
    });
    const toast = document.getElementById('emulator-toast');
    if (toast) {
      const modeLabel = mode === 'turbo' ? 'Turbo (2.2x)' : mode === 'fast' ? 'Fast (1.85x)' : mode === 'gentle' ? 'Gentle (1.35x)' : 'Balanced (1.6x)';
      toast.textContent = `🏃 Sprint Speed: ${modeLabel}`;
      toast.className = 'toast-success show';
      setTimeout(() => toast.className = '', 2000);
    }
  }

  sprintBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      setSprintSpeedMode(btn.dataset.sprint);
    });
  });

  if (checkAlwaysSprint) {
    checkAlwaysSprint.checked = localStorage.getItem('pokecraft-always-sprint') !== '0';
    checkAlwaysSprint.addEventListener('change', (e) => {
      window.__alwaysSprint = e.target.checked;
      try { localStorage.setItem('pokecraft-always-sprint', e.target.checked ? '1' : '0'); } catch(e){}
    });
  }

  if (checkToggleSprint) {
    checkToggleSprint.checked = localStorage.getItem('pokecraft-toggle-sprint') === '1';
    checkToggleSprint.addEventListener('change', (e) => {
      window.__toggleSprint = e.target.checked;
      try { localStorage.setItem('pokecraft-toggle-sprint', e.target.checked ? '1' : '0'); } catch(e){}
    });
  }

  // Sync settings state to UI
  function syncSettingsUI() {
    updateModeBtns();
    updateAudioUI();
    const currentScale = (window.__getGameScaleMode && window.__getGameScaleMode()) || 'fit';
    scaleBtns.forEach((b) => {
      if (b.dataset.scale === currentScale) b.classList.add('active');
      else b.classList.remove('active');
    });

    const curSprint = localStorage.getItem('pokecraft-sprint-speed') || 'balanced';
    window.__sprintSpeedMode = curSprint;
    sprintBtns.forEach(b => {
      if (b.dataset.sprint === curSprint) b.classList.add('active');
      else b.classList.remove('active');
    });
  }

  // Global key bindings
  window.addEventListener('keydown', (e) => {
    // Fullscreen toggle: F11
    if (e.key === 'F11') {
      e.preventDefault();
      toggleFullscreen();
      return;
    }

    // Settings toggle: F1
    if (e.key === 'F1') {
      e.preventDefault();
      if (modal.classList.contains('active')) closeSettings();
      else openSettings();
      return;
    }

    // Escape closes modal if open
    if (e.key === 'Escape' && modal.classList.contains('active')) {
      e.preventDefault();
      closeSettings();
      return;
    }
  });

  // Listen to native browser fullscreen changes
  document.addEventListener('fullscreenchange', () => {
    const isDocFs = !!document.fullscreenElement;
    if (isDocFs !== isFullscreen) {
      isFullscreen = isDocFs;
      if (isFullscreen) document.body.classList.add('game-fullscreen');
      else document.body.classList.remove('game-fullscreen');
      updateModeBtns();
      if (window.__refitGame) window.__refitGame();
    }
  });

  // Restore saved settings on boot
  window.addEventListener('DOMContentLoaded', () => {
    try {
      if (localStorage.getItem('pokecraft-fullscreen') === '1') {
        setTimeout(() => setFullscreenState(true), 300);
      }
    } catch(e){}
    syncSettingsUI();
  });

  // Window pywebview bridge readiness
  window.addEventListener('pywebviewready', async () => {
    try {
      if (window.pywebview && window.pywebview.api && window.pywebview.api.is_fullscreen) {
        const nativeFs = await window.pywebview.api.is_fullscreen();
        if (nativeFs) setFullscreenState(true);
      }
    } catch(e){}
  });

  // Emulator Top Menu Bar Wiring
  function initEmulatorMenuBar() {
    const qs = (id) => document.getElementById(id);

    // Save & Load Actions
    qs('menu-quick-save')?.addEventListener('click', () => {
      if (window.__pokeSaveManager) window.__pokeSaveManager.saveToSlot(window.__pokeSaveManager.getActiveSlot());
    });
    qs('menu-quick-load')?.addEventListener('click', () => {
      if (window.__pokeSaveManager) window.__pokeSaveManager.loadFromSlot(window.__pokeSaveManager.getActiveSlot());
    });
    qs('menu-save-slots')?.addEventListener('click', () => {
      if (window.__pokeSaveManager) window.__pokeSaveManager.openModal();
    });

    // Slot selector dropdown
    const quickSlotSelect = qs('quick-slot-select');
    if (quickSlotSelect) {
      quickSlotSelect.addEventListener('change', (e) => {
        if (window.__pokeSaveManager) window.__pokeSaveManager.setActiveSlot(Number(e.target.value));
      });
    }

    // Import save from file
    qs('menu-import-file')?.addEventListener('change', (e) => {
      if (e.target.files && e.target.files[0] && window.__pokeSaveManager) {
        window.__pokeSaveManager.importSaveFile(e.target.files[0], window.__pokeSaveManager.getActiveSlot());
      }
    });

    // Reset & Exit
    qs('menu-reset-game')?.addEventListener('click', () => {
      if (confirm('Restart game from beginning? Any unsaved progress will be lost.')) {
        location.reload();
      }
    });
    qs('menu-exit-game')?.addEventListener('click', () => {
      if (btnQuit) btnQuit.click();
    });

    // Controls
    qs('menu-remap-keys')?.addEventListener('click', () => {
      if (window.__pokeControlsManager) window.__pokeControlsManager.openModal();
    });
    qs('btn-open-remap-from-settings')?.addEventListener('click', () => {
      closeSettings();
      if (window.__pokeControlsManager) window.__pokeControlsManager.openModal();
    });
    qs('menu-reset-keys')?.addEventListener('click', () => {
      if (confirm('Reset all controls to default layout?')) {
        if (window.__pokeControls) window.__pokeControls.resetDefaults();
        if (window.__pokeControlsManager) window.__pokeControlsManager.renderControls();
      }
    });

    // Video / Scaling
    qs('menu-toggle-fs')?.addEventListener('click', toggleFullscreen);
    qs('menu-scale-fit')?.addEventListener('click', () => {
      document.querySelector('.opt-scale-btn[data-scale="fit"]')?.click();
    });
    qs('menu-scale-crisp')?.addEventListener('click', () => {
      document.querySelector('.opt-scale-btn[data-scale="crisp"]')?.click();
    });
    qs('menu-scale-stretch')?.addEventListener('click', () => {
      document.querySelector('.opt-scale-btn[data-scale="stretch"]')?.click();
    });

    // Audio
    qs('menu-toggle-music')?.addEventListener('click', () => {
      if (window.__pokeAudio) {
        const nextMute = !window.__pokeAudio.isMusicMuted();
        window.__pokeAudio.setMusicMuted(nextMute);
        if (musicMuteCheck) musicMuteCheck.checked = nextMute;
      }
    });

    // Settings
    qs('menu-open-settings')?.addEventListener('click', openSettings);

    // Town Map
    const triggerMap = () => {
      if (window.__openPokeMap) {
        window.__openPokeMap();
      } else {
        window.dispatchEvent(new KeyboardEvent('keydown', { key: 'm', code: 'KeyM' }));
      }
    };
    qs('menu-open-map')?.addEventListener('click', triggerMap);

    // Sprint Speed items in emulator menu bar
    document.querySelectorAll('.menu-sprint-speed').forEach(btn => {
      btn.addEventListener('click', () => {
        setSprintSpeedMode(btn.dataset.sprint);
      });
    });

    // Retrieve Local LAN IP for Multiplayer Host Display
    if (window.gameBridge && window.gameBridge.getNetworkIps) {
      window.gameBridge.getNetworkIps().then((ips) => {
        if (Array.isArray(ips) && ips.length) {
          window.__localIp = ips[0];
          console.log('[Multiplayer] Local network IPs:', ips);
        }
      }).catch(() => {});
    }
  }

  window.addEventListener('DOMContentLoaded', initEmulatorMenuBar);
})();

