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
VERIFY_PRE_PUSH_CANON="${VERIFY_PRE_PUSH_CANON:-0}"
VERIFY_CI_CANON="${VERIFY_CI_CANON:-0}"
VERIFY_TENANT_PERSIST="${VERIFY_TENANT_PERSIST:-0}"
VERIFY_TENANT_STORE="${VERIFY_TENANT_STORE:-0}"
VERIFY_TENANT_API="${VERIFY_TENANT_API:-0}"
VERIFY_TENANT_ADMIN_OPS="${VERIFY_TENANT_ADMIN_OPS:-0}"
VERIFY_TENANT_STAND_SMOKE="${VERIFY_TENANT_STAND_SMOKE:-0}"
VERIFY_TENANT_LOC_AUDIT="${VERIFY_TENANT_LOC_AUDIT:-0}"
VERIFY_TENANT_DOCS_CANON="${VERIFY_TENANT_DOCS_CANON:-0}"
VERIFY_TENANT_VISION_SYNC="${VERIFY_TENANT_VISION_SYNC:-0}"
VERIFY_TENANT_RATIO_ADVISORY="${VERIFY_TENANT_RATIO_ADVISORY:-0}"
VERIFY_TENANT_HORIZON="${VERIFY_TENANT_HORIZON:-0}"
VERIFY_SSO="${VERIFY_SSO:-0}"
VERIFY_SSO_STORE="${VERIFY_SSO_STORE:-0}"
VERIFY_SSO_API="${VERIFY_SSO_API:-0}"
VERIFY_SSO_ADMIN_OPS="${VERIFY_SSO_ADMIN_OPS:-0}"
VERIFY_SSO_STAND_SMOKE="${VERIFY_SSO_STAND_SMOKE:-0}"
VERIFY_AUDIT_STAND_SMOKE="${VERIFY_AUDIT_STAND_SMOKE:-0}"
VERIFY_AUDIT_LOC_AUDIT="${VERIFY_AUDIT_LOC_AUDIT:-0}"
VERIFY_AUDIT_DOCS_CANON="${VERIFY_AUDIT_DOCS_CANON:-0}"
VERIFY_AUDIT_VISION_SYNC="${VERIFY_AUDIT_VISION_SYNC:-0}"
VERIFY_AUDIT_RATIO_ADVISORY="${VERIFY_AUDIT_RATIO_ADVISORY:-0}"
VERIFY_AUDIT_HORIZON="${VERIFY_AUDIT_HORIZON:-0}"
VERIFY_POLICY="${VERIFY_POLICY:-0}"
VERIFY_POLICY_STORE="${VERIFY_POLICY_STORE:-0}"
VERIFY_POLICY_API="${VERIFY_POLICY_API:-0}"
VERIFY_POLICY_ADMIN_OPS="${VERIFY_POLICY_ADMIN_OPS:-0}"
VERIFY_POLICY_STAND_SMOKE="${VERIFY_POLICY_STAND_SMOKE:-0}"
VERIFY_POLICY_LOC_AUDIT="${VERIFY_POLICY_LOC_AUDIT:-0}"
VERIFY_POLICY_DOCS_CANON="${VERIFY_POLICY_DOCS_CANON:-0}"
VERIFY_POLICY_VISION_SYNC="${VERIFY_POLICY_VISION_SYNC:-0}"
VERIFY_POLICY_RATIO_ADVISORY="${VERIFY_POLICY_RATIO_ADVISORY:-0}"
VERIFY_POLICY_HORIZON="${VERIFY_POLICY_HORIZON:-0}"
VERIFY_MONITORING="${VERIFY_MONITORING:-0}"
VERIFY_MONITORING_STORE="${VERIFY_MONITORING_STORE:-0}"
VERIFY_MONITORING_API="${VERIFY_MONITORING_API:-0}"
VERIFY_MONITORING_ADMIN_OPS="${VERIFY_MONITORING_ADMIN_OPS:-0}"
VERIFY_MONITORING_STAND_SMOKE="${VERIFY_MONITORING_STAND_SMOKE:-0}"
VERIFY_MONITORING_LOC_AUDIT="${VERIFY_MONITORING_LOC_AUDIT:-0}"
VERIFY_SSO_LOC_AUDIT="${VERIFY_SSO_LOC_AUDIT:-0}"
VERIFY_SSO_DOCS_CANON="${VERIFY_SSO_DOCS_CANON:-0}"
VERIFY_SSO_VISION_SYNC="${VERIFY_SSO_VISION_SYNC:-0}"
VERIFY_SSO_RATIO_ADVISORY="${VERIFY_SSO_RATIO_ADVISORY:-0}"
VERIFY_SSO_HORIZON="${VERIFY_SSO_HORIZON:-0}"
VERIFY_AUDIT="${VERIFY_AUDIT:-0}"
VERIFY_AUDIT_STORE="${VERIFY_AUDIT_STORE:-0}"
VERIFY_AUDIT_API="${VERIFY_AUDIT_API:-0}"
VERIFY_AUDIT_ADMIN_OPS="${VERIFY_AUDIT_ADMIN_OPS:-0}"
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

