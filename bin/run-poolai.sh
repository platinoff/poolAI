#!/usr/bin/env bash
# PoolAI — єдиний локальний лаунчер (MSYS2 UCRT64; не WSL / не Windows Store bash).
# Документація: docs/development/RUN_LOCAL.md
# Windows PowerShell: .\bin\run-poolai.ps1  або  .\bin\poolai-msys.ps1 bin/run-poolai.sh …
set -euo pipefail

# MSYS2: /usr/bin/bash first — інакше `bash` може викликати WSL stub (немає дистрибутива).
case "$(uname -s 2>/dev/null)" in
  MINGW* | MSYS*)
    export PATH="/usr/bin:/ucrt64/bin:${HOME}/.cargo/bin:${PATH:-}"
    ;;
  *)
    export PATH="${HOME}/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:${PATH:-}"
    ;;
esac

POOLAI_BASH="/usr/bin/bash"
if [[ ! -x "$POOLAI_BASH" ]]; then
  POOLAI_BASH="$(command -v bash 2>/dev/null || true)"
fi
[[ -n "$POOLAI_BASH" ]] || {
  echo "bash not found; use MSYS2 UCRT64 or .\\bin\\run-poolai.ps1" >&2
  exit 1
}
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
export RUST_LOG="${RUST_LOG:-info}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FEATURES="${FEATURES:-enterprise,ml,cloud,test-utils}"
PORT="${POOLAI_HTTP_PORT:-8080}"
SKIP_BUILD="${SKIP_BUILD:-0}"
BG=0

usage() {
  cat <<'EOF'
Usage: /usr/bin/bash bin/run-poolai.sh <command> [options]
       (PowerShell: .\bin\run-poolai.ps1 single -Background)

Commands:
  single          One coordinator (default). UI: http://127.0.0.1:8080/ui
  lan             Two+ nodes on one host (FM-003 dev stand)
  virtual-node    Coordinator + poolai-worker (FM-016 dev stand)
  docker          docker compose up (docker/docker-compose.yml)
  build           cargo build (--features enterprise,ml,cloud,test-utils)
  stop            Stop poolai / poolai-worker processes
  status          curl health on common dev ports
  help            This message

Options (single):
  --bg            Run in background (logs under data/dev/logs/)
  --port N        HTTP port (default 8080)
  --skip-build    Skip cargo build

Environment:
  FEATURES        Cargo features (default: enterprise,ml,cloud,test-utils)
  POOLAI_HTTP_PORT, POOLAI_DATA_PATH, POOLAI_RAID_BASE_PATH, RUST_LOG

Examples:
  /usr/bin/bash bin/run-poolai.sh single
  /usr/bin/bash bin/run-poolai.sh single --bg
  .\bin\run-poolai.ps1 single -Background -SkipBuild   # PowerShell (no WSL)
EOF
}

poolai_bin() {
  if [[ -x "$ROOT/target/release/poolai.exe" ]]; then
    echo "$ROOT/target/release/poolai.exe"
  elif [[ -x "$ROOT/target/release/poolai" ]]; then
    echo "$ROOT/target/release/poolai"
  elif [[ -x "$ROOT/target/debug/poolai.exe" ]]; then
    echo "$ROOT/target/debug/poolai.exe"
  elif [[ -x "$ROOT/target/debug/poolai" ]]; then
    echo "$ROOT/target/debug/poolai"
  else
    echo ""
  fi
}

cmd_build() {
  echo "Building poolai (--features $FEATURES)..."
  cargo build --features "$FEATURES"
  cargo build --bin poolai-worker 2>/dev/null || true
}

cmd_stop() {
  echo "Stopping PoolAI processes..."
  if command -v pkill >/dev/null 2>&1; then
    pkill -f 'target/debug/poolai' 2>/dev/null || true
    pkill -f 'poolai-worker' 2>/dev/null || true
  fi
  if command -v taskkill >/dev/null 2>&1; then
    taskkill //IM poolai.exe //F 2>/dev/null || true
    taskkill //IM poolai-worker.exe //F 2>/dev/null || true
  fi
  echo "Done."
}

cmd_status() {
  local ports="${1:-8080 8081 9090}"
  for p in $ports; do
    if curl -sf --max-time 3 "http://127.0.0.1:${p}/api/v1/health" >/dev/null 2>&1; then
      echo "UP   http://127.0.0.1:${p}/api/v1/health"
    else
      echo "DOWN http://127.0.0.1:${p}/api/v1/health"
    fi
  done
}

cmd_single() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --bg) BG=1; shift ;;
      --port) PORT="$2"; shift 2 ;;
      --skip-build) SKIP_BUILD=1; shift ;;
      *) echo "Unknown option: $1"; usage; exit 1 ;;
    esac
  done

  local data="$ROOT/data/dev/single"
  local raid="$data/raid"
  local logs="$ROOT/data/dev/logs"
  mkdir -p "$raid" "$logs"

  if [[ "$SKIP_BUILD" != "1" ]]; then
    cmd_build
  fi

  local exe
  exe="$(poolai_bin)"
  [[ -n "$exe" ]] || { echo "Missing poolai binary. Run: bash bin/run-poolai.sh build"; exit 1; }

  export POOLAI_HTTP_PORT="$PORT"
  export POOLAI_DATA_PATH="$data"
  export POOLAI_RAID_BASE_PATH="$raid"

  local url="http://127.0.0.1:${PORT}"
  echo "PoolAI single node"
  echo "  API:  ${url}/api/v1/health"
  echo "  UI:   ${url}/ui"
  echo "  Admin:${url}/ui/admin  (login admin / admin123)"
  echo "  Data: $data"

  if [[ "$BG" == "1" ]]; then
    local log="$logs/single-${PORT}.log"
    nohup "$exe" >"$log" 2>&1 &
    echo "Background PID $! — log: $log"
    echo "Stop: bash bin/run-poolai.sh stop"
    sleep 3
    cmd_status "$PORT"
  else
    echo "Foreground (Ctrl+C to stop)..."
    exec "$exe"
  fi
}

cmd_docker() {
  if ! command -v docker >/dev/null 2>&1; then
    echo "docker not found in PATH"
    exit 1
  fi
  cd "$ROOT/docker"
  docker compose up -d "${@:---build}"
  echo ""
  echo "UI: http://localhost:8080/ui"
  docker compose ps
}

CMD="${1:-single}"
shift || true

case "$CMD" in
  help|-h|--help) usage ;;
  build) cmd_build ;;
  stop) cmd_stop ;;
  status) cmd_status "${*:-}" ;;
  single) cmd_single "$@" ;;
  lan) exec "$POOLAI_BASH" "$ROOT/bin/run-lan-nodes.sh" "$@" ;;
  virtual-node|vn) exec "$POOLAI_BASH" "$ROOT/bin/run-virtual-node-dev.sh" "$@" ;;
  docker) cmd_docker "$@" ;;
  *)
    echo "Unknown command: $CMD"
    usage
    exit 1
    ;;
esac
