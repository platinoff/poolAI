# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-13 · PH-S133 ✅ · vision manifest **rev 62** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S134** | Protocol middleware E2E smoke |
| відкрито | PH-S135 | Telegram wallet GET lookup API |
| відкрито | PH-S136 | Prefetch policy env wire stub |
| відкрито | PH-S137 | Trust gate settlement metrics stub |
| відкрито | PH-S138 | Locality rank integration test |
| відкрито | PH-S139 | Telegram wallet bind E2E |
| відкрито | PH-S140 | network_profile register-remote stub |
| відкрито | PH-S141 | Admin jobs migrating badge UI |
| відкрито | PH-S142 | Verification sample rate env stub |
| ✅ | PH-S133 | Job Migrating lifecycle E2E |

**Відкритих:** **9** (PH-S134…S142) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S134 (один PH-S*, VDT).

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
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md
  docs/concept/POOLAI_GALAXY_GRID.md   (§9.3 protocol compat)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 62, active_sprint → PH-S134, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cd e2e && npm run test:ci   # e2e scope

─── PH-S134 — що зробити ───
1. e2e/ — Playwright register-remote with `X-PoolAI-Protocol`; unsupported version → 403 (PH-S103 wire).
2. npm run test:ci green; без змін compat matrix wire.
3. DIGEST § Galaxy + FM §5.12 PH-S134 → ✅; HANDOFF; vision revision++.

Не повторювати: PH-S03…S133 (lease, OTel, pricing, locality, prefetch, trust, wallet, network_profile docs, migrating E2E).
Vision: .\bin\open-docs-vision.ps1 → http://127.0.0.1:8765/docs/vision/index.html
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S134 — Protocol middleware E2E smoke |
| **Відкритих у §5.12** | **9** (PH-S134…S142) |
| **Останні закриті** | PH-S133 (migrating E2E) · PH-S132 (network_profile §8.1) · PH-S131 (wallet) |
| **Research replenish** | 2026-06-08 — E2E + stubs/tests S135…S142 |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Роадмеп** | [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
