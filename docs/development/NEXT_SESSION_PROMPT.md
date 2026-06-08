# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-06-08 · docs-sync після **PH-S131** · vision manifest **rev 59** · UI **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Спринти §5.12 (зріз)

| Стан | Sprint | Тема |
|------|--------|------|
| **← наступний** | **PH-S132** | network_profile contract docs |
| відкрито | PH-S133 | Job Migrating lifecycle E2E |
| відкрито | PH-S134 | Protocol middleware E2E smoke |
| ✅ | PH-S131 | Telegram wallet bind API stub |
| ✅ | PH-S130 | Edge trust_score settlement gate stub |
| ✅ | PH-S129 | Seed inventory + prefetch policy stub |
| ✅ | PH-S128 | Locality score scheduler stub |

**Відкритих:** **6** (PH-S132…S134) · **BLOCKED:** PH-S02/S16/S35 (LAN) · **Deferred:** PH-S01/S15/S36 (Cloud SDK)

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S132 (один PH-S*, VDT).

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
  docs/catalog/FUNCTION_MANAGEMENT.md  (§5.12 — 6 відкритих)
  docs/catalog/FUNCTIONALITY_DIGEST_2026-04-06.md  (Galaxy modules + observability)
  docs/concept/POOLAI_GALAXY_GRID.md   (§8 network_profile)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 59, active_sprint → PH-S132, cyan ring = next scope)

─── Локальний CI (після code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci

─── PH-S132 — що зробити ───
1. docs/concept/POOLAI_GALAXY_GRID.md — §8.1 schema for network_profile.
2. DIGEST row + locality subset cross-link (PH-S128).
3. FM §5.12 PH-S132 → ✅; HANDOFF; vision revision++.

Не повторювати: PH-S03…S131 (lease, OTel, pricing /metrics, locality, prefetch, trust gate, wallet stub).
Vision: .\bin\open-docs-vision.ps1 → http://127.0.0.1:8765/docs/vision/index.html
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S132 — network_profile contract docs |
| **Відкритих у §5.12** | **6** (PH-S132…S134) |
| **Останні закриті** | PH-S131 (wallet bind stub) · PH-S130 (trust_score gate) · PH-S129 (prefetch stub) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` · **◎ Sprint** + cyan **next** ring |
