# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-27 · **VDT** [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — ітераційна сесія PH-S84 (VDT, один спринт)

## S0 (MSYS2 UCRT64 bash)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
Прочитати: HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.12 · цей файл

## Стан (2026-05-27)
- Закрито: PH-S03…S83 + PH-S76 + PH-S77 + PH-S90
- Відкритий sprint: PH-S84 (перший у §5.12)

## PH-S84 — scope
1. `docs/concept/POOLAI_GALAXY_GRID.md` — mark `GET /api/v1/grid/pricing` implemented (PH-S78/S79/S82); remove stale «майбутній wire»
2. Cross-links DIGEST/HANDOFF якщо потрібно
3. FM §5.12 (PH-S84 → ✅) + HANDOFF + цей prompt

## Локальний CI (docs-only)
cargo fmt --all   # якщо чіпали Rust — інакше пропустити
# без test-ci для чистих docs, якщо scope лише markdown

## Не повторювати
PH-S03…S83 · pricing API/UI wire · stale/force metrics · Cursor rules refactor

## Черга §5.12 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | PH-S84 | Galaxy §4.2.3 wire note |
| 2 | PH-S85 | verify-release fixtures |
| 3 | PH-S86 | Grid pricing E2E |
| 4 | PH-S87 | INDEX security cross-link |
| 5 | PH-S88 | Release manifest sample |
| 6 | PH-S89 | L1 stale TTL metadata |
```
