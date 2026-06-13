#!/usr/bin/env bash
# PH-S147: build poolai-ui-wasm for browser (grid-pricing panel POC).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Cargo / Rust toolchain (MSYS2 + Windows user profile).
CARGO_BIN=""
for d in "${HOME}/.cargo/bin" "/c/Users/${USER:-${USERNAME}}/.cargo/bin"; do
  if [[ -x "$d/cargo" ]]; then
    CARGO_BIN="$d"
    break
  fi
done
if [[ -z "$CARGO_BIN" ]]; then
  echo "error: cargo not found (install Rust toolchain)" >&2
  exit 1
fi

# MSYS2 UCRT64: prefer GNU host toolchain (avoids MSVC link.exe vs GNU link clash).
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable-x86_64-pc-windows-gnu}"
export PATH="${CARGO_BIN}:/ucrt64/bin:/usr/bin:${PATH:-}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"

TARGET="wasm32-unknown-unknown"
OUT_DIR="$ROOT/src/ui/wasm"
WASM_CRATE="poolai-ui-wasm"

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo not found in PATH" >&2
  exit 1
fi

if command -v rustup >/dev/null 2>&1; then
  rustup target add "$TARGET" >/dev/null 2>&1 || true
fi

echo "==> cargo build -p $WASM_CRATE --release --target $TARGET (toolchain: $RUSTUP_TOOLCHAIN)"
cargo build -p "$WASM_CRATE" --release --target "$TARGET"

WASM_ARTIFACT="$ROOT/target/$TARGET/release/${WASM_CRATE//-/_}.wasm"
if [[ ! -f "$WASM_ARTIFACT" ]]; then
  echo "error: missing artifact $WASM_ARTIFACT" >&2
  exit 1
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "error: wasm-bindgen CLI not found (cargo install wasm-bindgen-cli)" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
echo "==> wasm-bindgen -> $OUT_DIR"
wasm-bindgen "$WASM_ARTIFACT" \
  --out-dir "$OUT_DIR" \
  --target web \
  --no-typescript

echo "OK: $OUT_DIR/poolai_ui_wasm_bg.wasm"
