> [!NOTE]
> ### 📌 Branch: `main` — Legacy Rush Desktop Edition (v1.0.1)
> You are currently viewing the **v1.0.1 (Legacy Rush Desktop Edition)** branch.
> 
> ⚡ **For the new 100% Pure Native Rust Bevy Engine (v1.1.1)** with maximum size reduction (21.75 MB) and hardware-accelerated Vulkan GPU rendering, switch to the default branch: [**`native-bevy`**](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/tree/native-bevy)!

# ⚡ PokéCRAFT — Pokémon Redstone (Rush Edition / PC Desktop)

[![Original Creator: MozeTech](https://img.shields.io/badge/Original%20Creator-MozeTech%20(%40mozetech)-red.svg)](https://x.com/mozetech)
[![Emulator Creator: ANBU007T](https://img.shields.io/badge/Emulator%20Creator-ANBU007T-purple.svg)](https://github.com/ANBU007T)
[![Release](https://img.shields.io/badge/Release-v1.0.1-brightgreen.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/releases)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011-blue.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

An authentic standalone PC Desktop release of **PokéCRAFT** (Pokémon Redstone). Play as **Minecraft Steve in FireRed Kanto** with complete survival mechanics, block mining, crafting, wild Pokémon battles, and an offline desktop engine.

---

> [!IMPORTANT]
> ### 🌟 Full Respect to the Original Creator: MozeTech
> **PokéCRAFT (Pokémon Redstone)** was originally created, envisioned, and developed by **MozeTech** ([@mozetech on X](https://x.com/mozetech)).
> All original gameplay concepts, custom mechanics, sprites, and the incredible world bringing **Minecraft Steve into Pokémon FireRed Kanto** are the creative work of MozeTech.
>
> This update (**Rush Edition / PC Desktop Port**) is developed by **ANBU007T** ([@ANBU007T on GitHub](https://github.com/ANBU007T)) to deliver a lightweight, high-performance offline desktop engine and instant-boot standalone binary (`PokeCraft.exe`) for players everywhere with deepest gratitude and honor to the original author.

---

## 📊 Languages & Technology Breakdown (Legacy v1.0.1)

This legacy **v1.0.1** branch uses the original desktop hybrid architecture:

| Language | Percentage | Purpose |
| :--- | :--- | :--- |
| **JavaScript** | **93.6%** | Core game engine, Pokémon logic, Minecraft survival mechanics |
| **CSS** | **2.9%** | UI themes, emulator layouts, and menu styling |
| **HTML** | **2.4%** | Window structure and DOM viewports |
| **Rust / Other** | **1.1%** | Native desktop window launcher (`wry` / `tao` / `PokeCraft.exe`) |

> 💡 **Looking for 100% Pure Native Rust?** Switch to the default [**`native-bevy`**](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/tree/native-bevy) branch powered by the Bevy Engine (0% JavaScript, 0% HTML, 0% CSS).

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
git clone https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition.git
```
*(Or clone from `https://github.com/ANBU007T/pokecraft-redstone-version-pc.git`)*

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

## 🌐 Working Multiplayer Co-Op (Cable Club)

PokéCRAFT features real-time multiplayer co-op with player movement interpolation, custom skins (Steve, Alex, Zuri, etc.), block mining & placing sync, wild Pokémon battles, and in-game chat (`T` key)!

### 1. How to Host a World
1. Enter any Pokémon Center and talk to the **Cable Club** receptionist (or click **`> JOIN CO-OP <`** on the title screen).
2. Click **HOST**.
3. The game generates a **5-Letter Room Code** (e.g. `KT4RP`) and displays your **Local LAN IP** (e.g. `192.168.1.14:5901`).
4. Share the code (and IP if on local Wi-Fi) with your friends!

### 2. How to Join a Friend's World
1. Talk to the Cable Club receptionist or click **`> JOIN CO-OP <`**.
2. Enter the **5-Letter Room Code**.
3. **If playing on the same PC**: Leave the Host IP / Server field blank (defaults to local).
4. **If playing on local Wi-Fi / LAN / Radmin VPN**: Enter the host's IP in the Host IP field (e.g. `192.168.1.14`).
5. Choose your name and skin, then click **JOIN**!
6. Enjoy exploring Kanto together!

### 3. Global Cloud Relay (Internet Play)
A ready-to-deploy cloud relay template is included in [`relay-server/`](relay-server/). You can deploy it to **Render.com** (100% free) with 1 click so anyone across the world can join via 5-letter codes without needing VPNs or port forwarding!

---

## 👥 Credits & Attribution

### 🌟 Original Game Creator
* **Original Game**: **Pokémon Redstone** (Minecraft Steve in Pokémon FireRed)
* **Creator / Developer**: **MozeTech**
* **X (Twitter)**: [@mozetech](https://x.com/mozetech)
* *Huge respect and gratitude to MozeTech for creating this brilliant and creative fan crossover game.*

### 🤖 AI Engine & Porting Assistant
* **Assistant**: **Antigravity 3.8 Flash (High)**
* **Role**: 100% native Rust Bevy engine conversion, binary size optimization (84% reduction), and release packaging.

### 🛠️ Emulator & Platform Distribution (Rush Edition v1.0.1)
* **Emulator Creator**: **ANBU007T**
* **GitHub**: [@ANBU007T](https://github.com/ANBU007T)
* **Contributions**:
  * Standalone offline Windows PC desktop distribution architecture
  * Fast native Rust execution engine (`PokeCraft.exe` ~1.0 MB)
  * Asynchronous critical asset pre-warming with instant startup
  * Dedicated 6-slot save state system with visual screenshots
  * Git LFS bypass configuration ensuring zero-setup instant download & play

### 🎮 Original Franchises
* **Pokémon**: © Nintendo / Creatures Inc. / GAME FREAK inc.
* **Minecraft**: © Mojang Studios / Microsoft Corporation

---

## ⚖️ Legal Notice & Disclaimer
This is an unofficial fan-made project created for educational and preservation purposes. Free to play. Not affiliated with, endorsed by, or sponsored by Nintendo, Game Freak, Creatures, The Pokémon Company, Mojang, or Microsoft. Pokémon and Minecraft are registered trademarks of their respective owners.
