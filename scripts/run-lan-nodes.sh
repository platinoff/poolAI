#!/usr/bin/env bash
# DEPRECATED wrapper: canonical script is bin/run-lan-nodes.sh. Formerly scripts/run-lan-nodes.sh.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "$ROOT/bin/run-lan-nodes.sh" "$@"
