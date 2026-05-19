# Автономний прогін (PoolAI) — 2026-06-18 (S8)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-17.md`](./AUTO_RUN_SESSION_2026-06-17.md) (FM-019 pa11y S7 ✅ `c016d239`).

**Ціль:** **FM-019** — pa11y strict для admin subpages: tenants, audit, monitoring.

## Критерії S8

- [x] `bin/pa11y-ci.sh` — `/ui/admin/tenants`, `/ui/admin/audit`, `/ui/admin/monitoring`
- [x] `tests/pa11y_ci_script.rs`
- [x] `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — 0 errors (16 auth + login)
- [ ] `cargo test-ci`
- [x] runbook §3.1 + AUTO_DEV_PATTERNS
- [ ] push

**Поза обсягом:** FM-003 §4 (BLOCKED); FM-004/006/009/010; WCAG 2.2 AA auto; `/ui/admin/instances`, `/ui/admin/topology` — backlog.
