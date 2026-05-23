# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **HEAD:** `814fe29c` · **PH-S14 ✅**

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S03 (VM admin E2E + write-op contracts). PH-S14 high-contrast/axe на main.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S07…S14; PH-S11…S13 visual/matrix/topology.

## Черга PH (8 відкритих) — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · PH-S03 VM E2E ← наступна · PH-S04 Raft wire · PH-S05 RAID raft UI · PH-S06 multi-node harness

## Мета — PH-S03
VM admin E2E + write-op contracts (`vm_write_operations`, `vm_service`); канон: UI_QUALITY §P2, `admin.spec.ts`.

Альтернативи: PH-S04…S06 VM/Raft.

## Завершення
cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, commit-tree якщо subject = Co-authored-by:
push + Summary · HANDOFF · NEXT_SESSION → наступний PH з §5.9
```
