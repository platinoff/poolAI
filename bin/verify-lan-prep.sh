#!/usr/bin/env bash
# FM-027: LAN prep — health (+ optional discovery) before FM-003 §4 sign-off.
# Single host: checks 127.0.0.1:8080 and :8081 (after run-lan-nodes).
# Two hosts: set POOLAI_NODE_A_URL and POOLAI_NODE_B_URL (e.g. http://192.168.1.10:8080).
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:${PATH:-}"
TIMEOUT="${CURL_TIMEOUT:-5}"

NODE_A_URL="${POOLAI_NODE_A_URL:-http://127.0.0.1:8080}"
NODE_B_URL="${POOLAI_NODE_B_URL:-http://127.0.0.1:8081}"
TWO_HOST=0
if [[ -n "${POOLAI_NODE_A_URL:-}" && -n "${POOLAI_NODE_B_URL:-}" ]]; then
  TWO_HOST=1
fi

strip_trailing_slash() {
  local u="$1"
  u="${u%/}"
  echo "$u"
}

NODE_A_URL="$(strip_trailing_slash "$NODE_A_URL")"
NODE_B_URL="$(strip_trailing_slash "$NODE_B_URL")"

check_health() {
  local name="$1"
  local base="$2"
  local required="${3:-1}"
  if curl -sf --max-time "$TIMEOUT" "${base}/api/v1/health" >/dev/null; then
    echo "OK  $name health -> ${base}/api/v1/health"
    return 0
  fi
  if [[ "$required" == "1" ]]; then
    echo "FAIL $name health -> ${base}/api/v1/health"
    return 1
  fi
  echo "SKIP $name health -> ${base}/api/v1/health"
  return 0
}

check_peers() {
  local name="$1"
  local base="$2"
  local body
  if ! body="$(curl -sf --max-time "$TIMEOUT" "${base}/api/v1/discovery/peers" 2>/dev/null)"; then
    echo "SKIP $name discovery/peers (unavailable or 503)"
    return 0
  fi
  local count
  count="$(echo "$body" | grep -o '"peer_id"' | wc -l | tr -d ' ')"
  echo "OK  $name discovery/peers -> ${count} peer(s)"
}

fail=0
echo "LAN prep (FM-027): A=$NODE_A_URL B=$NODE_B_URL two_host=$TWO_HOST"
echo ""

check_health "node-A" "$NODE_A_URL" 1 || fail=1
if [[ "$TWO_HOST" == "1" ]]; then
  check_health "node-B" "$NODE_B_URL" 1 || fail=1
else
  check_health "node-B (dual-port dev)" "$NODE_B_URL" 0 || true
fi

check_peers "node-A" "$NODE_A_URL"
if [[ "$TWO_HOST" == "1" ]]; then
  check_peers "node-B" "$NODE_B_URL"
fi

if [[ "$fail" -ne 0 ]]; then
  echo ""
  echo "LAN prep failed. Start nodes: bin/run-lan-nodes.sh or deploy on two hosts."
  echo "Two-host: POOLAI_NODE_A_URL=... POOLAI_NODE_B_URL=... bash bin/verify-lan-prep.sh"
  exit 1
fi

echo ""
echo "LAN prep passed. Next: docs/performance/LAN_SIGNOFF_CHECKLIST.md (§4 sign-off when 2 physical hosts)."
