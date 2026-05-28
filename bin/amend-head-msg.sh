#!/usr/bin/env bash
set -euo pipefail
cd /s/rust/poolAI

resolve_commit_msg_file() {
  local f="${1:-.git/COMMIT_MSG_TMP.txt}"
  if [[ -f "$f" ]]; then
    printf '%s\n' "$f"
    return 0
  fi
  local base="${f##*/}"
  if [[ -f "comitmsg/$base" ]]; then
    printf '%s\n' "comitmsg/$base"
    return 0
  fi
  printf '%s\n' "$f"
  return 1
}

MSG_FILE="$(resolve_commit_msg_file "${1:-}")"
TREE="$(git rev-parse 'HEAD^{tree}')"
PARENT="$(git rev-parse 'HEAD^')"
NEW="$(git commit-tree "$TREE" -p "$PARENT" -F "$MSG_FILE")"
git reset --hard "$NEW"
git log -1 --oneline
git log -1 --format=%B
