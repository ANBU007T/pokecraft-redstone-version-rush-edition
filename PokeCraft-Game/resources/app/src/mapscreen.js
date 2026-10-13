// PokéCRAFT PC Edition - High-DPI Kanto Town Map with Non-Pixelated Sidebar
// Exclusively Overworld Game Map with 100% exact player tracking and full guidebook details

import { KANTO_LOCATIONS } from './kanto_db.js';

export class MapScreen {
  constructor(world, player, game, opts = {}) {
    this.kind = 'map';
    this.done = false;
    this.W = world;
    this.P = player;
    this.G = game;
    this.opts = opts;
    this.time = 0;

    // Load full 51 locations from database
    this.locations = KANTO_LOCATIONS;

    // Load saved discovered locations (Fog of War)
    this.discovered = new Set(['pallet', 'route1']);
    try {
      const saved = localStorage.getItem('pokecraft_discovered_locations_v3');
      if (saved) {
        JSON.parse(saved).forEach(id => this.discovered.add(id));
      }
    } catch (e) {}

    // Fog of War 100% unlock setting
    this.revealAll = false;
    try {
      this.revealAll = localStorage.getItem('pokecraft_map_reveal_all_v3') === '1';
    } catch (e) {}

    // Map mode is exclusively High-Res Overworld Game Map
    this.mapMode = 'overworld';

    // Viewport transform (Pan and Zoom)
    this.zoom = 1.0;
    this.panX = 0;
    this.panY = 0;
    this.minZoom = 0.6;
    this.maxZoom = 4.0;
    this.isDragging = false;
    this.dragStartX = 0;
    this.dragStartY = 0;
    this.lastMouseX = 0;
    this.lastMouseY = 0;

    // Active Category Filter: 'all', 'city', 'route', 'dungeon', 'gym', 'discovered'
    this.filter = 'all';

    // Toast notification
    this.toastMsg = '';
    this.toastTime = 0;

    // Load High-Resolution Overworld Map
    this.overworldImg = new Image();
    this.overworldImg.src = 'assets/ui/kanto_overworld_map.png';

    // Real-time Steve location detection
    this.detectCurrentPlayerLocation();

    // Select player's current location initially
    this.selectedIdx = 0;
    const curLoc = this.locations.find(l => l.name.toLowerCase() === (this.currentAreaName || '').toLowerCase());
    if (curLoc) {
      this.selectedIdx = this.locations.indexOf(curLoc);
    }

    // Modal DOM Elements
    this.modalEl = document.getElementById('pokecraft-map-modal');
    this.hiresCanvas = document.getElementById('pokecraft-map-hires-canvas');
    this.viewportWrap = document.getElementById('map-viewport-wrap');
    this.sidebarEl = document.getElementById('map-sidebar');
    this.toastEl = document.getElementById('map-toast');
    this.zoomIndicatorEl = document.getElementById('map-zoom-indicator');
    this.fogBtnTextEl = document.getElementById('map-fog-btn-text');

    // Bound listeners for cleanup on close
    this.boundHandlers = [];

    // Initialize High-DPI Modal
    this.initDOM();
  }

  detectCurrentPlayerLocation() {
    const px = Math.round(this.P.fx != null ? this.P.fx : this.P.tx);
    const py = Math.round(this.P.fy != null ? this.P.fy : this.P.ty);
    this.playerCoordX = px;
    this.playerCoordY = py;

    let tileX = px;
    let tileY = py;
    this.isIndoor = false;

    // Check world indoor state and resolve via room warps
    const w = this.W;
    if (w && typeof w.isOutdoor === 'function') {
      if (!w.isOutdoor(px, py)) {
        this.isIndoor = true;
        const curMap = typeof w.mapAt === 'function' ? w.mapAt(px, py) : null;
        if (curMap && Array.isArray(w.warps)) {
          // Look for warp exiting this room to outdoors
          const exitWarp = w.warps.find(wp => wp.map === curMap.id && w.isOutdoor(wp.ax, wp.ay));
          if (exitWarp) {
            tileX = exitWarp.ax;
            tileY = exitWarp.ay;
          } else {
            // Look for entrance warp entering this room from outdoors
            const enterWarp = w.warps.find(wp => wp.dest === curMap.id && w.isOutdoor(wp.x, wp.y));
            if (enterWarp) {
              tileX = enterWarp.x;
              tileY = enterWarp.y;
            }
          }
        }
      }
    } else {
      // Fallback indoor check for Viridian PC / Mart
      if (px >= 250 && px <= 290 && py >= 670 && py <= 710) {
        this.isIndoor = true;
        tileX = 74;
        tileY = 207;
      }
    }

    // Find nearest outdoor location for guidebook card & auto-discovery
    let closest = this.locations[0];
    let closestDist = Infinity;

    for (const loc of this.locations) {
      const dist = Math.hypot(tileX - loc.wx, tileY - loc.wy);
      if (dist < closestDist) {
        closestDist = dist;
        closest = loc;
      }
    }

    this.currentAreaName = closest ? closest.name : 'Pallet Town';

    // Auto-discover location when player walks within exploration radius
    if (closest && closestDist < 35) {
      this.discovered.add(closest.id);
      this.saveDiscovered();
    }

    // Exact normalized coordinates on the 408 x 400 Kanto outdoor world
    const outdoorW = (w && w.outdoor && w.outdoor.w) || 408;
    const outdoorH = (w && w.outdoor && w.outdoor.h) || 400;
    this.liveNormX = Math.max(0, Math.min(1000, (tileX / outdoorW) * 1000));
    this.liveNormY = Math.max(0, Math.min(1000, (tileY / outdoorH) * 1000));
  }

