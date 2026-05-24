# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **HEAD (origin/main):** `e343aab3` · **PH-S01…S14:** закрито (S01 Deferred, S02 BLOCKED)

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — post-PH: a11y HC contrast або FM-041 / ops.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S03…S14 (PH черга закрита).

## Закрито (PH-S06)
- `src/network/api/raft_rpc.rs` — `/raft/append-entries`, `/vote`, `/install-snapshot`
- `RaidRaftNode::handle_*` RPC; `tests/raft_multi_node_harness.rs` (2-node single-host)
- `cargo test-raft-ci` (wire + harness); BENCHMARKS.md §Raft harness

## Known
Playwright a11y: 13/16 fail — btn-primary color-contrast на admin (PH-S14 partial).

## Черга PH — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · **решта ✅**

## Мета (на вибір)
1. **a11y** — HC btn-primary contrast на admin (13/16 → green)
2. **FM-041** Cloud SDK deep (Deferred — лише за явним запитом)
3. **FM-003** LAN §4 (BLOCKED — 2 хости)

## Завершення
cargo fmt --all → cargo test-ci → cargo test-raft-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, commit-tree якщо subject = Co-authored-by:
push + Summary · HANDOFF · NEXT_SESSION
```
