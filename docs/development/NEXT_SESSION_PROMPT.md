# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `164adf30` (+ push PH-S47f) · **§5.11** — PH-S47 → PH-S37

---

```
PoolAI — PH-S47 CI green → PH-S37 PNG merge → PH-S44.

## S0
MSYS2 bash · HANDOFF · FM §5.1 · §5.10 · §5.11
df -h /s — Use% ≥99% → cargo clean перед cargo test-ci
Локально: CARGO_BUILD_JOBS=1; AV exclusion для target/ + ~/.cargo

## Стан
- **PH-S03…S34:** ✅
- **PH-S47 (відкрито):** OpenAPI secrets ✅ (`160a1f59`) · raid_admin TempDir ✅ · ProcessCollector Linux ✅ (`6dfe8b4e`) · test-ci inline (`164adf30`) · E2E `test:ci` без visual (`28bcb91f`) · pa11y/E2E debug без ml (`PH-S47f` push)
- **CI red (2026-05-25):** rustc SIGSEGV `release+ml` (pa11y/a11y job); Playwright build 101 — фікси в `PH-S47f`; pa11y WCAG окремий тікет якщо лишиться червоним
- **PH-S37:** workflow YAML ✅ (`a6f14cb2`); **PNG** — merge PR після зеленого CI
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED (2 хости)
- **PH-S36/S15, FM-041:** Deferred

## PH-S47 — закрити (перша черга)
1. Дочекатись зеленого CI: **ubuntu Test Suite**, **openapi-gap**, **Playwright** (smoke+admin+a11y)
2. Перевірити HEAD після `PH-S47f`: pa11y/a11y debug build, `CARGO_BUILD_JOBS=1`, cache keys
3. Pa11y WCAG 2.2 — не блокер gap/raid (окремий тікет якщо red)

## PH-S37 — після S47
1. Actions → **Update visual baselines (PH-S37)** → Run workflow
2. Merge PR `test(e2e): Linux visual baselines (PH-S37)`
3. Повернути full `npm test` + visual у CI (PH-S44)

## Не повторювати
PH-S03…S34; PH-S37 infra (rotation tab, workflow `a6f14cb2`); gap audit без нових routes; `cargo test-ci --verbose` (alias)

## Наступні 10 спринтів (§5.11)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S47** | CI green: OpenAPI, raid_admin, test-ci, E2E/pa11y debug (no release+ml SIGSEGV) |
| 2 | **PH-S37** | Linux visual PNG merge |
| 3 | **PH-S44** | CI: visual + axe gate на UI PR |
| 4 | **PH-S39** | VM Windows CPU/memory limits |
| 5 | **PH-S42** | Admin tables UX |
| 6 | **PH-S43** | ML/monitoring metrics admin UI |
| 7 | **PH-S45** | E2E: vm modal + axe audit |
| 8 | **PH-S38** | Job scheduler + on-chain |
| 9 | **PH-S46** | Solana on-chain program |
| 10 | **PH-S41** | macvlan (Linux) |

**Поза чергою:** PH-S35 LAN (BLOCKED) · PH-S36 Cloud SDK (Deferred) · PH-S40 hardware VM

## Перевірки
cargo fmt --all
cargo test-ci
cargo run --bin poolai-openapi-gap-audit
# E2E локально: export CARGO_BUILD_JOBS=1; bash bin/e2e-playwright.sh --start
```
