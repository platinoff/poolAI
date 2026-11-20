#!/usr/bin/env bash
# Open docs vision in Cursor Simple Browser (http://127.0.0.1:8765).
# Bash replacement for the former open-docs-vision-cursor.ps1 (GEN-3).
set -euo pipefail

URL="http://127.0.0.1:8765/docs/vision/index.html"
ENC_URL="$(printf '%s' "$URL" | sed -e 's/:/%3A/g' -e 's/\//%2F/g')"
VSCODE_URI="vscode://vscode.simple-browser/show?url=${ENC_URL}"

for exe in \
  "${LOCALAPPDATA:-}/Programs/cursor/Cursor.exe" \
  "${LOCALAPPDATA:-}/cursor/Cursor.exe" \
  "${ProgramFiles:-}/Cursor/Cursor.exe"; do
  if [ -n "$exe" ] && [ -f "$exe" ]; then
    "$exe" --open-url "$VSCODE_URI" &
    echo "Cursor Simple Browser: $URL"
    exit 0
  fi
done

if command -v cursor >/dev/null 2>&1; then
  cursor --open-url "$VSCODE_URI"
  echo "Cursor Simple Browser: $URL"
  exit 0
fi

echo "Cursor.exe not found. Paste in Simple Browser: $URL"
if command -v cmd.exe >/dev/null 2>&1; then
  cmd.exe /c start "" "$URL" >/dev/null 2>&1 || true
fi
