#!/usr/bin/env bash
# FM-028: capture single-host dual-port metrics (run-lan-nodes + health_load + TQ01 snapshot).
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/c/msys64/ucrt64/bin:/c/msys64/usr/bin:${PATH:-}"
export K8S_OPENAPI_ENABLED_VERSION="${K8S_OPENAPI_ENABLED_VERSION:-1.28}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FEATURES="${FEATURES:-enterprise,ml,cloud,test-utils}"
SKIP_BUILD="${SKIP_BUILD:-0}"
SKIP_LAN_START="${SKIP_LAN_START:-0}"
HEALTH_SECS="${HEALTH_SECS:-10}"
HEALTH_WORKERS="${HEALTH_WORKERS:-50}"
HOST_LABEL="${HOST_LABEL:-win10-local-26200-dual-stand}"
OUT_DIR="$ROOT/data/lan-stand"
STAMP="$(date +%Y%m%d)"
OUT_JSON="$OUT_DIR/metrics-fm028-${STAMP}.json"

mkdir -p "$OUT_DIR"

stop_poolai() {
  pkill -f '/target/debug/poolai' 2>/dev/null || true
  pkill -f 'poolai.exe' 2>/dev/null || true
  taskkill //IM poolai.exe //F 2>/dev/null || true
}

if [[ "$SKIP_LAN_START" != "1" ]]; then
  stop_poolai
  SKIP_BUILD="$SKIP_BUILD" bash bin/run-lan-nodes.sh
  echo "Waiting 18s for nodes..."
  sleep 18
fi

bash bin/verify-lan-prep.sh

if [[ "$SKIP_BUILD" != "1" ]]; then
  cargo build --release --bin poolai_health_load --features "$FEATURES"
  cargo build --bin poolai-p2b-tq01-snapshot --features ml
fi

health_a="$(mktemp)"
health_b="$(mktemp)"
cargo run --release --bin poolai_health_load --features "$FEATURES" -- \
  --json "http://127.0.0.1:8080/api/v1/health" "$HEALTH_SECS" "$HEALTH_WORKERS" >"$health_a"
cargo run --release --bin poolai_health_load --features "$FEATURES" -- \
  --json "http://127.0.0.1:8081/api/v1/health" "$HEALTH_SECS" "$HEALTH_WORKERS" >"$health_b"

tq01="$(mktemp)"
cargo run --bin poolai-p2b-tq01-snapshot --features ml >"$tq01"

if command -v jq >/dev/null 2>&1; then
  jq -n \
    --arg host "$HOST_LABEL" \
    --arg date "$STAMP" \
    --slurpfile ha "$health_a" \
    --slurpfile hb "$health_b" \
    --slurpfile tq "$tq01" \
    '{
      host_label: $host,
      date: $date,
      stand: "single-host dual-port (run-lan-nodes)",
      health_load: { node_a_8080: $ha[0], node_b_8081: $hb[0] },
      tq01_snapshot: $tq[0]
    }' >"$OUT_JSON"
else
  {
    printf '{ "host_label": "%s", "date": "%s", "stand": "single-host dual-port (run-lan-nodes)",\n' "$HOST_LABEL" "$STAMP"
    printf '  "health_load": { "node_a_8080": '
    cat "$health_a"
    printf ', "node_b_8081": '
    cat "$health_b"
    printf ' },\n  "tq01_snapshot": '
    cat "$tq01"
    printf '\n}\n'
  } >"$OUT_JSON"
fi
echo "Wrote $OUT_JSON"

echo ""
echo "=== FM-028 summary (paste into BENCHMARKS.md) ==="
echo "Host label: $HOST_LABEL"
echo "Node A health_load:"
cat "$health_a"
echo "Node B health_load:"
cat "$health_b"
echo "TQ01 snapshot:"
cat "$tq01"

if [[ "${STOP_POOLAI:-1}" == "1" && "$SKIP_LAN_START" != "1" ]]; then
  stop_poolai
fi
