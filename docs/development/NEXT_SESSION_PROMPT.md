# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **Фаза:** FM-042…041 · Post-Horizon **FM-020…038 ✅**

Скопіюй блок нижче в новий чат Cursor (Agent mode, MSYS2 bash для git/cargo).

---

```
PoolAI — розробка FM-042 (наступна в §5.1). FM-038 закрито (OpenTelemetry tracing).

## S0 — зріз

1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (якщо Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…038; FM-039 Playwright CI; FM-038 OpenTelemetry.

## Мета сесії — FM-042

Hot-path profiling + Criterion benchmarks (beyond FM-028 snapshot).
Канон: `PERCENTAGE_PLAN`, `BENCHMARKS.md`, FM-042 у `FUNCTION_MANAGEMENT.md`.

## Черга після FM-042 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| — | FM-041 | Cloud SDK deep (Deferred) |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

## Завершення

src/ → cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише FM-042, GIT_EDITOR=true, git log -1 перевірка subject
push + Summary у коміті (git-push.md) · оновити HANDOFF + §5.1
```
