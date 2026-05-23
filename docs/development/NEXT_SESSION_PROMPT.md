# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **HEAD:** _(після push PH-S10)_ · **PH-S10 ✅**

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S11 (visual regression / Percy або аналог). PH-S10 charts на main.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S07…S10; PH-S08 TLS; PH-S09 design system.

## Черга PH (10 відкритих)
PH-S01 (Deferred) · PH-S02 BLOCKED · PH-S03…S06 VM/Raft · PH-S11 visual regression ← наступна · PH-S12…S14

## Мета — PH-S11
Visual regression для admin UI (Playwright screenshots або Percy); канон e2e: `e2e/`, `UI_QUALITY_AND_E2E_PLAN`.

Альтернативи: PH-S03…S05 VM/Raft · PH-S12…S14.

## Завершення
cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, GIT_EDITOR=true, git log -1 subject
push + Summary · HANDOFF · NEXT_SESSION → PH-S12
```
