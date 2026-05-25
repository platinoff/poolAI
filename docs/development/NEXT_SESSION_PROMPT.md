# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-25 · **HEAD** `f11d3d4e` · **§5.11** — PH-S39

---

```
PoolAI — PH-S39 VM Windows resource limits (post-spawn).

## S0
MSYS2 UCRT64 bash · HANDOFF · FM §5.1 · §5.11
df -h /s — Use% ≥99% → cargo clean
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
export CARGO_BUILD_JOBS=1

## Принцип сесії
Перед push: fmt + test-ci + openapi-gap (0 errors). CI — підтвердження.

## Стан
- **PH-S03…S34, PH-S47:** ✅
- **PH-S37 ✅ infra:** `update-visual-baselines.yml` — Linux PNG refresh on-demand (Actions dispatch)
- **PH-S44 ✅:** `test:ci` = smoke+admin+a11y+visual; `ci.yml` paths-filter → Playwright + Pa11y WCAG on UI/e2e PR
- **PH-S35/S16, FM-003 LAN:** BLOCKED · **PH-S36/FM-041:** Deferred

## PH-S39 — перша черга
- VM Windows CPU/memory limits post-spawn (`vm/resources.rs`, `vm_windows_resource_limits_integration`)
- Див. AUTO_RUN §1.6, §5.10 FM

## Локально перед push
```bash
cd /s/rust/poolAI
cargo fmt --all
cargo test-ci
cargo run --bin poolai-openapi-gap-audit
```

## Не повторювати
PH-S03…S47 · PH-S37/PH-S44 (e2e test:ci visual gate) · `cargo test-ci --verbose`

## Наступні спринти (§5.11)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S39** | VM Windows CPU/memory limits |
| 2 | **PH-S42** | Admin tables UX |
| 3 | **PH-S43** | ML/monitoring metrics admin UI |
| 4 | **PH-S45** | E2E vm modal + axe audit |
| 5 | **PH-S38** | Job scheduler + on-chain |
| 6 | **PH-S46** | Solana on-chain program |
| 7 | **PH-S41** | macvlan (Linux) |

**Поза чергою:** PH-S35 LAN (BLOCKED) · PH-S36 Cloud SDK (Deferred) · PH-S40 hardware VM
```
