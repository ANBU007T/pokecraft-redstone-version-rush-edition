<p align="center">
  <img src="assets/logo.png" alt="PokéCRAFT Logo" width="460" />
  <br>
  <b>⚡ Pokémon Redstone — PC Desktop Edition (v1.0.1)</b>
  <br>
  <i>Play as Minecraft Steve in FireRed Kanto with Full Survival, Block Mining & Crafting</i>
</p>

<p align="center">
  <a href="https://github.com/ANBU007T"><img src="https://img.shields.io/badge/Release-v1.0.1-brightgreen.svg" alt="Release" /></a>
  <a href="https://github.com/ANBU007T"><img src="https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue.svg" alt="Platform" /></a>
  <a href="https://git-lfs.github.com/"><img src="https://img.shields.io/badge/Git%20LFS-Tracked%20(*.exe)-orange.svg" alt="Git LFS" /></a>
  <a href="https://github.com/ANBU007T"><img src="https://img.shields.io/badge/Creator-ANBU007T-purple.svg" alt="Creator" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License" /></a>
</p>

---

## 🚀 Instant Launch Options

Choose whichever launch option fits your setup best:

| Method | Executable | Description |
| :--- | :--- | :--- |
| **Option A (Recommended)** | **`Play-PokeCraft.bat`** | Auto-detects and launches the best available engine. |
| **Option B (Ultra-Fast 1MB)** | **`PokeCraft-Native.exe`** | Pure native Rust & WebView2 executable — only **1.0 MB**, zero lag, boots in ~0.5s! |
| **Option C (Full Standalone)** | **`PokeCraft-Game/PokeCraft.exe`** | Self-contained Electron desktop application with embedded Chromium runtime. |
| **Option D (From Source)** | `npm install && npm start` | Run directly using Node.js / Electron in `PokeCraft-Game/resources/app`. |

---

## 📥 Cloning & Downloading

Because `PokeCraft-Game/PokeCraft.exe` is **246.4 MB**, this repository uses **Git LFS (Large File Storage)** to track the pre-compiled Electron executable:

### Option 1: Clone with Git LFS (Full Electron Included)
```bash
git lfs install
git clone https://github.com/ANBU007T/PokeCraft-PC.git
```
*(If cloned without Git LFS, you can still play instantly with `PokeCraft-Native.exe` (1.0 MB), or run `git lfs pull` to fetch the Electron binary).*

### Option 2: Download Release ZIP
Head to [Releases](https://github.com/ANBU007T/PokeCraft-PC/releases) and download `PokeCraft-PC-v1.0.1.zip` (portable zero-install zip).

---

## 🎮 How to Play

1. Open the repository folder.
2. Double-click **`Play-PokeCraft.bat`**.
3. *(Optional)* Run **`Create-Desktop-Shortcut.bat`** to create a custom PokéCRAFT icon on your Windows Desktop.
4. The game starts **instantly** in offline desktop mode! Zero installation or browser required.

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
