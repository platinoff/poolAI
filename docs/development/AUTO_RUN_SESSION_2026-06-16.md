# Автономний прогін (PoolAI) — 2026-06-16 (S6)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-15.md`](./AUTO_RUN_SESSION_2026-06-15.md) (FM-019 pa11y S5 ✅ `e368ba11`).

**Ціль:** **FM-019** — pa11y strict для решти dashboard URL (`/ui/libs`, `/ui/vm`, `/ui/raid`); вирівняти `--danger` у JS `applyTheme`.

**Критерії:**
- [x] `bin/pa11y-ci.sh` — `/ui/libs`, `/ui/vm`, `/ui/raid` (не `/ui/libraries`)
- [x] `src/ui/mod.rs` — dark `danger: '#c62828'` (як `themes.rs`)
- [x] `tests/pa11y_ci_script.rs` + `dashboard_dark_theme_danger_matches_rust_canon`
- [x] `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — **0 errors** (login + 9 auth)
- [x] `cargo test-ci`
- [x] runbook / FM / HANDOFF / CHANGELOG
- [x] push

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

**BLOCKED:** FM-003 §4 LAN.

**Наступний:** [`AUTO_RUN_SESSION_2026-06-17.md`](./AUTO_RUN_SESSION_2026-06-17.md) — audit + pa11y status/health/metrics/admin.
