# ⚡ PokéCRAFT — Pokémon Redstone (Native Rust Rush Edition v1.1.1)

[![Platform: Windows 10/11 x64](https://img.shields.io/badge/Platform-Windows%2010%20%2F%2011%20x64-blue.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition)
[![Release: v1.1.1](https://img.shields.io/badge/Release-v1.1.1-brightgreen.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/releases)
[![Engine: Pure Rust Bevy](https://img.shields.io/badge/Engine-Pure%20Rust%20(Bevy)-orange.svg)](https://bevyengine.org)
[![Binary: 21.75 MB](https://img.shields.io/badge/Binary%20Size-21.75%20MB%20(-84%25)-red.svg)](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

An authentic, 100% pure native Rust desktop rewrite of **PokéCRAFT (Pokémon Redstone)** powered by the **Bevy Engine**. 

Play as **Minecraft Steve in Pokémon FireRed Kanto** with complete survival mechanics, block mining, crafting, wild Pokémon encounters, Gen 3 battle formulas, and Nether/End dimensions.

---

> [!TIP]
> **Looking for the legacy v1.0.1 desktop wrapper?** The previous branch is preserved at [**`main`**](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/tree/main).

---

> [!IMPORTANT]
> ### 🏆 Full Respect to the Original Creator: MozeTech
> **PokéCRAFT (Pokémon Redstone)** was originally created, envisioned, and developed by **MozeTech** ([@mozetech on X](https://x.com/mozetech)).
> All original gameplay concepts, custom mechanics, sprites, and the incredible world bringing **Minecraft Steve into Pokémon FireRed Kanto** are the creative work of MozeTech.
>
> This update (**Native Rust Rush Edition v1.1.1**) is engineered by **ANBU007T** ([@ANBU007T on GitHub](https://github.com/ANBU007T)) to deliver a pure native Rust game engine with zero web wrappers, instant boot times, and maximum size reduction.

---

## ⚡ What Makes Native Rust Rush Edition v1.1.1 Special?

* **100% Pure Native Rust**: Zero JavaScript, zero HTML, zero CSS, zero WebViews, zero Electron, zero WebAssembly.
* **Aggressive Size Optimization**: Executable reduced from **135 MB down to 21.75 MB** (-83.9% size reduction) using Fat Link-Time Optimization (LTO), single codegen unit, and symbol stripping.
* **Instant Boot**: Starts in under 0.2s with hardware-accelerated Vulkan GPU rendering.
* **Ultra-Low Memory Footprint**: Runs in ~150 MB RAM (compared to 1.5+ GB for Electron).
* **Complete Offline Play**: 100% standalone, no internet or browser runtimes required.

---

## 🎮 How to Play

1. Download [`PokeCraft-Rust-v1.1.1.zip`](https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition/releases) from Releases.
2. Extract the ZIP anywhere on your PC.
3. Double-click **`PokeCraft.exe`** (or **`Play-PokeCraft-Rust.bat`**)!
4. The game launches immediately!

---

## ⌨️ Controls & Shortcuts

| Action | Primary Key | Secondary Key |
| :--- | :--- | :--- |
| **Movement** | `W` / `A` / `S` / `D` | `Arrow Keys` |
| **Mine / Attack** | `Left Click` | `Z` |
| **Place / Eat** | `Right Click` | `X` |
| **Jump** | `Spacebar` | — |
| **Sprint (1.6x Speed)** | `Shift` (Hold) | Toggle Sprint |
| **Inventory & Crafting** | `E` | — |
| **Drop Item** | `Q` | — |
| **Hotbar Selection** | `1` – `9` | `Mouse Wheel` / `C` & `V` |
| **Pokémon Menu** | `Enter` | Party, Pokédex, Bag |
| **Town Map** | `M` | `J` |
| **Settings & Options** | `F1` | `Escape` |
| **Custom Key Remapper** | `F2` | Remap any key |
| **Quick Save** | `F5` | 6 Independent Slots |
| **Quick Load** | `F8` | Restore active state |
| **Fullscreen** | `F11` | Borderless / Windowed |

---

## 🛠️ Building From Source

Prerequisites: Rust stable toolchain (MSVC on Windows).

```bash
# Clone the repository
git clone -b native-bevy https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition.git
cd pokecraft-redstone-version-rush-edition

# Run all 32 unit & integration tests
cargo test --release

# Build optimized release executable (21.75 MB)
cargo build --release

# Run the game
cargo run --release
```

---

## 📜 Credits & Attribution
* **Original Game & Concept**: **MozeTech** ([@mozetech on X](https://x.com/mozetech))
* **Native Rust Architecture & Optimization**: **ANBU007T** ([@ANBU007T on GitHub](https://github.com/ANBU007T))
* **AI Engine & Porting Assistant**: **Antigravity 3.8 Flash (High)**
* **Pokémon**: © Nintendo / Creatures Inc. / GAME FREAK inc.
* **Minecraft**: © Mojang Studios / Microsoft Corporation
