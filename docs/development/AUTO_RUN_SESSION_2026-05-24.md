# Автономний прогін (PoolAI) — 2026-05-24

**Попередній:** [`AUTO_RUN_SESSION_2026-05-23.md`](./AUTO_RUN_SESSION_2026-05-23.md) (FM-003 dev stand).

**Ціль:** **FM-003 §4** real LAN sign-off (якщо два хости) **або** FM-016+++ pool workload on device.

**Критерії (FM-016+++):**
- [x] `pool_workers_probe` bootstrap + worker execution
- [x] `telegram_command` / `telegram_message` у `poolai-worker` (`/status`, `/raid`)
- [x] `src/workers/virtual_node_executor.rs` + unit tests
- [x] `cargo test-ci`
- [ ] push MSYS2 Summary

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010. Real LAN §4 — gated (два хости).
