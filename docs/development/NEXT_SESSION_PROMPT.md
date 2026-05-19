# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Копіюй блок нижче в новий чат** (оркестратор, `.cursor/rules/autonomous-orchestrator.mdc`).

---

## Промпт (S28 за замовчуванням)

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
3. Останні коміти: S27 Playwright admin E2E; прогрес продукту ~93% (шар A).

## Що вже закрито (не повторювати)

| Спринт | Результат |
|--------|-----------|
| S21–S21 | OpenAPI ai-ml optimization/automl/federated |
| S22 | FM-019 CI pa11y-wcag22 + pa11y-contract |
| S23 | Playwright smoke login → admin users |
| S24 | DELETE /ui/dashboards/{id} → 204 |
| S25–S26 | UI_QUALITY P1 ✅ — 27 admin JSON contract tests |
| S27 | Playwright admin tenants + monitoring E2E |

## Мета сесії — S28: OpenAPI gap audit (за замовчуванням)

**Фокус:** `rg '\.route\(' src/network` vs `docs/openapi.yaml` — закрити прогалини v1 + enterprise.

**Альтернатива:** розширити Playwright (security, audit) або ML ops за `PIPELINE_MANAGEMENT.md`.

**Критерій готовності S28:**
- [ ] Звіт прогалин + патчі yaml (або явний backlog у FM)
- [ ] cargo fmt → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28)
- [ ] Commit + push MSYS2 (.cursor/commands/git-push.md) з Summary у тілі
- [ ] HANDOFF, FM §5.1, CHANGELOG, AUTO_DEV_PATTERNS

**Поза обсягом:** FM-003 §4 (BLOCKED, 2 хости), FM-004/006/009/010.

**Не стаджити:** data/audit/*.log.gz
```

---

## Якщо користувач просить інший фокус

| Запит | Спринт | Док |
|-------|--------|-----|
| Playwright | — | `E2E_PLAYWRIGHT.md` (S27 ✅) |
| LAN | — | BLOCKED до 2 хостів |
| ML ops | — | `PIPELINE_MANAGEMENT.md` |
| SIMD / cloud-sdk deep | — | FM-004/006, Deferred |

Прогрес і «ніколи не зроблено»: [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md).
