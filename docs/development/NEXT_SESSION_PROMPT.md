# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Фаза:** **Horizon** (Layer C → 100%) · **Autoprogon A+B:** ✅ S34

**Копіюй блок нижче** в нову сесію.

---

## Промпт

```
PoolAI — horizon: довести проєкт до 100% (оркестратор + FM).

## S0 — зріз

1. git fetch && git status -sb (main).
2. HANDOFF_NEW_SESSION.md
3. FUNCTION_MANAGEMENT.md §5.1, §5.6, §5.3
4. HORIZON_TO_100_PLAN.md (методика Layer C)
5. AUTO_RUN_SESSION_2026_HORIZON.md (черга S35–S40)
6. .cursor/rules/autonomous-orchestrator.mdc
7. RUN_LOCAL.md — bash bin/run-poolai.sh single

Autoprogon A+B = 100% (S34). Не повторювати S21–S34.

## Мета ітерації (одна за сесію)

| Спринт | FM | Фокус | Критерій |
|--------|-----|--------|----------|
| S35 | FM-004 | SIMD TurboQuant | feature simd + bench + tests у turboquant.rs |
| S36 | FM-009 | Grid envelope v1 | src/grid + grid_network tests |
| S37 | FM-010 | Solana adapter MVP | crates/poolai-solana-adapter event schema stub |
| S38 | P6 | Job/Memory types | src/job + src/memory minimal |
| S39 | FM-006 | cloud-sdk Azure/GCP | TODO closure або documented scope |
| S40 | C | 100% closure | DEVELOPMENT_PROGRESS C=100%, project 100% |

Почни з першого незакритого [ ] у AUTO_RUN_SESSION_2026_HORIZON.md §4.

Концепти (перед кодом):
- GRID_PROTOCOL_CONCEPT_2026-04-06.md
- SOLANA_ADAPTER_CONCEPT_2026-04-06.md
- JOB_LAYER_CONCEPT, POOLAI_MEMORY_LAYER.md
- ml/TURBOQUANT_INTEGRATION.md

## Завершення

Зміни src/ → cargo fmt → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28).
Нові API → docs/openapi.yaml + OPENAPI_GAP_AUDIT.
commit + push MSYS2 з Summary (subject + 3–5 рядків).

Не стаджити: data/audit/*.log.gz, data/dev/, data/lan-stand/.
Поза обсягом: FM-003 §4 LAN (2 хости), mainnet Solana, KYC.
```

---

## Довідка

| Документ | Роль |
|----------|------|
| [`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md) | Методика C→100%, формула проєкту |
| [`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md) | Черга S35–S40 |
| [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md) | % по шарах |
| [`RUN_LOCAL.md`](./RUN_LOCAL.md) | Запуск |

**До зустрічі** — наступна сесія стартує з **S35** (FM-004 SIMD), якщо чеклист horizon §4 порожній для S35.
