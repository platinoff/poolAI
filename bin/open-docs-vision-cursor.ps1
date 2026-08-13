# Open docs vision in Cursor Simple Browser (http://127.0.0.1:8765).
#Requires -Version 5.1
$Url = "http://127.0.0.1:8765/GSV/docs/vision/index.html"
$Enc = [uri]::EscapeDataString($Url)
$VscodeUri = "vscode://vscode.simple-browser/show?url=$Enc"

$CursorPaths = @(
    "$env:LOCALAPPDATA\Programs\cursor\Cursor.exe",
    "$env:LOCALAPPDATA\cursor\Cursor.exe",
    "${env:ProgramFiles}\Cursor\Cursor.exe"
)

foreach ($exe in $CursorPaths) {
    if (Test-Path $exe) {
        Start-Process -FilePath $exe -ArgumentList "--open-url", $VscodeUri
        Write-Host "Cursor Simple Browser: $Url"
        exit 0
    }
}

$cursorCmd = Get-Command cursor -ErrorAction SilentlyContinue
if ($cursorCmd) {
    & cursor --open-url $VscodeUri
    Write-Host "Cursor Simple Browser: $Url"
    exit 0
}

Write-Host "Cursor.exe not found. Paste in Simple Browser: $Url"
Start-Process $Url
