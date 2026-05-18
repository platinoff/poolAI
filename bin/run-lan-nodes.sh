#!/usr/bin/env bash
# Start two PoolAI nodes on one host (FM-003 dev stand). MSYS2 / Git Bash.
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:${PATH:-}"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
export RUST_LOG="${RUST_LOG:-info}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FEATURES="${FEATURES:-enterprise,ml,cloud,test-utils}"
SKIP_BUILD="${SKIP_BUILD:-0}"
BASE_PORT="${BASE_PORT:-8080}"
NODE_COUNT="${NODE_COUNT:-2}"

EXE="$ROOT/target/debug/poolai.exe"
[[ -x "$ROOT/target/debug/poolai" ]] && EXE="$ROOT/target/debug/poolai"

if [[ "$SKIP_BUILD" != "1" ]]; then
  echo "Building poolai (--features $FEATURES)..."
  cargo build --features "$FEATURES"
fi

[[ -f "$EXE" ]] || { echo "Missing binary: $EXE"; exit 1; }

LOG_DIR="$ROOT/data/lan-stand/logs"
mkdir -p "$LOG_DIR"

for ((i = 0; i < NODE_COUNT; i++)); do
  port=$((BASE_PORT + i))
  node="node-$(printf '%c' $((65 + i)))"
  data_root="$ROOT/data/lan-stand/$node"
  raid_path="$data_root/raid"
  mkdir -p "$raid_path"
  log="$LOG_DIR/${node}-${port}.log"

  echo "Starting $node on http://127.0.0.1:$port"
  POOLAI_HTTP_PORT="$port" \
  POOLAI_RAID_BASE_PATH="$raid_path" \
  POOLAI_DATA_PATH="$data_root" \
  nohup "$EXE" >"$log" 2>&1 &
  sleep 2
done

echo ""
echo "Health (after ~15s):"
for ((i = 0; i < NODE_COUNT; i++)); do
  port=$((BASE_PORT + i))
  echo "  curl -s http://127.0.0.1:${port}/api/v1/health"
done
echo "Stop: pkill -f poolai || taskkill //IM poolai.exe //F"
