# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-07 · docs-sync після **PH-S129** · vision manifest **rev 57** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S130** | Edge trust_score settlement gate stub |
| відкрито | PH-S131 | Telegram wallet bind API stub |
| відкрито | PH-S132 | network_profile contract docs |
| відкрито | PH-S133 | Job Migrating lifecycle E2E |
| відкрито | PH-S134 | Protocol middleware E2E smoke |
| ✅ | PH-S129 | Seed inventory + prefetch policy stub |
| ✅ | PH-S128 | Locality score scheduler stub |
| ✅ | PH-S127 | Pricing oracle Prometheus export |
| ✅ | PH-S126 | OTel lease span instrumentation |

**Відкритих:** **8** (PH-S130…S134) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S130 (один PH-S*, VDT).

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
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12 — 8 відкритих)
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md  (Galaxy modules + observability)
  docs/concept/POOLAI_GALAXY_GRID.md   (§6.5 trust_score)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 57, active_sprint → PH-S130, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci

─── PH-S130 — що зробити ───
1. src/grid/ — trust_score 0–100 gate sketch on grid result path; unit tests.
2. Без settlement wire; gate stub only.
3. DIGEST § Galaxy + FM §5.12 PH-S130 → ✅; HANDOFF; vision revision++.

Не повторювати: PH-S03…S129 (lease, OTel, pricing /metrics, locality, prefetch stub).
Vision: .\bin\open-docs-vision.ps1 → http://127.0.0.1:8765/docs/vision/index.html
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S130 — Edge trust_score settlement gate stub |
| **Відкритих у §5.12** | **8** (PH-S130…S134) |
| **Останні закриті** | PH-S129 (SeedInventory + prefetch stub) · PH-S128 (locality_score) · PH-S127 (pricing `/metrics`) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` · **◎ Sprint** + cyan **next** ring |
