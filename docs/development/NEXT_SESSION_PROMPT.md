# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **HEAD (origin/main):** `9ee5fde1` · **Наступна:** PH-S05

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S05 RAID raft role / cluster status UI.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S03…S04; PH-S07…S14; PH-S11…S13 visual/matrix/topology.

## Закрито (PH-S04)
- `AppState::raft_node` + `attach_raft_node_for_test`; `RaidService::cluster_status` → `raft_status`
- `tests/raft_wire_integration.rs` — `GET /api/v1/raid/status` wire (2 tests)
- `cargo test-raft-ci` (після `cargo test-ci`; `--features raft,test-utils`)

## Known (локальний a11y 2026-05-24)
Playwright a11y: 13/16 fail — color-contrast btn-primary (#7e7e7e / #002200) на admin; login OK. Варто зняти перед push UI-heavy PH-S05.

## Черга PH (6 відкритих) — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · PH-S05 RAID raft UI ← наступна · PH-S06 multi-node harness

## Мета — PH-S05
RAID admin: відобразити `raft_status` (role/term/leader) у `/ui/admin/raid`; OpenAPI `RaidDistributedRaftRole`.

Альтернативи: PH-S06 harness; або a11y HC contrast (btn-primary).

## Завершення
cargo fmt --all → cargo test-ci → cargo test-raft-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, commit-tree якщо subject = Co-authored-by:
push + Summary · HANDOFF · NEXT_SESSION → наступний PH з §5.9
```
