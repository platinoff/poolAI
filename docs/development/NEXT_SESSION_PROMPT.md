# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-23 · **Фаза:** Post-Horizon **FM-020…042 ✅** · §5.1 code queue порожня (лише Deferred / BLOCKED)

Скопіюй блок нижче в новий чат Cursor (Agent mode, MSYS2 bash для git/cargo).

---

```
PoolAI — наступна сесія після FM-042 (hot-path Criterion закрито).

## S0 — зріз

1. MSYS2 bash: git fetch; git status -sb; git log -1 --oneline
2. df -h /s (якщо Avail <5G → cargo clean перед test-ci)
3. HANDOFF_NEW_SESSION.md · FUNCTION_MANAGEMENT.md §5.1 · poolai-session-iteration.mdc

Не повторювати: FM-020…042; FM-038 OTel; FM-039 Playwright CI.

## Мета сесії (обрати одну)

| Пріоритет | FM | Фокус | Примітка |
|-----------|-----|--------|----------|
| Deferred | **FM-041** | Cloud SDK deep (GCP SA JWT, Azure OAuth) | лише за явним запитом |
| Ops BLOCKED | **FM-003** | LAN §4 sign-off | 2 фізичні хости — verify-lan-prep / runbook |

Або maintenance: docs sync, OpenAPI gap audit, невеликі багфікси.

## Завершення

src/ → cargo fmt --all → cargo test-ci (K8S_OPENAPI_ENABLED_VERSION=1.28)
git: MSYS2, staging лише поточного FM, GIT_EDITOR=true, git log -1 перевірка subject
push + Summary у коміті (git-push.md) · оновити HANDOFF + §5.1
```
