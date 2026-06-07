# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-29 · зріз після **PH-S126** + FM replenish · vision manifest **rev 53**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S127 (один PH-S*, VDT).

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
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12 — 10 відкритих)
  docs/development/PROMETHEUS_METRICS.md
  docs/concept/POOLAI_GALAXY_GRID.md   (§4.2 metrics)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 53, active_sprint → PH-S127)

─── Локальний CI (після prometheus scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci

─── Черга §5.12: 10 відкритих (FM replenish 2026-05-29) ───
  PH-S127  Pricing oracle Prometheus export (code)        ← наступний
  PH-S128  Locality score scheduler stub
  PH-S129  Seed inventory + prefetch policy stub
  PH-S130  Edge trust_score settlement gate stub
  PH-S131  Telegram wallet bind API stub
  PH-S132  network_profile contract docs
  PH-S133  Job Migrating lifecycle E2E
  PH-S134  Protocol middleware E2E smoke

Не повторювати (закрито): PH-S03…S126, PH-S124 OTel lease docs, PH-S126 OTel lease spans.
BLOCKED: PH-S35/S16/S02 (LAN) · Deferred: PH-S36/S01/S15 (Cloud SDK).

─── PH-S127 — що зробити ───
1. galaxy_pricing_oracle + prometheus_export: export galaxy_pricing_*_served + forced_fallback_total on GET /metrics.
2. Unit test for metric names/values; cargo test-ci (prometheus feature).
3. FM §5.12 PH-S127 → ✅; HANDOFF; vision manifest revision++.

Vision: rev 53 · comitmsg/ для commit-msg чернеток.
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S127 — Pricing oracle Prometheus export |
| **Відкритих у §5.12** | **10** (PH-S127…S134) |
| **Останні закриті** | PH-S126 (OTel lease span instrumentation) · PH-S124 (OTel lease span attrs docs) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
