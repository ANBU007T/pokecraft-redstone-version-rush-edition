# ⚡ PokéCRAFT — Pokémon Redstone (PC Desktop Edition)

[![Release](https://img.shields.io/badge/Release-v1.0.1-brightgreen.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-pc/releases)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-pc)
[![Git LFS](https://img.shields.io/badge/Git%20LFS-Tracked%20(*.exe)-orange.svg)](https://git-lfs.github.com/)
[![Creator](https://img.shields.io/badge/Creator-ANBU007T-purple.svg)](https://github.com/ANBU007T)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

An authentic standalone PC Desktop release of **PokéCRAFT** (Pokémon Redstone). Play as **Minecraft Steve in FireRed Kanto** with complete survival mechanics, block mining, crafting, wild Pokémon battles, and an offline desktop engine.

---

## 🚀 Instant Launch Options

| Launch Method | Target | Description |
| :--- | :--- | :--- |
| **Option A (Instant Play)** | **`PokeCraft.exe`** | Direct native executable right at the root (~1.0 MB). Starts in ~0.2s with zero lag! |
| **Option B (1-Click Launcher)** | **`Play-PokeCraft.bat`** | 1-Click game launcher at repository root. Auto-starts the game! |
| **Option C (Desktop Shortcut)** | **`Create-Desktop-Shortcut.bat`** | Places a custom PokéCRAFT game icon directly onto your Windows Desktop. |
| **Option D (Legacy Electron)** | **`PokeCraft-Game/PokeCraft.exe`** | Optional standalone Electron build with embedded Chromium runtime. |

---

## 📥 Instant Play (Zero Setup)

Anyone on any Windows 10 / 11 PC can clone or download this repository and play immediately:

### Option 1: Direct Play via Git Clone
```bash
git clone https://github.com/ANBU007T/pokecraft-redstone-version-pc.git
```
Open the folder and double-click **`PokeCraft.exe`** (or **`Play-PokeCraft.bat`**)!

### Option 2: Download GitHub ZIP
Click **Code ➔ Download ZIP** on GitHub. Extract the folder and double-click **`PokeCraft.exe`**!
*(Zero dependencies needed: no Node.js, no Python, no Git LFS required for direct play).*

---

## 🎮 How to Play

1. Open the repository folder.
2. Double-click **`PokeCraft.exe`** or **`Play-PokeCraft.bat`**.
3. *(Optional)* Run **`Create-Desktop-Shortcut.bat`** to place a custom PokéCRAFT shortcut on your Windows Desktop.
4. The game starts **instantly in under 1 second** in offline desktop mode! Zero installation, zero internet, and no browser required.

---

## 🕹️ PC Controls & Shortcuts

| Action | Primary Key | Secondary Key |
| :--- | :--- | :--- |
| **Movement** | `W` / `A` / `S` / `D` | `Arrow Keys` |
| **Mine / Attack** | `Left Click` | `Z` |
| **Place / Eat** | `Right Click` | `X` |
| **Jump** | `Spacebar` | — |
| **Sprint (1.6x Speed)** | `Shift` (Hold) | Toggle Sprint Option |
| **Inventory & Crafting** | `E` | — |
| **Drop Item** | `Q` | — |
| **Hotbar Selection** | `1` – `9` | `Mouse Wheel` / `C` & `V` |
| **Pokémon Menu** | `Enter` | Party, Pokédex, Bag |
| **Town Map** | `M` | `J` |
| **Settings & Options** | `F1` | `Escape` |
| **Custom Key Remapper** | `F2` | Remap any key |
| **Quick Save** | `F5` | Save Slots 1–6 with Screenshots |
| **Quick Load** | `F8` | Restore active state |
| **Save State Manager** | `F6` | Import / Export `.sav` |
| **Fullscreen** | `F11` | Borderless / Windowed |

---

## 💾 Independent 6-Slot Save System
* PC saves use independent local storage (`pokecraft_pc_slot_1` through `6`).
* Features visual save state screenshots, timestamp recording, and `.sav` file export/import.
* 100% offline — save progress is kept locally on your machine.

---

---

## 👥 Credits & Attribution

### 🌟 Original Game Creator
* **Original Game**: **Pokémon Redstone** (Minecraft Steve in Pokémon FireRed)
* **Creator / Developer**: **MozeTech**
* **X (Twitter)**: [@mozetech](https://x.com/mozetech)
* *Huge respect and gratitude to MozeTech for creating this brilliant and creative fan crossover game.*

### 🛠️ Emulator & Platform Distribution (v1.0.1 Update)
* **Emulator Creator**: **ANBU007T**
* **GitHub**: [@ANBU007T](https://github.com/ANBU007T)
* **Contributions**:
  * Standalone offline Windows PC desktop distribution architecture
  * Fast-boot streaming engine (asynchronous sprite loading in < 1 second)
  * Dedicated 6-slot save state system with visual screenshots
  * Full offline Electron packaging and clean dependency optimization
  * Git LFS integration for GitHub repository hosting

### 🎮 Original Franchises
* **Pokémon**: © Nintendo / Creatures Inc. / GAME FREAK inc.
* **Minecraft**: © Mojang Studios / Microsoft Corporation

---

## ⚖️ Legal Notice & Disclaimer
This is an unofficial fan-made project created for educational and preservation purposes. Free to play. Not affiliated with, endorsed by, or sponsored by Nintendo, Game Freak, Creatures, The Pokémon Company, Mojang, or Microsoft. Pokémon and Minecraft are registered trademarks of their respective owners.

