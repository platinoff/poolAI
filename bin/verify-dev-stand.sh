#!/usr/bin/env bash
# FM-003: curl health checks for local dev stand (run after run-lan-nodes or run-virtual-node-dev).
set -euo pipefail

COORD_PORT="${COORD_PORT:-8080}"
WORKER_PORT="${WORKER_PORT:-9090}"
NODE_B_PORT="${NODE_B_PORT:-8081}"
TIMEOUT="${CURL_TIMEOUT:-5}"

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

fail=0
check "coordinator" "http://127.0.0.1:${COORD_PORT}/api/v1/health" || fail=1
check "node-B (optional)" "http://127.0.0.1:${NODE_B_PORT}/api/v1/health" || true
check "virtual worker" "http://127.0.0.1:${WORKER_PORT}/health" || fail=1

if [[ "$fail" -ne 0 ]]; then
  echo "One or more required endpoints failed."
  exit 1
fi
echo "Dev stand health checks passed."
