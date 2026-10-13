// Keyboard, mouse and touch input with "just pressed" edge detection.
// GBA buttons: A = Z, B = X (and Escape). Minecraft keys: Space jump, Shift sprint, E inventory,
// Q drop, 1-9 / wheel / C V hotbar, Enter the FireRed start menu.
// Dynamic, editable, and persistent key bindings for GameBoy & PC controls
export const DEFAULT_KEY_BINDINGS = {
  up: ['ArrowUp', 'w'],
  down: ['ArrowDown', 's'],
  left: ['ArrowLeft', 'a'],
  right: ['ArrowRight', 'd'],
  a: ['MouseLeft', 'z'],
  b: ['MouseRight', 'x'],
  start: ['Enter'],
  select: ['Backspace'],
  jump: [' '],
  sprint: ['Shift'],
  inv: ['e'],
  drop: ['q'],
  prev: ['c', '['],
  next: ['v', ']'],
  map: ['m', 'j'],
  mute: ['Control+m', 'n']
};

export const ACTION_LABELS = {
  up: 'D-Pad Up',
  down: 'D-Pad Down',
  left: 'D-Pad Left',
  right: 'D-Pad Right',
  a: 'Button A (Attack / Mine / Talk)',
  b: 'Button B (Place / Use / Eat)',
  start: 'START Button (Game Menu)',
  select: 'SELECT Button',
  jump: 'Jump (Space)',
  sprint: 'Sprint (Shift)',
  inv: 'Inventory (E)',
  drop: 'Drop Item (Q)',
  prev: 'Previous Hotbar Slot (C)',
  next: 'Next Hotbar Slot (V)',
  map: 'Town Map (M / J)',
  mute: 'Toggle Audio Mute (Ctrl+M / N)'
};

export function loadUserBindings() {
  try {
    const raw = localStorage.getItem('pokecraft-custom-controls');
    if (raw) {
      const parsed = JSON.parse(raw);
      if (parsed.sprint) {
        parsed.sprint = parsed.sprint.filter(k => k !== 'Control' && k !== 'Ctrl');
        if (!parsed.sprint.length) parsed.sprint = ['Shift'];
      }
      if (parsed.map && !parsed.map.includes('m') && !parsed.map.includes('M')) {
        parsed.map = ['m', ...parsed.map];
      }
      return { ...DEFAULT_KEY_BINDINGS, ...parsed };
    }
  } catch(e) {}
  return { ...DEFAULT_KEY_BINDINGS };
}

export let MAP = {};

export function updateMapFromBindings(bindings) {
  for (const k in MAP) delete MAP[k];
  for (const [action, keys] of Object.entries(bindings)) {
    if (Array.isArray(keys)) {
      keys.forEach((key) => {
        if (!key) return;
        MAP[key] = action;
        if (key.length === 1) {
          MAP[key.toLowerCase()] = action;
          MAP[key.toUpperCase()] = action;
        }
        // Modifier key aliases
        if (key === 'Control' || key === 'Ctrl') {
          MAP['Control'] = action;
          MAP['Ctrl'] = action;
          MAP['ControlLeft'] = action;
          MAP['ControlRight'] = action;
        }
        if (key === 'Shift') {
          MAP['Shift'] = action;
          MAP['ShiftLeft'] = action;
          MAP['ShiftRight'] = action;
        }
        if (key === 'Alt') {
          MAP['Alt'] = action;
          MAP['AltLeft'] = action;
          MAP['AltRight'] = action;
        }
        // Mouse button aliases
        if (key === 'MouseLeft' || key === 'Mouse0' || key === 'LeftClick') {
          MAP['MouseLeft'] = action;
          MAP['Mouse0'] = action;
          MAP['LeftClick'] = action;
        }
        if (key === 'MouseRight' || key === 'Mouse2' || key === 'RightClick') {
          MAP['MouseRight'] = action;
          MAP['Mouse2'] = action;
          MAP['RightClick'] = action;
        }
        if (key === 'MouseMiddle' || key === 'Mouse1' || key === 'MiddleClick') {
          MAP['MouseMiddle'] = action;
          MAP['Mouse1'] = action;
          MAP['MiddleClick'] = action;
        }
        if (key === 'Mouse4' || key === 'Mouse3') {
          MAP['Mouse4'] = action;
          MAP['Mouse3'] = action;
        }
        if (key === 'Mouse5') {
          MAP['Mouse5'] = action;
        }
      });
    }
  }
  for (let i = 1; i <= 9; i++) MAP[String(i)] = 'slot' + i;
  MAP['Escape'] = 'esc';
}

