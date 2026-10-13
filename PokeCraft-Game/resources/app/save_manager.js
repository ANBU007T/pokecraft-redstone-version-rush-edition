// ==========================================================================
// PokéCRAFT PC Edition - Save State & Manual Slot Manager (6 Slots)
// High-Speed Isolated Desktop Storage with Live Screenshots & Slot Management
// ==========================================================================

(() => {
  const TOTAL_SLOTS = 6;

  function getStoragePrefix() {
    return 'pokecraft_pc_slot_';
  }

  function getActiveSlot() {
    const val = localStorage.getItem('pokecraft_pc_active_slot') || localStorage.getItem('pokecraft_active_slot');
    if (val) return Number(val);
    return 1;
  }

  function setActiveSlot(slotId) {
    localStorage.setItem('pokecraft_pc_active_slot', String(slotId));
    localStorage.setItem('pokecraft_active_slot', String(slotId));
    activeSlot = Number(slotId);
  }

  function getSlotData(slotId) {
    const prefix = getStoragePrefix();
    let raw = localStorage.getItem(prefix + slotId);
    if (!raw) {
      raw = localStorage.getItem('pokecraft_state_slot_' + slotId);
    }
    return raw;
  }

  let activeSlot = getActiveSlot();

  // Toast notification system
  function showEmulatorToast(msg, isSuccess = true) {
    let toast = document.getElementById('emulator-toast');
    if (!toast) {
      toast = document.createElement('div');
      toast.id = 'emulator-toast';
      document.body.appendChild(toast);
    }
    toast.textContent = msg;
    toast.className = isSuccess ? 'toast-success show' : 'toast-error show';
    clearTimeout(toast._timer);
    toast._timer = setTimeout(() => {
      toast.className = toast.className.replace('show', '').trim();
    }, 2800);
  }

  // Capture canvas thumbnail (3:2 ratio)
  function captureThumbnail() {
    try {
      const canvas = document.getElementById('game');
      if (!canvas) return null;
      const thumb = document.createElement('canvas');
      thumb.width = 160;
      thumb.height = 106;
      const tctx = thumb.getContext('2d');
      tctx.imageSmoothingEnabled = false;
      tctx.drawImage(canvas, 0, 0, thumb.width, thumb.height);
      return thumb.toDataURL('image/jpeg', 0.65);
    } catch (e) {
      return null;
    }
  }

  // Gather current save state
  async function gatherSaveState() {
    if (window.__pokeGame && window.__pokeGame.save) {
      try {
        await Promise.resolve(window.__pokeGame.save());
      } catch (e) {}
    } else if (window.__pokeGameSave) {
      try {
        await Promise.resolve(window.__pokeGameSave());
      } catch (e) {}
    }

    const stateData = {};
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && (key.startsWith('poke-') || key.startsWith('pokecraft-'))) {
        stateData[key] = localStorage.getItem(key);
      }
    }

    const now = new Date();
    const dateFormatted = now.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }) + ', ' +
                          now.toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' });

    const info = {
      timestamp: Date.now(),
      dateStr: dateFormatted,
      thumbnail: captureThumbnail(),
      location: 'Classic Kanto',
      playTime: '0m',
      health: 20,
      level: 1,
      partyCount: 0
    };

    try {
      const mainSaveRaw = localStorage.getItem('poke-redstone-v1') || localStorage.getItem('poke-redstone');
      if (mainSaveRaw) {
        const parsed = JSON.parse(mainSaveRaw);
        if (parsed.P) {
          info.level = parsed.P.level || 1;
          info.health = parsed.P.health !== undefined ? parsed.P.health : 20;
          info.food = parsed.P.food !== undefined ? parsed.P.food : 20;
        }
        if (parsed.playTime) {
          const mins = Math.floor(parsed.playTime / 60);
          info.playTime = `${mins}m`;
        }
        if (parsed.world && parsed.world.name) {
          info.location = parsed.world.name;
        } else if (parsed.layoutId) {
          info.location = 'Kanto (' + parsed.layoutId + ')';
        }
        if (Array.isArray(parsed.party)) {
          info.partyCount = parsed.party.length;
        }
      }
    } catch (e) {}

    return {
      version: 1,
      profile: 'pc',
      info,
      storage: stateData
    };
  }

  // Save to slot
  async function saveToSlot(slotId) {
    try {
      const bundle = await gatherSaveState();
      bundle.profile = 'pc';
      const prefix = getStoragePrefix();
      localStorage.setItem(prefix + slotId, JSON.stringify(bundle));
      setActiveSlot(slotId);
      showEmulatorToast(`💾 Saved to PC Slot ${slotId}!`);
      updateSlotsUI();
      updateSaveDropdownSlots();
      return true;
    } catch (e) {
      console.error(e);
      showEmulatorToast(`❌ Save to Slot ${slotId} Failed!`, false);
      return false;
    }
  }

  // Load from slot
  async function loadFromSlot(slotId) {
    try {
      const raw = getSlotData(slotId);
      if (!raw) {
        showEmulatorToast(`⚠️ PC Slot ${slotId} is Empty!`, false);
        return false;
      }
      const bundle = JSON.parse(raw);
      if (!bundle || !bundle.storage) {
        throw new Error('Invalid slot format');
      }

      for (const [k, v] of Object.entries(bundle.storage)) {
        localStorage.setItem(k, v);
      }

      setActiveSlot(slotId);
      showEmulatorToast(`📂 Loading PC Slot ${slotId}...`);

      setTimeout(() => {
        location.reload();
      }, 500);
      return true;
    } catch (e) {
      console.error(e);
      showEmulatorToast(`❌ Error Loading Slot ${slotId}`, false);
      return false;
    }
  }

  // Delete slot
  function deleteSlot(slotId) {
    if (confirm(`Clear and delete PC Save Slot ${slotId}?`)) {
      const prefix = getStoragePrefix();
      localStorage.removeItem(prefix + slotId);
      localStorage.removeItem('pokecraft_state_slot_' + slotId);
      showEmulatorToast(`🗑️ Cleared PC Slot ${slotId}`);
      updateSlotsUI();
      updateSaveDropdownSlots();
    }
  }

  // Export slot to file (.sav)
  function exportSlot(slotId) {
    const raw = getSlotData(slotId);
    if (!raw) {
      alert(`Slot ${slotId} is empty! Please save to this slot first.`);
      return;
    }
    const blob = new Blob([raw], { type: 'application/json' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = `PokeCraft_PC_Slot_${slotId}_${new Date().toISOString().slice(0, 10)}.sav`;
    a.click();
    showEmulatorToast(`Exported PC Slot ${slotId} Save File`);
  }

  // Import save from file
  function importSaveFile(file, slotId) {
    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        const text = e.target.result;
        const parsed = JSON.parse(text);
        if (!parsed.storage && !parsed.v) {
          throw new Error('Invalid save file');
        }
        let bundle;
        if (parsed.storage) {
          bundle = parsed;
          bundle.profile = profile;
        } else {
          bundle = {
            version: 1,
            profile: profile,
            info: {
              timestamp: Date.now(),
              dateStr: new Date().toLocaleDateString(undefined, { month: 'short', day: 'numeric' }) + ' (Imported)',
              thumbnail: null,
              location: 'Imported Save',
              playTime: 'Loaded',
              health: 20,
              level: 1
            },
            storage: { 'poke-redstone-v1': text }
          };
        }
        const prefix = getStoragePrefix(profile);
        localStorage.setItem(prefix + slotId, JSON.stringify(bundle));
        showEmulatorToast(`📥 Imported Save into ${profile === 'mobile' ? 'Mobile' : 'PC'} Slot ${slotId}!`);
        updateSlotsUI();
        updateSaveDropdownSlots();
      } catch (err) {
        alert('Could not read save file. Please select a valid PokeCraft .sav or .json file.');
      }
    };
    reader.readAsText(file);
  }

  // Modal open & close
  const modal = document.getElementById('save-slots-modal');
  const btnClose = document.getElementById('save-slots-close-btn');

  function openSaveSlotsModal() {
    window.__settingsModalOpen = true;
    if (modal) {
      modal.classList.add('active');
      updateSlotsUI();
    }
  }

  function closeSaveSlotsModal() {
    window.__settingsModalOpen = false;
    if (modal) {
      modal.classList.remove('active');
      const canvas = document.getElementById('game');
      if (canvas) canvas.focus();
    }
  }

  if (btnClose) btnClose.addEventListener('click', closeSaveSlotsModal);

  // Render slots list in modal dialog (6 Slots)
  function updateSlotsUI() {
    const container = document.getElementById('slots-grid-container');
    if (!container) return;
    container.innerHTML = '';

    const slotsGrid = document.createElement('div');
    slotsGrid.className = 'modal-slots-inner-grid';

    for (let i = 1; i <= TOTAL_SLOTS; i++) {
      const raw = getSlotData(i);
      const slotCard = document.createElement('div');
      slotCard.className = `slot-card ${i === activeSlot ? 'active-slot' : ''}`;

      if (raw) {
        let slotData;
        try { slotData = JSON.parse(raw); } catch (e) {}
        const info = (slotData && slotData.info) || {};

        slotCard.innerHTML = `
          <div class="slot-thumb-box">
            ${info.thumbnail ? `<img src="${info.thumbnail}" class="slot-thumb-img" alt="Slot ${i}">` : `<div class="slot-thumb-empty">NO IMAGE</div>`}
            <span class="slot-num-badge">SLOT ${i}</span>
          </div>
          <div class="slot-info-box">
            <div class="slot-header-row">
              <span class="slot-name">Slot ${i} · ${info.location || 'Classic Kanto'}</span>
              ${i === activeSlot ? `<span class="active-badge">CURRENT</span>` : ''}
            </div>
            <div class="slot-meta-row">📅 ${info.dateStr || 'Unknown date'}</div>
            <div class="slot-meta-row">
              <span>❤️ ${info.health !== undefined ? info.health : 20}/20</span> ·
              <span>⭐ Lv.${info.level || 1}</span> ·
              <span>⏱️ ${info.playTime || '0m'}</span>
              ${info.partyCount ? ` · <span>🎒 ${info.partyCount} Mon</span>` : ''}
            </div>
            <div class="slot-actions">
              <button class="btn-slot-save" data-slot="${i}">💾 Overwrite</button>
              <button class="btn-slot-load" data-slot="${i}">📂 Load</button>
              <button class="btn-slot-export" data-slot="${i}" title="Export .sav">⬇ Export</button>
              <button class="btn-slot-del" data-slot="${i}" title="Delete slot">🗑️</button>
            </div>
          </div>
        `;
      } else {
        slotCard.innerHTML = `
          <div class="slot-thumb-box">
            <div class="slot-thumb-empty">[ EMPTY SLOT ]</div>
            <span class="slot-num-badge">SLOT ${i}</span>
          </div>
          <div class="slot-info-box">
            <div class="slot-header-row">
              <span class="slot-name">Save Slot ${i}</span>
              <span class="empty-badge">EMPTY</span>
            </div>
            <div class="slot-meta-row">No save data in this slot</div>
            <div class="slot-actions">
              <button class="btn-slot-save btn-highlight" data-slot="${i}">💾 Save Game Here</button>
              <label class="btn-slot-import-label" title="Import .sav file">
                📥 Import
                <input type="file" class="input-import-file" data-slot="${i}" accept=".sav,.json" style="display:none;">
              </label>
            </div>
          </div>
        `;
      }

      slotsGrid.appendChild(slotCard);
    }
    container.appendChild(slotsGrid);

    // Event listeners in modal
    container.querySelectorAll('.btn-slot-save').forEach(b => {
      b.addEventListener('click', () => saveToSlot(Number(b.dataset.slot)));
    });
    container.querySelectorAll('.btn-slot-load').forEach(b => {
      b.addEventListener('click', () => loadFromSlot(Number(b.dataset.slot)));
    });
    container.querySelectorAll('.btn-slot-export').forEach(b => {
      b.addEventListener('click', () => exportSlot(Number(b.dataset.slot)));
    });
    container.querySelectorAll('.btn-slot-del').forEach(b => {
      b.addEventListener('click', () => deleteSlot(Number(b.dataset.slot)));
    });
    container.querySelectorAll('.input-import-file').forEach(inp => {
      inp.addEventListener('change', (e) => {
        if (e.target.files && e.target.files[0]) {
          importSaveFile(e.target.files[0], Number(inp.dataset.slot));
        }
      });
    });
  }

  // Update Save ▾ dropdown menu with 6 slots, screenshots, and live details
  function updateSaveDropdownSlots() {
    const btn = document.getElementById('menu-save-dropdown-btn');
    if (btn) {
      btn.innerHTML = `<span>💾 Save [Slot ${activeSlot}] ▾</span>`;
    }

    const listEl = document.getElementById('save-dropdown-slots-list');
    if (!listEl) return;
    listEl.innerHTML = '';

    for (let i = 1; i <= TOTAL_SLOTS; i++) {
      const raw = getSlotData(i);
      const row = document.createElement('div');
      row.className = `save-menu-slot-row ${i === activeSlot ? 'active' : ''} ${raw ? 'has-data' : 'empty'}`;
      row.dataset.slot = String(i);

      let info = {};
      if (raw) {
        try {
          const parsed = JSON.parse(raw);
          info = parsed.info || {};
        } catch (e) {}
      }

      row.innerHTML = `
        <div class="save-slot-thumb-container">
          ${info.thumbnail
            ? `<img src="${info.thumbnail}" class="save-slot-thumb-img" alt="Slot ${i} Screenshot">`
            : `<div class="save-slot-thumb-empty"><span class="save-empty-icon">📁</span><span>EMPTY</span></div>`
          }
          <span class="save-slot-badge-num">SLOT ${i}</span>
        </div>

        <div class="save-slot-details-container">
          <div class="save-slot-row-top">
            <span class="save-slot-title">Slot ${i} · ${info.location || (raw ? 'Classic Kanto' : 'Empty')}</span>
            ${i === activeSlot ? `<span class="save-slot-current-badge">CURRENT</span>` : ''}
          </div>
          <div class="save-slot-row-date">📅 ${info.dateStr || 'Not saved yet'}</div>
          ${raw ? `
            <div class="save-slot-row-stats">
              <span>❤️ ${info.health !== undefined ? info.health : 20}/20</span>
              <span>⭐ Lv.${info.level || 1}</span>
              <span>⏱️ ${info.playTime || '0m'}</span>
              ${info.partyCount ? `<span>🎒 ${info.partyCount} Mon</span>` : ''}
            </div>
          ` : `
            <div class="save-slot-row-empty-hint">Click [💾 Save] to store game state</div>
          `}
        </div>

        <div class="save-slot-actions-col">
          <button type="button" class="btn-slot-quick-save" data-slot="${i}" title="Save Game State to Slot ${i}">💾 Save</button>
          ${raw ? `<button type="button" class="btn-slot-quick-load" data-slot="${i}" title="Load State from Slot ${i}">📂 Load</button>` : ''}
        </div>
      `;

      // Clicking row body selects this slot
      row.addEventListener('click', (e) => {
        if (e.target.closest('.btn-slot-quick-save') || e.target.closest('.btn-slot-quick-load')) return;
        setActiveSlot(i);
        updateSaveDropdownSlots();
        updateSlotsUI();
        showEmulatorToast(`Selected Target Slot ${i}`);
      });

      // Quick save button
      row.querySelector('.btn-slot-quick-save')?.addEventListener('click', async (e) => {
        e.stopPropagation();
        await saveToSlot(i);
      });

      // Quick load button
      row.querySelector('.btn-slot-quick-load')?.addEventListener('click', async (e) => {
        e.stopPropagation();
        await loadFromSlot(i);
      });

      listEl.appendChild(row);
    }
  }

  // Dropdown Positioning Engine (PC directly beneath Save ▾, Mobile directly beneath Top Bar / John Save)
  function positionSaveDropdown() {
    const menu = document.getElementById('save-dropdown-menu');
    if (!menu) return;

    const isMobileEmu = document.body.classList.contains('emu-layout-active');
    if (isMobileEmu) {
      const johnBtn = document.getElementById('john-btn-save') || document.getElementById('john-top-bar');
      if (johnBtn) {
        const r = johnBtn.getBoundingClientRect();
        menu.style.top = (r.bottom + 6) + 'px';
        const maxW = menu.offsetWidth || 390;
        const leftCoord = Math.max(8, Math.min(window.innerWidth - maxW - 8, r.left));
        menu.style.left = leftCoord + 'px';
        menu.style.right = 'auto';
      } else {
        menu.style.top = '48px';
        menu.style.left = '12px';
        menu.style.right = 'auto';
      }
    } else {
      const pcBtn = document.getElementById('menu-save-dropdown-btn');
      if (pcBtn) {
        const r = pcBtn.getBoundingClientRect();
        menu.style.top = (r.bottom + 2) + 'px';
        const rightOffset = Math.max(8, window.innerWidth - r.right);
        menu.style.right = rightOffset + 'px';
        menu.style.left = 'auto';
      }
    }
  }

  // Dropdown Open / Pin Toggle
  function toggleSaveDropdown(forceOpen) {
    const menu = document.getElementById('save-dropdown-menu');
    if (!menu) return;
    const isShowing = menu.classList.contains('show');
    const targetState = forceOpen !== undefined ? forceOpen : !isShowing;

    if (!targetState) {
      menu.classList.remove('show');
    } else {
      updateSaveDropdownSlots();
      menu.classList.add('show');
      positionSaveDropdown();
      requestAnimationFrame(positionSaveDropdown);
    }
  }

  function closeSaveDropdown() {
    const menu = document.getElementById('save-dropdown-menu');
    if (menu) menu.classList.remove('show');
  }

  // Initialize Save ▾ Dropdown on Right Side
  function initSaveDropdownControls() {
    updateSaveDropdownSlots();

    const pcBtn = document.getElementById('menu-save-dropdown-btn');
    const menu = document.getElementById('save-dropdown-menu');
    let hoverTimeout = null;

    if (pcBtn && menu) {
      // Hover: open dropdown directly beneath Save ▾
      pcBtn.addEventListener('mouseenter', () => {
        clearTimeout(hoverTimeout);
        toggleSaveDropdown(true);
      });

      pcBtn.addEventListener('mouseleave', () => {
        clearTimeout(hoverTimeout);
        hoverTimeout = setTimeout(() => {
          if (!menu.matches(':hover')) {
            closeSaveDropdown();
          }
        }, 220);
      });

      menu.addEventListener('mouseenter', () => {
        clearTimeout(hoverTimeout);
      });

      menu.addEventListener('mouseleave', () => {
        clearTimeout(hoverTimeout);
        hoverTimeout = setTimeout(() => {
          if (!pcBtn.matches(':hover')) {
            closeSaveDropdown();
          }
        }, 220);
      });

      // Click: toggle pin open/close
      pcBtn.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        clearTimeout(hoverTimeout);
        toggleSaveDropdown();
      });
    }

    // Window resize reposition
    window.addEventListener('resize', () => {
      if (menu && menu.classList.contains('show')) {
        positionSaveDropdown();
      }
    });

    // Top action buttons in dropdown
    document.getElementById('save-menu-quicksave')?.addEventListener('click', async (e) => {
      e.stopPropagation();
      await saveToSlot(activeSlot);
    });

    document.getElementById('save-menu-quickload')?.addEventListener('click', async (e) => {
      e.stopPropagation();
      await loadFromSlot(activeSlot);
    });

    document.getElementById('save-menu-show-details')?.addEventListener('click', (e) => {
      e.stopPropagation();
      closeSaveDropdown();
      openSaveSlotsModal();
    });

    document.getElementById('save-menu-export-file')?.addEventListener('click', (e) => {
      e.stopPropagation();
      exportSlot(activeSlot);
    });

    document.getElementById('save-menu-import-file')?.addEventListener('change', (e) => {
      if (e.target.files && e.target.files[0]) {
        importSaveFile(e.target.files[0], activeSlot);
      }
    });

    // Close dropdown on outside click
    document.addEventListener('click', (e) => {
      const curMenu = document.getElementById('save-dropdown-menu');
      const curBtn = document.getElementById('menu-save-dropdown-btn');
      if (!curMenu || !curMenu.classList.contains('show')) return;
      if (curMenu.contains(e.target) || (curBtn && curBtn.contains(e.target))) {
        return;
      }
      closeSaveDropdown();
    });
  }

  window.addEventListener('DOMContentLoaded', initSaveDropdownControls);
  setTimeout(initSaveDropdownControls, 300);

  // Global hotkeys: F5 (Quick Save) & F8 (Quick Load) & F6 (Open Slot Manager)
  window.addEventListener('keydown', (e) => {
    if (e.key === 'F5') {
      e.preventDefault();
      saveToSlot(activeSlot);
      return;
    }
    if (e.key === 'F8') {
      e.preventDefault();
      loadFromSlot(activeSlot);
      return;
    }
    if (e.key === 'F6') {
      e.preventDefault();
      if (modal && modal.classList.contains('active')) closeSaveSlotsModal();
      else openSaveSlotsModal();
      return;
    }
    if (e.key === 'Escape') {
      closeSaveDropdown();
      if (modal && modal.classList.contains('active')) closeSaveSlotsModal();
    }
  });

  // Export functions to global scope
  window.__pokeSaveManager = {
    saveToSlot: (s) => saveToSlot(s || activeSlot),
    loadFromSlot: (s) => loadFromSlot(s || activeSlot),
    exportSlot: (s) => exportSlot(s || activeSlot),
    importSaveFile: (f, s) => importSaveFile(f, s || activeSlot),
    openModal: openSaveSlotsModal,
    closeModal: closeSaveSlotsModal,
    toggleDropdown: toggleSaveDropdown,
    closeDropdown: closeSaveDropdown,
    getActiveSlot: () => activeSlot,
    setActiveSlot: (s) => setActiveSlot(s),
    updateSlotsUI,
    updateSaveDropdownSlots
  };
})();
