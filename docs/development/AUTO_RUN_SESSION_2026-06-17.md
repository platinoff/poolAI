# Автономний прогін (PoolAI) — 2026-06-17 (S7)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-16.md`](./AUTO_RUN_SESSION_2026-06-16.md) (FM-019 pa11y S6 ✅ `73c702a9`).

**Ціль:** **FM-019** — закрити прогалину pa11y по dashboard observability + admin home; оновити §5.3 backlog.

## Аудит «не зроблено» (2026-05-18)

| Пріоритет | Пункт | Стан | Наступна дія |
|-----------|--------|------|--------------|
| P1 | FM-003 §4 LAN | **BLOCKED** | runbook only (2 хости) |
| P2 | FM-019 pa11y strict | **Partial** | S7: `/ui/status`, `/ui/health`, `/ui/metrics`, `/ui/admin` |
| P2b | FM-019 admin subpages (tenants, audit, …) | **Backlog** | після S7 за потреби |
| P2c | FM-019 WCAG 2.2 AA auto | **Backlog** | `PA11Y_STANDARD`, не блокує baseline |
| P3 | `a11y.yml` у main CI | **Optional** | лишається `workflow_dispatch` |
| — | FM-004/006/009/010 | **Deferred / Concept** | поза автопрогоном |
| Docs | `UI_BUGFIXES_AND_OAUTH_PLAN` | **Stale** | archival banner S7 |

**Критерії S7:**
- [x] `bin/pa11y-ci.sh` — `/ui/status`, `/ui/health`, `/ui/metrics`, `/ui/admin`
- [x] `PA11Y_ADMIN_STRICT=1 bash bin/pa11y-ci.sh --start` — 0 errors (login + 13 auth)
- [x] `tests/pa11y_ci_script.rs`
- [x] `cargo test-ci` (2026-05-18, exit 0)
- [x] `FUNCTION_MANAGEMENT` §5.3 + runbook §3.1; `UI_BUGFIXES` archival
- [ ] push (MSYS2 UCRT64 — [`git-push.md`](../../.cursor/commands/git-push.md))

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.
