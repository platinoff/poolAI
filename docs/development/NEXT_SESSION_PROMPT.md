# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **Фаза:** FM-037…042 · Post-Horizon **FM-020…040 ✅**

Скопіюй блок нижче в новий чат Cursor (Agent mode, MSYS2 bash для git/cargo).

---

```
PoolAI — розробка FM-037 (наступна в §5.1). FM-040 закрито (admin UI field audit).

## S0 — зріз

1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (якщо Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…040; FM-036 sharding; FM-040 admin field audit.

## Мета сесії — FM-037

Cluster topology graph (D3/vis + latency matrix) на `src/ui/admin/topology.rs`.
Канон: EXO plan §4.1, `ARCHITECT_PLAN_EXO_INTEGRATION_2026-01-17.md`.

## Черга після FM-037 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-039 | Playwright у CI |
| 3–4 | FM-038,042 | OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

## Завершення

src/ → cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише FM-037, GIT_EDITOR=true, git log -1 перевірка subject
push + Summary у коміті (git-push.md) · оновити HANDOFF + §5.1 → FM-039 next
```
