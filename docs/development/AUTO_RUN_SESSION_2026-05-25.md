# Автономний прогін (PoolAI) — 2026-05-25

**Попередній:** [`AUTO_RUN_SESSION_2026-05-24.md`](./AUTO_RUN_SESSION_2026-05-24.md) (FM-016+++ ✅, FM-003 dev verify ✅).

**Ціль:** **FM-003 §4** real LAN (два хости) **або** FM-016+++ local artifact cache on device.

**Критерії:**
- [ ] Manual: `run-virtual-node-dev` + `verify-dev-stand` on stand
- [ ] Local probe cache dir on worker (`POOLAI_WORKER_CACHE_DIR`)
- [ ] `cargo test-ci` + push MSYS2 Summary

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

**Стартовий промпт:**

> PoolAI AUTO_RUN 2026-05-25: FM-016+++ local cache on device або FM-003 §4 LAN (якщо 2 хости). Після коду — cargo fmt, cargo test-ci. Push MSYS2 + Summary.
