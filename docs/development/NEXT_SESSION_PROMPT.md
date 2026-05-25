# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `9b4053ab` · **§5.11** — PH-S37 → PH-S44

---

```
PoolAI — PH-S37 Linux PNG merge → PH-S44 visual+axe gate.

## S0
MSYS2 UCRT64 bash (не PowerShell для git/cargo) · HANDOFF · FM §5.1 · §5.10 · §5.11
df -h /s — Use% ≥99% → cargo clean
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_BUILD_JOBS=1   # AV/памʼять; обовʼязково перед test-ci / e2e build

## Принцип сесії
Локально **0 errors** перед push (fmt + test-ci + openapi-gap). CI — підтвердження; pa11y WCAG — окремий тікет.

## Стан
- **PH-S03…S34:** ✅
- **PH-S47 ✅:** CI #1213 (`0fe21bf1`) green — ubuntu+windows Test Suite, openapi-gap; локально test-ci + gap audit ✅
- **PH-S37 (відкрито):** workflow `update-visual-baselines.yml` (`a6f14cb2`); infra ✅ — потрібен merge Linux PNG PR
- **PH-S35/S16, FM-003 LAN §4:** BLOCKED · **PH-S36/FM-041:** Deferred

## PH-S37 — перша черга
1. GitHub Actions → **Update visual baselines (PH-S37)** → Run workflow (`create_pr=true`)
2. Review + merge PR `test(e2e): Linux visual baselines (PH-S37)`
3. Переконатись: Playwright visual job green на merged HEAD
4. Не комітити Windows-local snapshots у main

## PH-S44 — після S37
- Увімкнути visual + axe required gate на UI PR (`ci.yml` paths-filter)
- Див. §5.10 DOCS_LEGACY, `AUTO_RUN` a11y merge

## Локальна розробка (якщо чіпаєш код)
```bash
cd /s/rust/poolAI
cargo fmt --all
cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # exit 0
# опційно: bash bin/e2e-playwright.sh --start
```

## Не повторювати
PH-S03…S34 · PH-S47 серія (`160a1f59`…`0fe21bf1`, CI #1213) · PH-S37 infra (rotation tab, workflow yaml)
cargo test-ci --verbose · gap audit без нових routes

## Наступні 10 спринтів (§5.11)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S37** | Linux visual PNG merge |
| 2 | **PH-S44** | CI visual + axe gate на UI PR |
| 3 | **PH-S39** | VM Windows CPU/memory limits |
| 4 | **PH-S42** | Admin tables UX |
| 5 | **PH-S43** | ML/monitoring metrics admin UI |
| 6 | **PH-S45** | E2E vm modal + axe audit |
| 7 | **PH-S38** | Job scheduler + on-chain |
| 8 | **PH-S46** | Solana on-chain program |
| 9 | **PH-S41** | macvlan (Linux) |
| 10 | *(reserve)* | PH-S40 hardware VM — великий scope, поза чергою |

**Поза чергою:** PH-S35 LAN (BLOCKED) · PH-S36 Cloud SDK (Deferred) · PH-S40 hardware VM
```