if [[ "$VERIFY_PRE_PUSH_CANON" == "1" ]]; then
  echo "Running poolai-loc-audit --pre-push-canon (PH-S1134)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --pre-push-canon); then
    echo "OK  pre-push canon gate loc-audit"
  else
    echo "FAIL pre-push canon gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_CI_CANON" == "1" ]]; then
  echo "Running poolai-openapi-gap-audit (PH-S1142)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-openapi-gap-audit); then
    echo "OK  openapi-gap-audit"
  else
    echo "FAIL openapi-gap-audit"
    fail=1
  fi
  echo "Running poolai-loc-audit --ci-canon (PH-S1142)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --ci-canon); then
    echo "OK  CI canon gate loc-audit"
  else
    echo "FAIL CI canon gate loc-audit"
    fail=1
  fi
  echo "Running poolai-loc-audit --advisory --min-ratio 0.95 (PH-S1142)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --advisory --min-ratio 0.95); then
    echo "OK  rust-ratio advisory loc-audit"
  else
    echo "FAIL rust-ratio advisory loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_PERSIST" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-persist (PH-S1153)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-persist); then
    echo "OK  tenant persist gate loc-audit"
  else
    echo "FAIL tenant persist gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_STORE" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-store (PH-S1162)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-store); then
    echo "OK  tenant store-wire gate loc-audit"
  else
    echo "FAIL tenant store-wire gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_API" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-api (PH-S1175)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-api); then
    echo "OK  tenant HTTP API contracts gate loc-audit"
  else
    echo "FAIL tenant HTTP API contracts gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_ADMIN_OPS" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-admin-ops (PH-S1184)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-admin-ops); then
    echo "OK  tenant admin/ops gate loc-audit"
  else
    echo "FAIL tenant admin/ops gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_STAND_SMOKE" == "1" ]]; then
  export POOLAI_BASE_URL="$COORD_URL"
  echo "Running poolai-http-stand-smoke --tenant-stand-smoke (PH-S1195)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-http-stand-smoke -- --tenant-stand-smoke); then
    echo "OK  tenant stand-smoke live suite"
  else
    echo "FAIL tenant stand-smoke live suite"
    fail=1
  fi
  echo "Running poolai-loc-audit --tenant-stand-smoke (PH-S1194)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-stand-smoke); then
    echo "OK  tenant stand-smoke gate loc-audit"
  else
    echo "FAIL tenant stand-smoke gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_LOC_AUDIT" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-loc-audit (PH-S1202)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-loc-audit); then
    echo "OK  tenant loc-audit aggregate gate"
  else
    echo "FAIL tenant loc-audit aggregate gate"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_DOCS_CANON" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-docs-canon (PH-S1212)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-docs-canon); then
    echo "OK  tenant docs-canon gate"
  else
    echo "FAIL tenant docs-canon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_VISION_SYNC" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-vision-sync (PH-S1222)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-vision-sync); then
    echo "OK  tenant vision-sync gate"
  else
    echo "FAIL tenant vision-sync gate"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_RATIO_ADVISORY" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-ratio-advisory (PH-S1232)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-ratio-advisory); then
    echo "OK  tenant ratio-advisory gate"
  else
    echo "FAIL tenant ratio-advisory gate"
    fail=1
  fi
fi

