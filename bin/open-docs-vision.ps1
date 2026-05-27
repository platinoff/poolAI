# PoolAI - open docs/vision in browser (workaround Cursor "Unable to resolve resource S:/...").
# Usage: .\bin\open-docs-vision.ps1
#        .\bin\open-docs-vision.ps1 -Port 8765 -NoBrowser
#Requires -Version 5.1
param(
    [int]$Port = 8765,
    [switch]$NoBrowser
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$VisionDir = Join-Path $RepoRoot "docs\vision"
if (-not (Test-Path (Join-Path $VisionDir "index.html"))) {
    throw "Not found: $VisionDir\index.html"
}

$Url = "http://127.0.0.1:$Port/index.html"

$existing = $null
try {
    $existing = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue | Select-Object -First 1
} catch { }

if ($existing) {
    Write-Host "Port $Port already in use - docs-vision server may be running."
    Write-Host "Simple Browser URL: $Url"
    if (-not $NoBrowser) { Start-Process $Url }
    exit 0
}

$listener = New-Object System.Net.HttpListener
$listener.Prefixes.Add("http://127.0.0.1:$Port/")
$listener.Start()
Write-Host "Serving: $VisionDir"
Write-Host "Cursor Simple Browser URL: $Url"
Write-Host "Stop: Ctrl+C"

if (-not $NoBrowser) { Start-Process $Url }

$mime = @{
    ".html" = "text/html; charset=utf-8"
    ".htm"  = "text/html; charset=utf-8"
    ".json" = "application/json; charset=utf-8"
    ".svg"  = "image/svg+xml"
    ".md"   = "text/plain; charset=utf-8"
    ".css"  = "text/css"
    ".js"   = "application/javascript"
}

try {
    while ($listener.IsListening) {
        $ctx = $listener.GetContext()
        $rel = $ctx.Request.Url.LocalPath.TrimStart("/")
        if ([string]::IsNullOrWhiteSpace($rel)) { $rel = "index.html" }
        $rel = $rel -replace "/", [IO.Path]::DirectorySeparatorChar
        $file = Join-Path $VisionDir $rel
        $file = [IO.Path]::GetFullPath($file)
        $visionRoot = [IO.Path]::GetFullPath($VisionDir)
        if (-not $file.StartsWith($visionRoot, [StringComparison]::OrdinalIgnoreCase)) {
            $ctx.Response.StatusCode = 403
            $ctx.Response.Close()
            continue
        }
        if (-not (Test-Path $file -PathType Leaf)) {
            $ctx.Response.StatusCode = 404
            $msg = "404 " + $rel
            $buf = [Text.Encoding]::UTF8.GetBytes($msg)
            $ctx.Response.OutputStream.Write($buf, 0, $buf.Length)
            $ctx.Response.Close()
            continue
        }
        $ext = [IO.Path]::GetExtension($file).ToLowerInvariant()
        if ($mime.ContainsKey($ext)) { $ctx.Response.ContentType = $mime[$ext] }
        $bytes = [IO.File]::ReadAllBytes($file)
        $ctx.Response.ContentLength64 = $bytes.Length
        $ctx.Response.OutputStream.Write($bytes, 0, $bytes.Length)
        $ctx.Response.Close()
    }
}
finally {
    $listener.Stop()
    $listener.Close()
}
