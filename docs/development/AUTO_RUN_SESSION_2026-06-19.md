# Автономний прогін (PoolAI) — 2026-06-19 (S9)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-18.md`](./AUTO_RUN_SESSION_2026-06-18.md) (FM-019 pa11y S8 ✅ `0af5a021`).

**Ціль:** **FM-019** — pa11y strict для `/ui/admin/instances`, `/ui/admin/topology` (останні admin-only subpages у backlog).

## Критерії S9

- [x] `bin/pa11y-ci.sh` — instances + topology
- [x] `tests/pa11y_ci_script.rs`
- [x] `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — 0 errors (18 auth + login)
- [x] `cargo test-ci` (exit 0)
- [x] runbook + FM + HANDOFF + AUTO_DEV_PATTERNS
- [x] push — `1b261ea7` (2026-05-18)

**Поза обсягом:** FM-003 §4 (BLOCKED); FM-004/006/009/010; WCAG 2.2 AA auto; `a11y.yml` у main CI.
