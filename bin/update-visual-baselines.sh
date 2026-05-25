#!/usr/bin/env bash
# PH-S31: Refresh Playwright visual baselines (requires running poolai on POOLAI_HTTP_PORT).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# Avoid broken TMPDIR (e.g. repo artifact `:-/` on Windows).
export TMPDIR="${TMPDIR:-/tmp}"
export TEMP="${TEMP:-/tmp}"
export PATH="${HOME}/.cargo/bin:/usr/bin:${PATH}"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"
taskkill //F //IM poolai.exe 2>/dev/null || true
taskkill //F //IM poolai-worker.exe 2>/dev/null || true
sleep 2
cargo build --release --bin poolai --features enterprise,ml,cloud,test-utils
bash bin/e2e-playwright.sh --start --update-snapshots