  saveDiscovered() {
    try {
      localStorage.setItem('pokecraft_discovered_locations_v3', JSON.stringify([...this.discovered]));
    } catch (e) {}
  }

  initDOM() {
    if (!this.modalEl) return;

    // Show modal
    this.modalEl.classList.add('active');

    // Update Fog button text
    this.updateFogButton();

    // Hook Close Button
    const closeBtn = document.getElementById('map-close-btn');
    if (closeBtn) {
      const onClose = (e) => { e.stopPropagation(); this.close(); };
      closeBtn.addEventListener('click', onClose);
      this.boundHandlers.push({ el: closeBtn, ev: 'click', fn: onClose });
    }

    // Hook Fog Status Button
    const fogBtn = document.getElementById('map-fog-status-btn');
    if (fogBtn) {
      const onFogClick = (e) => {
        e.stopPropagation();
        this.toggleFogOfWar();
      };
      fogBtn.addEventListener('click', onFogClick);
      this.boundHandlers.push({ el: fogBtn, ev: 'click', fn: onFogClick });
    }

    // Hook Floating Controls
    const zoomInBtn = document.getElementById('map-zoom-in');
    if (zoomInBtn) {
      const onZoomIn = (e) => { e.stopPropagation(); this.adjustZoom(0.35); };
      zoomInBtn.addEventListener('click', onZoomIn);
      this.boundHandlers.push({ el: zoomInBtn, ev: 'click', fn: onZoomIn });
    }

    const zoomOutBtn = document.getElementById('map-zoom-out');
    if (zoomOutBtn) {
      const onZoomOut = (e) => { e.stopPropagation(); this.adjustZoom(-0.35); };
      zoomOutBtn.addEventListener('click', onZoomOut);
      this.boundHandlers.push({ el: zoomOutBtn, ev: 'click', fn: onZoomOut });
    }

    const centerSteveBtn = document.getElementById('map-center-steve');
    if (centerSteveBtn) {
      const onCenter = (e) => { e.stopPropagation(); this.centerOnSteve(); };
      centerSteveBtn.addEventListener('click', onCenter);
      this.boundHandlers.push({ el: centerSteveBtn, ev: 'click', fn: onCenter });
    }

    const resetViewBtn = document.getElementById('map-reset-view');
    if (resetViewBtn) {
      const onReset = (e) => { e.stopPropagation(); this.resetView(); };
      resetViewBtn.addEventListener('click', onReset);
      this.boundHandlers.push({ el: resetViewBtn, ev: 'click', fn: onReset });
    }

    const floatCloseBtn = document.getElementById('map-float-close');
    if (floatCloseBtn) {
      const onFloatClose = (e) => { e.stopPropagation(); this.close(); };
      floatCloseBtn.addEventListener('click', onFloatClose);
      this.boundHandlers.push({ el: floatCloseBtn, ev: 'click', fn: onFloatClose });
    }

    // Hook Filter Chips
    const chips = document.querySelectorAll('.map-filter-chip');
    chips.forEach(chip => {
      const onChipClick = (e) => {
        e.stopPropagation();
        const f = chip.getAttribute('data-filter');
        this.setFilter(f);
      };
      chip.addEventListener('click', onChipClick);
      this.boundHandlers.push({ el: chip, ev: 'click', fn: onChipClick });
    });

    // Hook Viewport Mouse Drag & Wheel
    if (this.viewportWrap) {
      const onMouseDown = (e) => {
        if (e.button !== 0) return;
        this.isDragging = true;
        this.dragStartX = e.clientX;
        this.dragStartY = e.clientY;
        this.lastMouseX = e.clientX;
        this.lastMouseY = e.clientY;
        this.hasMoved = false;
      };

      const onMouseMove = (e) => {
        if (!this.isDragging) return;
        const dx = e.clientX - this.lastMouseX;
        const dy = e.clientY - this.lastMouseY;
        if (Math.hypot(e.clientX - this.dragStartX, e.clientY - this.dragStartY) > 4) {
          this.hasMoved = true;
        }
        this.panX += dx;
        this.panY += dy;
        this.lastMouseX = e.clientX;
        this.lastMouseY = e.clientY;
        this.clampPan();
      };

      const onMouseUp = (e) => {
        if (this.isDragging && !this.hasMoved) {
          // Click hit-test on pins
          this.handlePinClick(e);
        }
        this.isDragging = false;
      };

      const onWheel = (e) => {
        e.preventDefault();
        const delta = e.deltaY < 0 ? 0.25 : -0.25;
        this.adjustZoom(delta);
      };

      this.viewportWrap.addEventListener('mousedown', onMouseDown);
      window.addEventListener('mousemove', onMouseMove);
      window.addEventListener('mouseup', onMouseUp);
      this.viewportWrap.addEventListener('wheel', onWheel, { passive: false });

      this.boundHandlers.push({ el: this.viewportWrap, ev: 'mousedown', fn: onMouseDown });
      this.boundHandlers.push({ el: window, ev: 'mousemove', fn: onMouseMove });
      this.boundHandlers.push({ el: window, ev: 'mouseup', fn: onMouseUp });
      this.boundHandlers.push({ el: this.viewportWrap, ev: 'wheel', fn: onWheel });

      // Mobile Touch Pan (1-finger) and Pinch-to-Zoom (2-finger)
      let initialPinchDist = null;
      let initialZoom = 1.0;
      let touchStartX = 0;
      let touchStartY = 0;
      let touchMoved = false;

      const onTouchStart = (e) => {
        if (e.touches.length === 1) {
          this.isDragging = true;
          touchStartX = e.touches[0].clientX;
          touchStartY = e.touches[0].clientY;
          this.lastTouchX = touchStartX;
          this.lastTouchY = touchStartY;
          touchMoved = false;
        } else if (e.touches.length === 2) {
          this.isDragging = false;
          initialPinchDist = Math.hypot(
            e.touches[0].clientX - e.touches[1].clientX,
            e.touches[0].clientY - e.touches[1].clientY
          );
          initialZoom = this.zoom;
        }
      };

      const onTouchMove = (e) => {
        if (e.cancelable) e.preventDefault();
        if (e.touches.length === 1 && this.isDragging) {
          const dx = e.touches[0].clientX - this.lastTouchX;
          const dy = e.touches[0].clientY - this.lastTouchY;
          if (Math.hypot(e.touches[0].clientX - touchStartX, e.touches[0].clientY - touchStartY) > 6) {
            touchMoved = true;
          }
          this.panX += dx;
          this.panY += dy;
          this.lastTouchX = e.touches[0].clientX;
          this.lastTouchY = e.touches[0].clientY;
          this.clampPan();
        } else if (e.touches.length === 2 && initialPinchDist) {
          const curDist = Math.hypot(
            e.touches[0].clientX - e.touches[1].clientX,
            e.touches[0].clientY - e.touches[1].clientY
          );
          const scaleFactor = curDist / initialPinchDist;
          this.zoom = Math.max(this.minZoom, Math.min(this.maxZoom, initialZoom * scaleFactor));
          this.clampPan();
          if (this.zoomIndicatorEl) {
            const zoomText = this.zoomIndicatorEl.querySelector('span:first-child');
            if (zoomText) zoomText.textContent = `🔍 ${this.zoom.toFixed(1)}x`;
          }
        }
      };

      const onTouchEnd = (e) => {
        if (e.touches.length === 0 && this.isDragging && !touchMoved) {
          this.handlePinClick({ clientX: this.lastTouchX, clientY: this.lastTouchY });
        }
        if (e.touches.length < 2) initialPinchDist = null;
        if (e.touches.length === 0) this.isDragging = false;
      };

      this.viewportWrap.addEventListener('touchstart', onTouchStart, { passive: false });
      this.viewportWrap.addEventListener('touchmove', onTouchMove, { passive: false });
      this.viewportWrap.addEventListener('touchend', onTouchEnd, { passive: false });
      this.viewportWrap.addEventListener('touchcancel', onTouchEnd, { passive: false });

      this.boundHandlers.push({ el: this.viewportWrap, ev: 'touchstart', fn: onTouchStart });
      this.boundHandlers.push({ el: this.viewportWrap, ev: 'touchmove', fn: onTouchMove });
      this.boundHandlers.push({ el: this.viewportWrap, ev: 'touchend', fn: onTouchEnd });
      this.boundHandlers.push({ el: this.viewportWrap, ev: 'touchcancel', fn: onTouchEnd });
    }

    // Hook Sidebar Steve Radar Card click
    if (this.sidebarEl) {
      const onSidebarClick = (e) => {
        const radarCard = e.target.closest('.map-steve-radar-card');
        if (radarCard) {
          e.stopPropagation();
          this.centerOnSteve();
        }
      };
      this.sidebarEl.addEventListener('click', onSidebarClick);
      this.boundHandlers.push({ el: this.sidebarEl, ev: 'click', fn: onSidebarClick });
    }

    // Hook Keyboard Escape / M
    const onKeyDown = (e) => {
      if (e.key === 'Escape' || e.key === 'm' || e.key === 'M') {
        e.preventDefault();
        e.stopPropagation();
        this.close();
        return;
      }
      if (e.key === 'Tab') {
        e.preventDefault();
        this.cycleFilter();
        return;
      }
      if (e.key === 'c' || e.key === 'C') {
        e.preventDefault();
        this.centerOnSteve();
        return;
      }
      if (e.key === '+' || e.key === '=') {
        e.preventDefault();
        this.adjustZoom(0.35);
        return;
      }
      if (e.key === '-' || e.key === '_') {
        e.preventDefault();
        this.adjustZoom(-0.35);
        return;
      }
      if (e.key === 'ArrowDown' || e.key === 's' || e.key === 'S') {
        e.preventDefault();
        this.navigateLocation(1);
        return;
      }
      if (e.key === 'ArrowUp' || e.key === 'w' || e.key === 'W') {
        e.preventDefault();
        this.navigateLocation(-1);
        return;
      }
    };

    window.addEventListener('keydown', onKeyDown, { capture: true });
    this.boundHandlers.push({ el: window, ev: 'keydown', fn: onKeyDown, capture: true });

    // Initial render of High-DPI Sidebar
    this.updateSidebar();
  }

