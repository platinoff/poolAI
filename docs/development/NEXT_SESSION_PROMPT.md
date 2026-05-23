# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **HEAD:** `d1e3982d` · **PH-S13 ✅**

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S14 (high-contrast theme + axe contrast CI). PH-S11…S13 visual regression на main.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S07…S13; PH-S11 snapshots; PH-S12 theme/i18n matrix; PH-S13 topology masks.

## Черга PH (9 відкритих) — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · PH-S03 VM E2E · PH-S04 Raft wire · PH-S05 RAID raft UI · PH-S06 multi-node harness · PH-S14 high-contrast/axe ← наступна

## Мета — PH-S14
High-contrast theme + axe contrast CI fixes; канон: `UI_UX_MONITORING_IMPROVEMENTS_2026-01-21.md` §102, `a11y.spec.ts`, `themes.rs`.

Альтернативи: PH-S03 VM E2E · PH-S04…S06 VM/Raft.

## Завершення
cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, GIT_EDITOR=true, git log -1 subject
push + Summary · HANDOFF · NEXT_SESSION → наступний PH з §5.9
```
