# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **Фаза:** FM-036…042 · Post-Horizon **FM-020…035 ✅** · FM-034 ✅ (цей push)

Скопіюй блок нижче в новий чат Cursor (Agent mode, MSYS2 bash для git/cargo).

---

```
PoolAI — розробка FM-036 (наступна в §5.1). FM-034 закрито (job scheduler → VM/worker binding).

## S0 — зріз

1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (якщо Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…035; FM-033 Solana; FM-034 job scheduler binding.

## Мета сесії — FM-036

Tensor sharding runtime (`sharding.rs`, inter-worker sync, benches).
Канон: EXO plan §3.1–3.2, Architect P6.

## Черга після FM-036 (одна FM / сесію)

| # | FM | Фокус |
|---|-----|--------|
| 2 | FM-040 | Admin UI field audit |
| 3–6 | FM-037,039,038,042 | Topology graph, Playwright CI, OTel, perf |

Ops BLOCKED: FM-003 §4 LAN (2 хости) — лише verify-lan-prep / runbook.

## Завершення

src/ → cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише FM-036, GIT_EDITOR=true, git log -1 перевірка subject
push + Summary у коміті (git-push.md) · оновити HANDOFF + §5.1 → FM-040 next
```
