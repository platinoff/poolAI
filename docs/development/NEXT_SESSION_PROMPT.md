# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **HEAD (origin/main):** `0c1c62c5` · **Наступна:** PH-S06

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S06 multi-node Raft harness (single-host simulation).

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

Не повторювати: FM-020…045; PH-S03…S05; PH-S07…S14; PH-S11…S13 visual/matrix/topology.

## Закрито (PH-S05)
- `/ui/admin/raid` — `#raid-cluster-status`: `GET /api/v1/raid/status` → cluster + raft_status
- `admin_ui_api_contracts` + Playwright `admin.spec.ts` (PH-S05)
- i18n EN/UK для raft/cluster labels

## Known (локальний a11y 2026-05-24)
Playwright a11y: 13/16 fail — color-contrast btn-primary на admin. Варто зняти перед UI push.

## Черга PH (5 відкритих) — FM §5.9
PH-S01 Deferred (FM-041) · PH-S02 BLOCKED (FM-003 LAN) · PH-S06 multi-node harness ← наступна

## Мета — PH-S06
Multi-node Raft harness (single-host simulation); archive WEEK12, FM-027 prep.

Альтернативи: a11y HC contrast; PH-S02 коли 2 хости.

## Завершення
cargo fmt --all → cargo test-ci → cargo test-raft-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише спринта, commit-tree якщо subject = Co-authored-by:
push + Summary · HANDOFF · NEXT_SESSION → наступний PH з §5.9
```
