# Автономний прогін (PoolAI) — 2026-06-25 (S15 — OpenAPI discovery)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-24.md`](./AUTO_RUN_SESSION_2026-06-24.md) (FM-016+ Telegram OpenAPI S14).

**Ціль:** **OpenAPI** — sync FM-016 discovery routes з `src/network/api/discovery.rs`.

## Обраний спринт

Продовження п.2 OpenAPI: discovery peers/register/health (не FM-003 BLOCKED).

## Зміни

- `docs/openapi.yaml` — `GET /discovery/peers`, `GET .../peers/{peer_id}`, `POST /discovery/register`, `GET .../virtual-nodes/{peer_id}/health`; схеми `PeerInfo`, `RemoteHealthProbe`, `HeartbeatRemotePeerResponse`.

## Критерії S15

- [x] OpenAPI ↔ `discovery.rs`
- [x] HANDOFF + FM §5.1/§5.3
- [ ] `cargo fmt` + `cargo test-ci`
- [ ] push

**Поза обсягом:** FM-003 §4; FM-004/006/009/010; `data/audit/*`.