let currentBindings = loadUserBindings();
updateMapFromBindings(currentBindings);

export function saveUserBindings(newBindings) {
  currentBindings = { ...newBindings };
  updateMapFromBindings(currentBindings);
  try {
    localStorage.setItem('pokecraft-custom-controls', JSON.stringify(currentBindings));
  } catch(e) {}
  if (typeof window !== 'undefined' && window.__updateGuideHintText) {
    try { window.__updateGuideHintText(); } catch(e){}
  }
}

export function resetToDefaultBindings() {
  saveUserBindings(DEFAULT_KEY_BINDINGS);
  return { ...DEFAULT_KEY_BINDINGS };
}

if (typeof window !== 'undefined') {
  window.__pokeControls = {
    getDefaults: () => JSON.parse(JSON.stringify(DEFAULT_KEY_BINDINGS)),
    getBindings: () => JSON.parse(JSON.stringify(currentBindings)),
    getLabels: () => ACTION_LABELS,
    saveBindings: (b) => saveUserBindings(b),
    resetDefaults: () => resetToDefaultBindings(),
    getMap: () => MAP
  };
}

const DIRS = ['up', 'down', 'left', 'right'];
export function isButtonKey(k) { const b = MAP[k]; return !!b && !/^slot/.test(b) && b !== 'jump' && b !== 'map'; }

