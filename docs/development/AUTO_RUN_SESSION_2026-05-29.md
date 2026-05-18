# Автономний прогін (PoolAI) — 2026-05-29

**Попередній:** [`AUTO_RUN_SESSION_2026-05-28.md`](./AUTO_RUN_SESSION_2026-05-28.md) (ops hygiene ✅).

**Ціль:** **FM-017** — FM-005 залишок (`discovery` / `virtual_nodes` / `admin`) **або** docs-only якщо worker-compat потребує окремого дизайну.

**Критерії FM-017 (мінімум):**
- [ ] Документовано контракт помилок для `poolai-worker` (status-only vs JSON)
- [ ] `discovery` або `admin` — хоча б один маршрут на `HttpAppError` + тест
- [ ] `cargo test-ci` + push

**BLOCKED (не старт):** FM-003 §4 LAN — 2 фізичні хости.

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010, FM-018 (окремий спринт).

**Стартовий промпт:**

> PoolAI AUTO_RUN 2026-05-29: FM-017 HttpAppError залишок або worker-safe design doc. FM-003 LAN BLOCKED. cargo test-ci + push MSYS2 Summary.
