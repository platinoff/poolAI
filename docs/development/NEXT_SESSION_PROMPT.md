# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `0fe21bf1` · **§5.11** — PH-S47 → PH-S37

---

```
PoolAI — локально без помилок → PH-S47 CI green → PH-S37 PNG → PH-S44.

## S0
MSYS2 UCRT64 bash (не PowerShell для git/cargo) · HANDOFF · FM §5.1 · §5.10 · §5.11
df -h /s — Use% ≥99% → cargo clean
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_BUILD_JOBS=1   # AV/памʼять; обовʼязково перед test-ci / e2e build

## Принцип сесії
GitHub CI на main часто червоний (rustc 101, Windows matrix, pa11y) — **не блокує локальну розробку**.
Перед кожним push: локально **0 errors** (fmt + test-ci + openapi-gap). CI — підтвердження; pa11y WCAG — окремий тікет.

## Стан
- **PH-S03…S34:** ✅
- **PH-S47 (відкрито):** `160a1f59` OpenAPI/raid_admin/E2E debug · `6dfe8b4e` ProcessCollector · `164adf30` test-ci · `28bcb91f` E2E test:ci · `4460b282` pa11y debug · `0fe21bf1` Windows vm test imports
- **CI #1213** (`0fe21bf1`) — in progress; ubuntu часто green після S47f; Windows — `vm_windows` imports fixed
- **PH-S37:** workflow ✅ (`a6f14cb2`); Linux PNG merge після зеленого PH-S47
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED · **PH-S36/FM-041:** Deferred

## PH-S47 — закрити (перша черга, CI)
1. Дочекатись **зеленого** CI на HEAD: ubuntu Test Suite, openapi-gap, Playwright (`test:ci` smoke+admin+a11y)
2. Якщо red — лише мінімальний fix; не `cargo test-ci --verbose` (alias ламає `--`)
3. **Pa11y WCAG 2.2** — не блокер gap/raid (окремий FM/ticket)

## Локальна розробка (обовʼязково перед push)
```bash
cd /s/rust/poolAI
cargo fmt --all
cargo test-ci
cargo test-raft-ci          # якщо чіпали raft
cargo run --bin poolai-openapi-gap-audit   # exit 0
# опційно E2E: bash bin/e2e-playwright.sh --start  # повний npm test локально
```
- **Без Python** у репо (runtime-stack-policy)
- **cfg imports:** типи лише для windows-тестів → `#[cfg(target_os = "windows")] use …`
- Commit: MSYS2 + commit-tree якщо hook ламає subject (`GIT_EDITOR=true`)

## PH-S37 — після зеленого PH-S47
1. Actions → **Update visual baselines (PH-S37)** → Run workflow
2. Merge PR `test(e2e): Linux visual baselines (PH-S37)`
3. PH-S44: повернути visual + axe gate на UI PR

## Не повторювати
PH-S03…S34 · PH-S37 infra (rotation tab, workflow `a6f14cb2`) · gap audit без нових routes
OpenAPI/raid_admin/ProcessCollector/test-ci/E2E pa11y debug (S47 серія) · `cargo test-ci --verbose`

## Наступні 10 спринтів (§5.11)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S47** | CI green (ubuntu + openapi-gap + Playwright); локально test-ci |
| 2 | **PH-S37** | Linux visual PNG merge |
| 3 | **PH-S44** | CI visual + axe gate на UI PR |
| 4 | **PH-S39** | VM Windows CPU/memory limits |
| 5 | **PH-S42** | Admin tables UX |
| 6 | **PH-S43** | ML/monitoring metrics admin UI |
| 7 | **PH-S45** | E2E vm modal + axe audit |
| 8 | **PH-S38** | Job scheduler + on-chain |
| 9 | **PH-S46** | Solana on-chain program |
| 10 | **PH-S41** | macvlan (Linux) |

**Поза чергою:** PH-S35 LAN (BLOCKED) · PH-S36 Cloud SDK (Deferred) · PH-S40 hardware VM
```
