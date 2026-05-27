# PoolAI - static server for docs/vision UI + repo files (md preview).
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

$Url = "http://127.0.0.1:$Port/docs/vision/index.html"

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
Write-Host "Serving repo: $RepoRoot"
Write-Host "Vision UI: $Url"
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
    ".txt"  = "text/plain; charset=utf-8"
    ".rs"   = "text/plain; charset=utf-8"
}

function Send-Bytes($ctx, [byte[]]$bytes, [string]$contentType) {
    if ($contentType) { $ctx.Response.ContentType = $contentType }
    $ctx.Response.ContentLength64 = $bytes.Length
    $ctx.Response.OutputStream.Write($bytes, 0, $bytes.Length)
    $ctx.Response.Close()
}

try {
    while ($listener.IsListening) {
        $ctx = $listener.GetContext()
        $path = $ctx.Request.Url.LocalPath.TrimStart("/").TrimEnd("/")

        if ([string]::IsNullOrWhiteSpace($path)) {
            $ctx.Response.StatusCode = 302
            $ctx.Response.RedirectLocation = "/docs/vision/index.html"
            $ctx.Response.Close()
            continue
        }

        $rel = $path -replace "/", [IO.Path]::DirectorySeparatorChar
        $file = Join-Path $RepoRoot $rel
        $file = [IO.Path]::GetFullPath($file)
        $repoFull = [IO.Path]::GetFullPath($RepoRoot)

        if (-not $file.StartsWith($repoFull, [StringComparison]::OrdinalIgnoreCase)) {
            $ctx.Response.StatusCode = 403
            $ctx.Response.Close()
            continue
        }

        if (-not (Test-Path $file -PathType Leaf)) {
            $ctx.Response.StatusCode = 404
            $msg = [Text.Encoding]::UTF8.GetBytes("404 " + $path)
            Send-Bytes $ctx $msg "text/plain; charset=utf-8"
            continue
        }

        $ext = [IO.Path]::GetExtension($file).ToLowerInvariant()
        $ctype = $null
        if ($mime.ContainsKey($ext)) { $ctype = $mime[$ext] }
        $bytes = [IO.File]::ReadAllBytes($file)
        Send-Bytes $ctx $bytes $ctype
    }
}
finally {
    $listener.Stop()
    $listener.Close()
}
