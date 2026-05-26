# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-26 · **HEAD** `cce70bbe` (після PH-S48 `d9f2d0f1`) · **VDT** [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc)

---

```
PoolAI — PH-S49 research + поповнення §5.11

## Ролі (VDT)
- Людина: власник / креативний директор — пріоритети, BLOCKED/Deferred
- Ти: оркестратор Rust — один PH-S*, субагенти для explore/shell/модуль
- Правила: virtual-development-team.mdc · poolai-session-iteration.mdc · runtime-stack-policy.mdc

## S0 (MSYS2 UCRT64 bash)
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch; git status -sb; git log -1 --oneline
df -h /s | tail -1
HANDOFF · FM §5.11 · DOCS_LEGACY_AUDIT

## Локальний CI (канон)
cargo fmt --all
cargo test-ci
# за змін API: cargo run --bin poolai-openapi-gap-audit  # 0 errors

## Стан
- **PH-S03…S48, PH-S41, PH-S46, PH-S37/39/44/42/43/45/38/40:** ✅
- **Черга §5.11:** порожня → ця сесія поповнює (research)
- **BLOCKED:** PH-S35/S16 LAN · **Deferred:** PH-S36/S01 Cloud SDK (FM-041)

## PH-S49 — рекомендована наступна сесія (research)
1. **Research (обов’язково):** `rg "\- \[ \]" docs/development/NEXT_STEPS_ARCHITECT_*.md`; [`DOCS_LEGACY_AUDIT_2026-05-19.md`](./DOCS_LEGACY_AUDIT_2026-05-19.md); `rg "TODO|FIXME" src/` — прив’язка до FM/PH-S*
2. **Додати в FM §5.11** до 10 відкритих PH-S* (кандидати нижче)
3. **Якщо залишиться час — код PH-S49a:** ops/docs для RAID job store:
   - `HANDOFF` §2a: `POOLAI_JOB_STORE=raid`, `POOLAI_RAID_BASE_PATH`
   - `RUN_LOCAL.md` / README — приклад запуску coordinator з RAID snapshot jobs
   - (опційно) згадка в OpenAPI/README Job API persistence

## Завершення сесії
1. Оновити FM §5.11 (+ §5.1 якщо нові FM) + HANDOFF
2. Оновити цей NEXT_SESSION_PROMPT (наступний PH-S*)
3. git push (MSYS2) + самарі

## Не повторювати
PH-S03…S48 · PH-S41 macvlan · PH-S46 Solana · job RAID store implementation · повний test-ci --verbose без змін коду

## Черга §5.11 (кандидати після research — пріоритезувати в FM)
| # | Sprint | Фокус | Джерело |
|---|--------|--------|---------|
| 1 | **PH-S49** | Job store RAID ops/docs (`POOLAI_JOB_STORE=raid`) | PH-S48 follow-up, HANDOFF |
| 2 | **PH-S50** | OpenAPI / docs gap closure (залишки з OPENAPI_GAP) | DOCS_LEGACY §2 |
| 3 | **PH-S51** | VM Linux isolation hardening (cgroup/netns edge cases) | `vm/isolation/linux.rs` |
| 4 | **PH-S52** | E2E: job lifecycle + RAID stand smoke | `e2e/`, FM-020 |
| 5 | **PH-S53** | Admin UX: jobs panel / store backend hint | UI_UX, DIGEST §Job |

**Поза чергою:** PH-S35 LAN · PH-S36 Cloud SDK
```
