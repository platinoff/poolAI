# Автономний прогін (PoolAI) — 2026-05-22

**Попередній:** [`AUTO_RUN_SESSION_2026-05-21.md`](./AUTO_RUN_SESSION_2026-05-21.md) (FM-016 core ✅).

**Ціль:** **FM-016+** — Telegram bot ↔ virtual node / `poolai-worker` binding; production-oriented task store (замість in-memory bootstrap queue).

**Поза обсягом:** FM-003 (real LAN), FM-004, FM-006, FM-009, FM-010.

**Критерії (чернетка):**
- [ ] Маршрут або сервіс прив’язки Telegram user/chat → `worker_id` / virtual node
- [ ] Документований env/конфіг для coordinator + worker (без секретів у репо)
- [ ] Інтеграційний тест binding або poll path (mock Telegram webhook)
- [ ] `cargo fmt` + `cargo test-ci` зелені; push MSYS2 з Summary

**Стартовий промпт (оркестратор):**

> PoolAI AUTO_RUN 2026-05-22: FM-016+ Telegram binding. Прочитай HANDOFF §5.1, FUNCTION_MANAGEMENT FM-016. Після коду — cargo fmt, cargo test-ci. Push — MSYS2, Summary у коміті. Не стаджити `data/audit/*.log.gz`.
