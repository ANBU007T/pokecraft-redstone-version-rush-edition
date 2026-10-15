> [!NOTE]
> ### 📌 Branch: main — Legacy Rush Desktop Edition (v1.0.1)
> You are currently viewing the **v1.0.1 (Legacy Rush Desktop Edition)** branch.
> 
> ⚡ **For the new 100% Pure Native Rust Bevy Engine (v1.1.1)** with maximum size reduction (21.75 MB) and hardware-accelerated Vulkan GPU rendering, switch to the default branch: [**
ative-bevy**](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/tree/native-bevy)!

# âš¡ PokÃ©CRAFT â€” PokÃ©mon Redstone (Rush Edition / PC Desktop)

[![Original Creator: MozeTech](https://img.shields.io/badge/Original%20Creator-MozeTech%20(%40mozetech)-red.svg)](https://x.com/mozetech)
[![Emulator Creator: ANBU007T](https://img.shields.io/badge/Emulator%20Creator-ANBU007T-purple.svg)](https://github.com/ANBU007T)
[![Release](https://img.shields.io/badge/Release-v1.0.1-brightgreen.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/releases)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

An authentic standalone PC Desktop release of **PokÃ©CRAFT** (PokÃ©mon Redstone). Play as **Minecraft Steve in FireRed Kanto** with complete survival mechanics, block mining, crafting, wild PokÃ©mon battles, and an offline desktop engine.

---

> [!IMPORTANT]
> ### ðŸŒŸ Full Respect to the Original Creator: MozeTech
> **PokÃ©CRAFT (PokÃ©mon Redstone)** was originally created, envisioned, and developed by **MozeTech** ([@mozetech on X](https://x.com/mozetech)).
> All original gameplay concepts, custom mechanics, sprites, and the incredible world bringing **Minecraft Steve into PokÃ©mon FireRed Kanto** are the creative work of MozeTech.
>
> This update (**Rush Edition / PC Desktop Port**) is developed by **ANBU007T** ([@ANBU007T on GitHub](https://github.com/ANBU007T)) to deliver a lightweight, high-performance offline desktop engine and instant-boot standalone binary (`PokeCraft.exe`) for players everywhere with deepest gratitude and honor to the original author.

---

## ðŸš€ Instant Launch Options

| Launch Method | Target | Description |
| :--- | :--- | :--- |
| **Option A (Instant Play)** | **`PokeCraft.exe`** | Direct native executable right at the root (~1.0 MB). Starts in ~0.2s with zero lag! |
| **Option B (1-Click Launcher)** | **`Play-PokeCraft.bat`** | 1-Click game launcher at repository root. Auto-starts the game! |
| **Option C (Desktop Shortcut)** | **`Create-Desktop-Shortcut.bat`** | Places a custom PokÃ©CRAFT game icon directly onto your Windows Desktop. |
| **Option D (Legacy Electron)** | **`PokeCraft-Game/PokeCraft.exe`** | Optional standalone Electron build with embedded Chromium runtime. |

---

## ðŸ“¥ Instant Play (Zero Setup)

Anyone on any Windows 10 / 11 PC can clone or download this repository and play immediately:

### Option 1: Direct Play via Git Clone
```bash
git clone https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition.git
```
*(Or clone from `https://github.com/ANBU007T/pokecraft-redstone-version-pc.git`)*

Open the folder and double-click **`PokeCraft.exe`** (or **`Play-PokeCraft.bat`**)!

### Option 2: Download GitHub ZIP
Click **Code âž” Download ZIP** on GitHub. Extract the folder and double-click **`PokeCraft.exe`**!
*(Zero dependencies needed: no Node.js, no Python, no Git LFS required for direct play).*

---

## ðŸŽ® How to Play

1. Open the repository folder.
2. Double-click **`PokeCraft.exe`** or **`Play-PokeCraft.bat`**.
3. *(Optional)* Run **`Create-Desktop-Shortcut.bat`** to place a custom PokÃ©CRAFT shortcut on your Windows Desktop.
4. The game starts **instantly in under 1 second** in offline desktop mode! Zero installation, zero internet, and no browser required.

---

## ðŸ•¹ï¸ PC Controls & Shortcuts

| Action | Primary Key | Secondary Key |
| :--- | :--- | :--- |
| **Movement** | `W` / `A` / `S` / `D` | `Arrow Keys` |
| **Mine / Attack** | `Left Click` | `Z` |
| **Place / Eat** | `Right Click` | `X` |
| **Jump** | `Spacebar` | â€” |
| **Sprint (1.6x Speed)** | `Shift` (Hold) | Toggle Sprint Option |
| **Inventory & Crafting** | `E` | â€” |
| **Drop Item** | `Q` | â€” |
| **Hotbar Selection** | `1` â€“ `9` | `Mouse Wheel` / `C` & `V` |
| **PokÃ©mon Menu** | `Enter` | Party, PokÃ©dex, Bag |
| **Town Map** | `M` | `J` |
| **Settings & Options** | `F1` | `Escape` |
| **Custom Key Remapper** | `F2` | Remap any key |
| **Quick Save** | `F5` | Save Slots 1â€“6 with Screenshots |
| **Quick Load** | `F8` | Restore active state |
| **Save State Manager** | `F6` | Import / Export `.sav` |
| **Fullscreen** | `F11` | Borderless / Windowed |

---

## ðŸ’¾ Independent 6-Slot Save System
* PC saves use independent local storage (`pokecraft_pc_slot_1` through `6`).
* Features visual save state screenshots, timestamp recording, and `.sav` file export/import.
* 100% offline â€” save progress is kept locally on your machine.

---

## ðŸ‘¥ Credits & Attribution

### ðŸŒŸ Original Game Creator
* **Original Game**: **PokÃ©mon Redstone** (Minecraft Steve in PokÃ©mon FireRed)
* **Creator / Developer**: **MozeTech**
* **X (Twitter)**: [@mozetech](https://x.com/mozetech)
* *Huge respect and gratitude to MozeTech for creating this brilliant and creative fan crossover game.*

### ðŸ› ï¸ Emulator & Platform Distribution (Rush Edition v1.0.1)
* **Emulator Creator**: **ANBU007T**
* **GitHub**: [@ANBU007T](https://github.com/ANBU007T)
* **Contributions**:
  * Standalone offline Windows PC desktop distribution architecture
  * Ultra-fast native Rust execution engine (`PokeCraft.exe` ~1.0 MB)
  * Asynchronous critical asset pre-warming with instant startup
  * Dedicated 6-slot save state system with visual screenshots
  * Git LFS bypass configuration ensuring zero-setup instant download & play

### ðŸŽ® Original Franchises
* **PokÃ©mon**: Â© Nintendo / Creatures Inc. / GAME FREAK inc.
* **Minecraft**: Â© Mojang Studios / Microsoft Corporation

---

## âš–ï¸ Legal Notice & Disclaimer
This is an unofficial fan-made project created for educational and preservation purposes. Free to play. Not affiliated with, endorsed by, or sponsored by Nintendo, Game Freak, Creatures, The PokÃ©mon Company, Mojang, or Microsoft. PokÃ©mon and Minecraft are registered trademarks of their respective owners.
