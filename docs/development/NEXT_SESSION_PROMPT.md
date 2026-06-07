# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-29 · зріз після **PH-S127** + FM replenish · vision manifest **rev 54**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S128 (один PH-S*, VDT).

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
  docs/concept/POOLAI_GALAXY_GRID.md   (§5.1–5.2 locality)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 54, active_sprint → PH-S128)

─── Локальний CI ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci

─── Черга §5.12: 10 відкритих (FM replenish 2026-05-29) ───
  PH-S128  Locality score scheduler stub (code)          ← наступний
  PH-S129  Seed inventory + prefetch policy stub
  PH-S130  Edge trust_score settlement gate stub
  PH-S131  Telegram wallet bind API stub
  PH-S132  network_profile contract docs
  PH-S133  Job Migrating lifecycle E2E
  PH-S134  Protocol middleware E2E smoke

Не повторювати (закрито): PH-S03…S127, PH-S126 OTel lease spans, PH-S127 pricing /metrics.
BLOCKED: PH-S35/S16/S02 (LAN) · Deferred: PH-S36/S01/S15 (Cloud SDK).

─── PH-S128 — що зробити ───
1. src/grid/ — locality_score(worker, task) pure fn + unit tests (Galaxy §5.1–5.2).
2. No prefetch wire; scheduler stub only.
3. FM §5.12 PH-S128 → ✅; HANDOFF; vision manifest revision++.

Vision: rev 54 · comitmsg/ для commit-msg чернеток.
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S128 — Locality score scheduler stub |
| **Відкритих у §5.12** | **10** (PH-S128…S134) |
| **Останні закриті** | PH-S127 (pricing oracle Prometheus export) · PH-S126 (OTel lease spans) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
