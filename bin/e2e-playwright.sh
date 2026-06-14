#!/usr/bin/env bash
# S23–S34 + PH-S11: Playwright E2E (smoke + admin + axe + visual).
# PH-S158: stand lifecycle via poolai-e2e-stand (Rust).
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
export POOLAI_FEATURES="${FEATURES}"
export POOLAI_E2E_USER="${POOLAI_E2E_USER:-admin}"
export POOLAI_E2E_PASSWORD="${POOLAI_E2E_PASSWORD:-admin123}"
export POOLAI_GALAXY_PRICING_FALLBACK_JSON="${POOLAI_GALAXY_PRICING_FALLBACK_JSON:-{\"inference_blended_token\":470000}}"

cleanup() {
  if [[ -n "${POOLAI_E2E_STAND_ROOT:-}" ]]; then
    cargo run --bin poolai-e2e-stand -- stop --stand-root "${POOLAI_E2E_STAND_ROOT}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

start_poolai() {
  export PATH="${HOME}/.cargo/bin:/ucrt64/bin:/usr/bin:${PATH}"
  export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
  export TMPDIR="${TMPDIR:-/tmp}"
  export TEMP="${TEMP:-/tmp}"
  if command -v wasm-bindgen >/dev/null 2>&1; then
    echo "==> PH-S151: build poolai-ui-wasm for grid-pricing panel"
    bash "$ROOT/bin/build-ui-wasm.sh" || echo "WARN: poolai-ui-wasm build skipped"
  fi
  POOLAI_E2E_STAND_ROOT="$(cargo run --bin poolai-e2e-stand -- start --port "${PORT}" --print-stand-root)"
  export POOLAI_E2E_STAND_ROOT
  export POOLAI_JOB_STORE="${POOLAI_JOB_STORE:-raid}"
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
  export PATH="${HOME}/.cargo/bin:/ucrt64/bin:/usr/bin:${PATH}"
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
export PATH="/ucrt64/bin:${HOME}/.cargo/bin:/usr/bin:${PATH}"
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

if [[ -n "${POOLAI_E2E_STAND_ROOT:-}" ]]; then
  echo "==> PH-S156: RAID restart stand smoke (poolai-http-stand-smoke --raid-restart)"
  cargo run --bin poolai-http-stand-smoke -- --raid-restart
fi
