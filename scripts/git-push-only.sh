#!/bin/bash
# Push only (no add/commit). Use after git-push-poolai.sh commit or manual commit.
# Run in MSYS2 UCRT64 bash. See .cursor/rules/msys2-windows.mdc

set -e
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "=== git status ==="
git status -sb
echo "=== git push origin main ==="
git push origin main
echo "=== done ==="
