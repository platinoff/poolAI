# Galaxy Grid — роадмеп розробки (PoolAI)

**Оновлено:** 2026-06-14 · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**5** відкритих PH-S166…S170)

Операційний зріз сесій: [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) · старт наступної: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md)

---

## 1. Стан черги §5.12 (2026-06-13)

| Стан | Sprint |
|------|--------|
| **Відкрито** | **5** — PH-S166…S170 |
| **Закрито PH-S128…S165** | Galaxy wire + ratio hold gate + admin Rust slim |
| **Після S170** | replenish §5.12 (≤10) · formal 90–95%, hold **95%**, spirit **96%** |
| **Поза чергою** | PH-S35/S16/S02 (LAN BLOCKED) · PH-S36/S01/S15 (Cloud SDK Deferred) |

---

## 2. Що реалізовано (карта можливостей)

### 2.1 Pricing (§4.2) — MVP ✅

| Компонент | Де |
|-----------|-----|
| `GET /api/v1/grid/pricing` | `src/network/api/grid.rs` |
| Oracle L1/L2/L3, metrics | `src/grid/galaxy_pricing_oracle.rs` |
| Admin read-only | `/ui/admin/grid-pricing` |
| Wire tests (канон) | `tests/` + `cargo test-ci`; legacy Playwright → `e2e/archive/api-smoke/` |

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
| `JobStatus::Migrating` | PH-S104 ✅ | lifecycle + contract test |
| Grid result lease CAS | PH-S110 ✅ | `GridResultBody.lease_epoch` |
| Lease wire tests | PH-S107…S118 ✅ | Rust contracts |

### 2.4 Galaxy stubs (§5–§8) — MVP ✅

| Компонент | Спринт | Де |
|-----------|--------|-----|
| Locality score + rank integration | PH-S128, S138 | `galaxy_locality.rs` |
| Prefetch policy stub + env | PH-S129, S136 | `dispatch.rs` |
| Trust gate + metrics | PH-S130, S137 | `galaxy_trust_score.rs` |
| Verify sampling env | PH-S142 | `galaxy_verify_sampling.rs` |

---

## 3. Черга §5.12 (post-S165 replenish S166…S170)

| # | Sprint | Тема | Ratio / acceptance |
|---|--------|------|-------------------|
| — | **PH-S165** ✅ | **96%** hold gate | CI `--min-ratio 0.95` advisory; target **95%** |
| 1 | **PH-S166** | Design tokens CSS → Rust | slim `design_tokens.css` |
| 2 | **PH-S167** | Galaxy prefetch metrics stub | §5.5 Prometheus |
| 3 | **PH-S168** | Galaxy pricing cache age /metrics | §4.2 gauge |
| 4 | **PH-S169** | Locality stale profile penalty | §8.1 stub |
| 5 | **PH-S170** | Galaxy settlement pending_verification stub | §6.4 grid result |

Повна таблиця — FM **§5.12** · [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md).

**Research горизонт (Galaxy wire):** §8.2 fee settlement — **після** S170 або Rust-only stubs у S167…S169.

---

## 4. Rust ratio (§5.13)

| Фаза | Sprints | Результат |
|------|---------|-----------|
| Audit + dedupe | PH-S143…S145 | baseline + API→Rust |
| Portable UI core | PH-S146…S147 | ui-core + wasm POC |
| Slim browser E2E | PH-S148 | browser-only `test:ci` |
| Gate + stretch | PH-S150 ✅…S159 ✅ | CI stretch warn **93%** → **96% spirit** |
| Maintain + Galaxy wire | PH-S160…S170 | hold **95%** advisory; UI slim + §5–§6 stubs |

**Baseline:** **92.68%** · [`rust_ratio.json`](./rust_ratio.json).

---

## 5. Локальний CI (канон)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
cargo fmt --all && cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після API
cargo run --bin poolai-loc-audit           # ratio gate
bash bin/e2e-playwright.sh --start         # лише src/ui/ або axe/visual scope
```

---

## 6. Пов'язані документи

| Документ | Роль |
|----------|------|
| [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) | концепт v1 |
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 | єдина черга PH-S* |
| [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) | ratio 90–95% + **96% stretch** |
