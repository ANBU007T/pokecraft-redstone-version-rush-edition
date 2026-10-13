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
| **Option A (Recommended)** | **`Play-PokeCraft.bat`** | 1-Click game launcher at repository root. Starts immediately in < 1 second! |
| **Option B (Direct Executable)** | **`PokeCraft-Game/PokeCraft.exe`** | Direct offline standalone executable with hardware-accelerated 60 FPS graphics. |
| **Option C (Run from Source)** | `npm start` | Run directly using Node.js / Electron in `PokeCraft-Game/resources/app`. |

---

## 📥 Cloning & Downloading

Because `PokeCraft.exe` is **246.4 MB**, this repository uses **Git LFS (Large File Storage)** to track the executable without exceeding GitHub's file size limit.

### Option 1: Clone with Git LFS (Recommended)
Make sure you have [Git LFS](https://git-lfs.com/) installed:
```bash
git lfs install
git clone https://github.com/ANBU007T/pokecraft-redstone-version-pc.git
```
*(If cloned without Git LFS, simply run `git lfs pull` inside the repository folder to download the executable).*

### Option 2: Download Release ZIP
Head to [Releases](https://github.com/ANBU007T/pokecraft-redstone-version-pc/releases) and download the pre-packaged zip archive.

---

## 🎮 How to Play

1. Open the repository folder.
2. Double-click **`Play-PokeCraft.bat`** (or open `PokeCraft-Game/PokeCraft.exe`).
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

## 🛠️ Emulator Creator & Developer Attribution
* **Developer**: **ANBU007T**
* **GitHub**: [https://github.com/ANBU007T](https://github.com/ANBU007T)
* **Role**: PokéCRAFT Emulator Architecture, Desktop Engine, Fast-Boot Pipeline & Platform Distribution

---

## ⚖️ Legal Notice & Disclaimer
This is an unofficial fan-made project created for educational and preservation purposes. Not affiliated with, endorsed by, or sponsored by Nintendo, Game Freak, The Pokémon Company, Mojang, or Microsoft. Pokémon and Minecraft are registered trademarks of their respective owners.
