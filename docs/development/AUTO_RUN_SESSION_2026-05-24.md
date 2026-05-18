# Автономний прогін (PoolAI) — 2026-05-24

**Попередній:** [`AUTO_RUN_SESSION_2026-05-23.md`](./AUTO_RUN_SESSION_2026-05-23.md) (FM-003 dev stand).

**Ціль:** **FM-003 §4** real LAN sign-off (якщо два хости) **або** FM-016+++ pool workload on device.

**Критерії (FM-016+++):**
- [x] `pool_workers_probe` bootstrap + worker execution
- [x] `telegram_command` / `telegram_message` у `poolai-worker` (`/status`, `/raid`)
- [x] `src/workers/virtual_node_executor.rs` + unit tests
- [x] `cargo test-ci` + push (`6b0d76d`)
- [x] `POST /virtual-nodes/{id}/pool/join` + worker auto-join (push `864bd63`)
- [x] `raid_artifact_probe` bootstrap + PutArtifact wire on coordinator (push `4419502`)
- [x] `verify-dev-stand.*` — discovery + pool join + >=4 bootstrap tasks
- [x] verify-dev-stand e2e (push `a899ad5`)
- [x] FM-016+++ закрито; наступний — [`AUTO_RUN_SESSION_2026-05-25.md`](./AUTO_RUN_SESSION_2026-05-25.md)

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010. Real LAN §4 — gated (два хости).
