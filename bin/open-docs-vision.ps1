# PoolAI - static server for docs/vision UI + repo files (md preview).
# Usage: .\bin\open-docs-vision.ps1
#        .\bin\open-docs-vision.ps1 -Port 8765 -NoBrowser
#Requires -Version 5.1
param(
    [int]$Port = 8765,
    [switch]$NoBrowser
)

function GetMimeType {
    param([string]$Extension)
    $charsetSuffix = [string][char]59 + ' charset=utf-8'
    if ($Extension -eq '.html') { return 'text/html' + $charsetSuffix }
    if ($Extension -eq '.htm') { return 'text/html' + $charsetSuffix }
    if ($Extension -eq '.json') { return 'application/json' + $charsetSuffix }
    if ($Extension -eq '.svg') { return 'image/svg+xml' }
    if ($Extension -eq '.md') { return 'text/plain' + $charsetSuffix }
    if ($Extension -eq '.css') { return 'text/css' + $charsetSuffix }
    if ($Extension -eq '.js') { return 'application/javascript' }
    if ($Extension -eq '.txt') { return 'text/plain' + $charsetSuffix }
    if ($Extension -eq '.rs') { return 'text/plain' + $charsetSuffix }
    if ($Extension -eq '.png') { return 'image/png' }
    return $null
}

function SendVisionBytes {
    param($Context, $Body, $ContentType, [switch]$NoCache, [string]$VisionRevision)
    if ($ContentType) {
        $Context.Response.ContentType = $ContentType
    }
    if ($NoCache) {
        $Context.Response.Headers.Add('Cache-Control', 'no-cache, no-store, must-revalidate')
        $Context.Response.Headers.Add('Pragma', 'no-cache')
    }
    if ($VisionRevision) {
        $Context.Response.Headers.Add('X-PoolAI-Vision-Revision', $VisionRevision)
    }
    $Context.Response.ContentLength64 = $Body.Length
    $Context.Response.OutputStream.Write($Body, 0, $Body.Length)
    $Context.Response.Close()
}

function Get-GitHeadShort {
    param([string]$RepoRootPath)
    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        return ''
    }
    try {
        Push-Location -LiteralPath $RepoRootPath
        $hash = (& git rev-parse --short HEAD 2>$null)
        if ($hash) {
            return "$hash".Trim()
        }
    }
    catch {
        return ''
    }
    finally {
        Pop-Location
    }
    return ''
}

function Invoke-VisionManifestSync {
    param([string]$RepoRootPath)
    $syncExe = Join-Path $RepoRootPath 'target\debug\poolai-vision-sync.exe'
    $syncRelease = Join-Path $RepoRootPath 'target\release\poolai-vision-sync.exe'
    $bin = if (Test-Path -LiteralPath $syncExe) { $syncExe }
           elseif (Test-Path -LiteralPath $syncRelease) { $syncRelease }
           else { $null }
    try {
        Push-Location -LiteralPath $RepoRootPath
        if ($bin) {
            & $bin
        }
        else {
            $env:PATH = "$env:USERPROFILE\.cargo\bin;C:\msys64\ucrt64\bin;C:\msys64\usr\bin;" + $env:PATH
            cargo run --quiet --bin poolai-vision-sync
        }
    }
    catch {
        Write-Host "vision sync skipped: $_"
    }
    finally {
        Pop-Location
    }
}

function GetVisionWatchPayload {
    param(
        [string]$VisionDirectory,
        [string]$RepoRootPath
    )
    $watchNames = @(
        'manifest.json',
        'extensions.json',
        'index.html',
        'vision.css',
        'vision.js',
        'vision.svg'
    )
    $bundleTicks = [long]0
    $dataTicks = [long]0
    foreach ($name in $watchNames) {
        $filePath = Join-Path $VisionDirectory $name
        if (-not (Test-Path -LiteralPath $filePath -PathType Leaf)) {
            continue
        }
        $ticks = (Get-Item -LiteralPath $filePath).LastWriteTimeUtc.Ticks
        if ($name -eq 'index.html' -or $name -eq 'vision.css' -or $name -eq 'vision.js') {
            $bundleTicks += $ticks
        }
        else {
            $dataTicks += $ticks
        }
    }
    $revision = 0
    $manifestPath = Join-Path $VisionDirectory 'manifest.json'
    if (Test-Path -LiteralPath $manifestPath -PathType Leaf) {
        try {
            $manifestJson = Get-Content -LiteralPath $manifestPath -Raw -Encoding UTF8 | ConvertFrom-Json
            if ($null -ne $manifestJson.revision) {
                $revision = [int]$manifestJson.revision
            }
        }
        catch {
            $revision = 0
        }
    }
    $gitHead = Get-GitHeadShort -RepoRootPath $RepoRootPath
    $token = '{0}:{1}:{2}:{3}' -f $bundleTicks, $dataTicks, $revision, $gitHead
    return @{
        token    = $token
        bundle   = [string]$bundleTicks
        data     = [string]$dataTicks
        revision = $revision
        git_head = $gitHead
    }
}

