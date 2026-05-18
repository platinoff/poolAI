#!/usr/bin/env bash
# FM-019: optional pa11y CI / local scan (requires Node.js + running poolai).
# Usage:
#   bash bin/pa11y-ci.sh              # expects poolai on :8080
#   bash bin/pa11y-ci.sh --start      # build release, start poolai, scan, stop
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

PORT="${POOLAI_HTTP_PORT:-8080}"
BASE="http://127.0.0.1:${PORT}"
FEATURES="${POOLAI_FEATURES:-enterprise,ml,cloud,test-utils}"
PA11Y="${PA11Y:-npx --yes pa11y@9}"
THRESHOLD="${PA11Y_THRESHOLD:-0}"
RUNNER="${PA11Y_RUNNER:-axe}"
STANDARD="${PA11Y_STANDARD:-WCAG2AA}"

STRICT_URLS=(
  "${BASE}/ui/login"
)
OPTIONAL_URLS=(
  "${BASE}/ui/admin/users"
  "${BASE}/ui/admin/security"
  "${BASE}/ui/workers"
)

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
    if curl -sf --max-time 2 "${BASE}/api/v1/health" >/dev/null; then
      echo "OK  health -> ${BASE}/api/v1/health"
      return 0
    fi
    sleep 2
  done
  echo "FAIL health not ready on ${BASE}"
  return 1
}

resolve_poolai_bin() {
  if [[ -x "$ROOT/target/release/poolai" ]]; then
    echo "$ROOT/target/release/poolai"
  elif [[ -x "$ROOT/target/release/poolai.exe" ]]; then
    echo "$ROOT/target/release/poolai.exe"
  elif [[ -x "$ROOT/target/debug/poolai" ]]; then
    echo "$ROOT/target/debug/poolai"
  elif [[ -x "$ROOT/target/debug/poolai.exe" ]]; then
    echo "$ROOT/target/debug/poolai.exe"
  else
    echo "poolai binary not found; run: cargo build --release --features ${FEATURES}" >&2
    return 1
  fi
}

start_poolai() {
  export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
  STAND_ROOT="${TMPDIR:-/tmp}/poolai-pa11y-$$"
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

run_pa11y() {
  local url="$1"
  local optional="${2:-0}"
  echo "--- pa11y ${url} (runner=${RUNNER}, threshold=${THRESHOLD}) ---"
  set +e
  ${PA11Y} "${url}" \
    --runner "${RUNNER}" \
    --standard "${STANDARD}" \
    --threshold "${THRESHOLD}" \
    --chromeLaunchConfig '{"args":["--no-sandbox","--disable-dev-shm-usage"]}'
  local code=$?
  set -e
  if [[ "${code}" -ne 0 ]]; then
    if [[ "${optional}" -eq 1 ]]; then
      echo "WARN pa11y optional fail (exit ${code}): ${url}"
      return 0
    fi
    echo "FAIL pa11y (exit ${code}): ${url}"
    return "${code}"
  fi
  echo "OK  pa11y ${url}"
}

if [[ "${1:-}" == "--start" ]]; then
  export PATH="${HOME}/.cargo/bin:/usr/bin:${PATH}"
  cargo build --release --features "${FEATURES}"
  start_poolai
  shift
fi

if ! curl -sf --max-time 2 "${BASE}/api/v1/health" >/dev/null; then
  echo "poolai not reachable at ${BASE}; use: bash bin/pa11y-ci.sh --start" >&2
  exit 1
fi

fail=0
for url in "${STRICT_URLS[@]}"; do
  run_pa11y "${url}" 0 || fail=1
done
for url in "${OPTIONAL_URLS[@]}"; do
  run_pa11y "${url}" 1 || true
done

exit "${fail}"
