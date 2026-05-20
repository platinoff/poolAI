# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** **Post-Horizon розробка** (FM-020…) · **A+B+C:** **100%** · **job store JSON:** ✅ `cd1aaad`

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
6. JOB_LAYER_CONCEPT_2026-03-17.md (FM-020)
7. NEXT_STEPS_ARCHITECT_2026-03-17.md — P6 залишок (scheduler, on-chain)

Не повторювати: autoprogon S21–S34, Horizon S35–S40, FM-001…019 baseline, job store JSON (закрито).

## Мета ітерації (одна FM за сесію)

| Пор. | FM | Фокус | Критерій |
|------|-----|--------|----------|
| — | FM-020 | Job scheduler MVP | ✅ `scheduler.rs`, `POST /jobs/schedule` |
| 1 | FM-021 | Jobs PATCH + OpenAPI | PATCH status; openapi /jobs |
| 3 | FM-022 | Memory API | shard refs HTTP або RAID map |
| 4 | FM-023 | Grid integration | Job/Result на discovery/distributed path |
| 5 | FM-024 | Solana RPC stub | devnet config; sidecar only |
| 6 | FM-025 | OpenAPI DTO | VM template bodies |
| 7 | FM-026 | Jobs QA | contract або Playwright |
| 8 | FM-027 | LAN runbook | 2-host checklist (**BLOCKED** без хостів) |
| 9 | FM-028 | P2b metrics | single-host TQ01+RAID → BENCHMARKS |
| 10 | FM-029 | Job SQLite | optional backend |
| 11 | FM-030 | Monitoring persist | MONITORING_PERSISTENCE_PLAN MVP |
| 12 | FM-031 | WCAG expand | pa11y/axe URLs |

Почни з FM-021 (перший [ ] у AUTO_RUN §4).

Перед кодом FM-020:
- src/job/store.rs, src/network/api/jobs.rs
- docs/development/JOB_LAYER_CONCEPT_2026-03-17.md §2.2, §6

## Завершення ітерації

1. Зміни src/ → cargo fmt --all → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28).
2. Нові/змінені API → docs/openapi.yaml.
3. Оновити: AUTO_RUN §FM, FM §5.1/§5.7, HANDOFF, CHANGELOG, AUTO_DEV_PATTERNS.
4. commit + push MSYS2 з Summary; git -c commit.template= commit -F msgfile.

Не стаджити: data/audit/*.log.gz, data/dev/, data/lan-stand/, .commit-msg-*.txt.
Поза обсягом: FM-003 §4 sign-off без 2 хостів; mainnet Solana; KYC.
```

---

## Довідка

| Документ | Роль |
|----------|------|
| [`AUTO_RUN_SESSION_2026_POST_HORIZON.md`](./AUTO_RUN_SESSION_2026_POST_HORIZON.md) | Черга FM-020…031 |
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) | FM-* §5.1, §5.7 |
| [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md) | A+B+C **100%** |
| [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) | Операційний зріз |
| [`RUN_LOCAL.md`](./RUN_LOCAL.md) | `bin/run-poolai.sh` |

**Наступна сесія:** **FM-021** (Jobs PATCH + OpenAPI).
