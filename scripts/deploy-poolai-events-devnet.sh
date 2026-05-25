#!/usr/bin/env bash
# PH-S46 — build and deploy poolai-events to Solana devnet (requires Solana CLI + funded keypair).
set -euo pipefail

export PATH="${HOME}/.cargo/bin:/ucrt64/bin:/usr/bin:${PATH:-}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROGRAM_DIR="${ROOT}/crates/poolai-solana-adapter/program/poolai-events"
DEPLOY_SO="${PROGRAM_DIR}/target/deploy/poolai_events.so"

cd "${PROGRAM_DIR}"

if ! command -v solana >/dev/null 2>&1; then
  echo "error: solana CLI not found — install https://docs.solanalabs.com/cli/install" >&2
  exit 1
fi

if ! command -v cargo-build-sbf >/dev/null 2>&1; then
  echo "error: cargo-build-sbf not found (Solana platform tools)" >&2
  exit 1
fi

echo "==> build-sbf poolai-events"
cargo build-sbf

if [[ ! -f "${DEPLOY_SO}" ]]; then
  echo "error: missing ${DEPLOY_SO} after build-sbf" >&2
  exit 1
fi

RPC_URL="${POOLAI_SOLANA_RPC_URL:-https://api.devnet.solana.com}"
echo "==> deploy to devnet (${RPC_URL})"
PROGRAM_ID="$(solana program deploy "${DEPLOY_SO}" --url "${RPC_URL}" | awk '/Program Id:/{print $3}')"

if [[ -z "${PROGRAM_ID}" ]]; then
  echo "error: could not parse deployed program id from solana output" >&2
  exit 1
fi

echo ""
echo "Deployed poolai-events:"
echo "  POOLAI_SOLANA_PROGRAM_ID=${PROGRAM_ID}"
echo ""
echo "Example sidecar:"
echo "  export POOLAI_SOLANA_CLUSTER=devnet"
echo "  export POOLAI_SOLANA_MOCK_RPC=0"
echo "  export POOLAI_SOLANA_KEYPAIR_PATH=\"\${HOME}/.config/solana/id.json\""
echo "  export POOLAI_SOLANA_PROGRAM_ID=\"${PROGRAM_ID}\""
