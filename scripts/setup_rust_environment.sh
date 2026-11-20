#!/usr/bin/env bash
# Setup Rust Environment Script (bash/MSYS2 equivalent of former .ps1).
# Detects rust-toolchain channel and configures the current shell.
# Source for env changes:  source scripts/setup_rust_environment.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "Setting up Rust development environment..."
echo

if command -v rustup >/dev/null 2>&1; then
  echo "Current Rust toolchain:"
  rustup show || true
  echo
fi

TOOLCHAIN_FILE="$ROOT/rust-toolchain.toml"
CHANNEL=""
if [ -f "$TOOLCHAIN_FILE" ]; then
  echo "Reading $TOOLCHAIN_FILE..."
  CHANNEL="$(grep -E '^[[:space:]]*channel' "$TOOLCHAIN_FILE" | head -n1 | sed -E 's/.*"([^"]+)".*/\1/' || true)"
  echo "  Channel: ${CHANNEL:-<unknown>}"
fi

case "$CHANNEL" in
  *msvc*)
    echo "  MSVC toolchain detected."
    echo "  From bash/MSYS2, prefer: eval \"\$(bash scripts/setup_msvc_environment.sh --print-env)\""
    echo "  or build from a Visual Studio Developer prompt."
    ;;
  *gnu*|*)
    echo "  GNU/MSYS2 toolchain (default for this environment)."
    # shellcheck disable=SC1091
    source "$ROOT/scripts/setup_msys2_path.sh"
    export PATH="$HOME/.cargo/bin:$PATH"
    ;;
esac

echo
echo "Verifying Rust tools..."
if command -v rustc >/dev/null 2>&1; then echo "  rustc: $(rustc --version)"; else echo "  rustc: not found"; fi
if command -v cargo >/dev/null 2>&1; then echo "  cargo: $(cargo --version)"; else echo "  cargo: not found"; fi
echo
echo "Environment setup complete. You can now run: cargo build"
