# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** `23ecac5a` (PH-S107) · VDT — [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) · [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc)

---

## Copy-paste для наступної сесії

```
PoolAI — ітераційна сесія PH-S109 (VDT, один спринт)

## Ролі (VDT) — як у PH-S107

Правила: poolai-agent-roles.mdc · virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## Режим ітерації
- Один PH-S* · локальний CI перед push
- Закриття: PH-S109 → ✅ FM §5.12 + HANDOFF + цей файл
- Черга §5.12: **1** відкритий (PH-S109) — після закриття replenish (<10)

## S0
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
HANDOFF · FM §5.12 · GALAXY_GRID_ROADMAP · цей файл

## Локальний CI
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
# docs-only: openapi-gap / e2e за потреби

## Стан (2026-05-28)
- **Закрито:** PH-S03…S108 (смуга PH-S100…S109: 9✅ / 1 відкрито)
- **Відкритий:** **PH-S109** — Galaxy §4.3 lease wire docs sync
- **BLOCKED:** PH-S35/S16/S02 · **Deferred:** PH-S36/S01/S15

## PH-S109 — scope
1. `docs/concept/POOLAI_GALAXY_GRID.md` §4.3 — позначити implemented: Leased/Migrating, renew, grid ingest→leased, E2E (PH-S100…S108); без дублювання prose
2. INDEX, DIGEST, README Next Focus, `GALAXY_GRID_ROADMAP` (смуга 10/10 ✅)
3. FM §5.12 PH-S109 → ✅; HANDOFF; vision manifest

## Не повторювати
Код lease/E2E/grid ingest — уже в PH-S94…S108

## Після PH-S109
Replenish §5.12 (code-first, ≤10 відкритих) — `rg "TODO|FIXME" src/`, GALAXY_GRID_ROADMAP
```

---

## Короткий зріз

| | |
|---|---|
| **Наступний** | PH-S109 — §4.3 lease wire docs sync (закриває смугу PH-S100…S109) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