export class Input {
  constructor(canvas) {
    window.__pokeInput = this;
    this.down = new Set();
    this.pressed = new Set();
    this.released = new Set();
    this.lastDir = null;
    this.mouse = { x: -1, y: -1, inside: false, wheel: 0 };
    this.ctrl = false;
    this.typed = [];
    this.sprintToggled = false;

    const isToggleSprint = () => (typeof window !== 'undefined' && window.__toggleSprint) || (localStorage.getItem('pokecraft-toggle-sprint') === '1');

    // Text fields, modals, and key remapping inputs receive native keyboard actions
    const typing = (t) => t && (t.tagName === 'INPUT' || t.tagName === 'TEXTAREA' || t.isContentEditable || t.closest?.('#sponsor-slot') || t.closest?.('#settings-modal') || t.closest?.('#controls-modal') || t.closest?.('#save-slots-modal') || window.__settingsModalOpen || window.__remappingKeyActive);
    window.addEventListener('focusin', (e) => { if (typing(e.target)) { for (const k of this.down) this.released.add(k); this.down.clear(); } });
    window.addEventListener('keydown', (e) => {
      if (typing(e.target) || window.__remappingKeyActive) return;
      if (e.key && e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) this.typed.push(e.key);
      this.ctrl = !!(e.ctrlKey || e.metaKey || e.key === 'Control');

      const k = MAP[e.key] || MAP[e.code];
      if (k) {
        if (e.key !== 'F11' && e.key !== 'F1' && e.key !== 'F2' && e.key !== 'F5' && e.key !== 'F6' && e.key !== 'F8') {
          if (!(e.ctrlKey && e.key.toLowerCase() === 'r')) e.preventDefault();
        }
        if (k === 'sprint' && isToggleSprint()) {
          this.sprintToggled = !this.sprintToggled;
        }
        this.press(k);
        if (k === 'esc') this.press('b');
      }
    });
    window.addEventListener('keyup', (e) => {
      if (window.__remappingKeyActive) return;
      this.ctrl = !!(e.ctrlKey || e.metaKey);

      const k = MAP[e.key] || MAP[e.code];
      if (k) {
        if (k === 'sprint' && isToggleSprint() && this.sprintToggled) {
          // Keep active while toggle sprint is on
        } else {
          this.release(k);
        }
        if (k === 'esc') this.release('b');
      }
    });
    window.addEventListener('blur', () => { for (const k of this.down) this.released.add(k); this.down.clear(); this.ctrl = false; });

    const getMouseKey = (btn) => {
      if (btn === 0) return 'MouseLeft';
      if (btn === 1) return 'MouseMiddle';
      if (btn === 2) return 'MouseRight';
      if (btn === 3) return 'Mouse4';
      if (btn === 4) return 'Mouse5';
      return 'Mouse' + btn;
    };

    if (canvas) {
      const pos = (e) => {
        const r = canvas.getBoundingClientRect();
        this.mouse.x = ((e.clientX - r.left) / r.width) * canvas.width;
        this.mouse.y = ((e.clientY - r.top) / r.height) * canvas.height;
        this.mouse.inside = this.mouse.x >= 0 && this.mouse.y >= 0 && this.mouse.x < canvas.width && this.mouse.y < canvas.height;
      };
      canvas.addEventListener('mousemove', pos);
      canvas.addEventListener('mouseleave', () => { this.mouse.inside = false; });
      canvas.addEventListener('mousedown', (e) => {
        pos(e);
        e.preventDefault();
        this.press(e.button === 2 ? 'mouseR' : 'mouseL');
        const mk = getMouseKey(e.button);
        if (mk && MAP[mk]) {
          this.press(MAP[mk]);
        }
      });
      window.addEventListener('mouseup', (e) => {
        this.release(e.button === 2 ? 'mouseR' : 'mouseL');
        const mk = getMouseKey(e.button);
        if (mk && MAP[mk]) {
          this.release(MAP[mk]);
        }
      });
      canvas.addEventListener('contextmenu', (e) => e.preventDefault());
      // a second finger on the canvas (a touch that is not the primary pointer, so the browser sends no mousedown for
      // it) is 'tap2' at tap2.x, tap2.y: the minimap's tap while the other finger holds the pad (minimap-design 3.10);
      // nothing else reads it, so the menus' own touch handling is unchanged
      this.tap2 = { x: -1, y: -1 };
      canvas.addEventListener('pointerdown', (e) => {
        if (e.pointerType !== 'touch' || e.isPrimary) return;
        const r = canvas.getBoundingClientRect();
        if (!r.width || !r.height) return;
        this.tap2.x = ((e.clientX - r.left) / r.width) * canvas.width; this.tap2.y = ((e.clientY - r.top) / r.height) * canvas.height;
        this.press('tap2');
      });
      for (const ev of ['pointerup', 'pointercancel']) canvas.addEventListener(ev, (e) => { if (e.pointerType === 'touch' && !e.isPrimary) this.release('tap2'); });
      canvas.addEventListener('wheel', (e) => { e.preventDefault(); this.mouse.wheel += Math.sign(e.deltaY); }, { passive: false });
    }
    for (const b of document.querySelectorAll('#pad button')) {
      const k = b.dataset.k;
      const on = (e) => { e.preventDefault(); this.press(k); };
      const off = (e) => { e.preventDefault(); this.release(k); };
      b.addEventListener('pointerdown', on);
      b.addEventListener('pointerup', off);
      b.addEventListener('pointercancel', off);
      b.addEventListener('pointerleave', off);
    }
  }
  press(k) {
    if (!this.down.has(k)) this.pressed.add(k);
    this.down.add(k);
    if (DIRS.includes(k)) this.lastDir = k;
  }
  release(k) { if (this.down.has(k)) this.released.add(k); this.down.delete(k); }
  held(k) {
    if (k === 'sprint') {
      return this.down.has('sprint') || this.down.has('Shift') || (this.sprintToggled && this.down.size > 0);
    }
    return this.down.has(k);
  }
  just(k) { return this.pressed.has(k); }
  up(k) { return this.released.has(k); }
  dir() {
    if (this.lastDir && this.down.has(this.lastDir)) return this.lastDir;
    for (const d of DIRS) if (this.down.has(d)) return d;
    return null;
  }
  takeWheel() { const w = this.mouse.wheel; this.mouse.wheel = 0; return w; }
  endFrame() { this.pressed.clear(); this.released.clear(); this.typed.length = 0; }
}
