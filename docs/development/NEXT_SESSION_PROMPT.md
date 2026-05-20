# Промпт наступної автономної сесії (PoolAI)

**Оновлено:** 2026-05-20 · **Фаза:** **Maintenance** (Horizon S35–S40 ✅ · A+B+C **100%** · job store `cd1aaad`)

**Копіюй блок нижче** в нову сесію (утримання `main`, без нових horizon-спринтів без явного запиту).

---

## Промпт

```
PoolAI — maintenance: утримувати main (test-ci, FM-003 §4 лише з 2 хостами).

## S0 — зріз

1. git fetch && git status -sb (main).
2. HANDOFF_NEW_SESSION.md
3. FUNCTION_MANAGEMENT.md §5.1, §5.6
4. DEVELOPMENT_PROGRESS_2026-05-19.md — A+B+C **100%**

Horizon S35–S40 — не повторювати.

## Мета ітерації

- `cargo test-ci` (MSYS2, K8S_OPENAPI_ENABLED_VERSION=1.28) після змін у src/tests.
- Нові FM-* — лише за запитом користувача або регресія.
- FM-003 §4 LAN sign-off — **BLOCKED** (2 фізичні хости); runbook only.

Поза обсягом: mainnet Solana, KYC, FM-004/006/009/010 re-implementation без запиту.

## Завершення (якщо були зміни)

1. cargo fmt --all → test-ci за потреби.
2. Оновити HANDOFF / CHANGELOG / FM лише при зміні продукту.
3. commit + push MSYS2 з Summary; не стаджити data/audit/*.log.gz, .commit-msg-*.txt.
```

---

## Довідка

| Документ | Роль |
|----------|------|
| [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) | Операційний зріз |
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) | FM-* канон |
| [`DEVELOPMENT_PROGRESS_2026-05-19.md`](../status/DEVELOPMENT_PROGRESS_2026-05-19.md) | A+B+C **100%** |
| [`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md) | Horizon — **закрито** S35–S40 |
| [`RUN_LOCAL.md`](./RUN_LOCAL.md) | `bash bin/run-poolai.sh single` |

**Horizon закрито 2026-05-19 (S40).** Наступні епіки — лише за явним запитом.
