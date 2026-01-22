#!/usr/bin/env bash
# Git status + log. Run from poolAI root or via: bash bin/git-status.sh
set -e
cd "$(dirname "$0")/.."
git status --short
echo "---"
git log --oneline -5
git branch --show-current
