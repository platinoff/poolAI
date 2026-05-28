# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · зріз після **PH-S121** · HEAD `803ffaba` · vision manifest **rev 45**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S122 (один PH-S*, VDT).

Перед кодом — правила:
  poolai-agent-roles.mdc · poolai-session-iteration.mdc · virtual-development-team.mdc
  docs-vision.mdc (якщо чіпаємо docs/vision/)

─── S0 (MSYS2 UCRT64, не термінал Cursor) ───
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch && git status -sb && git log -1 --oneline

Прочитай коротко:
  docs/development/HANDOFF_NEW_SESSION.md
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 45, active_sprint → PH-S122 після закриття S121)

─── Локальний CI (після змін у openapi / API) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cargo run --bin poolai-openapi-gap-audit    # очікуємо Total missing: 0

─── Черга §5.12: 4 відкритих ───
  PH-S122  OpenAPI jobs/grid lease schemas (gap audit 0)
  PH-S123  Grid pricing E2E negative fallback
  PH-S124  OTel lease span attrs (FM-038)

Не повторювати (закрито): PH-S03…S121, vision PH-S113…S115 (UI rev 49).
BLOCKED: PH-S35/S16/S02 (LAN) · Deferred: PH-S36/S01/S15 (Cloud SDK).

─── PH-S122 — що зробити ───
1. docs/openapi.yaml — lease_epoch на grid Result body; приклади jobs lease acquire/renew/409.
2. cargo run --bin poolai-openapi-gap-audit → 0.
3. FM §5.12 PH-S122 → ✅; HANDOFF; vision manifest revision++.

Концепт (для контексту, не дублювати код):
  POOLAI_GALAXY_GRID.md §4.3.1 + §4.3.1.1 (worker lease heartbeat)
  DIGEST § Solana + § Job lease / worker ticker

Vision: rev 45 · git HEAD pill у шапці · comitmsg/ для commit-msg чернеток.
Після pull: .\bin\open-docs-vision.ps1 + Ctrl+Shift+R.

Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S122 — OpenAPI lease schemas + gap audit 0 |
| **Відкритих у §5.12** | 4 (PH-S122…S124) |
| **Останні закриті** | PH-S118…S121 (lease E2E, admin UI, Solana vision, §4.3.1.1 docs) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) §4.3 |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
