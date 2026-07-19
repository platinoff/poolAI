#!/usr/bin/env bash
# FM-003 / FM-016+++: health + virtual-node bootstrap checks for local dev stand.
# PH-S54: optional RAID job store step (VERIFY_RAID_JOB_STORE=1, coordinator on raid backend).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

COORD_PORT="${COORD_PORT:-8080}"
WORKER_PORT="${WORKER_PORT:-9090}"
NODE_B_PORT="${NODE_B_PORT:-8081}"
WORKER_ID="${WORKER_ID:-vn-dev-stand}"
TIMEOUT="${CURL_TIMEOUT:-5}"
WARMUP_SECS="${VERIFY_WARMUP_SECS:-50}"
TASK_RETRIES="${VERIFY_TASK_RETRIES:-12}"
TASK_SLEEP="${VERIFY_TASK_SLEEP:-5}"
MIN_COMPLETED="${VERIFY_MIN_COMPLETED:-4}"
VERIFY_ML_PIPELINE="${VERIFY_ML_PIPELINE:-1}"
VERIFY_RAID_JOB_STORE="${VERIFY_RAID_JOB_STORE:-0}"
VERIFY_STAND_SMOKE="${VERIFY_STAND_SMOKE:-0}"
VERIFY_MIGRATION_ADVISORY="${VERIFY_MIGRATION_ADVISORY:-0}"
VERIFY_STABLE_TOUCHUP="${VERIFY_STABLE_TOUCHUP:-0}"
VERIFY_EDGE_VERIFICATION="${VERIFY_EDGE_VERIFICATION:-0}"
HEALTH_RETRIES="${VERIFY_HEALTH_RETRIES:-45}"
HEALTH_SLEEP="${VERIFY_HEALTH_SLEEP:-2}"

COORD_URL="http://127.0.0.1:${COORD_PORT}"
ML_DEMO_URL="${COORD_URL}/api/enterprise/ai-ml/pipeline/demo"

check() {
  local name="$1"
  local url="$2"
  if curl -sf --max-time "$TIMEOUT" "$url" >/dev/null; then
    echo "OK  $name -> $url"
  else
    echo "FAIL $name -> $url"
    return 1
  fi
}

json_field() {
  local json="$1"
  local field="$2"
  echo "$json" | sed -n "s/.*\"${field}\":\s*\([0-9][0-9]*\).*/\1/p" | head -1
}

json_string_field() {
  local json="$1"
  local field="$2"
  echo "$json" | sed -n "s/.*\"${field}\":\"\([^\"]*\)\".*/\1/p" | head -1
}

wait_coord_health() {
  local attempt=0
  while [[ "$attempt" -lt "$HEALTH_RETRIES" ]]; do
    if curl -sf --max-time "$TIMEOUT" "${COORD_URL}/api/v1/health" >/dev/null; then
      return 0
    fi
    attempt=$((attempt + 1))
    sleep "$HEALTH_SLEEP"
  done
  echo "FAIL coordinator health not ready after restart (${COORD_URL})"
  return 1
}

resolve_poolai_bin() {
  local p
  for p in \
    "${POOLAI_BIN:-}" \
    "$ROOT/target/release/poolai.exe" \
    "$ROOT/target/release/poolai" \
    "$ROOT/target/debug/poolai.exe" \
    "$ROOT/target/debug/poolai"; do
    [[ -n "$p" && -x "$p" ]] && {
      echo "$p"
      return 0
    }
  done
  return 1
}

kill_listener_on_port() {
  local port="$1"
  local pid=""
  if command -v netstat >/dev/null 2>&1; then
    pid="$(
      netstat -ano 2>/dev/null \
        | grep -E ":${port}[[:space:]]" \
        | grep -Ei 'LISTEN' \
        | awk '{print $NF}' \
        | head -1
    )"
  fi
  if [[ -z "$pid" || "$pid" == "0" ]]; then
    return 0
  fi
  kill "$pid" 2>/dev/null || taskkill //PID "$pid" //F 2>/dev/null || true
  local i
  for ((i = 0; i < 10; i++)); do
    kill -0 "$pid" 2>/dev/null || return 0
    sleep 1
  done
}

