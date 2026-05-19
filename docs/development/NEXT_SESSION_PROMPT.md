# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-19 · **Фаза:** **Horizon** (Layer C → 100%) · **Autoprogon A+B:** ✅ S34 · **Horizon S35–S39:** ✅

**Копіюй блок нижче** в нову сесію (один спринт за ітерацію).

---

## Промпт

```
PoolAI — Horizon: довести Layer C і проєкт до 100% (оркестратор + FM).

## S0 — зріз (обов’язково)

1. git fetch && git status -sb (main).
2. HANDOFF_NEW_SESSION.md
3. FUNCTION_MANAGEMENT.md §5.1, §5.6, §5.3
4. HORIZON_TO_100_PLAN.md
5. AUTO_RUN_SESSION_2026_HORIZON.md — перший [ ] у §4
6. .cursor/rules/autonomous-orchestrator.mdc
7. NEXT_STEPS_ARCHITECT_2026-03-17.md — операційний порядок + P6 wire

Autoprogon S21–S34 і Horizon S35–S39 — не повторювати.

## Мета ітерації (одна за сесію)

| Спринт | FM | Фокус | Критерій |
|--------|-----|--------|----------|
| S40 | C | 100% closure | DEVELOPMENT_PROGRESS C=100%, project 100%, FM §5.6, CHANGELOG, NEXT_SESSION → maintenance |

Почни з S40 (перший незакритий у AUTO_RUN_SESSION_2026_HORIZON.md §4).

Перед кодом S40:
- HORIZON_TO_100_PLAN.md §«Перевірка Layer C = 100%»
- DEVELOPMENT_PROGRESS_2026-05-19.md
- FUNCTION_MANAGEMENT §5.6

Архітектура (вже в коді, лише утримувати):
- src/grid/, src/job/, src/memory/
- crates/poolai-solana-adapter/
- turboquant-simd feature

## Завершення ітерації

1. Зміни src/ → cargo fmt --all → cargo test-ci (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28).
2. Нові/змінені API → docs/openapi.yaml (+ OPENAPI_GAP_AUDIT за потреби).
3. Оновити: AUTO_RUN §Sxx, FM §5.6, HANDOFF, DEVELOPMENT_PROGRESS, AUTO_DEV_PATTERNS.
4. commit + push MSYS2 з Summary (subject + 3–5 рядків); git -c commit.template= commit -F msgfile.

Не стаджити: data/audit/*.log.gz, data/dev/, data/lan-stand/, .commit-msg-*.txt.
Поза обсягом: FM-003 §4 LAN (2 хости), mainnet Solana, KYC.

## Після S40 (maintenance)

NEXT_SESSION → режим утримання: test-ci на main, FM-003 §4 лише з 2 хостами.
```

---

## Довідка

| Документ | Роль |
|----------|------|
| [`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md) | Методика C→100% |
| [`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md) | Черга S39–S40 |
| [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md) | C **~80%**, проєкт **~93%** |
| [`ARCHITECTURE_BEST_PRACTICES.md`](../ARCHITECTURE_BEST_PRACTICES.md) | Дерево `src/` + workspace |
| [`RUN_LOCAL.md`](./RUN_LOCAL.md) | `bash bin/run-poolai.sh single` |

**Наступна сесія:** **S40** (Layer C + project 100% closure). Після S40 — maintenance mode.
