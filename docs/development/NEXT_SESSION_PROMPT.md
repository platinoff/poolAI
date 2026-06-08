# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-08 · docs-sync після **PH-S132** · vision manifest **rev 60** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S133** | Job Migrating lifecycle E2E |
| відкрито | PH-S134 | Protocol middleware E2E smoke |
| ✅ | PH-S132 | network_profile contract docs |
| ✅ | PH-S131 | Telegram wallet bind API stub |
| ✅ | PH-S130 | Edge trust_score settlement gate stub |
| ✅ | PH-S129 | Seed inventory + prefetch policy stub |

**Відкритих:** **5** (PH-S133…S134) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S133 (один PH-S*, VDT).

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
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12 — 5 відкритих)
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md
  docs/concept/POOLAI_GALAXY_GRID.md   (§4.3 Migrating lifecycle)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 60, active_sprint → PH-S133, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci   # UI/e2e scope

─── PH-S133 — що зробити ───
1. e2e/ — Playwright PATCH job migrating ↔ executing roundtrip (PH-S104 wire).
2. npm run test:ci green; без змін lease/failover wire.
3. DIGEST § Jobs + FM §5.12 PH-S133 → ✅; HANDOFF; vision revision++.

Не повторювати: PH-S03…S132 (lease, OTel, pricing, locality, prefetch, trust, wallet, network_profile docs).
Vision: .\bin\open-docs-vision.ps1 → http://127.0.0.1:8765/docs/vision/index.html
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S133 — Job Migrating lifecycle E2E |
| **Відкритих у §5.12** | **5** (PH-S133…S134) |
| **Останні закриті** | PH-S132 (network_profile §8.1) · PH-S131 (wallet bind) · PH-S130 (trust gate) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` · **◎ Sprint** + cyan **next** ring |