  toggleFogOfWar() {
    const allDiscovered = this.discovered.size >= this.locations.length;
    if (allDiscovered) {
      this.revealAll = !this.revealAll;
      try {
        localStorage.setItem('pokecraft_map_reveal_all_v3', this.revealAll ? '1' : '0');
      } catch (e) {}
      this.updateFogButton();
      this.showToast(this.revealAll ? '👁️ ALL REGION VISIBLE (100% UNLOCKED)!' : '🧭 FOG OF WAR ACTIVE');
      this.updateSidebar();
    } else {
      this.showToast(`🔒 FOG OF WAR LOCKED! Discover all 51 locations (${this.discovered.size}/51) to unlock full reveal.`);
    }
  }

  updateFogButton() {
    if (!this.fogBtnTextEl) return;
    const allDiscovered = this.discovered.size >= this.locations.length;
    if (allDiscovered) {
      this.fogBtnTextEl.textContent = this.revealAll ? '🏆 100% VISIBLE [R]' : '🧭 FOG OF WAR [R]';
    } else {
      this.fogBtnTextEl.textContent = `🔒 FOG LOCKED (${this.discovered.size}/${this.locations.length})`;
    }
  }

  computeMapLayout(w, h) {
    const mapAspect = 1024 / 1003;
    let dw, dh, dx, dy;
    if (w / h > mapAspect) {
      dh = h;
      dw = h * mapAspect;
      dx = (w - dw) / 2;
      dy = 0;
    } else {
      dw = w;
      dh = w / mapAspect;
      dx = 0;
      dy = (h - dh) / 2;
    }
    return { w, h, dx, dy, dw, dh };
  }

