# Автономний прогін (PoolAI) — 2026-06-24 (S14 — OpenAPI sync)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-23.md`](./AUTO_RUN_SESSION_2026-06-23.md) (§5.3 legacy docs audit S13).

**Ціль:** **OpenAPI** — sync FM-016+ Telegram virtual-node routes з `src/network/api/virtual_nodes.rs`.

## Обраний спринт

П.2 з «Рекомендований наступний спринт» (06-23): **OpenAPI diff** — не FM-003 (BLOCKED), не Playwright (опційно).

## Зміни

- `docs/openapi.yaml` — `GET /virtual-nodes/telegram/bindings`, `DELETE .../bindings/{telegram_user_id}`; схеми `TelegramBinding`, list/bind response.

## Критерії S14

- [x] OpenAPI ↔ `virtual_nodes.rs` (Telegram bindings)
- [x] HANDOFF + FM §5.1/§5.3
- [x] `cargo fmt` + `cargo test-ci`
- [x] push — `8138a70b`

**Поза обсягом:** FM-003 §4 sign-off; FM-004/006/009/010; `data/audit/*`.
