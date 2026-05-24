# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **HEAD (origin/main):** `0fe90211` · **Локально:** PH-S03 готово, **не закомічено**

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — спершу commit PH-S03 (якщо ще не на main), далі PH-S04 Raft wire.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S07…S14; PH-S11…S13 visual/matrix/topology.

## Локальний зріз (PH-S03, не на main)
- tests/vm_api_contracts.rs — VM write lifecycle + RBAC (4 tests, cargo test-ci ✅)
- e2e/tests/admin.spec.ts — create/delete VM modal
- docs: ADMIN_UI_JSON_CONTRACTS, FM §5.9, HANDOFF, E2E_PLAYWRIGHT
- Не стаджити: data/audit/*, bin/commit-*.sh, .commit-msg-*.txt

## Known (локальний a11y 2026-05-24)
Playwright a11y: 13/16 fail — color-contrast btn-primary (#7e7e7e / #002200) на admin; login OK. Не блокує PH-S04, але варто зняти перед push UI.

## Черга PH (7 відкритих) — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · PH-S04 Raft wire ← наступна · PH-S05 RAID raft UI · PH-S06 multi-node harness

## Мета — PH-S04
Raft feature wire tests (`--features raft`); канон: Architect L325, `NEXT_STEPS_2026-01-19`.

Альтернативи: PH-S05…S06 RAID/Raft UI; або закрити a11y HC contrast.

## Завершення
cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, commit-tree якщо subject = Co-authored-by:
push + Summary · HANDOFF · NEXT_SESSION → наступний PH з §5.9
```
