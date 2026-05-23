# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **Фаза:** FM-038…042 · Post-Horizon **FM-020…039 ✅**

Скопіюй блок нижче в новий чат Cursor (Agent mode, MSYS2 bash для git/cargo).

---

```
PoolAI — розробка FM-038 (наступна в §5.1). FM-039 закрито (Playwright у CI).

## S0 — зріз

1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (якщо Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…039; FM-037 topology graph; FM-039 Playwright CI.

## Мета сесії — FM-038

OpenTelemetry distributed tracing (middleware + export).
Канон: `NEXT_STEPS_2026-01-16`, FM-038 у `FUNCTION_MANAGEMENT.md`.

## Черга після FM-038 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-042 | Hot-path perf / Criterion |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

## Завершення

src/ → cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише FM-038, GIT_EDITOR=true, git log -1 перевірка subject
push + Summary у коміті (git-push.md) · оновити HANDOFF + §5.1 → FM-042 next
```
