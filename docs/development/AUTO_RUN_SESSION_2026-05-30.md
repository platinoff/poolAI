# Автономний прогін (PoolAI) — 2026-05-30

**Попередній:** [`AUTO_RUN_SESSION_2026-05-29.md`](./AUTO_RUN_SESSION_2026-05-29.md) (FM-017 discovery ✅).

**Ціль:** **FM-018** — admin/login a11y (skip links, focus, aria-live, aria-current).

**Критерії:**
- [x] Admin skip links + `#admin_main_content` / `#admin_nav` (format args — без `#` у format template)
- [x] Login skip + `role="main"` + alert `aria-live`
- [x] `cargo test-ci` + push

**Поза обсягом:** FM-003 §4 LAN BLOCKED, FM-004/006/009/010.
