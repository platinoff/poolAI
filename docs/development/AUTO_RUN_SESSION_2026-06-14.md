# Автономний прогін (PoolAI) — 2026-06-14 (S4)

**Попередній:** [`AUTO_RUN_SESSION_2026-06-13.md`](./AUTO_RUN_SESSION_2026-06-13.md) (FM-019 ops + test-utils ✅ `d70c3d33`).

**Ціль:** **FM-019 docs** — зачистити stale [`UI_IMPROVEMENTS_PLAN.md`](../UI_IMPROVEMENTS_PLAN.md); задокументувати наступні pa11y URLs (`/ui`, `/ui/admin/config`).

**Критерії:**
- [x] `UI_IMPROVEMENTS_PLAN.md` — архівний банер, без оманливих `[ ]` / «READY FOR IMPLEMENTATION»
- [x] `ADMIN_A11Y_RUNBOOK.md` §3.1 — матриця strict / planned URLs
- [x] `FUNCTION_MANAGEMENT` §5.3 — `UI_IMPROVEMENTS_PLAN` → Archived
- [x] HANDOFF / CHANGELOG / `AUTO_DEV_PATTERNS` sync
- [ ] push

## S4 — виконання (2026-05-18)

**Обсяг:** docs-only. **Не робити:** FM-004, FM-006, FM-009, FM-010; зміни `bin/pa11y-ci.sh` (наступний кодовий slice).

**Артефакти:** archival `UI_IMPROVEMENTS_PLAN.md`; runbook pa11y URL matrix; FM §5.3.

**BLOCKED:** FM-003 §4 LAN.

## Стартовий промпт (оркестратор)

```
S0: git fetch && git status -sb; HANDOFF + FUNCTION_MANAGEMENT §5.1.
Пріоритет: FM-019 docs — UI_IMPROVEMENTS_PLAN cleanup + pa11y URL roadmap (/ui, /ui/admin/config).
Не робити: FM-004/006/009/010; код pa11y-ci без окремого запиту.
Після docs: push MSYS2.
```

**Наступний:** кодовий slice — додати `/ui` + `/ui/admin/config` у `ADMIN_URLS` після `PA11Y_ADMIN_STRICT=1` прогону 0 errors.
