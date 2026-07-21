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
LIGHT_FEATURES="${LIGHT_FEATURES:-enterprise,test-utils}"
PORT="${POOLAI_HTTP_PORT:-8080}"
SKIP_BUILD="${SKIP_BUILD:-0}"
LIGHT_BUILD="${LIGHT_BUILD:-0}"
JOB_STORE="${POOLAI_JOB_STORE:-}"
BG=0
LAST_RUN_PATH="$ROOT/data/dev/last_run.json"

usage() {
  cat <<'EOF'
Usage: /usr/bin/bash bin/run-poolai.sh <command> [options]
       (PowerShell: .\bin\run-poolai.ps1 single -Background)

Commands:
  single          One coordinator (default). UI: http://127.0.0.1:8080/ui
  quick           Light build (optional) + single --bg + health wait (PH-S1012)
  lan             Two+ nodes on one host (FM-003 dev stand)
  virtual-node    Coordinator + poolai-worker (FM-016 dev stand)
  docker          docker compose up (docker/docker-compose.yml)
  build           cargo build (--features enterprise,ml,cloud,test-utils)
  stop            Stop poolai / poolai-worker processes
  status          curl health on common dev ports
  help            This message

Options (quick):
  --stand-smoke   After health wait, run poolai-http-stand-smoke --run-local-smoke (PH-S1095)
  --migration-advisory  After health wait, run poolai-loc-audit --migration-advisory (PH-S1104)
  --stable-touchup      After health wait, run poolai-loc-audit --stable-touchup (PH-S1114)
  --edge-verification   After health wait, run poolai-loc-audit --edge-verification-advisory (PH-S1125)
  --pre-push-canon      After health wait, run poolai-loc-audit --pre-push-canon (PH-S1134)
  --ci-canon            After health wait, run CI canon gate (PH-S1143)
  --tenant-persist      After health wait, run poolai-loc-audit --tenant-persist (PH-S1154)
  --tenant-store        After health wait, run poolai-loc-audit --tenant-store (PH-S1162)
  --tenant-api          After health wait, run poolai-loc-audit --tenant-api (PH-S1175)
  --tenant-admin-ops    After health wait, run poolai-loc-audit --tenant-admin-ops (PH-S1184)
  --tenant-stand-smoke  After health wait, live stand-smoke + loc-audit --tenant-stand-smoke (PH-S1194)
  --skip-build    Skip cargo build
  --port N        HTTP port (default 8080)

Options (single):
  --bg            Run in background (logs under data/dev/logs/)
  --port N        HTTP port (default 8080)
  --skip-build    Skip cargo build
  --light         Use LIGHT_FEATURES for faster compile (PH-S1011)
  --raid-jobs     Preset POOLAI_JOB_STORE=raid + auto RAID path
  --job-store X   Set POOLAI_JOB_STORE (json|sqlite|raid)

Environment:
  FEATURES        Cargo features (default: enterprise,ml,cloud,test-utils)
  LIGHT_FEATURES  Light profile (default: enterprise,test-utils)
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
  local feats="$FEATURES"
  if [[ "$LIGHT_BUILD" == "1" ]]; then
    feats="$LIGHT_FEATURES"
  fi
  echo "Building poolai (--features $feats)..."
  cargo build --features "$feats"
  cargo build --bin poolai-worker 2>/dev/null || true
}

save_last_run_snapshot() {
  local preset="$1" port="$2" feats="$3" job_store="${4:-}" pid="${5:-}"
  mkdir -p "$(dirname "$LAST_RUN_PATH")"
  local js="${job_store:-null}"
  if [[ -n "$job_store" ]]; then
    js="\"$job_store\""
  fi
  local pid_json="${pid:-null}"
  cat >"$LAST_RUN_PATH" <<EOF
{
  "preset": "$preset",
  "port": $port,
  "features": "$feats",
  "job_store": $js,
  "pid": $pid_json,
  "saved_at": "$(date +%s)"
}
EOF
}

load_last_run_port() {
  if [[ ! -f "$LAST_RUN_PATH" ]]; then
    return 1
  fi
  local p
  p="$(sed -n 's/.*"port"[[:space:]]*:[[:space:]]*\([0-9][0-9]*\).*/\1/p' "$LAST_RUN_PATH" | head -1)"
  [[ -n "$p" ]] || return 1
  PORT="$p"
}

wait_health() {
  local port="$1" tries="${2:-30}"
  local i=1
  while [[ "$i" -le "$tries" ]]; do
    if curl -sf --max-time 3 "http://127.0.0.1:${port}/api/v1/health" >/dev/null 2>&1; then
      echo "Health OK http://127.0.0.1:${port}/api/v1/health"
      return 0
    fi
    sleep 1
    i=$((i + 1))
  done
  echo "Health wait timeout on port $port" >&2
  return 1
}

