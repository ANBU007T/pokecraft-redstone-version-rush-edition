// PokéCRAFT - GameBoy Emulator Controls Remapper & Key Configurator
// Full support for Mouse Left/Right/Middle/Macro keys, Control, Shift, and custom keyboard keys

(() => {
  const modal = document.getElementById('controls-modal');
  const btnClose = document.getElementById('controls-close-btn');
  const btnReset = document.getElementById('controls-reset-btn');
  const container = document.getElementById('controls-grid-container');

  let currentEditingAction = null;
  let activeRemapCleanup = null;

  function formatKeyName(key) {
    if (!key) return 'None';
    if (key === ' ') return 'Spacebar';
    if (key === 'Control' || key === 'Ctrl') return 'Ctrl';
    if (key === 'Shift') return 'Shift';
    if (key === 'Alt') return 'Alt';
    if (key === 'MouseLeft' || key === 'Mouse0' || key === 'LeftClick') return '🖱️ Left Click';
    if (key === 'MouseRight' || key === 'Mouse2' || key === 'RightClick') return '🖱️ Right Click';
    if (key === 'MouseMiddle' || key === 'Mouse1' || key === 'MiddleClick') return '🖱️ Middle Click';
    if (key === 'Mouse4' || key === 'Mouse3') return '🖱️ Mouse 4 (Back)';
    if (key === 'Mouse5') return '🖱️ Mouse 5 (Fwd)';
    if (key === 'ArrowUp') return '↑ Up';
    if (key === 'ArrowDown') return '↓ Down';
    if (key === 'ArrowLeft') return '← Left';
    if (key === 'ArrowRight') return '→ Right';
    if (key.startsWith('Numpad')) return 'Num ' + key.replace('Numpad', '');
    if (key.length === 1) return key.toUpperCase();
    return key;
  }

  function updateGuideHintText() {
    const hintEl = document.getElementById('hint');
    if (!hintEl || !window.__pokeControls) return;

    const b = window.__pokeControls.getBindings();
    const fmt = (act, fallback) => {
      const keys = b[act] || [];
      if (!keys.length) return fallback || '';
      return keys.slice(0, 2).map(k => formatKeyName(k)).filter(Boolean).join(' / ');
    };

    const attackKey = fmt('a', 'Left Click / Z');
    const placeKey = fmt('b', 'Right Click / X');
    const jumpKey = fmt('jump', 'Space');
    const sprintKey = fmt('sprint', 'Shift');
    const invKey = fmt('inv', 'E');
    const dropKey = fmt('drop', 'Q');
    const mapKey = fmt('map', 'M');
    const menuKey = fmt('start', 'Enter');

    hintEl.innerHTML = `<b>WASD/Arrows</b> move · <b>${attackKey}</b> mine/attack · <b>${placeKey}</b> place/eat · <b>${jumpKey}</b> jump · <b>${sprintKey}</b> sprint · <b>${invKey}</b> inventory · <b>${dropKey}</b> drop · <b>1–9/Wheel</b> hotbar · <b>${menuKey}</b> menu · <b>${mapKey}</b> town map · <b>F1</b> settings · <b>F2</b> remap keys`;
  }
  window.__updateGuideHintText = updateGuideHintText;

  function renderControls() {
    if (!container || !window.__pokeControls) return;
    container.innerHTML = '';

    const bindings = window.__pokeControls.getBindings();
    const labels = window.__pokeControls.getLabels();

    const sections = [
      {
        title: '🎮 MAIN GAME CONTROLS',
        actions: ['up', 'down', 'left', 'right', 'a', 'b', 'start', 'select']
      },
      {
        title: '⛏️ MINECRAFT ACTION CONTROLS & SPRINT',
        actions: ['jump', 'sprint', 'inv', 'drop', 'prev', 'next', 'map', 'mute']
      }
    ];

    sections.forEach(sec => {
      const secHeader = document.createElement('div');
      secHeader.className = 'controls-section-header';
      secHeader.textContent = sec.title;
      container.appendChild(secHeader);

      const grid = document.createElement('div');
      grid.className = 'controls-remap-grid';

      sec.actions.forEach(act => {
        const row = document.createElement('div');
        row.className = `remap-row ${currentEditingAction === act ? 'editing' : ''}`;

        const keys = bindings[act] || [];
        const mainKey = keys[0] || '';
        const altKey = keys[1] || '';

        const isEditing = currentEditingAction === act;

        row.innerHTML = `
          <div class="remap-label-wrap">
            <span class="remap-action-name">${labels[act] || act}</span>
          </div>
          <div class="remap-keys-wrap">
            <button class="btn-remap-key ${isEditing ? 'pulse' : ''}" data-action="${act}">
              ${isEditing ? 'Press key or Click...' : formatKeyName(mainKey)}
            </button>
            ${(!isEditing && altKey) ? `<span class="remap-alt-key">or ${formatKeyName(altKey)}</span>` : ''}
          </div>
          ${isEditing ? `
            <div class="remap-quick-bar">
              <span class="remap-quick-title">Quick Select:</span>
              <button type="button" class="remap-quick-pill" data-key="MouseLeft">🖱️ Left Click</button>
              <button type="button" class="remap-quick-pill" data-key="MouseRight">🖱️ Right Click</button>
              <button type="button" class="remap-quick-pill" data-key="MouseMiddle">🖱️ Middle Click</button>
              <button type="button" class="remap-quick-pill" data-key="Control">Ctrl</button>
              <button type="button" class="remap-quick-pill" data-key="Shift">Shift</button>
              <button type="button" class="remap-quick-pill" data-key=" ">Space</button>
              <button type="button" class="remap-quick-pill pill-cancel" data-key="cancel">✕ Cancel</button>
            </div>
          ` : ''}
        `;

        const btn = row.querySelector('.btn-remap-key');
        btn.addEventListener('click', (e) => {
          e.stopPropagation();
          startRemapping(act);
        });

        if (isEditing) {
          row.querySelectorAll('.remap-quick-pill').forEach(pill => {
            pill.addEventListener('click', (e) => {
              e.stopPropagation();
              const k = pill.dataset.key;
              if (k === 'cancel') {
                cancelRemapping();
              } else {
                applyBinding(act, k);
              }
            });
          });
        }

        grid.appendChild(row);
      });

      container.appendChild(grid);
    });
  }

  function applyBinding(action, newKey) {
    const bindings = window.__pokeControls.getBindings();

    // Check if another action uses this key, if so remove it
    for (const [otherAct, otherKeys] of Object.entries(bindings)) {
      if (otherAct !== action && otherKeys.includes(newKey)) {
        bindings[otherAct] = otherKeys.filter(k => k !== newKey);
      }
    }

    // Assign primary key
    bindings[action] = [newKey];

    // Save bindings
    window.__pokeControls.saveBindings(bindings);
    updateGuideHintText();

    cancelRemapping();

    // Toast feedback
    const toast = document.getElementById('emulator-toast');
    if (toast) {
      toast.textContent = `Mapped ${window.__pokeControls.getLabels()[action]} to [ ${formatKeyName(newKey)} ]`;
      toast.className = 'toast-success show';
      setTimeout(() => toast.className = '', 2500);
    }
  }

  function cancelRemapping() {
    if (activeRemapCleanup) {
      activeRemapCleanup();
      activeRemapCleanup = null;
    }
    currentEditingAction = null;
    window.__remappingKeyActive = false;
    renderControls();
  }

  function startRemapping(action) {
    if (activeRemapCleanup) {
      activeRemapCleanup();
      activeRemapCleanup = null;
    }

    currentEditingAction = action;
    window.__remappingKeyActive = true;
    renderControls();

    const onKey = (e) => {
      e.preventDefault();
      e.stopPropagation();

      if (e.key === 'Escape') {
        cancelRemapping();
        return;
      }

      // Ignore Windows key
      if (e.key === 'Meta') return;

      let keyName = e.key;
      applyBinding(action, keyName);
    };

    const onMouseDown = (e) => {
      // Allow clicking quick-pill buttons
      if (e.target && e.target.closest && e.target.closest('.remap-quick-pill')) return;

      e.preventDefault();
      e.stopPropagation();

      let mouseKey;
      if (e.button === 0) mouseKey = 'MouseLeft';
      else if (e.button === 1) mouseKey = 'MouseMiddle';
      else if (e.button === 2) mouseKey = 'MouseRight';
      else if (e.button === 3) mouseKey = 'Mouse4';
      else if (e.button === 4) mouseKey = 'Mouse5';
      else mouseKey = 'Mouse' + e.button;

      applyBinding(action, mouseKey);
    };

    const onContextMenu = (e) => {
      e.preventDefault();
      e.stopPropagation();
    };

    // Small delay so the initial button click doesn't trigger onMouseDown
    const timer = setTimeout(() => {
      window.addEventListener('keydown', onKey, true);
      window.addEventListener('mousedown', onMouseDown, true);
      window.addEventListener('contextmenu', onContextMenu, true);
    }, 80);

    activeRemapCleanup = () => {
      clearTimeout(timer);
      window.removeEventListener('keydown', onKey, true);
      window.removeEventListener('mousedown', onMouseDown, true);
      window.removeEventListener('contextmenu', onContextMenu, true);
    };
  }

  function openControlsModal() {
    window.__settingsModalOpen = true;
    if (modal) {
      modal.classList.add('active');
      renderControls();
    }
  }

  function closeControlsModal() {
    window.__settingsModalOpen = false;
    cancelRemapping();
    if (modal) {
      modal.classList.remove('active');
      const canvas = document.getElementById('game');
      if (canvas) canvas.focus();
    }
  }

  if (btnClose) btnClose.addEventListener('click', closeControlsModal);

  if (btnReset) {
    btnReset.addEventListener('click', () => {
      if (confirm('Reset all controls to default (Ctrl Sprint, Mouse Left Attack, Mouse Right Place)?')) {
        window.__pokeControls.resetDefaults();
        renderControls();
        updateGuideHintText();
        const toast = document.getElementById('emulator-toast');
        if (toast) {
          toast.textContent = 'Controls reset to default layout!';
          toast.className = 'toast-success show';
          setTimeout(() => toast.className = '', 2500);
        }
      }
    });
  }

  // Hotkey for Controls Config: F2
  window.addEventListener('keydown', (e) => {
    if (e.key === 'F2') {
      e.preventDefault();
      if (modal && modal.classList.contains('active')) closeControlsModal();
      else openControlsModal();
      return;
    }
    if (e.key === 'Escape' && modal && modal.classList.contains('active') && !currentEditingAction) {
      e.preventDefault();
      closeControlsModal();
      return;
    }
  });

  // Initial sync of hint text
  setTimeout(updateGuideHintText, 150);

  window.__pokeControlsManager = {
    openModal: openControlsModal,
    closeModal: closeControlsModal,
    renderControls,
    updateGuideHintText
  };
})();
