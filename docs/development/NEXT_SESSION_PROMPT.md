# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S134 ✅ · vision manifest **rev 63** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S135** | Telegram wallet GET lookup API |
| відкрито | PH-S136 | Prefetch policy env wire stub |
| відкрито | PH-S137 | Trust gate settlement metrics stub |
| відкрито | PH-S138 | Locality rank integration test |
| відкрито | PH-S139 | Telegram wallet bind E2E |
| відкрито | PH-S140 | network_profile register-remote stub |
| відкрито | PH-S141 | Admin jobs migrating badge UI |
| відкрито | PH-S142 | Verification sample rate env stub |
| ✅ | PH-S134 | Protocol middleware E2E smoke |
| ✅ | PH-S133 | Job Migrating lifecycle E2E |

**Відкритих:** **8** (PH-S135…S142) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S135 (один PH-S*, VDT).

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
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md
  docs/concept/POOLAI_GALAXY_GRID.md   (§3.2 Telegram wallet)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 63, active_sprint → PH-S135, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # API scope
cd e2e && npm run test:ci   # e2e scope

─── PH-S135 — що зробити ───
1. `GET /api/v1/virtual-nodes/telegram/wallets/{telegram_user_id}` — lookup stub (PH-S131 wire).
2. OpenAPI sync; integration test; `poolai-openapi-gap-audit` 0.
3. DIGEST § Telegram + FM §5.12 PH-S135 → ✅; HANDOFF; vision revision++.

Не повторювати: PH-S03…S134 (lease, OTel, pricing, locality, prefetch, trust, wallet POST, network_profile docs, migrating/protocol E2E).
Vision: .\bin\open-docs-vision.ps1 → http://127.0.0.1:8765/docs/vision/index.html
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S135 — Telegram wallet GET lookup API |
| **Відкритих у §5.12** | **8** (PH-S135…S142) |
| **Останні закриті** | PH-S134 (protocol E2E) · PH-S133 (migrating E2E) · PH-S132 (network_profile §8.1) |
| **Research replenish** | 2026-06-08 — stubs/tests S136…S142 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Роадмеп** | [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
