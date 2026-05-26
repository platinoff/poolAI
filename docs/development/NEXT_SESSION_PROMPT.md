# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **HEAD** (після PH-S50 docs) · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S51 VM Linux isolation hardening

## Ролі (VDT)
- Людина: власник / креативний директор — пріоритети, BLOCKED/Deferred
- Ти: оркестратор Rust — один PH-S*, субагенти для explore/shell/модуль
- Правила: virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## S0 (MSYS2 UCRT64 bash)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
df -h /s | tail -1
HANDOFF · FM §5.11

## Локальний CI (канон)
cargo fmt --all
cargo test-ci
# Linux host + vm scope: tests/vm_isolation_integration.rs, feature vm-isolation-linux

## Стан
- **PH-S03…S50:** ✅
- **Черга §5.11:** PH-S51…S60 (10 відкритих)
- **BLOCKED:** PH-S35/S16/S02 LAN · **Deferred:** PH-S36/S01/S15 Cloud SDK (FM-041)

## PH-S51 — наступна сесія (код)
Scope: `src/vm/isolation/linux.rs`, `tests/vm_isolation_integration.rs`
1. feature `vm-isolation-linux`: apply + cleanup integration (veth, macvlan edge cases)
2. netns/cgroup cleanup on VM remove; warn paths covered тестами
3. `cargo test-ci` (+ targeted linux tests якщо MSYS2/Linux)

## Завершення сесії
1. FM §5.11 (PH-S51 → ✅) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (PH-S52)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S50 · OpenAPI jobs/DIGEST · PH-S48/49 RAID store docs

## Черга §5.11 (відкриті)
| # | Sprint | Фокус |
|---|--------|--------|
| 1 | **PH-S51** | VM Linux isolation hardening |
| 2 | **PH-S52** | E2E jobs + RAID smoke |
| 3 | **PH-S53** | Admin jobs UI |
| 4–10 | **PH-S54…S60** | verify-dev-stand, run-poolai, grid, CI docs, runbooks, RUN_PARAMETERS, openapi maintenance |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