cmd_stop() {
  echo "Stopping PoolAI processes..."
  save_last_run_snapshot "stop" "$PORT" "$FEATURES" "${POOLAI_JOB_STORE:-}" ""
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
      --light) LIGHT_BUILD=1; shift ;;
      --raid-jobs) JOB_STORE="raid"; shift ;;
      --job-store) JOB_STORE="$2"; shift 2 ;;
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
  if [[ -n "$JOB_STORE" ]]; then
    export POOLAI_JOB_STORE="$JOB_STORE"
  fi

  local url="http://127.0.0.1:${PORT}"
  echo "PoolAI single node"
  echo "  API:  ${url}/api/v1/health"
  echo "  UI:   ${url}/ui"
  echo "  Admin:${url}/ui/admin  (login admin / admin123)"
  echo "  Data: $data"
  echo "  Job store: ${POOLAI_JOB_STORE:-json}"
  if [[ "${POOLAI_JOB_STORE:-}" == "raid" ]]; then
    echo "  RAID path: $POOLAI_RAID_BASE_PATH"
  fi

  if [[ "$BG" == "1" ]]; then
    local log="$logs/single-${PORT}.log"
    nohup "$exe" >"$log" 2>&1 &
    local pid=$!
    save_last_run_snapshot "single" "$PORT" "${FEATURES}" "${POOLAI_JOB_STORE:-}" "$pid"
    echo "Background PID $pid — log: $log"
    echo "Stop: bash bin/run-poolai.sh stop"
    sleep 3
    cmd_status "$PORT"
  else
    echo "Foreground (Ctrl+C to stop)..."
    exec "$exe"
  fi
}

cmd_quick() {
  local stand_smoke=0
  local migration_advisory=0
  local stable_touchup=0
  local edge_verification=0
  local pre_push_canon=0
  local ci_canon=0
  local tenant_persist=0
  local tenant_store=0
  local tenant_api=0
  local tenant_admin_ops=0
  local tenant_stand_smoke=0
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --skip-build) SKIP_BUILD=1; shift ;;
      --stand-smoke) stand_smoke=1; shift ;;
      --migration-advisory) migration_advisory=1; shift ;;
      --stable-touchup) stable_touchup=1; shift ;;
      --edge-verification) edge_verification=1; shift ;;
      --pre-push-canon) pre_push_canon=1; shift ;;
      --ci-canon) ci_canon=1; shift ;;
      --tenant-persist) tenant_persist=1; shift ;;
      --tenant-store) tenant_store=1; shift ;;
      --tenant-api) tenant_api=1; shift ;;
      --tenant-admin-ops) tenant_admin_ops=1; shift ;;
      --tenant-stand-smoke) tenant_stand_smoke=1; shift ;;
      --port) PORT="$2"; shift 2 ;;
      *) echo "Unknown option: $1"; usage; exit 1 ;;
    esac
  done
  load_last_run_port || true
  cmd_single --bg --port "$PORT" --light ${SKIP_BUILD:+--skip-build}
  wait_health "$PORT"
  if [[ "$stand_smoke" == "1" ]]; then
    export POOLAI_BASE_URL="http://127.0.0.1:${PORT}"
    echo "Running poolai-http-stand-smoke --run-local-smoke (PH-S1095)..."
    cargo run --quiet --bin poolai-http-stand-smoke -- --run-local-smoke
  fi
  if [[ "$migration_advisory" == "1" ]]; then
    echo "Running poolai-loc-audit --migration-advisory (PH-S1104)..."
    cargo run --quiet --bin poolai-loc-audit -- --migration-advisory
  fi
  if [[ "$stable_touchup" == "1" ]]; then
    echo "Running poolai-loc-audit --stable-touchup (PH-S1114)..."
    cargo run --quiet --bin poolai-loc-audit -- --stable-touchup
  fi
  if [[ "$edge_verification" == "1" ]]; then
    echo "Running poolai-loc-audit --edge-verification-advisory (PH-S1125)..."
    cargo run --quiet --bin poolai-loc-audit -- --edge-verification-advisory
  fi
  if [[ "$pre_push_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --pre-push-canon (PH-S1134)..."
    cargo run --quiet --bin poolai-loc-audit -- --pre-push-canon
  fi
  if [[ "$ci_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --ci-canon (PH-S1143)..."
    cargo run --quiet --bin poolai-loc-audit -- --ci-canon
    echo "Running poolai-openapi-gap-audit (PH-S1143)..."
    cargo run --quiet --bin poolai-openapi-gap-audit
  fi
  if [[ "$tenant_persist" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-persist (PH-S1154)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-persist
  fi
  if [[ "$tenant_store" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-store (PH-S1162)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-store
  fi
  if [[ "$tenant_api" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-api (PH-S1175)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-api
  fi
  if [[ "$tenant_admin_ops" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-admin-ops (PH-S1184)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-admin-ops
  fi
  if [[ "$tenant_stand_smoke" == "1" ]]; then
    export POOLAI_BASE_URL="http://127.0.0.1:${PORT}"
    echo "Running poolai-http-stand-smoke --tenant-stand-smoke (PH-S1195)..."
    cargo run --quiet --bin poolai-http-stand-smoke -- --tenant-stand-smoke
    echo "Running poolai-loc-audit --tenant-stand-smoke (PH-S1194)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-stand-smoke
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
  quick) cmd_quick "$@" ;;
  lan) exec "$POOLAI_BASH" "$ROOT/bin/run-lan-nodes.sh" "$@" ;;
  virtual-node|vn) exec "$POOLAI_BASH" "$ROOT/bin/run-virtual-node-dev.sh" "$@" ;;
  docker) cmd_docker "$@" ;;
  *)
    echo "Unknown command: $CMD"
    usage
    exit 1
    ;;
esac
