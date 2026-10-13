$desktop = [Environment]::GetFolderPath('Desktop')

# Delete any existing PokeCraft shortcuts
Get-ChildItem $desktop -Filter "*Poke*.lnk" -ErrorAction SilentlyContinue | Remove-Item -Force -ErrorAction SilentlyContinue
Get-ChildItem $desktop -Filter "*craft*.lnk" -ErrorAction SilentlyContinue | Where-Object { $_.Name -notlike ".*" } | Remove-Item -Force -ErrorAction SilentlyContinue

$exePath = if (Test-Path "$PSScriptRoot\PokeCraft.exe") {
    (Resolve-Path "$PSScriptRoot\PokeCraft.exe").Path
} else {
    (Resolve-Path "$PSScriptRoot\PokeCraft-Game\PokeCraft.exe").Path
}
$workDir = Split-Path -Parent $exePath
$iconPath = if (Test-Path "$PSScriptRoot\pokecraft.ico") {
    (Resolve-Path "$PSScriptRoot\pokecraft.ico").Path
} else {
    (Resolve-Path "$PSScriptRoot\PokeCraft-Game\resources\app\PokeCraft.ico").Path
}

# Use explicit Unicode codepoint 0x00E9 for 'é' to guarantee perfect display across all Windows languages
$eAcute = [char]0x00E9
$shortcutFilename = "Pok" + $eAcute + "CRAFT.lnk"
$shortcutFullPath = Join-Path $desktop $shortcutFilename

$ws = New-Object -ComObject WScript.Shell
$shortcut = $ws.CreateShortcut($shortcutFullPath)
$shortcut.TargetPath = $exePath
$shortcut.WorkingDirectory = $workDir
$shortcut.IconLocation = "$iconPath,0"
$shortcut.Description = "Play Pok" + $eAcute + "CRAFT PC Edition v1.0.1"
$shortcut.Save()

Write-Host ""
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host " [SUCCESS] Shortcut created: $shortcutFilename" -ForegroundColor Green
Write-Host " Target: $exePath" -ForegroundColor Gray
Write-Host " Icon:   $iconPath" -ForegroundColor Gray
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""
