#!/usr/bin/env bash
# PoolAI - static server for docs/vision UI + repo files (md preview).
# Bash replacement for the former open-docs-vision.ps1 (GEN-3 format hygiene).
# Usage:
#   bash bin/open-docs-vision.sh [--port 8765] [--no-browser]
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${POOLAI_VISION_PORT:-8765}"
NO_BROWSER=0

while [ $# -gt 0 ]; do
  case "$1" in
    --port|-Port) PORT="${2:?--port requires a value}"; shift 2 ;;
    --port=*) PORT="${1#*=}"; shift ;;
    --no-browser|-NoBrowser) NO_BROWSER=1; shift ;;
    -h|--help)
      echo "Usage: bash bin/open-docs-vision.sh [--port 8765] [--no-browser]"
      exit 0
      ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

VISION_DIR="$ROOT/docs/vision"
if [ ! -f "$VISION_DIR/index.html" ]; then
  echo "Not found: $VISION_DIR/index.html" >&2
  exit 1
fi

URL="http://127.0.0.1:${PORT}/docs/vision/index.html"

# Prefer syncing manifest via the Rust vision-sync bin when available.
if [ -x "$ROOT/target/debug/poolai-vision-sync" ]; then
  (cd "$ROOT" && "$ROOT/target/debug/poolai-vision-sync") || true
elif [ -x "$ROOT/target/release/poolai-vision-sync" ]; then
  (cd "$ROOT" && "$ROOT/target/release/poolai-vision-sync") || true
elif command -v cargo >/dev/null 2>&1; then
  (cd "$ROOT" && cargo run --quiet --bin poolai-vision-sync) || true
fi

echo "Serving repo: $ROOT"
echo "Vision UI: $URL"
echo "Stop: Ctrl+C"

open_browser() {
  [ "$NO_BROWSER" -eq 1 ] && return 0
  if command -v cmd.exe >/dev/null 2>&1; then
    cmd.exe /c start "" "$URL" >/dev/null 2>&1 || true
  elif command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$URL" >/dev/null 2>&1 || true
  fi
}

# Serve from repo root so /docs/... paths resolve (python stdlib, no deps).
open_browser
cd "$ROOT"
if command -v python3 >/dev/null 2>&1; then
  exec python3 -m http.server "$PORT" --bind 127.0.0.1
elif command -v python >/dev/null 2>&1; then
  exec python -m http.server "$PORT" --bind 127.0.0.1
else
  echo "python3/python not found; cannot start static server" >&2
  exit 1
fi
