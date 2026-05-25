# PoolAI on-chain program (FM-033 / PH-S46)

Minimal BPF program **`poolai-events`** logs domain event anchors (`JobCompleted`, `SeedProvided`, `MemoryUpdated`). PH-S46 adds on-chain wire validation (shared `wire/limits.rs`).

## Layout

| Path | Purpose |
|------|---------|
| `poolai-events/` | `solana-program` crate (`cdylib` + unit tests) |
| `../src/instruction.rs` | Instruction builder (Borsh, shared enum layout) |

## Deploy (devnet)

Requires [Solana CLI](https://docs.solanalabs.com/cli/install) and devnet keypair with SOL (airdrop).

From repo root (MSYS2 bash):

```bash
bash scripts/deploy-poolai-events-devnet.sh
```

Manual:

```bash
cd crates/poolai-solana-adapter/program/poolai-events
cargo build-sbf
solana program deploy target/deploy/poolai_events.so --url devnet
```

Set deployed program id in sidecar config:

```bash
export POOLAI_SOLANA_PROGRAM_ID="<deployed_pubkey>"
export POOLAI_SOLANA_MOCK_RPC=0
export POOLAI_SOLANA_KEYPAIR_PATH="$HOME/.config/solana/id.json"
```

Until a custom program is deployed, the sidecar falls back to the **Memo** program for a real devnet signature when `program_id` is the bundled placeholder (`1111…`).

## Tests (no deploy)

```bash
cargo test -p poolai-events
```
