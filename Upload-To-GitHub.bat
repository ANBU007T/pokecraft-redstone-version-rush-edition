@echo off
title PokéCRAFT - Push to GitHub
cd /d "%~dp0"

echo ========================================================
echo   Uploading PokéCRAFT to GitHub
echo   Target Repo: pokecraft-redstone-version-rush-edition
echo ========================================================
echo.
echo Pushing commits to GitHub...
echo.

git branch -M main

:: Try pushing to origin first
git push -u origin main

if %errorlevel% equ 0 (
    echo.
    echo ========================================================
    echo   [SUCCESS] Repository successfully uploaded to GitHub!
    echo   URL: https://github.com/ANBU007T/pokecraft-redstone-version-rush-edition
    echo   (or: https://github.com/ANBU007T/pokecraft-redstone-version-pc)
    echo ========================================================
) else (
    echo.
    echo ========================================================
    echo   [NOTICE] If push failed with "Repository not found":
    echo   To rename your existing repository on GitHub:
    echo     1. Go to: https://github.com/ANBU007T/pokecraft-redstone-version-pc/settings
    echo     2. In "Repository name", enter: pokecraft-redstone-version-rush-edition
    echo     3. Click "Rename"
    echo   Or create a new repo at: https://github.com/new
    echo ========================================================
)

echo.
pause
