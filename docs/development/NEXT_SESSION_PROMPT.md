# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-28 · зріз після **PH-S122** · vision manifest **rev 46**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S123 (один PH-S*, VDT).

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
  docs/vision/  (manifest rev 46, active_sprint → PH-S123)

─── Локальний CI (після змін у e2e / grid pricing) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci    # після e2e scope

─── Черга §5.12: 3 відкритих ───
  PH-S123  Grid pricing E2E negative fallback
  PH-S124  OTel lease span attrs (FM-038)

Не повторювати (закрито): PH-S03…S122, vision PH-S113…S115 (UI rev 49).
BLOCKED: PH-S35/S16/S02 (LAN) · Deferred: PH-S36/S01/S15 (Cloud SDK).

─── PH-S123 — що зробити ───
1. e2e/tests/grid_pricing.spec.ts — force fallback env → stable quote snapshot.
2. `cd e2e && npm run test:ci` (з `--start` stand).
3. FM §5.12 PH-S123 → ✅; HANDOFF; vision manifest revision++.

Контекст:
  src/grid/galaxy_pricing_oracle.rs · GET /api/v1/grid/pricing
  PH-S102 live fetch · PH-S111 pricing E2E positives

Vision: rev 46 · git HEAD pill у шапці · comitmsg/ для commit-msg чернеток.
Після pull: .\bin\open-docs-vision.ps1 + Ctrl+Shift+R.

Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S123 — Grid pricing E2E negative fallback |
| **Відкритих у §5.12** | 3 (PH-S123…S124) |
| **Останні закриті** | PH-S122 (OpenAPI lease schemas + gap audit 0) · PH-S118…S121 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) §4.3 |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
