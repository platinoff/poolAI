# Автономний прогін (PoolAI) — 2026-05-27

**Попередній:** [`AUTO_RUN_SESSION_2026-05-26.md`](./AUTO_RUN_SESSION_2026-05-26.md) (FM-012 webhook ✅, OpenAPI sync).

**Ціль:** **FM-012** OAuth allowlist hardening — **закрито** (2026-05-27).

**Критерії:**
- [x] Constant-time hash compare; `POOLAI_TELEGRAM_AUTH_MAX_AGE_SECS`
- [x] Allowlist trim; `sign_telegram_login_widget_query` + HTTP integration tests
- [x] `cargo test-ci` + push

**Поза обсягом:** FM-003 §4 LAN (BLOCKED), FM-004, FM-006, FM-009, FM-010.

**Стартовий промпт:**

> PoolAI AUTO_RUN 2026-05-27: FM-012 OAuth або наступний FM з §5.1. FM-003 LAN BLOCKED. cargo test-ci + push MSYS2 Summary.