restart_coordinator() {
  if [[ -n "${POOLAI_E2E_STAND_ROOT:-}" && -x "${POOLAI_E2E_STAND_ROOT}/restart.sh" ]]; then
    bash "${POOLAI_E2E_STAND_ROOT}/restart.sh"
    wait_coord_health
    return
  fi

  local raid="${POOLAI_RAID_BASE_PATH:-$ROOT/data/dev/single/raid}"
  local data="${POOLAI_DATA_PATH:-$ROOT/data/dev/single}"
  local job_store="${POOLAI_JOB_STORE:-raid}"
  local bin log
  bin="$(resolve_poolai_bin)" || {
    echo "FAIL restart: poolai binary not found (build or set POOLAI_BIN)"
    return 1
  }

  kill_listener_on_port "$COORD_PORT"
  sleep 2

  log="${POOLAI_VERIFY_LOG:-$ROOT/data/dev/logs/verify-restart-${COORD_PORT}.log}"
  mkdir -p "$(dirname "$log")"
  echo "  ... restarting coordinator (job_store=${job_store}, raid=${raid})"
  POOLAI_HTTP_PORT="$COORD_PORT" \
    POOLAI_RAID_BASE_PATH="$raid" \
    POOLAI_DATA_PATH="$data" \
    POOLAI_JOB_STORE="$job_store" \
    RUST_LOG="${RUST_LOG:-warn}" \
    nohup "$bin" >"$log" 2>&1 &
  wait_coord_health
}

verify_raid_job_store() {
  # PH-S851: optional RAID job store restart persist (VERIFY_RAID_JOB_STORE=1).
  local jobs_json backend job_json job_id detail_json got_id got_kind got_status

  jobs_json="$(curl -sf --max-time "$TIMEOUT" "${COORD_URL}/api/v1/jobs" || true)"
  if [[ -z "$jobs_json" ]]; then
    echo "FAIL RAID job store: GET /api/v1/jobs unavailable"
    return 1
  fi

  backend="$(json_string_field "$jobs_json" "store_backend")"
  if [[ "$backend" != "raid" ]]; then
    echo "SKIP RAID job store -> store_backend=${backend:-?} (need raid; start with POOLAI_JOB_STORE=raid and POOLAI_RAID_BASE_PATH before poolai)"
    return 0
  fi
  echo "OK  job store backend -> raid"

  job_json="$(
    curl -sf --max-time "$TIMEOUT" -X POST "${COORD_URL}/api/v1/jobs" \
      -H "Content-Type: application/json" \
      -d '{"kind":"inference","priority":7,"input_artifact_ids":["ph-s54-raid-smoke"]}' \
      || true
  )"
  job_id="$(json_string_field "$job_json" "id")"
  if [[ -z "$job_id" ]]; then
    echo "FAIL RAID job store: POST /api/v1/jobs did not return id"
    return 1
  fi
  echo "OK  RAID job create -> id=${job_id}"

  restart_coordinator || return 1
  echo "OK  coordinator restart -> health"

  detail_json="$(curl -sf --max-time "$TIMEOUT" "${COORD_URL}/api/v1/jobs/${job_id}" || true)"
  if [[ -z "$detail_json" ]] \
    || ! echo "$detail_json" | grep -q "\"id\":\"${job_id}\"" \
    || ! echo "$detail_json" | grep -q '"kind":"inference"' \
    || ! echo "$detail_json" | grep -q '"status":"scheduled"'; then
    echo "FAIL RAID job persist after restart (GET /jobs/${job_id})"
    return 1
  fi
  echo "OK  RAID job persist -> GET /jobs/${job_id} (inference, scheduled)"
  return 0
}

fail=0
check "coordinator" "${COORD_URL}/api/v1/health" || fail=1
check "node-B (optional)" "http://127.0.0.1:${NODE_B_PORT}/api/v1/health" || true
check "virtual worker" "http://127.0.0.1:${WORKER_PORT}/health" || fail=1

if [[ "$fail" -ne 0 ]]; then
  echo "One or more required health endpoints failed."
  exit 1
fi

echo "Waiting ${WARMUP_SECS}s for worker bootstrap tasks..."
sleep "$WARMUP_SECS"

vn_json="$(curl -sf --max-time "$TIMEOUT" "${COORD_URL}/api/v1/discovery/virtual-nodes" || true)"
if echo "$vn_json" | grep -q "\"peer_id\":\"${WORKER_ID}\""; then
  echo "OK  discovery virtual-node -> ${WORKER_ID}"
else
  echo "FAIL discovery virtual-node missing ${WORKER_ID}"
  fail=1
fi

workers_json="$(curl -sf --max-time "$TIMEOUT" "${COORD_URL}/api/v1/workers" || true)"
if echo "$workers_json" | grep -q "\"id\":\"${WORKER_ID}\""; then
  echo "OK  pool join -> worker ${WORKER_ID} in /workers"
