# UI Improvements Plan (архів)

**Статус:** **архів / не канон** (2026-05-18). Не використовувати `[ ]` чеклисти нижче для пріоритизації автопрогону.

| Канон | Де |
|-------|-----|
| FM-019 baseline (зроблено в коді) | [`catalog/FUNCTION_MANAGEMENT.md`](catalog/FUNCTION_MANAGEMENT.md) **§5.4** |
| Регресійна верифікація | [`development/ADMIN_A11Y_RUNBOOK.md`](development/ADMIN_A11Y_RUNBOOK.md) |
| pa11y CI (strict URLs) | `bin/pa11y-ci.sh`, [`.github/workflows/a11y.yml`](../.github/workflows/a11y.yml) |
| UX / E2E план | [`development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](development/UI_QUALITY_AND_E2E_PLAN_2026-04-06.md) |

## Що вже покрито baseline (FM-018 / FM-019)

| Можливість | Де |
|------------|-----|
| Skip links | `src/ui/mod.rs`, `admin/mod.rs`, login |
| Ctrl+K search, Esc modals/mobile drawer | `src/ui/mod.rs` |
| `aria-live` (notifications, errors) | dashboard, admin, login |
| `aria-current` nav | `adminMarkCurrentNav`, `dashMarkCurrentNav` |
| Admin modals / forms / tabs / tables | `admin_common.js`, `src/ui/admin/*.rs` |
| Dashboard modals (workers, libs, VM, RAID) | `src/ui/mod.rs`, `ui::dashboard_a11y_tests` |
| i18n UA/EN | `i18n_core.js` |
| pa11y strict (login + `/ui` + admin users/security/config + workers) | `bin/pa11y-ci.sh`, `PA11Y_ADMIN_STRICT=1` — **0 errors** (S5 2026-05-18) |

## Backlog (не baseline)

- Повний **WCAG 2.2 AA** автомат у CI — backlog FM-019.
- ~~Розширення pa11y `/ui`, `/ui/admin/config`~~ — **Partial ✅** runbook §3.1.
- Не в автопрогоні: FM-004, FM-006, FM-009, FM-010.

## Історичний контекст (2025-12-30)

Оригінальний план Architect описував «remaining 5%» UI (accessibility, компоненти, responsive). Більшість пунктів **реалізовано** у 2026-01–2026-06; детальні тижневі чеклисти з `[ ]` видалено, щоб не конфліктувати з §5.4.

**Джерела зрізу:** [`status/STABLE_STATE_SUMMARY.md`](status/STABLE_STATE_SUMMARY.md), [`development/NEXT_STEPS_PLAN.md`](development/NEXT_STEPS_PLAN.md).

**Last updated:** 2026-05-18 — S4 docs cleanup; канон → runbook + FUNCTION_MANAGEMENT §5.4.
