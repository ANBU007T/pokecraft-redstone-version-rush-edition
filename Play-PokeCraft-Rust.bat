@echo off
title PokéCRAFT - Native Bevy Rust Edition v1.1.1
cd /d "%~dp0"
if exist "PokeCraft.exe" (
    start "" "PokeCraft.exe"
) else (
    start "" "pokecraft-bevy.exe"
)
exit
