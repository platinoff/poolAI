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
# CI: skip `ml` feature (PH-S47 rustc/memory); full Playwright incl. visual (PH-S44 gate)
if [[ "${CI:-}" == "true" ]]; then
  FEATURES="${POOLAI_FEATURES:-enterprise,cloud,test-utils}"
else
  FEATURES="${POOLAI_FEATURES:-enterprise,ml,cloud,test-utils}"
fi
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
  local profile="${POOLAI_E2E_PROFILE:-}"
  if [[ -z "${profile}" && "${CI:-}" == "true" ]]; then
    profile="debug"
  fi
  if [[ -z "${profile}" ]]; then
    profile="release"
  fi
  if [[ -x "$ROOT/target/${profile}/poolai" ]]; then
    echo "$ROOT/target/${profile}/poolai"
  elif [[ -x "$ROOT/target/${profile}/poolai.exe" ]]; then
    echo "$ROOT/target/${profile}/poolai.exe"
  else
    echo "poolai binary not found; run: cargo build --${profile} --features ${FEATURES}" >&2
    return 1
  fi
}

start_poolai() {
  export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
  export TMPDIR="${TMPDIR:-/tmp}"
  export TEMP="${TEMP:-/tmp}"
  STAND_ROOT="/tmp/poolai-e2e-$$"
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
  E2E_PROFILE="release"
  if [[ "${CI:-}" == "true" ]]; then
    E2E_PROFILE="debug"
    export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
    export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"
  fi
  export POOLAI_E2E_PROFILE="${E2E_PROFILE}"
  cargo build "--${E2E_PROFILE}" --features "${FEATURES}"
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
elif [[ "${CI:-}" == "true" ]]; then
  npm run test:ci
else
  npm test
fi
