#!/usr/bin/env bash
# FM-019: optional pa11y CI / local scan (requires Node.js + running poolai).
# Usage:
#   bash bin/pa11y-ci.sh              # expects poolai on :8080
#   bash bin/pa11y-ci.sh --start      # build release, start poolai, scan, stop
# Env:
#   PA11Y_ADMIN_STRICT=1  — admin URLs with login actions (default in CI)
#   PA11Y_USER / PA11Y_PASSWORD — dev defaults admin / admin123 (see user_manager.rs)
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
PA11Y_USER="${PA11Y_USER:-admin}"
PA11Y_PASSWORD="${PA11Y_PASSWORD:-admin123}"
PA11Y_ADMIN_STRICT="${PA11Y_ADMIN_STRICT:-0}"

STRICT_URLS=(
  "${BASE}/ui/login"
)
OPTIONAL_URLS=()
ADMIN_URLS=(
  "${BASE}/ui"
  "${BASE}/ui/status"
  "${BASE}/ui/health"
  "${BASE}/ui/metrics"
  "${BASE}/ui/admin"
  "${BASE}/ui/admin/users"
  "${BASE}/ui/admin/security"
  "${BASE}/ui/admin/config"
  "${BASE}/ui/workers"
  "${BASE}/ui/libs"
  "${BASE}/ui/vm"
  "${BASE}/ui/raid"
)

if [[ "${PA11Y_ADMIN_STRICT}" == "1" ]]; then
  OPTIONAL_URLS=()
else
  OPTIONAL_URLS=("${ADMIN_URLS[@]}")
fi

POOLAI_PID=""
STAND_ROOT=""
PA11Y_CFG_DIR=""

cleanup() {
  if [[ -n "${POOLAI_PID}" ]] && kill -0 "${POOLAI_PID}" 2>/dev/null; then
    kill "${POOLAI_PID}" 2>/dev/null || true
    wait "${POOLAI_PID}" 2>/dev/null || true
  fi
  if [[ -n "${PA11Y_CFG_DIR}" && -d "${PA11Y_CFG_DIR}" ]]; then
    rm -rf "${PA11Y_CFG_DIR}"
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
  PA11Y_CFG_DIR="${STAND_ROOT}/pa11y-cfg"
  mkdir -p "${STAND_ROOT}/raid" "${STAND_ROOT}/data" "${PA11Y_CFG_DIR}"
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

write_pa11y_config() {
  local target="$1"
  local cfg="$2"
  cat >"${cfg}" <<JSON
{
  "standard": "${STANDARD}",
  "runner": "${RUNNER}",
  "threshold": ${THRESHOLD},
  "chromeLaunchConfig": {
    "args": ["--no-sandbox", "--disable-dev-shm-usage"]
  },
  "actions": [
    "navigate to ${BASE}/ui/login",
    "wait for element #username to be visible",
    "set field #username to ${PA11Y_USER}",
    "set field #password to ${PA11Y_PASSWORD}",
    "click element #loginBtn",
    "wait for path to be /ui",
    "navigate to ${target}",
    "wait for element body to be visible"
  ]
}
JSON
}

write_pa11y_simple_config() {
  local target="$1"
  local cfg="$2"
  cat >"${cfg}" <<JSON
{
  "standard": "${STANDARD}",
  "runner": "${RUNNER}",
  "threshold": ${THRESHOLD},
  "chromeLaunchConfig": {
    "args": ["--no-sandbox", "--disable-dev-shm-usage"]
  },
  "actions": [
    "navigate to ${target}",
    "wait for element body to be visible"
  ]
}
JSON
}

run_pa11y() {
  local url="$1"
  local optional="${2:-0}"
  PA11Y_CFG_DIR="${PA11Y_CFG_DIR:-${TMPDIR:-/tmp}/poolai-pa11y-cfg-$$}"
  mkdir -p "${PA11Y_CFG_DIR}"
  local cfg="${PA11Y_CFG_DIR}/$(echo "${url}" | tr '/:' '__').json"
  write_pa11y_simple_config "${url}" "${cfg}"
  echo "--- pa11y ${url} (runner=${RUNNER}, threshold=${THRESHOLD}) ---"
  set +e
  ${PA11Y} "${url}" --config "${cfg}"
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

run_pa11y_authenticated() {
  local url="$1"
  local optional="${2:-0}"
  PA11Y_CFG_DIR="${PA11Y_CFG_DIR:-${TMPDIR:-/tmp}/poolai-pa11y-cfg-$$}"
  mkdir -p "${PA11Y_CFG_DIR}"
  local cfg="${PA11Y_CFG_DIR}/$(echo "${url}" | tr '/:' '__').json"
  write_pa11y_config "${url}" "${cfg}"
  echo "--- pa11y (auth) ${url} user=${PA11Y_USER} ---"
  set +e
  ${PA11Y} "${url}" --config "${cfg}"
  local code=$?
  set -e
  if [[ "${code}" -ne 0 ]]; then
    if [[ "${optional}" -eq 1 ]]; then
      echo "WARN pa11y optional fail (exit ${code}): ${url}"
      return 0
    fi
    echo "FAIL pa11y auth (exit ${code}): ${url}"
    return "${code}"
  fi
  echo "OK  pa11y auth ${url}"
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

if [[ "${PA11Y_ADMIN_STRICT}" == "1" ]]; then
  for url in "${ADMIN_URLS[@]}"; do
    run_pa11y_authenticated "${url}" 0 || fail=1
  done
fi

for url in "${OPTIONAL_URLS[@]}"; do
  run_pa11y "${url}" 1 || true
done

exit "${fail}"