if [[ "$VERIFY_TENANT_HORIZON" == "1" ]]; then
  echo "Running poolai-loc-audit --tenant-horizon (PH-S1242)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --tenant-horizon); then
    echo "OK  tenant horizon gate"
  else
    echo "FAIL tenant horizon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO" == "1" ]]; then
  echo "Running poolai-loc-audit --sso (PH-S1252)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso); then
    echo "OK  SSO depth gate loc-audit"
  else
    echo "FAIL SSO depth gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_STORE" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-store (PH-S1262)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-store); then
    echo "OK  SSO store wire gate loc-audit"
  else
    echo "FAIL SSO store wire gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_API" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-api (PH-S1275)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-api); then
    echo "OK  SSO API contracts gate loc-audit"
  else
    echo "FAIL SSO API contracts gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_ADMIN_OPS" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-admin-ops (PH-S1284)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-admin-ops); then
    echo "OK  SSO admin/ops gate loc-audit"
  else
    echo "FAIL SSO admin/ops gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_STAND_SMOKE" == "1" ]]; then
  echo "Running poolai-http-stand-smoke --sso-stand-smoke (PH-S1295)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-http-stand-smoke -- --sso-stand-smoke); then
    echo "OK  SSO stand-smoke live suite"
  else
    echo "FAIL SSO stand-smoke live suite"
    fail=1
  fi
  echo "Running poolai-loc-audit --sso-stand-smoke (PH-S1294)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-stand-smoke); then
    echo "OK  SSO stand-smoke gate loc-audit"
  else
    echo "FAIL SSO stand-smoke gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_STAND_SMOKE" == "1" ]]; then
  echo "Running poolai-http-stand-smoke --audit-stand-smoke (PH-S1395)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-http-stand-smoke -- --audit-stand-smoke); then
    echo "OK  audit stand-smoke live suite"
  else
    echo "FAIL audit stand-smoke live suite"
    fail=1
  fi
  echo "Running poolai-loc-audit --audit-stand-smoke (PH-S1394)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-stand-smoke); then
    echo "OK  audit stand-smoke gate loc-audit"
  else
    echo "FAIL audit stand-smoke gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_LOC_AUDIT" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-loc-audit (PH-S1402)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-loc-audit); then
    echo "OK  audit loc-audit aggregate gate"
  else
    echo "FAIL audit loc-audit aggregate gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_DOCS_CANON" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-docs-canon (PH-S1412)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-docs-canon); then
    echo "OK  audit docs-canon gate"
  else
    echo "FAIL audit docs-canon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_VISION_SYNC" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-vision-sync (PH-S1422)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-vision-sync); then
    echo "OK  audit vision-sync gate"
  else
    echo "FAIL audit vision-sync gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_RATIO_ADVISORY" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-ratio-advisory (PH-S1432)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-ratio-advisory); then
    echo "OK  audit ratio-advisory gate"
  else
    echo "FAIL audit ratio-advisory gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_HORIZON" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-horizon (PH-S1442)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-horizon); then
    echo "OK  audit horizon gate"
  else
    echo "FAIL audit horizon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY" == "1" ]]; then
  echo "Running poolai-loc-audit --policy (PH-S1452)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy); then
    echo "OK  policies depth gate loc-audit"
  else
    echo "FAIL policies depth gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_STORE" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-store (PH-S1462)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-store); then
    echo "OK  policies store-wire gate loc-audit"
  else
    echo "FAIL policies store-wire gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_API" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-api (PH-S1474)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-api); then
    echo "OK  policies HTTP API contracts gate loc-audit"
  else
    echo "FAIL policies HTTP API contracts gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_ADMIN_OPS" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-admin-ops (PH-S1484)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-admin-ops); then
    echo "OK  policies admin/ops glue gate loc-audit"
  else
    echo "FAIL policies admin/ops glue gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_STAND_SMOKE" == "1" ]]; then
  echo "Running poolai-http-stand-smoke --policy-stand-smoke (PH-S1495)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-http-stand-smoke -- --policy-stand-smoke); then
    echo "OK  policies live stand smoke"
  else
    echo "FAIL policies live stand smoke"
    fail=1
  fi
  echo "Running poolai-loc-audit --policy-stand-smoke (PH-S1495)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-stand-smoke); then
    echo "OK  policies stand smoke gate loc-audit"
  else
    echo "FAIL policies stand smoke gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_LOC_AUDIT" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-loc-audit (PH-S1502)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-loc-audit); then
    echo "OK  policies loc-audit aggregate gate"
  else
    echo "FAIL policies loc-audit aggregate gate"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_DOCS_CANON" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-docs-canon (PH-S1512)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-docs-canon); then
    echo "OK  policies docs-canon gate"
  else
    echo "FAIL policies docs-canon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_VISION_SYNC" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-vision-sync (PH-S1522)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-vision-sync); then
    echo "OK  policies vision-sync gate"
  else
    echo "FAIL policies vision-sync gate"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_RATIO_ADVISORY" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-ratio-advisory (PH-S1532)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-ratio-advisory); then
    echo "OK  policies ratio-advisory gate"
  else
    echo "FAIL policies ratio-advisory gate"
    fail=1
  fi
fi

