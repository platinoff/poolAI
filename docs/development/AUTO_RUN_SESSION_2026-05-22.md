# Автономний прогін (PoolAI) — 2026-05-22

**Попередній:** [`AUTO_RUN_SESSION_2026-05-21.md`](./AUTO_RUN_SESSION_2026-05-21.md) (FM-016 core ✅).

**Ціль:** **FM-016+** — Telegram bot ↔ virtual node / `poolai-worker` binding; production-oriented task store (замість in-memory bootstrap queue).

**Поза обсягом:** FM-003 (real LAN), FM-004, FM-006, FM-009, FM-010.

**Критерії:**
- [x] `POST/GET/DELETE /virtual-nodes/telegram/bind*` + `POST /virtual-nodes/telegram/webhook`
- [x] `VirtualNodeTelegramBindingService`; auto-bind з `metadata.telegram_id` при register-remote
- [x] `POOLAI_VIRTUAL_NODE_DATA_DIR`, `POOLAI_TELEGRAM_WEBHOOK_SECRET`, worker `POOLAI_TELEGRAM_ID`
- [x] `tests/virtual_node_telegram_binding_integration.rs` (2 tests)
- [x] `cargo test-ci` + push MSYS2 з Summary (`de8eb415`)

## FM-016++ (2026-05-18, та сама гілка)

- [x] `src/tgbot/coordinator.rs` — bridge до webhook
- [x] Feature `tgbot` + `poolai-telegram-bot` (teloxide)
- [x] `tests/tgbot_coordinator_bridge_integration.rs`
- [x] `cargo test-ci` зелений

**Стартовий промпт (оркестратор):**

> PoolAI AUTO_RUN 2026-05-22: FM-016+ Telegram binding. Прочитай HANDOFF §5.1, FUNCTION_MANAGEMENT FM-016. Після коду — cargo fmt, cargo test-ci. Push — MSYS2, Summary у коміті. Не стаджити `data/audit/*.log.gz`.
