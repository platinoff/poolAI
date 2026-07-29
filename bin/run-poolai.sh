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
  --tenant-loc-audit    After health wait, loc-audit --tenant-loc-audit aggregate (PH-S1202)
  --tenant-docs-canon   After health wait, loc-audit --tenant-docs-canon (PH-S1212)
  --tenant-vision-sync  After health wait, loc-audit --tenant-vision-sync (PH-S1222)
  --tenant-ratio-advisory  After health wait, loc-audit --tenant-ratio-advisory (PH-S1232)
  --tenant-horizon  After health wait, loc-audit --tenant-horizon (PH-S1242)
  --sso             After health wait, loc-audit --sso (PH-S1252)
  --sso-store       After health wait, loc-audit --sso-store (PH-S1262)
  --sso-api         After health wait, loc-audit --sso-api (PH-S1275)
  --sso-admin-ops   After health wait, loc-audit --sso-admin-ops (PH-S1284)
  --sso-stand-smoke After health wait, live stand-smoke + loc-audit --sso-stand-smoke (PH-S1294)
  --sso-loc-audit   After health wait, loc-audit --sso-loc-audit aggregate (PH-S1302)
  --sso-docs-canon  After health wait, loc-audit --sso-docs-canon (PH-S1312)
  --sso-vision-sync After health wait, loc-audit --sso-vision-sync (PH-S1322)
  --sso-ratio-advisory After health wait, loc-audit --sso-ratio-advisory (PH-S1332)
  --sso-horizon   After health wait, loc-audit --sso-horizon (PH-S1342)
  --audit         After health wait, loc-audit --audit (PH-S1352)
  --audit-store   After health wait, loc-audit --audit-store (PH-S1362)
  --audit-api     After health wait, loc-audit --audit-api (PH-S1374)
  --audit-admin-ops  After health wait, loc-audit --audit-admin-ops (PH-S1384)
  --audit-stand-smoke After health wait, live stand-smoke + loc-audit --audit-stand-smoke (PH-S1394)
  --audit-loc-audit After health wait, loc-audit --audit-loc-audit aggregate (PH-S1402)
  --audit-docs-canon After health wait, loc-audit --audit-docs-canon (PH-S1412)
  --audit-vision-sync After health wait, loc-audit --audit-vision-sync (PH-S1422)
  --audit-ratio-advisory After health wait, loc-audit --audit-ratio-advisory (PH-S1432)
  --audit-horizon After health wait, loc-audit --audit-horizon (PH-S1442)
  --policy        After health wait, loc-audit --policy (PH-S1452)
  --policy-store  After health wait, loc-audit --policy-store (PH-S1462)
  --policy-api    After health wait, loc-audit --policy-api (PH-S1474)
  --policy-admin-ops After health wait, loc-audit --policy-admin-ops (PH-S1484)
  --policy-stand-smoke After health wait, stand smoke + loc-audit --policy-stand-smoke (PH-S1495)
  --policy-loc-audit After health wait, loc-audit --policy-loc-audit aggregate (PH-S1502)
  --policy-docs-canon After health wait, loc-audit --policy-docs-canon (PH-S1512)
  --policy-vision-sync After health wait, loc-audit --policy-vision-sync (PH-S1522)
  --policy-ratio-advisory After health wait, loc-audit --policy-ratio-advisory (PH-S1532)
  --policy-horizon After health wait, loc-audit --policy-horizon (PH-S1544)
  --monitoring    After health wait, loc-audit --monitoring (PH-S1552)
  --monitoring-store After health wait, loc-audit --monitoring-store (PH-S1562)
  --monitoring-api After health wait, loc-audit --monitoring-api (PH-S1574)
  --monitoring-admin-ops After health wait, loc-audit --monitoring-admin-ops (PH-S1584)
  --monitoring-stand-smoke After health wait, stand smoke + loc-audit --monitoring-stand-smoke (PH-S1595)
  --monitoring-loc-audit After health wait, loc-audit --monitoring-loc-audit aggregate (PH-S1602)
  --monitoring-docs-canon After health wait, loc-audit --monitoring-docs-canon (PH-S1614)
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
  local tenant_loc_audit=0
  local tenant_docs_canon=0
  local tenant_vision_sync=0
  local tenant_ratio_advisory=0
  local tenant_horizon=0
  local sso=0
  local sso_store=0
  local sso_api=0
  local sso_admin_ops=0
  local sso_stand_smoke=0
  local sso_loc_audit=0
  local sso_docs_canon=0
  local sso_vision_sync=0
  local sso_ratio_advisory=0
  local sso_horizon=0
  local audit=0
  local audit_store=0
  local audit_api=0
  local audit_admin_ops=0
  local audit_stand_smoke=0
  local audit_loc_audit=0
  local audit_docs_canon=0
  local audit_vision_sync=0
  local audit_ratio_advisory=0
  local audit_horizon=0
  local policy=0
  local policy_store=0
  local policy_api=0
  local policy_admin_ops=0
  local policy_stand_smoke=0
  local policy_loc_audit=0
  local policy_docs_canon=0
  local policy_vision_sync=0
  local policy_ratio_advisory=0
  local policy_horizon=0
  local monitoring=0
  local monitoring_store=0
  local monitoring_api=0
  local monitoring_admin_ops=0
  local monitoring_stand_smoke=0
  local monitoring_loc_audit=0
  local monitoring_docs_canon=0
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
      --tenant-loc-audit) tenant_loc_audit=1; shift ;;
      --tenant-docs-canon) tenant_docs_canon=1; shift ;;
      --tenant-vision-sync) tenant_vision_sync=1; shift ;;
      --tenant-ratio-advisory) tenant_ratio_advisory=1; shift ;;
      --tenant-horizon) tenant_horizon=1; shift ;;
      --sso) sso=1; shift ;;
      --sso-store) sso_store=1; shift ;;
      --sso-api) sso_api=1; shift ;;
      --sso-admin-ops) sso_admin_ops=1; shift ;;
      --sso-stand-smoke) sso_stand_smoke=1; shift ;;
      --sso-loc-audit) sso_loc_audit=1; shift ;;
      --sso-docs-canon) sso_docs_canon=1; shift ;;
      --sso-vision-sync) sso_vision_sync=1; shift ;;
      --sso-ratio-advisory) sso_ratio_advisory=1; shift ;;
      --sso-horizon) sso_horizon=1; shift ;;
      --audit-store) audit_store=1; shift ;;
      --audit-api) audit_api=1; shift ;;
      --audit-admin-ops) audit_admin_ops=1; shift ;;
      --audit-stand-smoke) audit_stand_smoke=1; shift ;;
      --audit-loc-audit) audit_loc_audit=1; shift ;;
      --audit-docs-canon) audit_docs_canon=1; shift ;;
      --audit-vision-sync) audit_vision_sync=1; shift ;;
      --audit-ratio-advisory) audit_ratio_advisory=1; shift ;;
      --audit-horizon) audit_horizon=1; shift ;;
      --audit) audit=1; shift ;;
      --policy-admin-ops) policy_admin_ops=1; shift ;;
      --policy-stand-smoke) policy_stand_smoke=1; shift ;;
      --policy-loc-audit) policy_loc_audit=1; shift ;;
      --policy-docs-canon) policy_docs_canon=1; shift ;;
      --policy-vision-sync) policy_vision_sync=1; shift ;;
      --policy-ratio-advisory) policy_ratio_advisory=1; shift ;;
      --policy-horizon) policy_horizon=1; shift ;;
      --policy) policy=1; shift ;;
      --monitoring) monitoring=1; shift ;;
      --monitoring-store) monitoring_store=1; shift ;;
      --monitoring-api) monitoring_api=1; shift ;;
      --monitoring-admin-ops) monitoring_admin_ops=1; shift ;;
      --monitoring-stand-smoke) monitoring_stand_smoke=1; shift ;;
      --monitoring-loc-audit) monitoring_loc_audit=1; shift ;;
      --monitoring-docs-canon) monitoring_docs_canon=1; shift ;;
      --policy-store) policy_store=1; shift ;;
      --policy-api) policy_api=1; shift ;;
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
  if [[ "$tenant_loc_audit" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-loc-audit (PH-S1202)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-loc-audit
  fi
  if [[ "$tenant_docs_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-docs-canon (PH-S1212)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-docs-canon
  fi
  if [[ "$tenant_vision_sync" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-vision-sync (PH-S1222)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-vision-sync
  fi
  if [[ "$tenant_ratio_advisory" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-ratio-advisory (PH-S1232)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-ratio-advisory
  fi
  if [[ "$tenant_horizon" == "1" ]]; then
    echo "Running poolai-loc-audit --tenant-horizon (PH-S1242)..."
    cargo run --quiet --bin poolai-loc-audit -- --tenant-horizon
  fi
  if [[ "$sso" == "1" ]]; then
    echo "Running poolai-loc-audit --sso (PH-S1252)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso
  fi
  if [[ "$sso_store" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-store (PH-S1262)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-store
  fi
  if [[ "$sso_api" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-api (PH-S1275)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-api
  fi
  if [[ "$sso_admin_ops" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-admin-ops (PH-S1284)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-admin-ops
  fi
  if [[ "$sso_stand_smoke" == "1" ]]; then
    export POOLAI_BASE_URL="http://127.0.0.1:${PORT}"
    echo "Running poolai-http-stand-smoke --sso-stand-smoke (PH-S1295)..."
    cargo run --quiet --bin poolai-http-stand-smoke -- --sso-stand-smoke
    echo "Running poolai-loc-audit --sso-stand-smoke (PH-S1294)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-stand-smoke
  fi
  if [[ "$sso_loc_audit" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-loc-audit (PH-S1302)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-loc-audit
  fi
  if [[ "$sso_docs_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-docs-canon (PH-S1312)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-docs-canon
  fi
  if [[ "$sso_vision_sync" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-vision-sync (PH-S1322)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-vision-sync
  fi
  if [[ "$sso_ratio_advisory" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-ratio-advisory (PH-S1332)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-ratio-advisory
  fi
  if [[ "$sso_horizon" == "1" ]]; then
    echo "Running poolai-loc-audit --sso-horizon (PH-S1342)..."
    cargo run --quiet --bin poolai-loc-audit -- --sso-horizon
  fi
  if [[ "$audit" == "1" ]]; then
    echo "Running poolai-loc-audit --audit (PH-S1352)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit
  fi
  if [[ "$audit_store" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-store (PH-S1362)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-store
  fi
  if [[ "$audit_api" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-api (PH-S1374)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-api
  fi
  if [[ "$audit_admin_ops" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-admin-ops (PH-S1384)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-admin-ops
  fi
  if [[ "$audit_stand_smoke" == "1" ]]; then
    export POOLAI_BASE_URL="http://127.0.0.1:${PORT}"
    echo "Running poolai-http-stand-smoke --audit-stand-smoke (PH-S1395)..."
    cargo run --quiet --bin poolai-http-stand-smoke -- --audit-stand-smoke
    echo "Running poolai-loc-audit --audit-stand-smoke (PH-S1394)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-stand-smoke
  fi
  if [[ "$audit_loc_audit" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-loc-audit (PH-S1402)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-loc-audit
  fi
  if [[ "$audit_docs_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-docs-canon (PH-S1412)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-docs-canon
  fi
  if [[ "$audit_vision_sync" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-vision-sync (PH-S1422)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-vision-sync
  fi
  if [[ "$audit_ratio_advisory" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-ratio-advisory (PH-S1432)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-ratio-advisory
  fi
  if [[ "$audit_horizon" == "1" ]]; then
    echo "Running poolai-loc-audit --audit-horizon (PH-S1442)..."
    cargo run --quiet --bin poolai-loc-audit -- --audit-horizon
  fi
  if [[ "$policy" == "1" ]]; then
    echo "Running poolai-loc-audit --policy (PH-S1452)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy
  fi
  if [[ "$policy_store" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-store (PH-S1462)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-store
  fi
  if [[ "$policy_api" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-api (PH-S1474)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-api
  fi
  if [[ "$policy_admin_ops" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-admin-ops (PH-S1484)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-admin-ops
  fi
  if [[ "$policy_stand_smoke" == "1" ]]; then
    echo "Running poolai-http-stand-smoke --policy-stand-smoke (PH-S1495)..."
    cargo run --quiet --bin poolai-http-stand-smoke -- --policy-stand-smoke
    echo "Running poolai-loc-audit --policy-stand-smoke (PH-S1495)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-stand-smoke
  fi
  if [[ "$policy_loc_audit" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-loc-audit (PH-S1502)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-loc-audit
  fi
  if [[ "$policy_docs_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-docs-canon (PH-S1512)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-docs-canon
  fi
  if [[ "$policy_vision_sync" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-vision-sync (PH-S1522)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-vision-sync
  fi
  if [[ "$policy_ratio_advisory" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-ratio-advisory (PH-S1532)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-ratio-advisory
  fi
  if [[ "$policy_horizon" == "1" ]]; then
    echo "Running poolai-loc-audit --policy-horizon (PH-S1544)..."
    cargo run --quiet --bin poolai-loc-audit -- --policy-horizon
  fi
  if [[ "$monitoring" == "1" ]]; then
    echo "Running poolai-loc-audit --monitoring (PH-S1552)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring
  fi
  if [[ "$monitoring_store" == "1" ]]; then
    echo "Running poolai-loc-audit --monitoring-store (PH-S1562)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring-store
  fi
  if [[ "$monitoring_api" == "1" ]]; then
    echo "Running poolai-loc-audit --monitoring-api (PH-S1574)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring-api
  fi
  if [[ "$monitoring_admin_ops" == "1" ]]; then
    echo "Running poolai-loc-audit --monitoring-admin-ops (PH-S1584)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring-admin-ops
  fi
  if [[ "$monitoring_stand_smoke" == "1" ]]; then
    export POOLAI_BASE_URL="http://127.0.0.1:${PORT}"
    echo "Running poolai-http-stand-smoke --monitoring-stand-smoke (PH-S1595)..."
    cargo run --quiet --bin poolai-http-stand-smoke -- --monitoring-stand-smoke
    echo "Running poolai-loc-audit --monitoring-stand-smoke (PH-S1595)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring-stand-smoke
  fi
  if [[ "$monitoring_loc_audit" == "1" ]]; then
    echo "Running poolai-loc-audit --monitoring-loc-audit (PH-S1602)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring-loc-audit
  fi
  if [[ "$monitoring_docs_canon" == "1" ]]; then
    echo "Running poolai-loc-audit --monitoring-docs-canon (PH-S1614)..."
    cargo run --quiet --bin poolai-loc-audit -- --monitoring-docs-canon
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
