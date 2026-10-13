@echo off
title PokéCRAFT - Push to GitHub
cd /d "%~dp0"

echo ========================================================
echo   Uploading PokéCRAFT to GitHub (pokecraft-redstone-version-pc)
echo ========================================================
echo.
echo Pushing commits and Git LFS objects to GitHub...
echo.

git branch -M main
git push -u origin main

if %errorlevel% equ 0 (
    echo.
    echo ========================================================
    echo   [SUCCESS] Repository successfully uploaded to GitHub!
    echo   URL: https://github.com/ANBU007T/pokecraft-redstone-version-pc
    echo ========================================================
) else (
    echo.
    echo ========================================================
    echo   [NOTICE] If upload failed with "Repository not found":
    echo   Please make sure you have created the empty repo:
    echo   https://github.com/new
    echo   Name: pokecraft-redstone-version-pc
    echo ========================================================
)

echo.
pause
