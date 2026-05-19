# Автономний прогін (PoolAI) — 2026-06-26 (S16 — OpenAPI admin/topology/instances)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-25.md`](./AUTO_RUN_SESSION_2026-06-25.md) (FM-016 discovery OpenAPI S15).

**Ціль:** **OpenAPI** — `admin/overview`, topology, model instance routes.

## Зміни

- `docs/openapi.yaml` — admin, topology (4 paths), instance CRUD + previews + `/state`.

## Критерії S16

- [x] OpenAPI ↔ `admin.rs`, `topology.rs`, `instances.rs`
- [x] HANDOFF + FM §5.1/§5.3
- [x] `cargo fmt` + `cargo test-ci`
- [x] push — `fe712c7d`

**Поза обсягом:** FM-003 §4; `ai_ml/pipeline`; FM-004/006/009/010.
