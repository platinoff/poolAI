#!/usr/bin/env bash
# Scan Clippy JSON messages → docs/development/rust_diagnostics.json
# (+ mirror docs/vision/rust_diagnostics.json for Galaxy Rust panel).
#
# Usage (MSYS2 / CI):
#   bash bin/record-rust-diagnostics.sh
#   bash bin/record-rust-diagnostics.sh --skip-run   # print current index
#   bash bin/record-rust-diagnostics.sh --ci         # source=ci (GitHub Actions)
#   HOST_LABEL=win10-local bash bin/record-rust-diagnostics.sh
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/ucrt64/bin:/usr/bin:${PATH:-}"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*)
    # MSYS2 local dev: force the native Windows GNU toolchain + repo target dir.
    export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable-x86_64-pc-windows-gnu}"
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/s/rust/poolAI/target}"
    ;;
  *)
    # Linux/macOS CI: keep the rustup default toolchain + target dir.
    ;;
esac

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

SOURCE="local"
SKIP=0
for arg in "$@"; do
  case "$arg" in
    --skip-run) SKIP=1 ;;
    --ci) SOURCE="ci" ;;
    -h|--help)
      echo "Usage: bash bin/record-rust-diagnostics.sh [--skip-run] [--ci]"
      exit 0
      ;;
  esac
done

if [[ "$SKIP" -eq 1 ]]; then
  cargo run -q --bin poolai-rust-diagnostics -- --print
  exit 0
fi

HOST_LABEL="${HOST_LABEL:-${COMPUTERNAME:-${HOSTNAME:-local}}}"
# Default scan matches CI test feature set so all --all-targets files compile (no phantom E0432/E0425).
CMD="${RUST_DIAGNOSTICS_CMD:-cargo clippy --message-format=json --all-targets --features ml,enterprise,cloud,test-utils,job-store-sqlite,prometheus}"

echo "==> rust diagnostics scan (source=$SOURCE host=$HOST_LABEL)"
echo "==> $CMD"

set +e
cargo run -q --bin poolai-rust-diagnostics -- \
  --scan \
  --command "$CMD" \
  --host "$HOST_LABEL" \
  --source "$SOURCE"
STATUS=$?
set -e

echo "==> recorded (exit=$STATUS) — vision panel: Rust"
exit "$STATUS"
