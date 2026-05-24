# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-24 · **HEAD:** `462dbc74` · **PH-S03…S14:** ✅ · **PH-S15…S24:** черга legacy

Скопіюй блок нижче в новий чат (Agent mode, MSYS2 bash).

---

```
PoolAI — PH-S17 (перший Planned) або ops FM-003 / FM-041 за запитом.

## S0
1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · §5.9 · poolai-session-iteration.mdc

## Стан репо (2026-05-24)
- **Гілка:** main · **HEAD:** `462dbc74` feat(ui): post-PH a11y axe 16/16
- **PH-S03…S14:** ✅ (post-PH a11y закрито в останньому push)
- **a11y:** `e2e/tests/a11y.spec.ts` — **16/16** (MSYS2 + poolai :8080)
- **FM-001…045:** ✅ (лише FM-041 Deferred, FM-003 §4 BLOCKED)

## Не повторювати
FM-020…045 · PH-S03…S14 · post-PH a11y (admin_styles HC, onclick JSON, waitForAdminAxeReady, topology SVG title)

## Черга PH-S15…S24 (з legacy, не зроблено)
| # | Sprint | Фокус | Стан |
|---|--------|--------|------|
| — | PH-S15 | FM-041 Cloud SDK deep | **Deferred** |
| — | PH-S16 | FM-003 LAN §4 | **BLOCKED** (2 хости) |
| 1 | **PH-S17** | ML pipeline ops (metrics, verify-dev-stand) | **Planned** |
| 2 | **PH-S18** | BurstRAID/SmallWorld admin metrics | **Planned** |
| 3 | **PH-S19** | OpenAPI gap audit у CI | **Planned** |
| 4 | **PH-S20** | VM Windows isolation (AppContainer) | **Planned** |
| 5 | **PH-S21** | Raft membership з log | **Planned** |
| 6 | **PH-S22** | Topology WebSocket live | **Planned** |
| 7 | **PH-S23** | Playwright admin flows expand | **Planned** |
| 8 | **PH-S24** | Security ops (rotation, pen-test doc) | **Planned** |

## Мета сесії (рекомендація)
**PH-S17** — ML pipeline: крокові метрики в runbook, `verify-dev-stand` / health_load зріз; `cargo test-ci`.

Альтернативи: **PH-S19** (openapi-gap-audit CI) · **PH-S23** (Playwright) — без FM-003/FM-041 без інфра/запиту.

## Перевірки
cargo fmt --all
cargo test-ci                    # K8S_OPENAPI_ENABLED_VERSION=1.28
cargo test-raft-ci               # якщо чіпали raft/
# e2e: cd e2e && POOLAI_BASE_URL=http://127.0.0.1:8080 npx playwright test tests/a11y.spec.ts

## Git
MSYS2 bash · staging лише sprint files
Не стаджити: data/audit/*, data/dev/, bin/commit-*.sh, .commit-msg-*.txt
commit-tree якщо subject = Co-authored-by:
push + Summary · оновити HANDOFF · NEXT_SESSION · FM §5.9

## Доки (після змін)
FUNCTIONALITY_DIGEST · ADMIN_A11Y_RUNBOOK (якщо a11y) · FM §5.9 · HANDOFF · NEXT_SESSION
```