  adjustZoom(delta) {
    const oldZoom = this.zoom;
    this.zoom = Math.max(this.minZoom, Math.min(this.maxZoom, Math.round((this.zoom + delta) * 10) / 10));
    if (oldZoom > 0) {
      this.panX = (this.panX / oldZoom) * this.zoom;
      this.panY = (this.panY / oldZoom) * this.zoom;
    }
    this.clampPan();
    if (this.zoomIndicatorEl) {
      const zoomText = this.zoomIndicatorEl.querySelector('span:first-child');
      if (zoomText) zoomText.textContent = `🔍 ${this.zoom.toFixed(1)}x`;
    }
  }

  clampPan() {
    const layout = this.lastMapLayout;
    const maxPanX = layout ? (layout.dw * this.zoom) * 0.6 : 800 * this.zoom;
    const maxPanY = layout ? (layout.dh * this.zoom) * 0.6 : 800 * this.zoom;
    this.panX = Math.max(-maxPanX, Math.min(maxPanX, this.panX));
    this.panY = Math.max(-maxPanY, Math.min(maxPanY, this.panY));
  }

  centerOnSteve() {
    this.zoom = 2.0;
    const rect = this.hiresCanvas ? this.hiresCanvas.getBoundingClientRect() : null;
    const w = rect ? rect.width : 700;
    const h = rect ? rect.height : 600;
    const layout = this.computeMapLayout(w, h);
    this.lastMapLayout = layout;

    const cx = w / 2;
    const cy = h / 2;
    const steveX = layout.dx + (this.liveNormX / 1000) * layout.dw;
    const steveY = layout.dy + (this.liveNormY / 1000) * layout.dh;

    this.panX = (cx - steveX) * this.zoom;
    this.panY = (cy - steveY) * this.zoom;
    this.clampPan();

    this.showToast(`🎯 Centered on Steve in ${this.currentAreaName}`);
    if (this.zoomIndicatorEl) {
      const zoomText = this.zoomIndicatorEl.querySelector('span:first-child');
      if (zoomText) zoomText.textContent = `🔍 ${this.zoom.toFixed(1)}x`;
    }
  }

