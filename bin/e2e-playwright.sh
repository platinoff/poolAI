#!/usr/bin/env bash
# S23–S34 + PH-S11: Playwright E2E (smoke + admin + axe + visual).
# Usage:
#   bash bin/e2e-playwright.sh              # expects poolai on :8080
#   bash bin/e2e-playwright.sh --start      # build release, start poolai, run tests, stop
#   bash bin/e2e-playwright.sh --update-snapshots   # refresh visual baselines (PH-S11)
# Env: POOLAI_HTTP_PORT, POOLAI_E2E_USER, POOLAI_E2E_PASSWORD (defaults admin / admin123)
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

PORT="${POOLAI_HTTP_PORT:-8080}"
export POOLAI_BASE_URL="${POOLAI_BASE_URL:-http://127.0.0.1:${PORT}}"
FEATURES="${POOLAI_FEATURES:-enterprise,ml,cloud,test-utils}"
export POOLAI_E2E_USER="${POOLAI_E2E_USER:-admin}"
export POOLAI_E2E_PASSWORD="${POOLAI_E2E_PASSWORD:-admin123}"

POOLAI_PID=""
STAND_ROOT=""

cleanup() {
  if [[ -n "${POOLAI_PID}" ]] && kill -0 "${POOLAI_PID}" 2>/dev/null; then
    kill "${POOLAI_PID}" 2>/dev/null || true
    wait "${POOLAI_PID}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

wait_health() {
  local tries="${1:-60}"
  local i
  for ((i = 1; i <= tries; i++)); do
    if curl -sf --max-time 2 "${POOLAI_BASE_URL}/api/v1/health" >/dev/null; then
      echo "OK  health -> ${POOLAI_BASE_URL}/api/v1/health"
      return 0
    fi
    sleep 2
  done
  echo "FAIL health not ready on ${POOLAI_BASE_URL}"
  return 1
}

resolve_poolai_bin() {
  if [[ -x "$ROOT/target/release/poolai" ]]; then
    echo "$ROOT/target/release/poolai"
  elif [[ -x "$ROOT/target/release/poolai.exe" ]]; then
    echo "$ROOT/target/release/poolai.exe"
  else
    echo "poolai binary not found; run: cargo build --release --features ${FEATURES}" >&2
    return 1
  fi
}

start_poolai() {
  export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
  STAND_ROOT="${TMPDIR:-/tmp}/poolai-e2e-$$"
  mkdir -p "${STAND_ROOT}/raid" "${STAND_ROOT}/data"
  local bin
  bin="$(resolve_poolai_bin)"
  echo "Starting poolai (${bin}) on port ${PORT}..."
  POOLAI_HTTP_PORT="${PORT}" \
    POOLAI_RAID_BASE_PATH="${STAND_ROOT}/raid" \
    POOLAI_DATA_PATH="${STAND_ROOT}/data" \
    RUST_LOG="${RUST_LOG:-warn}" \
    "${bin}" >"${STAND_ROOT}/poolai.log" 2>&1 &
  POOLAI_PID=$!
  wait_health 90
}

PLAYWRIGHT_ARGS=()
DO_START=false
for arg in "$@"; do
  case "$arg" in
    --update-snapshots) PLAYWRIGHT_ARGS+=(--update-snapshots) ;;
    --start) DO_START=true ;;
  esac
done

if [[ "$DO_START" == true ]]; then
  export PATH="${HOME}/.cargo/bin:/usr/bin:${PATH}"
  cargo build --release --features "${FEATURES}"
  start_poolai
fi

if ! curl -sf --max-time 2 "${POOLAI_BASE_URL}/api/v1/health" >/dev/null; then
  echo "poolai not reachable at ${POOLAI_BASE_URL}; use: bash bin/e2e-playwright.sh --start" >&2
  exit 1
fi

cd "${ROOT}/e2e"
if [[ ! -d node_modules/@playwright/test ]]; then
  echo "Installing e2e dependencies..."
  npm install
  npx playwright install chromium
fi

if [[ ${#PLAYWRIGHT_ARGS[@]} -gt 0 ]]; then
  npx playwright test visual "${PLAYWRIGHT_ARGS[@]}"
elif [[ -n "${POOLAI_E2E_FILTER:-}" ]]; then
  # shellcheck disable=SC2086
  npx playwright test ${POOLAI_E2E_FILTER}
else
  npm test
fi
