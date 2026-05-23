# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **Фаза:** FM-040…042 · Post-Horizon **FM-020…036 ✅**

Скопіюй блок нижче в новий чат Cursor (Agent mode, MSYS2 bash для git/cargo).

---

```
PoolAI — розробка FM-040 (наступна в §5.1). FM-036 закрито (tensor sharding runtime).

## S0 — зріз

1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (якщо Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…036; FM-033 Solana; FM-034 job scheduler; FM-036 tensor sharding.

## Мета сесії — FM-040

Admin UI field audit (усі `src/ui/admin/*.rs` vs API handlers / OpenAPI).
Канон: UI_QUALITY_AND_E2E_PLAN §P1, FM-013…015 baseline.

## Черга після FM-040 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-037 | Topology graph (D3/vis) |
| 3–5 | FM-039,038,042 | Playwright CI, OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

## Завершення

src/ → cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише FM-040, GIT_EDITOR=true, git log -1 перевірка subject
push + Summary у коміті (git-push.md) · оновити HANDOFF + §5.1 → FM-037 next
```
