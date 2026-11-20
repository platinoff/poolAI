#!/usr/bin/env bash
# Invoke MSYS2 UCRT64 bash environment with PoolAI PATH.
# Bash/MSYS2 equivalent of the former poolai-msys.ps1 (GEN-3 format hygiene).
# Usage:
#   bash bin/poolai-msys.sh bin/run-poolai.sh status
#   bash bin/poolai-msys.sh bin/run-poolai.sh single --bg --skip-build
#   bash bin/poolai-msys.sh -lc "export K8S_OPENAPI_ENABLED_VERSION=1.28; cargo test-ci"
#   bash bin/poolai-msys.sh -Command "cargo fmt --all"
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ $# -eq 0 ]; then
  cat <<'USAGE'
Usage: bash bin/poolai-msys.sh <command> [args...]

Examples:
  bash bin/poolai-msys.sh bin/run-poolai.sh status
  bash bin/poolai-msys.sh bin/run-poolai.sh single --bg --skip-build
  bash bin/poolai-msys.sh -lc "export K8S_OPENAPI_ENABLED_VERSION=1.28; cargo test-ci"
  bash bin/poolai-msys.sh -Command "cargo fmt --all"

Runs commands in the PoolAI repo with the MSYS2 UCRT64/cargo PATH and
K8S_OPENAPI_ENABLED_VERSION (default 1.28). See docs/development/RUN_LOCAL.md.
USAGE
  exit 0
fi

# Honour POOLAI_MSYS_BASH only as a hint to locate cargo/home on odd setups;
# when invoked from MSYS2 bash we are already in the right shell.
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"

cd "$ROOT" 2>/dev/null || cd /s/rust/poolAI || exit 1

# -Command is accepted for parity with the former .ps1 wrapper.
if [ "${1:-}" = "-Command" ]; then
  shift
  set -- -lc "$@"
fi

if [ "${1:-}" = "-lc" ]; then
  shift
  if [ $# -lt 1 ]; then
    echo "poolai-msys.sh -lc requires a command string." >&2
    exit 1
  fi
  # Join remaining args as the -lc command string (parity with .ps1).
  exec bash -lc "$*"
else
  exec "$@"
fi
