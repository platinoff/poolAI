# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `28bcb91f` · **§5.11** — PH-S47 (дочекатись CI)

---

```
PoolAI — PH-S47 CI green → PH-S37 PNG merge → PH-S44.

## S0
MSYS2 bash · HANDOFF · FM §5.1 · §5.10 · §5.11
df -h /s — Use% ≥99% → cargo clean перед cargo test-ci

## Стан
- **PH-S03…S34:** ✅
- **CI main:** `28bcb91f` PH-S47c (E2E без ml, `test:ci` без visual); `c624d189` test-ci; **дочекатись зеленого** ubuntu + openapi-gap + Playwright → **PH-S47**
- **PH-S37:** workflow YAML ✅ (`a6f14cb2`); **PNG** — merge PR після зеленого CI
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED (2 хости)
- **PH-S36/S15, FM-041:** Deferred

## PH-S47 — закрити (перша черга)
1. OpenAPI: `/admin/secrets/rotation`, `/admin/secrets/rotate` + schemas → `poolai-openapi-gap-audit` exit 0
2. `tests/raid_admin_api_integration.rs` — TempDir (не `default_for_platform` на CI)
3. `bin/e2e-playwright.sh` + `e2e.yml` — `CI=true` → debug + `CARGO_BUILD_JOBS=1` (`c624d189`)
4. `ci.yml` — `cargo test-ci` на ubuntu (`c624d189`) · перевірити Actions (Test Suite ubuntu, openapi-gap, Playwright)

## PH-S37 — після S47
1. Actions → **Update visual baselines (PH-S37)** → Run workflow
2. Merge PR `test(e2e): Linux visual baselines (PH-S37)`

## Не повторювати
PH-S03…S34; PH-S37 infra (rotation tab, workflow YAML fix `a6f14cb2`); повторний gap audit без нових routes

## Наступні 10 спринтів (§5.11)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S47** | CI red: OpenAPI secrets, raid_admin test, E2E debug build |
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
bash bin/e2e-playwright.sh --start
```
