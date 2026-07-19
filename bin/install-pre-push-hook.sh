#!/usr/bin/env bash
# Install tracked pre-push hook (delegates to bin/pre-push-hook.sh).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HOOK="${ROOT}/.git/hooks/pre-push"

cat >"$HOOK" <<'EOF'
#!/bin/bash
exec "$(git rev-parse --show-toplevel)/bin/pre-push-hook.sh"
EOF

chmod +x "$HOOK" "${ROOT}/bin/pre-push-hook.sh"
echo "Installed pre-push hook -> bin/pre-push-hook.sh"
