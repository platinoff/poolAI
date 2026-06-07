# Промпт наступної сесії (PoolAI)

**Оновлено:** 2026-05-29 · зріз після **PH-S124** + FM replenish · vision manifest **rev 52**

Один спринт за раз — канон у [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12.

---

## Copy-paste для агента (наступна сесія)

```
Привіт! Продовжуємо PoolAI ітераційно — спринт PH-S126 (один PH-S*, VDT).

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
  docs/development/OPENTELEMETRY_TRACING.md  (§ Job lease spans — PH-S124 contract)
  docs/concept/POOLAI_GALAXY_GRID.md   (§4.3 lease)
  docs/development/GALAXY_GRID_ROADMAP_2026-05-27.md
  docs/development/NEXT_SESSION_PROMPT.md
  docs/vision/  (manifest rev 52, active_sprint → PH-S126)

─── Локальний CI (після otel code scope) ───
cargo fmt --all
K8S_OPENAPI_ENABLED_VERSION=1.28 cargo test-ci
cargo test --test observability_otel --features otel

─── Черга §5.12: 10 відкритих (FM replenish 2026-05-29) ───
  PH-S126  OTel lease span instrumentation (code)         ← наступний
  PH-S127  Pricing oracle Prometheus export
  PH-S128  Locality score scheduler stub
  PH-S129  Seed inventory + prefetch policy stub
  PH-S130  Edge trust_score settlement gate stub
  PH-S131  Telegram wallet bind API stub
  PH-S132  network_profile contract docs
  PH-S133  Job Migrating lifecycle E2E
  PH-S134  Protocol middleware E2E smoke

Не повторювати (закрито): PH-S03…S125, PH-S123 grid pricing E2E, PH-S124 OTel lease span attrs docs.
BLOCKED: PH-S35/S16/S02 (LAN) · Deferred: PH-S36/S01/S15 (Cloud SDK).

─── PH-S126 — що зробити ───
1. src/observability/lease_trace.rs — span builders per OPENTELEMETRY_TRACING.md § Job lease spans.
2. Wire spans: acquire/renew/reject in src/job/, jobs API, grid dispatch (feature otel).
3. tests/observability_otel.rs — assert span names/attrs on acquire + lease_epoch_rejected.
4. FM §5.12 PH-S126 → ✅; HANDOFF; vision manifest revision++.

Контекст FM-менеджер (gap analysis):
  Galaxy §4.3 lease MVP ✅ · §4.2 pricing MVP ✅ · §9 governance MVP ✅
  Gaps → PH-S127…S134: pricing /metrics, §5 locality/prefetch stubs,
  §6 trust_score, §3.2 wallet API, §8 network_profile docs, E2E migrating + protocol.

Vision: rev 52 · comitmsg/ для commit-msg чернеток.
Git: не git add -A; не data/audit/, comitmsg/*.txt, bin/commit-*.sh.
Push — лише зовнішній MSYS2 за git-push.md.
```

---

## Короткий зріз

| | |
|--|--|
| **Наступний спринт** | PH-S126 — OTel lease span instrumentation |
| **Відкритих у §5.12** | **10** (PH-S126…S134) |
| **Останні закриті** | PH-S124 (OTel lease span attrs docs) · PH-S123 (grid pricing force-fallback E2E) |
| **Handoff** | [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) |
| **FM** | [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) |
| **Концепт Galaxy** | [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) |
| **Vision map** | [`docs/vision/`](../vision/) · `.\bin\open-docs-vision.ps1` |
