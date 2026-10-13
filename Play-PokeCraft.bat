@echo off
title PokéCRAFT PC Edition
cd /d "%~dp0"
echo Launching PokéCRAFT PC Edition...
if exist "PokeCraft-Game\PokeCraft.exe" (
    start "" "PokeCraft-Game\PokeCraft.exe"
) else if exist "PokeCraft.exe" (
    start "" "PokeCraft.exe"
) else (
    echo Error: Could not locate PokeCraft.exe launcher.
    pause
)
