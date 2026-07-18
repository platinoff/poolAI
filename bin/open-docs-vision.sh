#!/usr/bin/env bash
# MSYS2 shim for open-docs-vision (PH-S1013).
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$ROOT/bin/open-docs-vision.ps1" "$@"
