# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `260858ea` · **§5.11** — наступні 10 спринтів PH-S37…S46

---

```
PoolAI — PH-S37 merge Linux PNG → PH-S44; §5.11 черга 10 спринтів.

## S0
MSYS2 bash · HANDOFF · FM §5.1 · §5.10 · §5.11

## Стан
- **PH-S03…S34:** ✅
- **PH-S37:** workflow + `create_pr` + security rotation tab ✅ (`24108c15`); **PNG** — merge PR з Actions
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED (2 хости)
- **PH-S36/S15, FM-041:** Deferred

## PH-S37 — закрити (сьогодні)
1. Actions → **Update visual baselines (PH-S37)** → Run workflow
2. Merge PR `test(e2e): Linux visual baselines (PH-S37)`
3. FM §5.10 / §5.11 #1 → ✅

## Не повторювати
PH-S03…S34; PH-S37 infra (workflow, rotation tab, `record_secret_rotation`); docs §5.11 sync

## Наступні 10 спринтів (§5.11 — канон FM)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S37** | Linux visual PNG merge |
| 2 | **PH-S44** | CI: visual + axe gate на UI PR |
| 3 | **PH-S39** | VM Windows CPU/memory limits |
| 4 | **PH-S42** | Admin tables UX (sort/filter/export) |
| 5 | **PH-S43** | ML/monitoring metrics admin UI |
| 6 | **PH-S45** | E2E: vm create modal + axe audit page |
| 7 | **PH-S38** | Job scheduler + on-chain epics |
| 8 | **PH-S46** | Solana on-chain program (post FM-024) |
| 9 | **PH-S41** | macvlan (Linux) |
| 10 | **PH-S40** | Hardware VM isolation |

**Поза чергою:** PH-S35 LAN (BLOCKED) · PH-S36 Cloud SDK (Deferred)

## Перевірки
cargo fmt --all
cargo test-ci
bash bin/e2e-playwright.sh --start
```
