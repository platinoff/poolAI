# poolai-solana-adapter

Sidecar crate for **FM-010** (Horizon S37) and **FM-024** (devnet mock RPC stub). Translates PoolAI domain events into future on-chain instructions.

## Scope

| Version | Capability |
|---------|------------|
| v0.1 (FM-010) | JSON schema **v1**: `JobCompleted`, `SeedProvided`, `MemoryUpdated` |
| v0.2 (FM-024) | Devnet config (`config/devnet.toml`), mock RPC submit on valid events |
| Always | **No** `solana-sdk` — the main `poolai` crate does not depend on this package |
| Out of scope | Mainnet, real RPC/network I/O, on-chain program |

## Build & test

```bash
cargo test -p poolai-solana-adapter -j 1
cargo build -p poolai-solana-adapter --release
```

## Configuration (FM-024)

Default profile: [`config/devnet.toml`](config/devnet.toml) (bundled at compile time).

| Variable | Purpose |
|----------|---------|
| `POOLAI_SOLANA_CONFIG` | Path to TOML file (overrides bundled defaults as base) |
| `POOLAI_SOLANA_CLUSTER` | `devnet` or `localnet` (`mainnet-beta` rejected) |
| `POOLAI_SOLANA_RPC_URL` | RPC endpoint URL |
| `POOLAI_SOLANA_MOCK_RPC` | `1` / `true` to enable in-process mock submit |

Example:

```bash
export POOLAI_SOLANA_CLUSTER=devnet
export POOLAI_SOLANA_MOCK_RPC=1
```

## Sidecar usage

Schema validation only (no RPC block):

```bash
echo '{"schema_version":1,"emitted_at":"2026-05-19T12:00:00Z","event_id":"e1","type":"job_completed","job_id":"j1","executor_peer_id":"peer-1"}' \
  | poolai-solana-adapter
```

With mock RPC (default devnet config), stdout ack includes `rpc`:

```json
{"status":"acked","event_id":"e1","rpc":{"status":"submitted","signature":"mocksig…","slot":1,"cluster":"devnet","rpc_url":"https://api.devnet.solana.com"}}
```

## Docs

- [`docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../../docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md)