else
  echo "FAIL pool join: ${WORKER_ID} not listed in /api/v1/workers"
  fail=1
fi

completed=0
attempt=0
while [[ "$attempt" -lt "$TASK_RETRIES" ]]; do
  status_json="$(curl -sf --max-time "$TIMEOUT" \
    "${COORD_URL}/api/v1/virtual-nodes/${WORKER_ID}/tasks/status" || true)"
  completed="$(json_field "$status_json" "completed")"
  completed="${completed:-0}"
  pending="$(json_field "$status_json" "pending")"
  pending="${pending:-?}"
  if [[ "$completed" -ge "$MIN_COMPLETED" ]]; then
    echo "OK  bootstrap tasks -> completed=${completed} pending=${pending}"
    break
  fi
  attempt=$((attempt + 1))
  if [[ "$attempt" -lt "$TASK_RETRIES" ]]; then
    echo "  ... tasks completed=${completed}/${MIN_COMPLETED}, retry in ${TASK_SLEEP}s"
    sleep "$TASK_SLEEP"
  fi
done

if [[ "${completed:-0}" -lt "$MIN_COMPLETED" ]]; then
  echo "FAIL bootstrap tasks: completed=${completed:-0} (need >= ${MIN_COMPLETED})"
  fail=1
fi

health_json="$(curl -sf --max-time "$TIMEOUT" "http://127.0.0.1:${WORKER_PORT}/health" || true)"
cached="$(echo "$health_json" | sed -n 's/.*"cached_artifacts":\s*\([0-9][0-9]*\).*/\1/p' | head -1)"
cached="${cached:-0}"
if [[ "$cached" -ge 1 ]]; then
  echo "OK  worker cache -> cached_artifacts=${cached}"
else
  echo "FAIL worker cache: cached_artifacts=${cached} (need >= 1 after raid_artifact_probe)"
  fail=1
fi

if [[ "$VERIFY_ML_PIPELINE" == "1" ]]; then
  ml_demo="$(curl -sf --max-time "$TIMEOUT" "$ML_DEMO_URL" 2>/dev/null || true)"
  if [[ -z "$ml_demo" ]]; then
    echo "SKIP ML pipeline demo -> ${ML_DEMO_URL} (endpoint unavailable; build with enterprise+ml or set VERIFY_ML_PIPELINE=0)"
  elif echo "$ml_demo" | grep -q '"step_kind":"profiling"' \
    && echo "$ml_demo" | grep -q '"status":"completed"'; then
    echo "OK  ML pipeline demo -> step_kind=profiling status=completed"
  else
    echo "FAIL ML pipeline demo: missing profiling step metrics in response"
    fail=1
  fi
fi

if [[ "$VERIFY_RAID_JOB_STORE" == "1" ]]; then
  verify_raid_job_store || fail=1
fi

if [[ "$VERIFY_STAND_SMOKE" == "1" ]]; then
  export POOLAI_BASE_URL="$COORD_URL"
  echo "Running poolai-http-stand-smoke --run-local-smoke (PH-S1094)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-http-stand-smoke -- --run-local-smoke); then
    echo "OK  stand smoke run-local subset"
  else
    echo "FAIL stand smoke run-local subset"
    fail=1
  fi
fi

if [[ "$VERIFY_MIGRATION_ADVISORY" == "1" ]]; then
  echo "Running poolai-loc-audit --migration-advisory (PH-S1103)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --migration-advisory); then
    echo "OK  rust migration advisory loc-audit"
  else
    echo "FAIL rust migration advisory loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_STABLE_TOUCHUP" == "1" ]]; then
  echo "Running poolai-loc-audit --stable-touchup (PH-S1113)..."
  if (cd "$ROOT" && cargo run --bin poolai-loc-audit -- --stable-touchup); then
    echo "OK  STABLE touch-up loc-audit"
  else
    echo "FAIL STABLE touch-up loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_EDGE_VERIFICATION" == "1" ]]; then
  echo "Running poolai-loc-audit --edge-verification-advisory (PH-S1125)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --edge-verification-advisory); then
    echo "OK  edge verification advisory loc-audit"
  else
    echo "FAIL edge verification advisory loc-audit"
    fail=1
  fi
fi

if [[ "$fail" -ne 0 ]]; then
  echo "Dev stand verification failed."
  exit 1
fi
echo "Dev stand verification passed (health + virtual-node bootstrap + ML pipeline demo when enabled + optional RAID job store + optional stand smoke)."
