# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **HEAD (origin/main):** `a308e333` · **PH-S01…S14:** закрито (S01 Deferred, S02 BLOCKED)

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — post-PH maintenance: a11y HC contrast (рекомендовано) або ops/FM-041.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

## Стан репо (2026-05-24)
- **Гілка:** main (`origin/main` синхронізовано)
- **PH черга §5.9:** S03…S14 ✅ · S01 Deferred · S02 BLOCKED
- **FM Post-Horizon:** FM-001…045 ✅ (ops LAN BLOCKED; FM-041 Deferred)

## Не повторювати
FM-020…045 · PH-S03…S14 · PH-S07…S14 visual/matrix/topology (S11–S13)

## Закрито PH-S03…S06 (довідка)
| Sprint | Deliverable |
|--------|-------------|
| PH-S03 | `tests/vm_api_contracts.rs`; Playwright VM create/delete |
| PH-S04 | `AppState::raft_node`; `tests/raft_wire_integration.rs`; `cargo test-raft-ci` |
| PH-S05 | `/ui/admin/raid` `#raid-cluster-status` ← `GET /api/v1/raid/status` |
| PH-S06 | `src/network/api/raft_rpc.rs`; `tests/raft_multi_node_harness.rs` (2-node harness) |

## Known (локальний a11y 2026-05-24)
Playwright axe: **13/16 fail** — `color-contrast` на `.btn-primary` (#7e7e7e text / #002200 bg) на admin; login OK.
PH-S14 закрив HC theme CSS, але btn-primary на admin pages ще не проходить.

## Черга — FM §5.1 / §5.9
| # | Фокус | Стан |
|---|--------|------|
| 1 | **a11y HC** — btn-primary contrast admin | **← рекомендовано** |
| 2 | **FM-003** LAN §4 sign-off | **BLOCKED** (2 хости) |
| 3 | **FM-041** Cloud SDK deep | **Deferred** |

## Мета сесії (на вибір)
### A — a11y HC (рекомендовано)
- Виправити `--btn-primary-*` / `.btn-primary` у HC + default admin CSS
- `e2e/tests/a11y.spec.ts` → 16/16 green (або documented exceptions)
- Не ламати PH-S14 HC theme / visual baselines без оновлення snapshots

### B — FM-003 LAN (лише якщо є 2 хости)
- `bin/verify-lan-prep.*`, LAN_BENCHMARK_RUNBOOK §4

### C — FM-041 (лише за явним запитом)
- GCP SA JWT, Azure OAuth refresh

## Перевірки
cargo fmt --all
cargo test-ci                    # K8S_OPENAPI_ENABLED_VERSION=1.28
cargo test-raft-ci               # якщо чіпали raft/
# опційно: cd e2e && npx playwright test tests/a11y.spec.ts

## Git
MSYS2 bash · staging лише sprint files
Не стаджити: data/audit/*, data/dev/, bin/commit-*.sh, .commit-msg-*.txt
commit-tree якщо subject = Co-authored-by:
push + Summary · оновити HANDOFF · NEXT_SESSION

## Доки (після змін)
FUNCTIONALITY_DIGEST · ADMIN_UI_JSON_CONTRACTS (якщо API/UI) · FM §5.1 · HANDOFF · NEXT_SESSION · file_list.csv
```