if [[ "$VERIFY_POLICY_HORIZON" == "1" ]]; then
  echo "Running poolai-loc-audit --policy-horizon (PH-S1544)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --policy-horizon); then
    echo "OK  policies horizon gate"
  else
    echo "FAIL policies horizon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_MONITORING" == "1" ]]; then
  echo "Running poolai-loc-audit --monitoring (PH-S1552)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --monitoring); then
    echo "OK  monitoring depth gate loc-audit"
  else
    echo "FAIL monitoring depth gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_MONITORING_STORE" == "1" ]]; then
  echo "Running poolai-loc-audit --monitoring-store (PH-S1562)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --monitoring-store); then
    echo "OK  monitoring store-wire gate loc-audit"
  else
    echo "FAIL monitoring store-wire gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_MONITORING_API" == "1" ]]; then
  echo "Running poolai-loc-audit --monitoring-api (PH-S1574)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --monitoring-api); then
    echo "OK  monitoring API contracts gate loc-audit"
  else
    echo "FAIL monitoring API contracts gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_MONITORING_ADMIN_OPS" == "1" ]]; then
  echo "Running poolai-loc-audit --monitoring-admin-ops (PH-S1584)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --monitoring-admin-ops); then
    echo "OK  monitoring admin/ops gate loc-audit"
  else
    echo "FAIL monitoring admin/ops gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_MONITORING_STAND_SMOKE" == "1" ]]; then
  echo "Running poolai-http-stand-smoke --monitoring-stand-smoke (PH-S1595)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-http-stand-smoke -- --monitoring-stand-smoke); then
    echo "OK  monitoring stand-smoke live suite"
  else
    echo "FAIL monitoring stand-smoke live suite"
    fail=1
  fi
  echo "Running poolai-loc-audit --monitoring-stand-smoke (PH-S1595)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --monitoring-stand-smoke); then
    echo "OK  monitoring stand-smoke gate loc-audit"
  else
    echo "FAIL monitoring stand-smoke gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_MONITORING_LOC_AUDIT" == "1" ]]; then
  echo "Running poolai-loc-audit --monitoring-loc-audit (PH-S1602)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --monitoring-loc-audit); then
    echo "OK  monitoring loc-audit aggregate gate"
  else
    echo "FAIL monitoring loc-audit aggregate gate"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_LOC_AUDIT" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-loc-audit (PH-S1302)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-loc-audit); then
    echo "OK  SSO loc-audit aggregate gate"
  else
    echo "FAIL SSO loc-audit aggregate gate"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_DOCS_CANON" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-docs-canon (PH-S1312)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-docs-canon); then
    echo "OK  SSO docs-canon gate"
  else
    echo "FAIL SSO docs-canon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_VISION_SYNC" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-vision-sync (PH-S1322)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-vision-sync); then
    echo "OK  SSO vision-sync gate"
  else
    echo "FAIL SSO vision-sync gate"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_RATIO_ADVISORY" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-ratio-advisory (PH-S1332)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-ratio-advisory); then
    echo "OK  SSO ratio-advisory gate"
  else
    echo "FAIL SSO ratio-advisory gate"
    fail=1
  fi
fi

if [[ "$VERIFY_SSO_HORIZON" == "1" ]]; then
  echo "Running poolai-loc-audit --sso-horizon (PH-S1342)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --sso-horizon); then
    echo "OK  SSO horizon gate"
  else
    echo "FAIL SSO horizon gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT" == "1" ]]; then
  echo "Running poolai-loc-audit --audit (PH-S1352)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit); then
    echo "OK  audit depth gate loc-audit"
  else
    echo "FAIL audit depth gate loc-audit"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_STORE" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-store (PH-S1362)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-store); then
    echo "OK  audit store gate"
  else
    echo "FAIL audit store gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_API" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-api (PH-S1374)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-api); then
    echo "OK  audit API gate"
  else
    echo "FAIL audit API gate"
    fail=1
  fi
fi

if [[ "$VERIFY_AUDIT_ADMIN_OPS" == "1" ]]; then
  echo "Running poolai-loc-audit --audit-admin-ops (PH-S1384)..."
  if (cd "$ROOT" && cargo run --quiet --bin poolai-loc-audit -- --audit-admin-ops); then
    echo "OK  audit admin/ops gate"
  else
    echo "FAIL audit admin/ops gate"
    fail=1
  fi
fi

if [[ "$fail" -ne 0 ]]; then
  echo "Dev stand verification failed."
  exit 1
fi
echo "Dev stand verification passed (health + virtual-node bootstrap + ML pipeline demo when enabled + optional RAID job store + optional stand smoke)."
