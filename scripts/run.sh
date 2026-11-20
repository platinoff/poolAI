#!/usr/bin/env bash
# DEPRECATED wrapper: use bin/run-poolai.sh (canonical). Formerly scripts/run.sh.
# Run PoolAI with optional enterprise features / debug profile.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "scripts/run.sh is deprecated; forwarding to bin/run-poolai.sh" >&2
exec "$ROOT/bin/run-poolai.sh" single "$@"
