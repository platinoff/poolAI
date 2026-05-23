# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **HEAD:** *(після push)* · **PH-S12 ✅**

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S13 (topology masked SVG visual). PH-S11…S12 visual regression на main.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S07…S12; PH-S11 snapshots; PH-S12 theme/i18n matrix.

## Черга PH (10 відкритих) — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · PH-S03 VM E2E · PH-S04 Raft wire · PH-S05 RAID raft UI · PH-S06 multi-node harness · PH-S13 topology visual ← наступна · PH-S14 high-contrast/axe

## Мета — PH-S13
Admin topology graph: masked SVG visual baseline у `e2e/tests/visual.spec.ts`; канон: `topology_graph.js`, [`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md).

Альтернативи: PH-S03 VM E2E · PH-S04…S06 VM/Raft · PH-S14 axe contrast.

## Завершення
cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, GIT_EDITOR=true, git log -1 subject
push + Summary · HANDOFF · NEXT_SESSION → PH-S14
```
