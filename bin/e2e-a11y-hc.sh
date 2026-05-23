#!/usr/bin/env bash
# PH-S14: run high-contrast axe subset via e2e-playwright.sh --start
set -euo pipefail
export POOLAI_E2E_FILTER='a11y --grep high-contrast'
exec bash "$(dirname "$0")/e2e-playwright.sh" --start "$@"
