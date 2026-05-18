#!/usr/bin/env bash
# FM-003 / FM-016+++: health + virtual-node bootstrap checks for local dev stand.
set -euo pipefail

COORD_PORT="${COORD_PORT:-8080}"
WORKER_PORT="${WORKER_PORT:-9090}"
NODE_B_PORT="${NODE_B_PORT:-8081}"
WORKER_ID="${WORKER_ID:-vn-dev-stand}"
TIMEOUT="${CURL_TIMEOUT:-5}"
WARMUP_SECS="${VERIFY_WARMUP_SECS:-50}"
TASK_RETRIES="${VERIFY_TASK_RETRIES:-12}"
TASK_SLEEP="${VERIFY_TASK_SLEEP:-5}"
MIN_COMPLETED="${VERIFY_MIN_COMPLETED:-4}"

COORD_URL="http://127.0.0.1:${COORD_PORT}"

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

if [[ "$fail" -ne 0 ]]; then
  echo "Dev stand verification failed."
  exit 1
fi
echo "Dev stand verification passed (health + virtual-node bootstrap)."
