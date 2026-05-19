# Автономний прогін (PoolAI) — 2026-06-27 (S17 — OpenAPI config/ui/completions/ai-ml)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-26.md`](./AUTO_RUN_SESSION_2026-06-26.md) (S16 — OpenAPI admin/topology/instances).

**Ціль:** **OpenAPI** — `GET/PUT /config`, `/ui/*`, `/v1/chat/completions`, enterprise `/ai-ml/pipeline*`.

## Зміни

- `docs/openapi.yaml` — Config, UI (dashboards/themes/components), Completions, Enterprise ML pipeline paths + schemas; enterprise `servers` + path-level `servers` for `/ai-ml/*`.

## Критерії S17

- [x] OpenAPI ↔ `system.rs`, `ui.rs`, `completions.rs`, `ai_ml.rs`
- [x] HANDOFF + FM §5.3
- [x] `cargo test-ci` (MSYS2, разом із S18)
- [x] push — `a2749689` (wave S17–S20)

**Поза обсягом:** FM-003 §4; Playwright E2E (опційно); FM-004/006/009/010.

**Далі (S18+):** enterprise OAuth/monitoring OpenAPI; Playwright smoke за `UI_QUALITY_AND_E2E_PLAN`.