  resetView() {
    this.zoom = 1.0;
    this.panX = 0;
    this.panY = 0;
    if (this.zoomIndicatorEl) {
      const zoomText = this.zoomIndicatorEl.querySelector('span:first-child');
      if (zoomText) zoomText.textContent = `🔍 1.0x`;
    }
  }

  cycleFilter() {
    const filters = ['all', 'city', 'route', 'dungeon', 'gym', 'discovered'];
    const nextIdx = (filters.indexOf(this.filter) + 1) % filters.length;
    this.setFilter(filters[nextIdx]);
  }

  setFilter(f) {
    this.filter = f;
    const chips = document.querySelectorAll('.map-filter-chip');
    chips.forEach(c => {
      if (c.getAttribute('data-filter') === f) c.classList.add('active');
      else c.classList.remove('active');
    });

    const flist = this.getFilteredLocations();
    if (flist.length && !flist.includes(this.locations[this.selectedIdx])) {
      this.selectedIdx = this.locations.indexOf(flist[0]);
    }
    this.updateSidebar();
  }

  getFilteredLocations() {
    if (this.filter === 'city') return this.locations.filter(l => l.type === 'city');
    if (this.filter === 'route') return this.locations.filter(l => l.type === 'route');
    if (this.filter === 'dungeon') return this.locations.filter(l => l.type === 'dungeon' || l.type === 'special');
    if (this.filter === 'gym') return this.locations.filter(l => l.gym && l.gym.length > 0);
    if (this.filter === 'discovered') return this.locations.filter(l => this.revealAll || this.discovered.has(l.id));
    return this.locations;
  }

  navigateLocation(dir) {
    const list = this.getFilteredLocations();
    if (!list.length) return;
    const currentLoc = this.locations[this.selectedIdx] || list[0];
    let idx = list.indexOf(currentLoc);
    if (idx < 0) idx = 0;
    idx = (idx + dir + list.length) % list.length;
    this.selectedIdx = this.locations.indexOf(list[idx]);
    this.updateSidebar();
    if (this.opts.sfx && this.opts.sfx.select) {
      try { this.opts.sfx.select(); } catch (e) {}
    }
  }

  handlePinClick(e) {
    if (!this.hiresCanvas || !this.lastMapLayout) return;
    const rect = this.hiresCanvas.getBoundingClientRect();
    const clickX = e.clientX - rect.left;
    const clickY = e.clientY - rect.top;

    const { w, h, dx, dy, dw, dh } = this.lastMapLayout;
    const cx = w / 2;
    const cy = h / 2;

    // Invert zoom and pan transform
    const transX = (clickX - cx - this.panX) / this.zoom + cx;
    const transY = (clickY - cy - this.panY) / this.zoom + cy;

    // Convert to normalized map coordinate (0 to 1000)
    const normX = ((transX - dx) / dw) * 1000;
    const normY = ((transY - dy) / dh) * 1000;

    let closestIdx = -1;
    let closestDist = 40; // Click tolerance in normalized units

    for (let i = 0; i < this.locations.length; i++) {
      const loc = this.locations[i];
      const dist = Math.hypot(normX - loc.sx, normY - loc.sy);
      if (dist < closestDist) {
        closestDist = dist;
        closestIdx = i;
      }
    }

    if (closestIdx >= 0) {
      this.selectedIdx = closestIdx;
      this.updateSidebar();
      if (this.opts.sfx && this.opts.sfx.select) {
        try { this.opts.sfx.select(); } catch (e) {}
      }
    }
  }

