# Galaxy Grid — роадмеп розробки (PoolAI)

**Оновлено:** 2026-06-13 · **HEAD:** pending · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**10** відкритих PH-S143…S150)

Операційний зріз сесій: [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) · старт наступної: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md)

---

## 1. Стан черги §5.12 (2026-06-13)

| Стан | Sprint |
|------|--------|
| **Відкрито** | **10** — PH-S143…S150 (Rust ratio band) |
| **Закрито PH-S128…S142** | Galaxy wire complete (locality, trust, verify env, wallet, migrating UI, network_profile) |
| **Після S150** | replenish §5.12 (≤10) · ratio target **90–95%** |
| **Поза чергою** | PH-S35/S16/S02 (LAN BLOCKED) · PH-S36/S01/S15 (Cloud SDK Deferred) |

---

## 2. Що реалізовано (карта можливостей)

### 2.1 Pricing (§4.2) — MVP ✅

| Компонент | Де |
|-----------|-----|
| `GET /api/v1/grid/pricing` | `src/network/api/grid.rs` |
| Oracle L1/L2/L3, metrics | `src/grid/galaxy_pricing_oracle.rs` |
| Admin read-only | `/ui/admin/grid-pricing` |
| Wire tests (канон) | `tests/` + `cargo test-ci`; legacy `e2e/tests/grid_pricing.spec.ts` → §5.13 PH-S144 |

Env: `POOLAI_GALAXY_PRICE_*`, `POOLAI_GALAXY_PRICING_FALLBACK_JSON`, `POOLAI_GALAXY_PRICING_FORCE_FALLBACK`, `POOLAI_GALAXY_PRICING_PROVIDERS`.

### 2.2 Governance (§9) — ops MVP ✅

| Компонент | Де |
|-----------|-----|
| `poolai-verify-release` | `src/bin/poolai_verify_release.rs` |
| Protocol compat | `src/grid/protocol_compat.rs`, register-remote |
| Docs hub | `docs/security/SECURITY_HARDENING.md` |
| Admin read-only | `/ui/admin/updates-compat` |

### 2.3 Job lease (§4.3) — wire MVP ✅

| Компонент | Спринт | Де |
|-----------|--------|-----|
| Поля `lease_*` на `JobRecord` | PH-S94 | `src/job/types.rs`, POST/GET jobs |
| PATCH CAS `lease_epoch` | PH-S95 | `PATCH /api/v1/jobs/{id}` → `409 lease_epoch_rejected` |
| `JobStatus::Migrating` | PH-S104 ✅ | lifecycle + contract test + legacy E2E PH-S133 |
| Grid result lease CAS | PH-S110 ✅ | `GridResultBody.lease_epoch` |
| Lease wire tests | PH-S107…S118 ✅ | Rust contracts + legacy Playwright → §5.13 |

### 2.4 Galaxy stubs (§5–§8) — MVP ✅

| Компонент | Спринт | Де |
|-----------|--------|-----|
| Locality score + rank integration | PH-S128, S138 | `galaxy_locality.rs`, `tests/galaxy_locality_rank_integration.rs` |
| Prefetch policy stub + env | PH-S129, S136 | `dispatch.rs` |
| Trust gate + metrics | PH-S130, S137 | `galaxy_trust_score.rs` |
| Verify sampling env | PH-S142 | `galaxy_verify_sampling.rs` |

**Відкрито §5.12:** PH-S143…S150 Rust ratio band (див. FM §5.12).

---

## 3. Черга §5.12 (Galaxy wire закрито; ratio band)

| # | Sprint | Тема | Acceptance | Стан |
|---|--------|------|------------|------|
| — | **PH-S143…S150** | Rust ratio 90–95% | див. FM §5.12 + [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) | відкрито |

---

## 4. Rust ratio 90–95% (§5.13, після S142)

| Фаза | Sprints | Результат |
|------|---------|-----------|
| Audit baseline | PH-S143 | ratio report |
| Dedupe API tests | PH-S144, S145 | Playwright API → Rust; `poolai-http-stand-smoke` |
| Portable UI core | PH-S146, S147 | shared Rust crate + wasm32 POC |
| Slim browser E2E | PH-S148, S150 | `e2e/` UI-only; CI ratio gate |

Повна таблиця — FM **§5.13** · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).

**Research горизонт (Galaxy wire):** §8.2 settlement · §5.3/§6 Prometheus — **після** ratio фази B або паралельно як Rust-only stubs.

---

## 5. Локальний CI (канон)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
cargo fmt --all && cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після API
bash bin/e2e-playwright.sh --start         # лише src/ui/ або axe/visual scope
```

---

## 6. Пов’язані документи

| Документ | Роль |
|----------|------|
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12, §5.13 | Таблиця PH-S* |
| [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) | 90–95% + portability |
| [`FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) | Витяг модулів |
| [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) | Концепт |
| [`docs/vision/index.html`](../vision/index.html) | Візуальна карта |
