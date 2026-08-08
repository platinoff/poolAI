#!/bin/bash
# Git add, commit, push for poolAI.
# Run in MSYS2 UCRT64 bash (NOT PowerShell). Close Cursor Source Control first.
# Usage: bash scripts/git-push-poolai.sh
# See: .cursor/rules/msys2-windows.mdc, docs/troubleshooting/GIT_PUSH_FAILED.md

set -e

# MSYS2 UCRT64 PATH (per .cursor/rules/msys2-windows.mdc)
export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"

# Repo root (script lives in scripts/)
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

[ -f .git/index.lock ] && rm -f .git/index.lock

echo "=== cargo fmt --all ==="
cargo fmt --all

echo "=== git add ==="
git add Cargo.toml src/ tests/ scripts/
git add -f docs/CHANGELOG.md docs/cloud/CLOUD_SDK_STATUS.md \
  docs/concept/poolAI_concept_root.txt docs/development/NEXT_STEPS_2026-01-19.md \
  docs/status/STABLE_STATE_SUMMARY.md docs/troubleshooting/GIT_PUSH_FAILED.md
git add -f .cursor/rules/ .cursor/commands/ 2>/dev/null || true

echo "=== git status ==="
git status -sb

echo "=== git commit ==="
git commit -m "feat: AWS base_url_override, Stage 4.4 AI/ML stubs, Cursor rules (MSYS2 only, CL); docs"

echo "=== git push origin main ==="
git push origin main || {
  echo "FAILED: git push. See docs/troubleshooting/GIT_PUSH_FAILED.md"
  echo "Retry: bash scripts/git-push-only.sh"
  exit 1
}

echo "=== done ==="
