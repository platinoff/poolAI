# Автономний прогін (PoolAI) — 2026-05-21

**Попередній:** [`AUTO_RUN_SESSION_2026-05-20.md`](./AUTO_RUN_SESSION_2026-05-20.md) (FM-015).

**Ціль:** **FM-016** фаза 1 — Telegram / virtual node workers: HTTP реєстрація в discovery, `poolai-worker` → coordinator.

**Поза обсягом:** FM-003 (real LAN), FM-004, FM-006, FM-009, FM-010.

**Критерії фази 1:**
- [x] `POST /api/v1/discovery/register-remote`
- [x] `poolai-worker` — `POOLAI_COORDINATOR_URL`, періодична реєстрація
- [x] `tests/discovery_remote_register_integration.rs`
- [x] Фаза 2: `heartbeat-remote`, `GET /discovery/virtual-nodes`, probe `/virtual-nodes/{id}/health`; `poolai-worker` `GET /health` + pool API link check
- [x] Фаза 3: `virtual-nodes/{id}/tasks` poll/complete, bootstrap ping + `raid_health_check`; worker виконує tasks + RAID wire
