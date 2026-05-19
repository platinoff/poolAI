# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Копіюй блок нижче в новий чат** (оркестратор, `.cursor/rules/autonomous-orchestrator.mdc`).

---

## Промпт (S27 за замовчуванням)

```
PoolAI — автономна ітеративна сесія (оркестратор).

## S0 — зріз (обов’язково спочатку)

1. `git fetch && git status -sb` (гілка main).
2. Прочитай (кроки 1–12, не весь docs/):
   - docs/development/HANDOFF_NEW_SESSION.md
   - docs/development/AUTO_RUN_SESSION_2026-07-01.md (§1–2, §4)
   - docs/status/DEVELOPMENT_PROGRESS_2026-05-19.md
   - docs/catalog/FUNCTION_MANAGEMENT.md §5.1, §5.3, §5.5
   - .cursor/rules/autonomous-orchestrator.mdc
3. Останні коміти: S25–S26 (2720c3d3…285b898d); прогрес продукту ~93% (шар A).

## Що вже закрито (не повторювати)

| Спринт | Результат |
|--------|-----------|
| S21–S21 | OpenAPI ai-ml optimization/automl/federated |
| S22 | FM-019 CI pa11y-wcag22 + pa11y-contract |
| S23 | Playwright smoke login → admin users |
| S24 | DELETE /ui/dashboards/{id} → 204 |
| S25–S26 | UI_QUALITY P1 ✅ — 27 admin JSON contract tests |

## Мета сесії — S27: Playwright E2E розширення (за замовчуванням)

**Фокус:** розширити `e2e/tests/smoke.spec.ts` або додати 1 spec — login + відкриття 1–2 admin сторінок (tenants або monitoring); узгодити з `E2E_PLAYWRIGHT.md`.

**Альтернатива (якщо E2E недоцільно):** S28 OpenAPI gap audit — `rg '\.route\(' src/network` vs `docs/openapi.yaml`.

**Критерій готовності S27:**
- [ ] 1–2 нові Playwright сценарії (або стабілізація smoke) + оновлення `E2E_PLAYWRIGHT.md`
- [ ] `bin/e2e-playwright.sh` / `e2e.yml` за потреби
- [ ] cargo fmt → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28)
- [ ] Commit + push MSYS2 (.cursor/commands/git-push.md) з Summary у тілі
- [ ] HANDOFF, FM §5.1, CHANGELOG, AUTO_DEV_PATTERNS, DEVELOPMENT_PROGRESS

**Поза обсягом:** FM-003 §4 (BLOCKED, 2 хости), FM-004/006/009/010.

**Не стаджити:** data/audit/*.log.gz
```

---

## Якщо користувач просить інший фокус

| Запит | Спринт | Док |
|-------|--------|-----|
| OpenAPI | S28 | `rg` routes vs yaml |
| LAN | — | BLOCKED до 2 хостів |
| ML ops | — | `PIPELINE_MANAGEMENT.md` |
| SIMD / cloud-sdk deep | — | FM-004/006, Deferred |

Прогрес і «ніколи не зроблено»: [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md).
