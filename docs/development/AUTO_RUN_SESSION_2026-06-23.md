# Автономний прогін (PoolAI) — 2026-06-23 (S13 — аудит docs)

> **Оновлено FM (2026-05-19):** повний зріз після S21–S29 — [`DOCS_LEGACY_AUDIT_2026-05-19.md`](./DOCS_LEGACY_AUDIT_2026-05-19.md). Цей файл — історичний S13; таблиця нижче частково застаріла.

**Попередній:** [`AUTO_RUN_SESSION_2026-06-22.md`](./AUTO_RUN_SESSION_2026-06-22.md) (FM-019 pa11y-contract S12 ✅ `e9729152`).

**Ціль:** менеджер функціоналу — звірка **старих** планів у `docs/` з кодом; оновити §5.3; підготовка HANDOFF для наступної сесії.

## Аудит legacy docs (2026-05-18)

| Документ | Дата | Стан | Дія наступної сесії |
|----------|------|------|---------------------|
| [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) | 03-17 | **Канон P1–P6** | 2 відкриті чекбокси: LAN (BLOCKED), cloud-sdk (Deferred) |
| [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md) | 04-06 | **Канон UX** | без `[ ]`; звірка E2E за потреби |
| [`UI_IMPROVEMENTS_PLAN.md`](../UI_IMPROVEMENTS_PLAN.md) | — | **Archived** | не читати чеклисти |
| [`UI_BUGFIXES_AND_OAUTH_PLAN.md`](./UI_BUGFIXES_AND_OAUTH_PLAN.md) | 01-16 | **Archived** | FM-012/FM-019 канон |
| [`CONCEPT_PENDING_FEATURES.md`](./CONCEPT_PENDING_FEATURES.md) | 01-17 | **Archived** | ML/RAID у коді ✅ |
| [`STATUS_UPDATE_2026-01-16.md`](./STATUS_UPDATE_2026-01-16.md) | 01-16 | **Stale** | Azure/GCP SDK — FM-006 Deferred |
| [`RUST_ARCHITECT_STATUS_2026-01-19.md`](./RUST_ARCHITECT_STATUS_2026-01-19.md) | 01-19 | **Stale** | опційні v0.2 — не автопрогін |
| [`STABLE_STATE_UPDATE_2026-01-19.md`](../status/STABLE_STATE_UPDATE_2026-01-19.md) | 01-19 | **Stale** | канон → `STABLE_STATE_SUMMARY.md` |
| [`PERCENTAGE_PLAN.md`](../status/PERCENTAGE_PLAN.md) | — | **Stale** | % плани — не канон |
| [`ADMIN_PANEL_STATUS.md`](../status/ADMIN_PANEL_STATUS.md) | 01-19 | **Stale** | admin UI — код + runbook |
| [`BUTTON_FUNCTIONS_AUDIT_2026-01-19.md`](../status/BUTTON_FUNCTIONS_AUDIT_2026-01-19.md) | 01-19 | **Reference** | ручна регресія за потреби |
| [`UI_UX_IMPROVEMENTS_PLAN.md`](./UI_UX_IMPROVEMENTS_PLAN.md) | 01-21 | **Stale** | monitoring graphs — звірити код |
| [`RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md`](./RUST_ARCHITECT_NEXT_STEPS_2026-01-19.md) | 01-19 | **Superseded** | канон → Architect 03-17 |
| [`docs/openapi.yaml`](../openapi.yaml) | — | **Ongoing** | sync при API diff |

## Канон backlog (після S7–S12)

| Пріоритет | FM / план | Стан |
|-----------|-----------|------|
| P1 | FM-003 §4 LAN | **BLOCKED** (2 хости) |
| P2 | FM-019 pa11y | **Partial ✅** — 18 auth, WCAG22, `a11y.yml`, `pa11y-contract` |
| P3 | OpenAPI sync | **Ongoing** |
| — | FM-004 SIMD TurboQuant | **Deferred** |
| — | FM-006 cloud-sdk Azure/GCP | **Deferred** |
| — | FM-009/010 Grid/Solana | **Concept-only** |
| P? | UI E2E / Playwright | **Planned** — `UI_QUALITY_AND_E2E_PLAN` |
| P? | Virtual nodes / ML pipeline ops | **Implemented** — runbooks ✅ |

## Рекомендований наступний спринт (сесія N+1)

1. **FM-003** — лише runbook/checklist (BLOCKED) або **virtual node** hardening за DIGEST.
2. **OpenAPI** — diff `src/network/` vs `docs/openapi.yaml` (якщо були API зміни).
3. **UI E2E** — перший Playwright smoke з `UI_QUALITY_AND_E2E_PLAN` (опційно).
4. **Не стартувати:** FM-004, FM-006, FM-009, FM-010 без запиту.

## Промпт наступної сесії

```
Оркестратор AUTO_RUN: HANDOFF + FUNCTION_MANAGEMENT §5.1/§5.3 + AUTO_RUN_SESSION_2026-06-23.md.
FM-003 §4 BLOCKED. FM-019 pa11y закритий для автопрогону (S7–S12).
Обери 1 пункт з «Рекомендований наступний спринт»; cargo fmt + cargo test-ci; git push MSYS2.
```

## Критерії S13

- [x] §5.3 legacy audit + цей файл
- [x] HANDOFF + README Next Focus + FM §5.1
- [x] sync AUTO_RUN S7–S12 push markers
- [x] push

**Поза обсягом:** FM-004/006/009/010; staging `data/audit/*`.