  showToast(msg) {
    this.toastMsg = msg;
    this.toastTime = 3.5;
    if (this.toastEl) {
      this.toastEl.textContent = msg;
      this.toastEl.classList.add('visible');
    }
  }

  // Update High-DPI Sidebar with Crisp, Non-Pixelated Typography
  updateSidebar() {
    if (!this.sidebarEl) return;

    const loc = this.locations[this.selectedIdx] || this.locations[0];
    const isDiscovered = this.revealAll || this.discovered.has(loc.id);

    // Pill color
    let typeClass = 'pill-city';
    if (loc.type === 'route') typeClass = 'pill-route';
    else if (loc.type === 'dungeon') typeClass = 'pill-dungeon';
    else if (loc.type === 'special') typeClass = 'pill-special';

    // Exploration calculations
    const totalCount = this.locations.length;
    const discCount = this.discovered.size;
    const discPct = Math.round((discCount / totalCount) * 100);

    // Exact block distance calculation
    const px = Math.round(this.P.fx != null ? this.P.fx : this.P.tx);
    const py = Math.round(this.P.fy != null ? this.P.fy : this.P.ty);
    const dist = Math.round(Math.hypot(px - loc.wx, py - loc.wy));

    let html = '';

    // Header Card
    html += `
      <div class="map-side-card-header">
        <div class="map-side-name-row">
          <h3 class="map-side-name">${loc.name}</h3>
          <span class="map-side-type-pill ${typeClass}">${loc.csvType || loc.type.toUpperCase()}</span>
        </div>
        ${loc.gym ? `<div class="map-gym-chip"><span>🥊</span> <span>${loc.gym}</span></div>` : ''}
      </div>
    `;

    if (!isDiscovered) {
      // Undiscovered Fog of War Area
      html += `
        <div class="map-undiscovered-card">
          <div class="map-undiscovered-title">
            <span>❓</span> <span>UNDISCOVERED TERRITORY</span>
          </div>
          <div class="map-undiscovered-desc">
            Explore near this location in the overworld to unlock full MapGenie guidebook details, wild encounter rates, and items.
          </div>
          <div class="map-dist-badge">📍 Distance from Steve: ~${dist} blocks</div>
        </div>
      `;
    } else {
      // Discovered Area: Full MapGenie and CSV details
      html += `
        <div>
          <div class="map-side-sec-title sec-highlights">
            <span>📖</span> <span>HIGHLIGHTS &amp; GUIDEBOOK</span>
          </div>
          <div class="map-highlights-body">
            ${loc.highlights || loc.desc}
          </div>
        </div>
      `;

      if (loc.connects) {
        const connParts = loc.connects.split(';').map(s => s.trim()).filter(Boolean);
        html += `
          <div>
            <div class="map-side-sec-title sec-connects">
              <span>🔗</span> <span>CONNECTED ROUTES &amp; CITIES</span>
            </div>
            <div class="map-connects-list">
              ${connParts.map(c => `<span class="map-connect-pill">${c}</span>`).join('')}
            </div>
          </div>
        `;
      }

      if (loc.pokemon && loc.pokemon.length) {
        html += `
          <div>
            <div class="map-side-sec-title sec-spawns">
              <span>🌿</span> <span>WILD POKÉMON SPAWNS</span>
            </div>
            <div class="map-spawns-wrap">
              ${loc.pokemon.map(p => `<span class="map-spawn-tag">${p}</span>`).join('')}
            </div>
          </div>
        `;
      }

      if (loc.pois && loc.pois.length) {
        html += `
          <div>
            <div class="map-side-sec-title sec-pois">
              <span>🏢</span> <span>NOTABLE LANDMARKS</span>
            </div>
            <div class="map-poi-list">
              ${loc.pois.map(p => `<div class="map-poi-item"><span class="map-poi-bullet">•</span> <span>${p}</span></div>`).join('')}
            </div>
          </div>
        `;
      }

      if (loc.items && loc.items.length) {
        html += `
          <div>
            <div class="map-side-sec-title sec-highlights">
              <span>🎒</span> <span>KEY ITEMS &amp; REWARDS</span>
            </div>
            <div class="map-poi-list">
              ${loc.items.map(it => `<div class="map-poi-item"><span class="map-poi-bullet">•</span> <span>${it}</span></div>`).join('')}
            </div>
          </div>
        `;
      }
    }

    // Steve Live GPS Radar Unit at bottom of sidebar
    html += `
      <div class="map-steve-radar-card">
        <div class="map-radar-header">
          <span>🧭 STEVE LIVE TRACKING</span>
          <span class="map-radar-beacon"></span>
        </div>
        <div class="map-radar-area">
          <span>${this.currentAreaName}</span>
          ${this.isIndoor ? '<span style="font-size:11px; background:#1e293b; padding:2px 6px; border-radius:4px; color:#38bdf8;">🏠 Indoor</span>' : ''}
        </div>
        <div class="map-radar-coords">
          Pos: X:${px} · Y:${py} &nbsp;|&nbsp; Facing: ${(this.P.dir || 'DOWN').toUpperCase()}
        </div>
        <div class="map-exploration-bar-wrap">
          <div class="map-exploration-label">
            <span>Exploration Progress</span>
            <span>${discPct}% (${discCount}/${totalCount})</span>
          </div>
          <div class="map-progress-track">
            <div class="map-progress-fill" style="width: ${discPct}%;"></div>
          </div>
        </div>
      </div>
    `;

    this.sidebarEl.innerHTML = html;
  }

