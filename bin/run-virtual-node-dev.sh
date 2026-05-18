#!/usr/bin/env bash
# FM-003 + FM-016: coordinator + poolai-worker on one host (MSYS2 / Git Bash).
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:${PATH:-}"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
export RUST_LOG="${RUST_LOG:-info}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FEATURES="${FEATURES:-enterprise,ml,cloud,test-utils}"
SKIP_BUILD="${SKIP_BUILD:-0}"
COORD_PORT="${COORD_PORT:-8080}"
WORKER_PORT="${WORKER_PORT:-9090}"

POOLAI="$ROOT/target/debug/poolai.exe"
WORKER="$ROOT/target/debug/poolai-worker.exe"
[[ -x "$ROOT/target/debug/poolai" ]] && POOLAI="$ROOT/target/debug/poolai"
[[ -x "$ROOT/target/debug/poolai-worker" ]] && WORKER="$ROOT/target/debug/poolai-worker"

if [[ "$SKIP_BUILD" != "1" ]]; then
  echo "Building poolai (--features $FEATURES) + poolai-worker..."
  cargo build --features "$FEATURES"
  cargo build --bin poolai-worker
fi

[[ -f "$POOLAI" ]] || { echo "Missing $POOLAI"; exit 1; }
[[ -f "$WORKER" ]] || { echo "Missing $WORKER"; exit 1; }

STAND="$ROOT/data/lan-stand/virtual-node"
COORD_DATA="$STAND/coordinator"
RAID_PATH="$COORD_DATA/raid"
VN_STORE="$STAND/vn-store"
WORKER_CACHE="$STAND/worker-cache"
LOG_DIR="$STAND/logs"
mkdir -p "$RAID_PATH" "$VN_STORE" "$WORKER_CACHE" "$LOG_DIR"

COORD_URL="http://127.0.0.1:${COORD_PORT}"

echo "Starting coordinator on $COORD_URL"
POOLAI_HTTP_PORT="$COORD_PORT" \
POOLAI_RAID_BASE_PATH="$RAID_PATH" \
POOLAI_DATA_PATH="$COORD_DATA" \
POOLAI_VIRTUAL_NODE_DATA_DIR="$VN_STORE" \
nohup "$POOLAI" >"$LOG_DIR/coordinator-${COORD_PORT}.log" 2>&1 &

sleep 12

echo "Starting worker on http://127.0.0.1:${WORKER_PORT}/health"
POOLAI_COORDINATOR_URL="$COORD_URL" \
POOLAI_WORKER_ADDRESS=127.0.0.1 \
POOLAI_WORKER_PORT="$WORKER_PORT" \
POOLAI_WORKER_CHANNEL=dev \
POOLAI_TELEGRAM_ID=dev-stand-user \
POOLAI_WORKER_CACHE_DIR="$WORKER_CACHE" \
nohup "$WORKER" --worker-id vn-dev-stand >"$LOG_DIR/worker-${WORKER_PORT}.log" 2>&1 &

echo ""
echo "Verify after bootstrap (~50s):"
echo "  bash bin/verify-dev-stand.sh"
echo "  # or: VERIFY_WARMUP_SECS=0 bash bin/verify-dev-stand.sh  # if already warm"
echo "Stop: pkill -f 'poolai|poolai-worker' || taskkill //IM poolai.exe //F"
