# poolai-solana-adapter

Sidecar crate for **FM-010** (Horizon S37). Translates PoolAI domain events into future on-chain instructions.

## Scope (v0.1)

- JSON schema **v1**: `JobCompleted`, `SeedProvided`, `MemoryUpdated`
- NDJSON sidecar binary: validate lines on stdin, emit ack on stdout
- **No** `solana-sdk` — the main `poolai` crate does not depend on this package

## Build & test

```bash
cargo test -p poolai-solana-adapter -j 1
cargo build -p poolai-solana-adapter --release
```

## Sidecar usage

```bash
echo '{"schema_version":1,"emitted_at":"2026-05-19T12:00:00Z","event_id":"e1","type":"job_completed","job_id":"j1","executor_peer_id":"peer-1"}' \
  | poolai-solana-adapter
```

## Docs

- [`docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md`](../../docs/development/SOLANA_ADAPTER_CONCEPT_2026-04-06.md)
