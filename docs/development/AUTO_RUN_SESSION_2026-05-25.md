# Автономний прогін (PoolAI) — 2026-05-25

**Попередній:** [`AUTO_RUN_SESSION_2026-05-24.md`](./AUTO_RUN_SESSION_2026-05-24.md) (FM-016+++ ✅, FM-003 dev verify ✅).

**Ціль:** FM-016+++ local cache ✅; FM-003 §4 **BLOCKED** (немає real LAN).

**Критерії:**
- [x] Local probe cache (`POOLAI_WORKER_CACHE_DIR`, push `0456aff`)
- [x] `cargo test-ci`
- [x] FM-003 §4 задокументовано як BLOCKED; наступний — [`AUTO_RUN_SESSION_2026-05-26.md`](./AUTO_RUN_SESSION_2026-05-26.md)

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

**Стартовий промпт:**

> PoolAI AUTO_RUN 2026-05-25: FM-016+++ local cache on device або FM-003 §4 LAN (якщо 2 хости). Після коду — cargo fmt, cargo test-ci. Push MSYS2 + Summary.
