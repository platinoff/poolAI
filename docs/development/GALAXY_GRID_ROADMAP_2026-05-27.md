# Galaxy Grid — роадмеп розробки (PoolAI)

**Оновлено:** 2026-06-13 · **HEAD:** pending · **Канон черги:** [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 (**8** відкритих PH-S135…S142)

Операційний зріз сесій: [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) · старт наступної: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md)

---

## 1. Стан черги §5.12 (2026-06-08)

| Стан | Sprint |
|------|--------|
| **Відкрито** | **8** — PH-S135…S142 (replenish 2026-06-08) |
| **Закрито PH-S65…S132** | pricing, governance, protocol, lease wire, locality/prefetch/trust/wallet/network_profile |
| **Смуга PH-S128…S132** | **5/5 ✅** (2026-06-08) |
| **Поза чергою** | PH-S35/S16/S02 (LAN BLOCKED) · PH-S36/S01/S15 (Cloud SDK Deferred) |

Research replenish ✅ (2026-06-08): PH-S135…S142 — code-first stubs/tests (wallet GET, prefetch/trust/verify env, locality integration, network_profile register-remote, admin migrating UI).

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

### 2.3 Job lease (§4.3) — wire MVP ✅

| Компонент | Спринт | Де |
|-----------|--------|-----|
| Поля `lease_*` на `JobRecord` | PH-S94 | `src/job/types.rs`, POST/GET jobs |
| PATCH CAS `lease_epoch` | PH-S95 | `PATCH /api/v1/jobs/{id}` → `409 lease_epoch_rejected` |
| `JobStatus::Migrating` | PH-S104 ✅ | lifecycle + contract test `jobs_patch_migrating_lifecycle_roundtrip` + E2E `jobs_migrating.spec.ts` (PH-S133) |
| Grid result lease CAS | PH-S110 ✅ | `GridResultBody.lease_epoch` |
| E2E lease suite | PH-S107…S118 ✅ | `jobs_lease`, `grid_job_lease`, `grid_result_lease` |

### 2.4 Galaxy stubs (§5–§8) — MVP ✅

| Компонент | Спринт | Де |
|-----------|--------|-----|
| Locality score | PH-S128 | `galaxy_locality.rs` |
| Prefetch policy stub | PH-S129 | `dispatch.rs` |
| Trust gate stub | PH-S130 | `galaxy_trust_score.rs` |
| Wallet bind POST | PH-S131 | `virtual_node_telegram_wallet_service.rs` |
| network_profile §8.1 | PH-S132 | `POOLAI_GALAXY_GRID.md` |

**Post-MVP (черга §5.12):** PH-S135…S142 — stubs/tests (див. §3).

---

## 3. Черга §5.12 (8 відкритих, 2026-06-13)

| # | Sprint | Тема | Джерело | Стан |
|---|--------|------|---------|------|
| 1 | **PH-S135** | Telegram wallet GET lookup API | Galaxy §3.2 | відкрито |
| 2 | **PH-S136** | Prefetch policy env wire stub | Galaxy §5.6 | відкрито |
| 3 | **PH-S137** | Trust gate settlement metrics stub | Galaxy §6.5 | відкрито |
| 4 | **PH-S138** | Locality rank integration test | PH-S128 | відкрито |
| 5 | **PH-S139** | Telegram wallet bind E2E | PH-S131 | відкрито |
| 6 | **PH-S140** | network_profile register-remote stub | Galaxy §8.1 | відкрито |
| 7 | **PH-S141** | Admin jobs migrating badge UI | PH-S104 | відкрито |
| 8 | **PH-S142** | Verification sample rate env stub | Galaxy §6.1 | відкрито |
| — | **PH-S133** | Job Migrating lifecycle E2E | PH-S104 | **✅** |
| — | **PH-S134** | Protocol middleware E2E smoke | PH-S103 | **✅** |

---

## 4. Research горизонт (після S142)

| Джерело | Прогалина |
|---------|-----------|
| Galaxy §8.2 | fee settlement wire · Telegram VM probros MVP |
| Galaxy §5.3 / §6 | `galaxy_shard_*`, `galaxy_verification_*` Prometheus |
| Architect | LAN replication benchmarks — **BLOCKED** (2 хости) |

---

## 5. Локальний CI (канон)

```bash
export PATH="$HOME/.cargo/bin:/ucrt64/bin:/usr/bin:$PATH"
export K8S_OPENAPI_ENABLED_VERSION=1.28
cd /s/rust/poolAI
cargo fmt --all && cargo test-ci
cargo run --bin poolai-openapi-gap-audit   # після API
cd e2e && npm run test:ci                  # після src/ui/ або e2e/
```

---

## 6. Пов’язані документи

| Документ | Роль |
|----------|------|
| [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.12 | Таблиця PH-S* |
| [`FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md) | Витяг модулів |
| [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) | Концепт §8.1 network_profile |
| [`docs/vision/index.html`](../vision/index.html) | Візуальна карта (localhost:8765) |
