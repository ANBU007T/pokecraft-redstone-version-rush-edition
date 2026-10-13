@echo off
title PokéCRAFT - Desktop Shortcut Creator
cd /d "%~dp0"
echo ========================================================
echo   Creating PokéCRAFT v1.0.1 Desktop Shortcut
echo ========================================================
echo.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0create_shortcut.ps1"
echo.
pause
