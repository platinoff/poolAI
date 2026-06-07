# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-07 · docs-sync після **PH-S128** · vision manifest **rev 56** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S129** | Seed inventory + prefetch policy stub |
| відкрито | PH-S130 | Edge trust_score settlement gate stub |
| відкрито | PH-S131 | Telegram wallet bind API stub |
| відкрито | PH-S132 | network_profile contract docs |
| відкрито | PH-S133 | Job Migrating lifecycle E2E |
| відкрито | PH-S134 | Protocol middleware E2E smoke |
| ✅ | PH-S128 | Locality score scheduler stub |
| ✅ | PH-S127 | Pricing oracle Prometheus export |
| ✅ | PH-S126 | OTel lease span instrumentation |
| ✅ | PH-S124 | OTel lease span attrs docs |

**Відкритих:** **9** (PH-S129…S134) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S129 (один PH-S*, VDT).

Перед кодом — правила:
  poolai-agent-roles.mdc · poolai-session-iteration.mdc · virtual-development-team.mdc
  docs-vision.mdc (після змін docs/vision/)

─── S0 (MSYS2 UCRT64, не термінал Cursor) ───
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
git fetch && git status -sb && git log -1 --oneline

Прочитай коротко:
  docs/development/HANDOFF_NEW_SESSION.md
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12 — 9 відкритих)
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md  (Galaxy modules + observability)
  docs/concept/POOLAI_GALAXY_GRID.md   (§5.5 prefetch)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 56, active_sprint → PH-S129, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci

─── PH-S129 — що зробити ───
1. src/grid/dispatch.rs — `SeedInventoryEntry` DTO + noop prefetch hook; unit tests.
2. Без live prefetch wire; policy stub only.
3. DIGEST § Galaxy + FM §5.12 PH-S129 → ✅; HANDOFF; vision revision++.

Не повторювати: PH-S03…S128 (lease, OTel, pricing /metrics, locality_score stub).
Vision: .\bin\open-docs-vision.ps1 → http://127.0.0.1:8765/docs/vision/index.html
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S129 — Seed inventory + prefetch policy stub |
| **Відкритих у §5.12** | **9** (PH-S129…S134) |
| **Останні закриті** | PH-S128 (locality_score stub) · PH-S127 (pricing `/metrics`) · PH-S126 (OTel lease spans) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` · **◎ Sprint** + cyan **next** ring |
