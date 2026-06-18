#!/usr/bin/env bash
set -euo pipefail
cd /s/rust/poolAI
MSG_FILE="${1:?message file required}"
shift
if (($# == 0)); then
  echo "usage: git-commit-tree-msg.sh MSG_FILE path..." >&2
  exit 1
fi
git add "$@"
TREE="$(git write-tree)"
PARENT="$(git rev-parse HEAD)"
NEW="$(git commit-tree "$TREE" -p "$PARENT" -F "$MSG_FILE")"
git reset --hard "$NEW"
git log -1 --oneline
git log -1 --format=%B
