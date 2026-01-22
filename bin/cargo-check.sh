#!/usr/bin/env bash
# cargo check (GNU + MSYS2 PATH on Windows). Run: bash bin/cargo-check.sh
set -e
cd "$(dirname "$0")/.."
if [[ "$(uname -s)" == MINGW* || "$(uname -s)" == MSYS* ]]; then
  export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
  rustup override set stable-x86_64-pc-windows-gnu 2>/dev/null || true
fi
cargo check --no-default-features --lib
