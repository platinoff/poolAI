# Автономний прогін (PoolAI) — 2026-06-11

**Попередній:** [`AUTO_RUN_SESSION_2026-06-10.md`](./AUTO_RUN_SESSION_2026-06-10.md) (pa11y CI ✅ `8c5dc1df`).

**Ціль:** **FM-019** — auth fixture для strict admin pa11y (`PA11Y_ADMIN_STRICT`).

**Критерії:**
- [x] `bin/pa11y-ci.sh` — login actions + strict admin URLs
- [x] `a11y.yml` — `PA11Y_ADMIN_STRICT=1`, dev credentials env
- [x] `tests/pa11y_ci_script.rs`
- [ ] push (MSYS2)

**BLOCKED:** FM-003 §4 LAN.

**Поза обсягом:** FM-004, FM-006, FM-009, FM-010.

## S1 — виконання (2026-05-18)

**Патерн:** pa11y `--config` з `actions`: login `#username`/`#password` → `wait for path /ui` → navigate target.

**Далі:** знизити pa11y findings на admin (якщо CI red); WCAG 2.2 AA auto — backlog.
