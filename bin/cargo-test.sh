#!/usr/bin/env bash
# cargo test --lib. Run: bash bin/cargo-test.sh
# Optional: bash bin/cargo-test.sh raid  → raid_cross_strategy + raid_smallworld_integration
set -e
cd "$(dirname "$0")/.."
if [[ "$(uname -s)" == MINGW* || "$(uname -s)" == MSYS* ]]; then
  export PATH="/c/msys64/ucrt64/bin:/c/msys64/usr/bin:$PATH"
  rustup override set stable-x86_64-pc-windows-gnu 2>/dev/null || true
fi
if [[ "$1" == raid ]]; then
  cargo test --no-default-features --test raid_cross_strategy --test raid_smallworld_integration
else
  cargo test --no-default-features --lib
fi
