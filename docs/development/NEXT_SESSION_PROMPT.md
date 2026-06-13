# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-08 · research replenish · vision manifest **rev 61** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S133** | Job Migrating lifecycle E2E |
| відкрито | PH-S134 | Protocol middleware E2E smoke |
| відкрито | PH-S135 | Telegram wallet GET lookup API |
| відкрито | PH-S136 | Prefetch policy env wire stub |
| відкрито | PH-S137 | Trust gate settlement metrics stub |
| відкрито | PH-S138 | Locality rank integration test |
| відкрито | PH-S139 | Telegram wallet bind E2E |
| відкрито | PH-S140 | network_profile register-remote stub |
| відкрито | PH-S141 | Admin jobs migrating badge UI |
| відкрито | PH-S142 | Verification sample rate env stub |
| ✅ | PH-S132 | network_profile contract docs |

**Відкритих:** **10** (PH-S133…S142) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

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
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12 — 10 відкритих)
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md
  docs/concept/POOLAI_GALAXY_GRID.md   (§4.3 Migrating lifecycle)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 61, active_sprint → PH-S133, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci   # e2e scope

─── PH-S133 — що зробити ───
1. e2e/ — Playwright PATCH job migrating ↔ executing roundtrip (PH-S104 wire; contract test уже в jobs_api_contracts).
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
| **Відкритих у §5.12** | **10** (PH-S133…S142) |
| **Останні закриті** | PH-S132 (network_profile §8.1) · PH-S131 (wallet) · PH-S130 (trust gate) |
| **Research replenish** | 2026-06-08 — E2E + stubs/tests S135…S142 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Роадмеп** | [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
