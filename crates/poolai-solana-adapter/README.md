# poolai-solana-adapter

Sidecar crate for **FM-010** (Horizon S37), **FM-024** (devnet mock RPC stub), and **FM-033** (on-chain program + real devnet JSON-RPC).

## Scope

| Version | Capability |
|---------|------------|
| v0.1 (FM-010) | JSON schema **v1**: `JobCompleted`, `SeedProvided`, `MemoryUpdated` |
| v0.2 (FM-024) | Devnet config (`config/devnet.toml`), mock RPC submit on valid events |
| v0.3 (FM-033) | `poolai-events` BPF prototype, HTTP `sendTransaction` to devnet, Memo fallback |
| Always | **`solana-sdk` only in this crate** — main `poolai` has no Solana dependency |
| Out of scope | Mainnet, `solana-client` crate |

## Build & test

```bash
cargo test -p poolai-solana-adapter -p poolai-events -j 1
cargo build -p poolai-solana-adapter --release
```

On-chain program: see [`program/README.md`](program/README.md).

## Configuration

Default profile: [`config/devnet.toml`](config/devnet.toml) — **`mock_rpc = false`** (real devnet submit).

| Variable | Purpose |
|----------|---------|
| `POOLAI_SOLANA_CONFIG` | Path to TOML file (overrides bundled defaults as base) |
| `POOLAI_SOLANA_CLUSTER` | `devnet` or `localnet` (`mainnet-beta` rejected) |
| `POOLAI_SOLANA_RPC_URL` | RPC endpoint URL |
| `POOLAI_SOLANA_MOCK_RPC` | `1` / `true` — in-process mock submit (FM-024) |
| `POOLAI_SOLANA_PROGRAM_ID` | Deployed `poolai-events` program id (overrides TOML) |
| `POOLAI_SOLANA_KEYPAIR_PATH` | Solana CLI JSON keypair file (64-byte array) for signing |

### Devnet submit (FM-033)

```bash
export POOLAI_SOLANA_CLUSTER=devnet
export POOLAI_SOLANA_MOCK_RPC=0
export POOLAI_SOLANA_KEYPAIR_PATH="$HOME/.config/solana/id.json"
# Optional after deploy:
# export POOLAI_SOLANA_PROGRAM_ID="<program_pubkey>"
solana airdrop 1   # fund payer on devnet
echo '{"schema_version":1,"emitted_at":"2026-05-22T12:00:00Z","event_id":"e1","type":"job_completed","job_id":"j1","executor_peer_id":"peer-1"}' \
  | poolai-solana-adapter
```

Until `POOLAI_SOLANA_PROGRAM_ID` is set, placeholder `1111…` uses the **Memo** program for a real devnet signature.

### Mock RPC (FM-024 / local dev)

```bash
export POOLAI_SOLANA_MOCK_RPC=1
```

Stdout ack includes `rpc` block with `mocksig…` signature.

## Docs

- [`docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../../docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md)