  update(dtOrInput, maybeInput) {
    const dt = typeof dtOrInput === 'number' ? dtOrInput : 0.016;
    const input = typeof dtOrInput === 'number' ? maybeInput : dtOrInput;
    this.time += dt;

    // Update Steve's live location
    this.detectCurrentPlayerLocation();

    // Fade toast notification
    if (this.toastTime > 0) {
      this.toastTime -= dt;
      if (this.toastTime <= 0 && this.toastEl) {
        this.toastEl.classList.remove('visible');
      }
    }

    // Controller / retro keyboard fallback if modal is focused on canvas
    if (input) {
      if (input.just && (input.just('start') || input.just('b') || input.just('esc'))) {
        this.close();
      }
    }
  }

  draw(ctx) {
    // 1. Draw to the High-Resolution DOM Canvas (#pokecraft-map-hires-canvas)
    if (this.hiresCanvas && this.viewportWrap) {
      const rect = this.viewportWrap.getBoundingClientRect();
      const dpr = window.devicePixelRatio || 1;
      const targetW = Math.round(rect.width * dpr);
      const targetH = Math.round(rect.height * dpr);

      if (this.hiresCanvas.width !== targetW || this.hiresCanvas.height !== targetH) {
        this.hiresCanvas.width = targetW;
        this.hiresCanvas.height = targetH;
      }

      const hctx = this.hiresCanvas.getContext('2d');
      if (hctx) {
        hctx.save();
        hctx.scale(dpr, dpr);
        this.renderHiresViewport(hctx, rect.width, rect.height);
        hctx.restore();
      }
    }

    // 2. Also draw cleanly on the retro GBA/Minecraft canvas (`ctx`)
    if (ctx) {
      ctx.fillStyle = '#0a0d14';
      ctx.fillRect(0, 0, 480, 320);

      // Draw subtle retro backdrop
      ctx.fillStyle = '#161e2e';
      ctx.fillRect(8, 8, 464, 304);
      ctx.strokeStyle = '#273449';
      ctx.lineWidth = 1.5;
      ctx.strokeRect(8, 8, 464, 304);

      ctx.fillStyle = '#facc15';
      ctx.font = 'bold 12px monospace';
      ctx.fillText('🗺️ KANTO REGION MAP (OVERWORLD ACTIVE)', 20, 28);

      ctx.fillStyle = '#94a3b8';
      ctx.font = '10px monospace';
      ctx.fillText(`Press [M] or [Esc] to exit map and resume gameplay.`, 20, 48);
    }
  }

