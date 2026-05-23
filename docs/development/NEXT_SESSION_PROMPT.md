# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **HEAD:** `54543028` · **FM-001…045 ✅**

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S10 (admin charts / real-time graphs). FM-020…045 на main @ 54543028.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S07 Prometheus; PH-S08 TLS; PH-S09 design system.

## Черга PH (11 відкритих)
PH-S01 (Deferred) · PH-S02 BLOCKED · PH-S03…S06 VM/Raft · PH-S10 charts ← наступна · PH-S11…S14

## Мета — PH-S10
Chart layer для admin monitoring (`src/ui/admin/monitoring.rs`, UI_UX P4).
Стилі: DESIGN_SYSTEM.md · метрики API: `/api/enterprise/monitoring/metrics`.

Альтернативи: PH-S11 visual regression · PH-S03…S05 VM.

## Завершення
cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, GIT_EDITOR=true, git log -1 subject
push + Summary · HANDOFF · NEXT_SESSION → PH-S11
```
