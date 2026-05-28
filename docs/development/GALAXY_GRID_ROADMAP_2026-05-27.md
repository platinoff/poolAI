# Galaxy Grid — роадмеп розробки (PoolAI)

**Оновлено:** 2026-05-28 · **HEAD:** PH-S106 (`poolai-worker` lease renew client stub) · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 · **Концепт:** [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md)

Операційний зріз сесій: [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) · старт наступної: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md)

---

## 1. Стан черги §5.12 (2026-05-28)

| Стан | Sprint |
|------|--------|
| **Відкрито** | **PH-S108…S109** (2) — grid ingest, docs sync |
| **Закрито PH-S65…S106** | protocol/register + protocol middleware, verify-release, pricing API/oracle + live provider HTTP fetch, admin UI lease columns + active/expired badge, lease wire + worker renew stub + failover requeue + `Migrating` lifecycle |
| **Поза чергою** | PH-S35/S16/S02 (LAN BLOCKED) · PH-S36/S01/S15 (Cloud SDK Deferred) |

Research replenish ✅ (2026-05-27): 6 нових PH-S* у FM §5.12.

---

## 2. Що реалізовано (карта можливостей)

### 2.1 Pricing (§4.2) — MVP ✅

| Компонент | Де |
|-----------|-----|
| `GET /api/v1/grid/pricing` | `src/network/api/grid.rs` |
| Oracle L1/L2/L3, metrics | `src/grid/galaxy_pricing_oracle.rs` |
| Admin read-only | `/ui/admin/grid-pricing` |
| E2E | `e2e/tests/grid_pricing.spec.ts` |

Env: `POOLAI_GALAXY_PRICE_*`, `POOLAI_GALAXY_PRICING_FALLBACK_JSON`, `POOLAI_GALAXY_PRICING_FORCE_FALLBACK`, `POOLAI_GALAXY_PRICING_PROVIDERS`.

### 2.2 Governance (§9) — ops MVP ✅

| Компонент | Де |
|-----------|-----|
| `poolai-verify-release` | `src/bin/poolai_verify_release.rs` |
| Protocol compat | `src/grid/protocol_compat.rs`, register-remote |
| Docs hub | `docs/security/SECURITY_HARDENING.md` |
| Admin read-only | `/ui/admin/updates-compat` |

### 2.3 Job lease (§4.3.1) — wire stub 🟡

| Компонент | Спринт | Де |
|-----------|--------|-----|
| Поля `lease_*` на `JobRecord` | PH-S94 | `src/job/types.rs`, POST/GET jobs |
| PATCH CAS `lease_epoch` | PH-S95 | `PATCH /api/v1/jobs/{id}` → `409 lease_epoch_rejected` |
| Admin колонки | PH-S96 | `/ui/admin/jobs` |
| TTL env | PH-S97 ✅ | `POOLAI_JOB_LEASE_TTL_SECS` → `src/job/lease_config.rs` |
| Acquire | PH-S98 ✅ | scheduler + `POST /jobs/{id}/lease` → `src/job/lease_acquire.rs` |

| Renew | PH-S99 ✅ | `POST /jobs/{id}/lease/renew` |
| `JobStatus::Leased` | PH-S100 ✅ | `allows_transition`, acquire/schedule → `leased` |
| Failover requeue stub | PH-S101 ✅ | expired `leased` → `submitted` → scheduler rebind |
| Live provider HTTP fetch | PH-S102 ✅ | endpoint pull from provider catalog + timeout env |
| `X-PoolAI-Protocol` middleware | PH-S103 ✅ | selected routes negotiation, protocol headers, unsupported reject |
| `JobStatus::Migrating` | PH-S104 ✅ | lifecycle transitions `Leased/Executing ↔ Migrating`; OpenAPI + contract tests |
| Admin lease active/expired badge | PH-S105 ✅ | `/ui/admin/jobs` lease-state badge from `lease_expires_at`; i18n + Playwright smoke updates |
| Worker lease renew client stub | PH-S106 ✅ | `poolai-worker` calls `/api/v1/jobs/{id}/lease/renew` on task payload `job_id+lease_epoch` |

| E2E lease acquire+renew | PH-S107 ✅ | `e2e/tests/jobs_lease.spec.ts` |

**Ще не в коді:** grid ingest → `leased` (PH-S108); §4.3 docs sync (PH-S109).

---

## 3. Черга §5.12 (2 відкритих)

| # | Sprint | Тема | Джерело |
|---|--------|------|---------|
| 1 | **PH-S108** | Grid ingest → Leased | §4.3 |
| 2 | PH-S109 | §4.3 wire docs sync | docs |

---

## 4. Локальний CI (канон)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
cargo fmt --all && cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після API
cd e2e && npm run test:ci                  # після src/ui/
```

---

## 5. Діаграма фаз

```mermaid
flowchart LR
  A[S55-S64 concept] --> B[S65-S77 wire security]
  B --> C[S78-S92 pricing]
  C --> D[S93-S96 governance UI + lease stub]
  D --> E[S97 TTL env]
  E --> F[S98-S103 lease pricing protocol]
  F --> G[lease acquire renew failover]
```

---

## 6. Пов’язані документи

| Документ | Роль |
|----------|------|
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 | Таблиця PH-S* |
| [`FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) | Витяг модулів |
| [`docs/vision/index.html`](../vision/index.html) | Візуальна карта (localhost:8765) |
