#!/usr/bin/env bash
# Record wall-clock for `cargo test-ci` into docs/development/speed_index.json
# (+ mirror docs/vision/speed_index.json for Galaxy Speeds panel).
#
# Usage (MSYS2):
#   bash bin/record-test-ci-speed.sh
#   bash bin/record-test-ci-speed.sh --skip-run   # only print current index
#   HOST_LABEL=win10-local-26200 bash bin/record-test-ci-speed.sh
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/ucrt64/bin:/usr/bin:${PATH:-}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable-x86_64-pc-windows-gnu}"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/s/rust/poolAI/target}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ "${1:-}" == "--skip-run" ]]; then
  cargo run -q --bin poolai-speed-index -- --print
  exit 0
fi

HOST_LABEL="${HOST_LABEL:-${COMPUTERNAME:-${HOSTNAME:-local}}}"
CMD='cargo test-ci'

echo "==> timing: $CMD (host=$HOST_LABEL)"
START=$(date +%s)
set +e
$CMD
STATUS=$?
set -e
END=$(date +%s)
WALL=$((END - START))

OK_FLAG=(--ok)
if [[ "$STATUS" -ne 0 ]]; then
  OK_FLAG=(--fail)
fi

cargo run -q --bin poolai-speed-index -- \
  --record-test-ci \
  --wall-secs "$WALL" \
  "${OK_FLAG[@]}" \
  --command "$CMD" \
  --host "$HOST_LABEL"

echo "==> recorded wall_secs=${WALL} exit=${STATUS}"
exit "$STATUS"