  renderHiresViewport(ctx, w, h) {
    // Clear background with ocean blue matching Kanto's surrounding sea
    ctx.fillStyle = '#1c3d5a';
    ctx.fillRect(0, 0, w, h);

    const cx = w / 2;
    const cy = h / 2;

    ctx.save();
    ctx.translate(cx + this.panX, cy + this.panY);
    ctx.scale(this.zoom, this.zoom);
    ctx.translate(-cx, -cy);

    const layout = this.computeMapLayout(w, h);
    this.lastMapLayout = layout;
    const { dx, dy, dw, dh } = layout;

    // Draw high-resolution overworld map image
    if (this.overworldImg && this.overworldImg.complete && this.overworldImg.naturalWidth > 0) {
      ctx.imageSmoothingEnabled = true;
      ctx.imageSmoothingQuality = 'high';
      ctx.drawImage(this.overworldImg, dx, dy, dw, dh);
    } else {
      // Vector fallback
      ctx.fillStyle = '#4e8548';
      ctx.fillRect(dx + 50, dy + 50, dw - 100, dh - 100);
    }

    // Render Location Pins with Exact Alignment on Overworld Tiles
    const flist = this.getFilteredLocations();

    for (let i = 0; i < this.locations.length; i++) {
      const loc = this.locations[i];
      const isDiscovered = this.revealAll || this.discovered.has(loc.id);
      const isSelected = i === this.selectedIdx;
      const isFiltered = flist.includes(loc);

      // Map pin coordinate (0-1000 scale converted to overworld layout bounds)
      const pinX = dx + (loc.sx / 1000) * dw;
      const pinY = dy + (loc.sy / 1000) * dh;

      ctx.save();
      if (!isFiltered && this.filter !== 'all') {
        ctx.globalAlpha = 0.2;
      }

      // Selected Pin Glowing Halo
      if (isSelected) {
        const pulse = (Math.sin(this.time * 6) + 1) * 0.5;
        ctx.strokeStyle = '#facc15';
        ctx.lineWidth = 3;
        ctx.beginPath();
        ctx.arc(pinX, pinY, 11 + pulse * 4, 0, Math.PI * 2);
        ctx.stroke();
      }

      if (isDiscovered) {
        // Color by location type
        let color = '#3b82f6';
        if (loc.type === 'city') color = loc.gym ? '#ef4444' : '#3b82f6';
        else if (loc.type === 'route') color = '#eab308';
        else if (loc.type === 'dungeon') color = '#a855f7';
        else if (loc.type === 'special') color = '#ec4899';

        ctx.fillStyle = color;
        ctx.beginPath();
        ctx.arc(pinX, pinY, loc.type === 'city' ? 6.5 : 5, 0, Math.PI * 2);
        ctx.fill();

        ctx.strokeStyle = '#ffffff';
        ctx.lineWidth = 1.6;
        ctx.stroke();
      } else {
        // Mystery Marker (Fog of War)
        ctx.fillStyle = 'rgba(15, 23, 42, 0.85)';
        ctx.beginPath();
        ctx.arc(pinX, pinY, 5, 0, Math.PI * 2);
        ctx.fill();

        ctx.strokeStyle = '#94a3b8';
        ctx.lineWidth = 1.2;
        ctx.stroke();

        ctx.fillStyle = '#f1f5f9';
        ctx.font = 'bold 9px monospace';
        ctx.fillText('?', pinX - 2.5, pinY + 3);
      }

      ctx.restore();
    }

    // Live Steve Minecraft Marker (Beacon & Direction Arrow) with Exact 100% Tile Alignment
    if (this.liveNormX != null && this.liveNormY != null) {
      const steveX = dx + (this.liveNormX / 1000) * dw;
      const steveY = dy + (this.liveNormY / 1000) * dh;
      const pulse = (Math.sin(this.time * 6) + 1) * 0.5;

      // Radar Pulse Rings
      ctx.strokeStyle = `rgba(34, 197, 94, ${0.4 + pulse * 0.6})`;
      ctx.lineWidth = 2.5 + pulse * 1.5;
      ctx.beginPath();
      ctx.arc(steveX, steveY, 12 + pulse * 9, 0, Math.PI * 2);
      ctx.stroke();

      // Steve Direction Arrow Pointer (▲)
      ctx.save();
      ctx.translate(steveX, steveY);
      let angle = 0;
      if (this.P.dir === 'up') angle = 0;
      else if (this.P.dir === 'right') angle = Math.PI / 2;
      else if (this.P.dir === 'down') angle = Math.PI;
      else if (this.P.dir === 'left') angle = -Math.PI / 2;
      ctx.rotate(angle);

      ctx.beginPath();
      ctx.moveTo(0, -9);
      ctx.lineTo(6.5, 7);
      ctx.lineTo(0, 3.5);
      ctx.lineTo(-6.5, 7);
      ctx.closePath();

      ctx.fillStyle = '#22c55e';
      ctx.fill();
      ctx.strokeStyle = '#ffffff';
      ctx.lineWidth = 1.8;
      ctx.stroke();
      ctx.restore();

      // High-DPI Floating Label with Exact Coordinates
      ctx.fillStyle = '#22c55e';
      ctx.font = 'bold 11px system-ui, sans-serif';
      ctx.fillText(`STEVE [${this.playerCoordX}, ${this.playerCoordY}]`, steveX - 30, steveY - 16);
    }

    ctx.restore();
  }

  close() {
    this.done = true;

    // Hide Modal
    if (this.modalEl) {
      this.modalEl.classList.remove('active');
    }

    // Clean up event listeners
    this.boundHandlers.forEach(h => {
      try {
        h.el.removeEventListener(h.ev, h.fn, h.capture || false);
      } catch (e) {}
    });
    this.boundHandlers = [];

    // Trigger game canvas focus
    const gameCanvas = document.getElementById('game');
    if (gameCanvas) {
      gameCanvas.focus();
    }
  }
}