function StartVisionServerLoop {
    param(
        $HttpListener,
        [string]$RepoRootPath,
        [string]$VisionDirectory,
        [string]$JsonContentType,
        [string]$PlainContentType
    )
    while ($HttpListener.IsListening) {
        $ctx = $HttpListener.GetContext()
        $path = $ctx.Request.Url.LocalPath.TrimStart('/').TrimEnd('/')

        if ($path -eq 'docs/vision/__watch') {
            $payload = GetVisionWatchPayload -VisionDirectory $VisionDirectory -RepoRootPath $RepoRootPath
            $json = $payload | ConvertTo-Json -Compress
            $bytes = [Text.Encoding]::UTF8.GetBytes($json)
            SendVisionBytes -Context $ctx -Body $bytes -ContentType $JsonContentType
            continue
        }

        if ($path -eq 'docs/vision/__sync') {
            Invoke-VisionManifestSync -RepoRootPath $RepoRootPath
            $payload = GetVisionWatchPayload -VisionDirectory $VisionDirectory -RepoRootPath $RepoRootPath
            $json = $payload | ConvertTo-Json -Compress
            $bytes = [Text.Encoding]::UTF8.GetBytes($json)
            SendVisionBytes -Context $ctx -Body $bytes -ContentType $JsonContentType
            continue
        }

        if ([string]::IsNullOrWhiteSpace($path)) {
            $ctx.Response.StatusCode = 302
            $ctx.Response.RedirectLocation = '/docs/vision/index.html'
            $ctx.Response.Close()
            continue
        }

        $rel = $path -replace '/', [IO.Path]::DirectorySeparatorChar
        $file = Join-Path $RepoRootPath $rel
        $file = [IO.Path]::GetFullPath($file)
        $repoFull = [IO.Path]::GetFullPath($RepoRootPath)

        if (-not $file.StartsWith($repoFull, [StringComparison]::OrdinalIgnoreCase)) {
            $ctx.Response.StatusCode = 403
            $ctx.Response.Close()
            continue
        }

        if (-not (Test-Path -LiteralPath $file -PathType Leaf)) {
            $ctx.Response.StatusCode = 404
            $msg = [Text.Encoding]::UTF8.GetBytes('404 ' + $path)
            SendVisionBytes -Context $ctx -Body $msg -ContentType $PlainContentType
            continue
        }

        $ext = [IO.Path]::GetExtension($file).ToLowerInvariant()
        $ctype = GetMimeType -Extension $ext
        $bytes = [IO.File]::ReadAllBytes($file)
        $noCache = $ext -in '.html', '.css', '.js', '.json', '.svg'
        $visionRev = ''
        if ($path -eq 'docs/vision/manifest.json') {
            try {
                $manifestJson = [Text.Encoding]::UTF8.GetString($bytes) | ConvertFrom-Json
                if ($null -ne $manifestJson.revision) {
                    $visionRev = [string]$manifestJson.revision
                }
            }
            catch {
                $visionRev = ''
            }
        }
        SendVisionBytes -Context $ctx -Body $bytes -ContentType $ctype -NoCache:$noCache -VisionRevision $visionRev
    }
}

$ErrorActionPreference = 'Stop'
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$VisionDir = Join-Path $RepoRoot 'docs\vision'
if (-not (Test-Path (Join-Path $VisionDir 'index.html'))) {
    throw "Not found: $VisionDir\index.html"
}

$Url = "http://127.0.0.1:$Port/docs/vision/index.html"
$JsonContentType = 'application/json' + [string][char]59 + ' charset=utf-8'
$PlainContentType = 'text/plain' + [string][char]59 + ' charset=utf-8'

$existing = $null
try {
    $existing = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue | Select-Object -First 1
}
catch {
    $existing = $null
}

if ($existing) {
    Write-Host "Port $Port already in use - docs-vision server may be running (OLD bundle — CSS/title may be stale)."
    Write-Host 'Restart: stop the process on this port, then run this script again (or hard-refresh Ctrl+Shift+R).'
    Write-Host "Simple Browser URL: $Url"
    if (-not $NoBrowser) {
        Start-Process $Url
    }
    exit 0
}

$listener = New-Object System.Net.HttpListener
$listener.Prefixes.Add("http://127.0.0.1:$Port/")
Write-Host 'Vision sync: indexing git-tracked files into manifest.json …'
Invoke-VisionManifestSync -RepoRootPath $RepoRoot
$listener.Start()
Write-Host "Serving repo: $RepoRoot"
Write-Host "Vision UI: $Url"
Write-Host 'Auto-reload: GET /docs/vision/__watch · manual sync: GET /docs/vision/__sync'
Write-Host 'Stop: Ctrl+C'

if (-not $NoBrowser) {
    Start-Process $Url
}

try {
    StartVisionServerLoop -HttpListener $listener -RepoRootPath $RepoRoot -VisionDirectory $VisionDir -JsonContentType $JsonContentType -PlainContentType $PlainContentType
}
catch {
    Write-Error $_
}
finally {
    $listener.Stop()
    $listener.Close()
}
