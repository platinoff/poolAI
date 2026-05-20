# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** **Post-Horizon розробка** (FM-020…) · **A+B+C:** **100%**

**Копіюй блок нижче** в нову сесію (одна FM за ітерацію).

---

## Промпт

```
PoolAI — Post-Horizon: FM-020…031 (оркестратор + менеджер функціоналу).

## S0 — зріз (обов’язково)

1. git fetch && git status -sb (main).
2. HANDOFF_NEW_SESSION.md
3. FUNCTION_MANAGEMENT.md §5.1, §5.7, §5.3
4. AUTO_RUN_SESSION_2026_POST_HORIZON.md — перший [ ] у §4
5. .cursor/rules/autonomous-orchestrator.mdc
6. .cursor/rules/runtime-stack-policy.mdc — Rust primary; NO Python runtime
7. JOB_LAYER_CONCEPT_2026-03-17.md (FM-026 jobs QA)

Не повторювати: autoprogon S21–S34, Horizon S35–S40, FM-001…025 baseline.

Не використовувати архівні docs з Python sidecar як план імплементації.

## Мета ітерації (одна FM за сесію)

| Пор. | FM | Фокус | Критерій |
|------|-----|--------|----------|
| — | FM-020…025 | … | ✅ |
| 1 | FM-026 | Jobs QA | contract або Playwright `/jobs` |
| 8 | FM-027 | LAN runbook | 2-host checklist (**BLOCKED** без хостів) |
| 9 | FM-028 | P2b metrics | single-host TQ01+RAID → BENCHMARKS |
| 10 | FM-029 | Job SQLite | optional backend |
| 11 | FM-030 | Monitoring persist | MONITORING_PERSISTENCE_PLAN MVP |
| 12 | FM-031 | WCAG expand | pa11y/axe URLs |

Почни з FM-026 (перший [ ] у AUTO_RUN §4).

Перед кодом FM-026:
- tests/admin_ui_api_contracts.rs, e2e/tests/ для `/api/v1/jobs`

## Завершення ітерації

1. Зміни src/ → cargo fmt --all → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28).
2. Нові/змінені API → docs/openapi.yaml.
3. Оновити: AUTO_RUN §FM, FM §5.1/§5.7, HANDOFF, CHANGELOG, AUTO_DEV_PATTERNS.
4. commit + push MSYS2 з Summary; git -c commit.template= commit -F msgfile.

Не стаджити: data/audit/*.log.gz, data/dev/, data/lan-stand/, .commit-msg-*.txt.
Поза обсягом: FM-003 §4 sign-off без 2 хостів; mainnet Solana; KYC; Python runtime.
```

---

## Довідка

| Документ | Роль |
|----------|------|
| [`AUTO_RUN_SESSION_2026_POST_HORIZON.md`](./AUTO_RUN_SESSION_2026_POST_HORIZON.md) | Черга FM-020…031 |
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) | FM-* §5.1, §5.7 |
| [`runtime-stack-policy.mdc`](../../.cursor/rules/runtime-stack-policy.mdc) | Rust-only; block Python |
| [`STRUCTURE.md`](../STRUCTURE.md) | §7 — мови стеку |
| [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) | Операційний зріз |

**Наступна сесія:** **FM-026** (Jobs contract/E2E).
