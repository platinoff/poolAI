# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-06-13 (PH-S144 ✅ · §5.12 **9** відкритих PH-S145…S150 · rust_ratio **91.91%**) · VDT — [`.cursor/rules/poolai-agent-roles.mdc`](../../.cursor/rules/poolai-agent-roles.mdc) — [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc) · ітерація — [`.cursor/rules/poolai-session-iteration.mdc`](../../.cursor/rules/poolai-session-iteration.mdc).

**Autoprogon:** [`AUTO_RUN_SESSION_2026-07-01.md`](./AUTO_RUN_SESSION_2026-07-01.md) S21–S34 ✅. **Horizon:** [`AUTO_RUN_SESSION_2026_HORIZON.md`](./AUTO_RUN_SESSION_2026_HORIZON.md) · [`HORIZON_TO_100_PLAN.md`](./HORIZON_TO_100_PLAN.md).

**FM-003:** dev stand ✅; LAN §4 — **BLOCKED** (2 хости).

**FM-016 ✅:** virtual nodes + `poolai-worker`. **FM-016+ ✅:** bind/webhook/store. **FM-016++ ✅:** `poolai-telegram-bot`. **FM-016+++ ✅:** pool join, `raid_artifact_probe`, artifact cache, verify-dev-stand e2e. **FM-012 ✅:** OAuth (2026-05-27). **P4 ✅ (2026-05-18):** `poolai_health_load` → [`BENCHMARKS.md`](../performance/BENCHMARKS.md). **FM-019 partial ✅ (S7–S12):** pa11y 18 auth + login; `PA11Y_WCAG22=1`; `a11y.yml` PR; `ci.yml` `pa11y-contract`. **OpenAPI (S14–S21)** ✅. **FM-019 CI (S22)** ✅ — `ci.yml` `pa11y-contract` + `pa11y-wcag22` (paths-filter → reusable `a11y.yml`). **S23 ✅:** Playwright smoke. **S24 ✅:** `DELETE /ui/dashboards/{id}` → 204. **S25 ✅:** UI_QUALITY P1 — tenants, OAuth2, dashboards (+3 tests). **S26 ✅:** metrics, alert-rules, SAML, policies (+4 tests; **UI_QUALITY P1 закрито**, 27 contract tests). **S27 ✅:** Playwright admin E2E — tenants + monitoring. **S28 ✅:** OpenAPI gap audit — [`OPENAPI_GAP_AUDIT_2026-05-19.md`](./OPENAPI_GAP_AUDIT_2026-05-19.md). **S29 ✅:** Playwright — `/ui/admin/security`, `/ui/admin/audit` (`admin.spec.ts`). **S30 ✅:** FM legacy docs — [`DOCS_LEGACY_AUDIT_2026-05-19.md`](./DOCS_LEGACY_AUDIT_2026-05-19.md), stale banners. **S33 ✅:** OpenAPI DTO, axe, E2E vm/workers. **S34 ✅:** docs sync A+B 100%, Playwright libs, `data/dev/` gitignore. **Прогрес autoprogon:** **100%** (A+B). **Horizon:** S35–S40 ✅ · **Post-Horizon:** FM-020…031 — [`AUTO_RUN_SESSION_2026_POST_HORIZON.md`](./AUTO_RUN_SESSION_2026_POST_HORIZON.md). **FM-026 ✅:** Jobs API contracts — `tests/jobs_api_contracts.rs`. **FM-027 ✅:** LAN sign-off prep. **FM-028 ✅:** P2b dual-port metrics — `capture-p2b-single-host-metrics.*`, `poolai-p2b-tq01-snapshot`, `BENCHMARKS.md` §FM-028. **FM-029 ✅:** Job store SQLite. **FM-030 ✅:** Monitoring SQLite MVP (`POOLAI_MONITORING_DATA_DIR`). **FM-031 ✅:** pa11y/axe — 21 auth URLs (`/ui/admin/vm|workers|libs|raid` + matrix у `a11y.spec.ts`). **Post-Horizon FM-020…031 закрито.** **Ops BLOCKED:** FM-003 §4 / FM-027 (2 хости). **Нещодавно:** FM-025 `VmTemplate` OpenAPI; FM-024 Solana devnet mock RPC. **FM-020…025 ✅.** Звірка — FM **§5.1**, **§5.7**.

**Зріз:** FM-015 ✅, FM-012 ✅. §5.1 [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md).

**Гілка роботи:** `main` (`git push origin main` → `origin/main`).

**Maintenance (2026-05-23):** `54543028` — PH-S07…S09 (FM-043 Prometheus, FM-044 TLS, FM-045 design system). **PH-S10 ✅** — `admin_charts.js` (line/sparkline charts, polling), `monitoring.rs` + `dashboard.rs` refactor, `DESIGN_SYSTEM.md`. **PH-S11 ✅** — Playwright visual regression (`e2e/tests/visual.spec.ts`, 11 baselines, [`VISUAL_REGRESSION_E2E.md`](./VISUAL_REGRESSION_E2E.md)). **PH-S12 ✅** — theme (dark/light) + i18n (EN/UK) matrix (+12 snapshots; `admin_common.js` `poolaiApplyTheme`). **PH-S13 ✅** — topology admin masked SVG visual (`topology.png`, `TOPOLOGY_VISUAL_MASKS`; commit `d37210f7`). **PH-S14 ✅** — high-contrast admin theme (`poolaiNormalizeTheme`, HC CSS) + axe `color-contrast` E2E. **PH-S03 ✅** — `tests/vm_api_contracts.rs` (VM write lifecycle + RBAC); Playwright VM create/delete (`admin.spec.ts`). **PH-S04 ✅** — `tests/raft_wire_integration.rs` (`GET /api/v1/raid/status` + `AppState::raft_node`); `cargo test-raft-ci`. **PH-S05 ✅** — `/ui/admin/raid` `#raid-cluster-status`. **PH-S06 ✅** — `tests/raft_multi_node_harness.rs` + `raft_rpc` HTTP; `cargo test-raft-ci`. **PH-S01…S14 закрито** (лише S01 Deferred, S02 BLOCKED). **PH-S17…S24 ✅** (2026-05-24). **PH-S25…S34 ✅** (2026-05-24–25): `5f41a919` E2E stability; `476c5c20` secrets OpenAPI; `82d35fd3` metrics test. **PH-S47 ✅ (2026-05-25):** CI #1213 green на `0fe21bf1`. **PH-S37 ✅ infra:** workflow `update-visual-baselines.yml` (PNG on-demand). **PH-S44 ✅:** `test:ci` incl. visual + axe; paths-filter gates Playwright/Pa11y on UI PR. **PH-S39 ✅:** Windows Job Object post-spawn CPU/memory limits (`WindowsJobObjectLimiter`, `apply_limits_post_spawn`, `vm_windows_resource_limits_integration`). **PH-S42 ✅:** Admin tables UX — sort/filter/export toolbar, `adminEmptyStateHtml`, auto-init via `adminInitTablesIn` (`admin_common.js`, `admin_styles.css`). **PH-S43 ✅:** `/ui/admin/monitoring` ML pipeline step metrics panel (`poolaiRenderMlPipelineMetricsPanel`, Run ML Demo, sparklines from `step_results`). **PH-S45 ✅:** VM admin onclick globals (`showCreateVmModal`/`handleCreateVm` on `globalThis` — IIFE fix); Playwright VM create/delete waits POST/DELETE; axe audit settle in `waitForAdminAxeReady`; E2E viewport 1920 + visual baselines refresh. **PH-S38 ✅:** scheduler GPU/deadline hardening (`ScheduleOutcome.expired`, VM/worker GPU placement); core NDJSON on-chain epics (`POOLAI_ONCHAIN_EVENTS_DIR`, `src/job/domain_events.rs`, `onchain.rs`); grid peer bind + memory/seed events; `tests/job_onchain_events.rs`. **PH-S46 ✅:** Solana wire limits + devnet deploy path. **PH-S41 ✅:** `NetworkInterfaceMode` (`veth`/`macvlan`), macvlan create → `unshare` → `ip link set netns` → optional CIDR, cleanup on remove; OpenAPI `NetworkIsolationConfig` fields. **Черга:** §5.11 PH-S40 → PH-S48.

**PH-S48 ✅:** Job store RAID-backed persistence (snapshot у RAID артефактах).
**PH-S49 ✅:** Research + ops/docs — `POOLAI_JOB_STORE=raid` + `POOLAI_RAID_BASE_PATH` (HANDOFF §2a, [`RUN_LOCAL.md`](./RUN_LOCAL.md)).
**PH-S50 ✅:** OpenAPI `JobStoreBackend` + Jobs tag; DIGEST `src/job/`; `poolai-openapi-gap-audit` 0.
**PH-S51 ✅:** Linux veth create on host → `unshare` → peer in netns; tracked cleanup on remove (`vm-isolation-linux`).
**PH-S52 ✅:** Playwright `jobs_raid.spec.ts` (POST job → restart stand → GET); `POOLAI_JOB_STORE=raid` у `e2e-playwright.sh --start`; fix RAID persist `block_on` з HTTP handlers (`src/job/store.rs`).
**PH-S53 ✅:** `/ui/admin/jobs` — таблиця задач, badge `json`/`sqlite`/`raid` з `GET /api/v1/jobs` (`store_backend`).
**PH-S54 ✅:** `bin/verify-dev-stand.*` — `VERIFY_RAID_JOB_STORE=1`: POST job → restart coordinator → GET persisted (патерн PH-S52 / `job_store_raid_persistence`).
**PH-S55 ✅:** `run-poolai` RAID jobs preset — `single` (`--raid-jobs` / `-RaidJobs`) + documented one-liner для `lan` з `POOLAI_JOB_STORE=raid` і `POOLAI_RAID_BASE_PATH` (`RUN_LOCAL.md`).
**PH-S56 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` — lease/TTL (`lease_owner`, `lease_epoch`, `lease_expires_at`), at-most-once через CAS `job_id + lease_epoch`, failover triggers, retry budget, мінімальна state-модель (`Queued/Leased/Running/Migrating/Completed|Failed|Cancelled`).
**PH-S57 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` — unified worker DTO sketch (`origin`, `admin_id`, `capabilities`, `network_profile`, `limits`) + UI правила badges/filter/sort для local/cloud/telegram.
**PH-S58 ✅:** `src/grid/galaxy_fee_split.rs` — primary 0.1% + secondary 1–5% payout (lamports, floor bps), UX hint constant, unit tests; `cargo bench --bench galaxy_fee_split_benchmarks`.
**PH-S59 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` §4.2 — pricing oracle: unit keys (tokens/GPU-sec/job flat), `floor(market_min×0.9)` US providers, cache TTL/SWR, L1–L3 fallback + ops env (`POOLAI_GALAXY_PRICE_*`).
**PH-S60 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` §3.1–3.2 — Telegram seats (`member_cap` / `bound_wallet_cap` / `session_cap`), `seat_limit`, wallet bind flow; ref `POST /api/v1/virtual-nodes/telegram/bind` (FM-016+).
**PH-S61 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` §5.1–5.6 — locality_score, telemetry signals, hot tiers (L0–L3), task-driven prefetch, `seed_inventory` DTO; ref Memory Layer + `src/grid/dispatch.rs`.
**PH-S62 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` §6.1–6.6 — untrusted `telegram_edge`: sampling, replay, K-of-M replication, `trust_score` settlement gate; ZK/TEE — roadmap only.
**PH-S63 ✅:** `docs/concept/POOLAI_GALAXY_GRID.md` §9 — open-source governance: signed releases, protocol compat matrix, opt-in update policies, без root super-admin; §8 TBD #4 закрито.
**PH-S64 ✅:** canonical pointers — README, `docs/README`, INDEX крок 5 + §1, STRUCTURE `concept/`; short pointer Galaxy Grid.
**Galaxy Grid (концепт):** `docs/concept/POOLAI_GALAXY_GRID.md` (повний v1: ролі, fees, pricing, Telegram, lease, locality, verify, governance).

**PH-S65 ✅:** `protocol_version` / `build_id` на `POST /api/v1/discovery/register-remote`; `src/grid/protocol_compat.rs` (Galaxy §9.3 matrix); відповіді `compat_status` + HTTP 403/426; `poolai-worker` шле wire fields; тести `protocol_compat` + `galaxy_protocol_register_integration`.
**PH-S66 ✅:** `poolai-verify-release` — ed25519 manifest + optional artifact SHA-256 (`src/release/`, `cargo run --bin poolai-verify-release`); unit tests; SECURITY_HARDENING ↔ Galaxy §9.2 cross-link.
**PH-S67 ✅:** `FUNCTIONALITY_DIGEST` — zріз Galaxy Grid modules (`galaxy_fee_split`, `dispatch`, `protocol_compat`, virtual nodes API/services, `release/`); INDEX cross-link.
**PH-S68 ✅:** `src/grid/galaxy_pricing_oracle.rs` — unit keys, `floor(market_min×0.9)`, TTL/SWR cache, `POOLAI_GALAXY_PRICE_*` env; unit tests; cross-link §4.2 + DIGEST.
**PH-S69 ✅:** `docs/security/SECURITY_HARDENING.md` — Galaxy §9.2/§9.3 cross-links, `poolai-verify-release` verify flow pointer, без дублювання governance prose.
**PH-S70 ✅:** `docs/development/NEXT_STEPS_ARCHITECT_2026-01-16.md` — legacy warning + canonical pointers (`NEXT_STEPS_ARCHITECT_2026-03-17`, FM §5.12, HANDOFF); historical unchecked boxes позначені як audit-only.
**PH-S71 ✅:** `docs/security/SECURITY_HARDENING.md` — operator quickstart для `poolai-verify-release` (trust root + manifest + optional artifact verify), з посиланням на Galaxy §9.2/§9.3 без дублювання концепту.
**PH-S72 ✅:** `docs/security/SECURITY_HARDENING.md` — protocol compatibility triage checklist (`compat_status`, HTTP 403/426), pointers на Galaxy §9.3 + PH-S65 wire baseline.
**PH-S73 ✅:** `docs/security/SECURITY_HARDENING.md` — protocol reject troubleshooting pointer з escalation path (verify signed build → check protocol window → retry), mismatch tuple для ops review.
**PH-S74 ✅:** `docs/security/SECURITY_HARDENING.md` + `docs/security/DEPENDENCY_SECURITY.md` — advisory/update-policy cross-links (Galaxy §9.6) без дублювання governance prose.
**PH-S75 ✅ (code):** `src/grid/galaxy_pricing_oracle.rs` — L2 configured fallback `POOLAI_GALAXY_PRICING_FALLBACK_JSON` (unit-key usd_micro map), fallback quote path when provider refresh unavailable, unit tests for parser + quote fallback.
**PH-S78 ✅ (code):** `src/network/api/grid.rs` — read-only `GET /api/v1/grid/pricing` (task/model/unit), shared oracle cache+fallback snapshot path, endpoint tests; `docs/openapi.yaml` synced; `cargo run --bin poolai-openapi-gap-audit` → Total missing: 0.
**PH-S79 ✅ (code):** `src/network/api/grid.rs` — API pricing oracle init switched to `GalaxyPricingOracle::from_env()` so env fallback JSON (`POOLAI_GALAXY_PRICING_FALLBACK_JSON`) is actually applied in HTTP path; `cargo test-ci` + openapi-gap 0.
**PH-S76 ✅ (docs):** `docs/security/SECURITY_HARDENING.md` — added concise operator actions pointer for signed release advisories (`CVE-*`, `key_transition`, `protocol_sunset`) with canonical links to Galaxy §9.2/§9.3/§9.6 and dependency advisory flow.
**PH-S77 ✅ (docs):** `docs/security/SECURITY_HARDENING.md` + `DEPENDENCY_SECURITY.md` — single Galaxy §9.2/§9.3/§9.6 canonical pointer hub; deduplicated PH-S71–S76 link blocks; bidirectional DEPENDENCY_SECURITY ↔ hub cross-link.
**PH-S80 ✅ (code):** `galaxy_pricing_oracle::try_quote` L3 (`GalaxyPricingUnavailable`); `GET /api/v1/grid/pricing` → HTTP 503 `pricing_unavailable` when no L1 cache and no L2 fallback; OpenAPI 503; unit tests.
**PH-S81 ✅ (code):** `galaxy_pricing_oracle` — `POOLAI_GALAXY_PRICING_FORCE_FALLBACK=1` always L2 (`pricing_forced_fallback` log + `galaxy_pricing_forced_fallback_total` counter); API skips L1 provider cache when forced (serves cached L2); unit tests; `cargo test-ci`.
**PH-S82 ✅ (code):** `src/ui/admin/grid_pricing.rs` — read-only `/ui/admin/grid-pricing` panel (`task_profile` / `model_profile` / `unit_key` → `GET /api/v1/grid/pricing`); i18n EN/UK; Playwright smoke (`admin.spec.ts`); `cargo test-ci`.
**PH-S83 ✅ (code):** `galaxy_pricing_oracle` — `galaxy_pricing_stale_served` counter + `pricing_oracle_stale_served` log on L1 stale serves (`try_quote` + HTTP cache path); unit test; `cargo test-ci`.
**PH-S84 ✅ (docs):** `docs/concept/POOLAI_GALAXY_GRID.md` §4.2.3 — `GET /api/v1/grid/pricing` + `/ui/admin/grid-pricing` позначені implemented (PH-S78…S83); прибрано «майбутній wire»; Rust reference §4.2 оновлено.
**PH-S85 ✅ (docs+fixtures):** `tests/fixtures/release/dev/` — sample manifest/sig/trust root + `poolai-sample.bin`; `SECURITY_HARDENING` + `RUN_LOCAL` verify-release pointer; Galaxy §9.2 cross-link; `cargo run --bin poolai-verify-release` OK на fixtures.
**PH-S86 ✅ (e2e):** `e2e/tests/grid_pricing.spec.ts` — `GET /api/v1/grid/pricing` L2 fallback + cache + 400 invalid unit; `bin/e2e-playwright.sh --start` sets `POOLAI_GALAXY_PRICING_FALLBACK_JSON`; `npm run test:ci` includes `grid_pricing`.
**PH-S87 ✅ (docs):** `INDEX_2026-03-17.md` крок 8 + §7 security → `SECURITY_HARDENING` Galaxy hub; bidirectional INDEX pointer у hub (PH-S77); без дублювання governance prose.
**PH-S88 ✅ (docs):** `docs/development/RELEASE_MANIFEST_SAMPLE.md` — operator manifest/sig/trust-root schema + verify copy-paste; cross-link `tests/fixtures/release/dev/` (PH-S85); Galaxy §9.2 pointer.
**PH-S89 ✅ (code):** `galaxy_pricing_oracle` + `GET /api/v1/grid/pricing` — `l1_cache` TTL metadata (`cache_age_secs`, fresh/stale until) on L1 hits; `freshness` fresh/stale; unit tests; OpenAPI sync.
**PH-S90 ✅ (ops):** `.cursor/rules/` — `poolai-agent-roles.mdc` (ролі + субагенти); slim VDT; `poolai-session-iteration` → globs; §5.12 sync; `git-commit-msys.mdc`; README/check оновлено.
**PH-S91 ✅ (code):** `galaxy_pricing_oracle` — `galaxy_pricing_fresh_served` counter + `pricing_oracle_fresh_served` log on L1 fresh serves (`try_quote` + HTTP cache path); unit tests; не дублює PH-S83 stale metric; `cargo test-ci`.
**PH-S92 ✅ (code):** `galaxy_pricing_oracle` — `POOLAI_GALAXY_PRICING_PROVIDERS` JSON allow-list parser (`GalaxyPricingProviderCatalog`), bundled US default, `from_env` wire; `matching_entries`; unit tests; без live HTTP fetch; `cargo test-ci`.
**PH-S93 ✅ (code):** `src/ui/admin/updates_compat.rs` — read-only `/ui/admin/updates-compat` (protocol version from `protocol_compat`, verify-release doc pointers, Galaxy §9.3 matrix link); i18n EN/UK; Playwright smoke (`admin.spec.ts`); `cargo test-ci` + `e2e npm run test:ci`.
**PH-S94 ✅ (code):** `JobRecord` optional `lease_owner` / `lease_epoch` / `lease_expires_at` (Galaxy §4.3.1); `lease_active_at` / `lease_epoch_matches` stubs; POST/GET `/api/v1/jobs` wire; OpenAPI sync; `tests/jobs_api_contracts.rs` + unit tests; backward compatible JSON/SQLite store.
**PH-S95 ✅ (code):** `check_patch_lease_epoch` + optional `lease_epoch` on `PATCH /api/v1/jobs/{id}`; HTTP `409 lease_epoch_rejected`; OpenAPI sync; contract tests; backward compatible when `lease_epoch` omitted.
**PH-S96 ✅ (code):** `/ui/admin/jobs` — read-only lease columns (`lease_owner`, `lease_epoch`, `lease_expires_at`); i18n EN/UK; Playwright smoke (`admin.spec.ts`); `cargo test-ci` + e2e PH-S96.
**PH-S97 ✅ (code):** `src/job/lease_config.rs` — `POOLAI_JOB_LEASE_TTL_SECS` (default `90`, Galaxy §4.3.1); `JobLeaseConfig::from_env()` + `lease_renew_interval_secs()` stub; unit tests; без renew/failover wire; `cargo test-ci`.
**PH-S98 ✅ (code):** `src/job/lease_acquire.rs` — lease acquire on scheduler bind + `POST /api/v1/jobs/{id}/lease`; `409 lease_already_active`; OpenAPI sync; unit + `jobs_api_contracts`; `cargo test-ci` + openapi-gap 0.
**PH-S99 ✅ (code):** `renew_lease_on_record` + `POST /api/v1/jobs/{id}/lease/renew`; `409 lease_epoch_rejected` / `lease_expired`; OpenAPI; unit + contract tests.
**PH-S100 ✅ (code):** `JobStatus::Leased` + `allows_transition` (Galaxy §4.3.2); `maybe_transition_to_leased` on lease acquire/schedule bind; OpenAPI `leased`; JSON/SQLite roundtrip; unit + contract + scheduler/grid tests; `cargo test-ci` + openapi-gap 0.
**PH-S101 ✅ (code):** failover / re-migrate stub: expired `Leased` jobs are requeued to `Submitted` and rebound by scheduler (clears stale owner/binding, preserves `lease_epoch` monotonic bump); unit tests in `scheduler.rs`; `cargo test-ci`.
**PH-S102 ✅ (code):** live pricing provider HTTP fetch (Galaxy §4.2.5): API `GET /api/v1/grid/pricing` on L1 miss fetches provider endpoints from `POOLAI_GALAXY_PRICING_PROVIDERS`; timeout via `POOLAI_GALAXY_PRICING_TIMEOUT_MS`; tests for live HTTP path + timeout env parse; `cargo test-ci`.
**PH-S103 ✅ (code):** middleware `X-PoolAI-Protocol` на selected wire routes (`/grid/*`, `register-remote`, `heartbeat-remote`, `virtual-nodes/*`) з negotiation через compat matrix (`src/grid/protocol_compat.rs`), response headers (`coordinator/compat/docs`) та reject unsupported (`403 protocol_unsupported`); unit tests; `cargo test-ci`.
**PH-S104 ✅ (code):** `JobStatus::Migrating` у job wire/lifecycle (`src/job/types.rs`, `src/job/lifecycle.rs`): transitions `Leased/Executing ↔ Migrating`, backward compatible serde JSON; OpenAPI `JobStatus` enum sync + API contract test `jobs_patch_migrating_lifecycle_roundtrip`; `cargo test-ci`.
**PH-S105 ✅ (code):** `/ui/admin/jobs` lease state badge `active/expired` derived from `lease_expires_at`; i18n EN/UK keys in `src/ui/i18n_core.js`; Playwright admin smoke updated with lease-state assertions (`e2e/tests/admin.spec.ts`); `cargo test-ci` (e2e toolchain unavailable locally: `npm` missing in PATH).
**PH-S106 ✅ (code):** `src/bin/poolai-worker.rs` lease renew client stub: when task payload carries `job_id` + `lease_epoch`, worker issues `POST /api/v1/jobs/{id}/lease/renew`; includes payload parser helper and async HTTP renew stub unit tests; no full failover logic changes; `cargo test-ci`.
**PH-S107 ✅ (e2e):** `e2e/tests/jobs_lease.spec.ts` — Playwright API smoke: `POST /api/v1/jobs/{id}/lease` (acquire → `leased`, epoch `1`) + `POST …/lease/renew` (extends `lease_expires_at`); 409 `lease_already_active` / `lease_epoch_rejected`; `npm run test:ci` includes `jobs_lease`; `bin/e2e-playwright.sh --start`.
**PH-S108 ✅ (code):** `src/grid/dispatch.rs` — grid `Job` ingest via `schedule_with_grid_peer`: source peer → `worker_id` + scheduler lease acquire → `JobStatus::Leased` with `lease_owner`/`lease_epoch`/`lease_expires_at`; without peer → `Scheduled` without lease; unit tests; `cargo test dispatch::tests`.
**PH-S109 ✅ (docs):** `POOLAI_GALAXY_GRID.md` §4.3 — compact implemented table PH-S94…S108; §4.3.2 wire note; roadmap смуга PH-S100…S109 **10/10 ✅**; replenish PH-S110…S112 у FM §5.12.
**PH-S110 ✅ (code):** `GridResultBody.lease_epoch` + `check_grid_result_lease_epoch` on grid `Result` ingest; `409 lease_epoch_rejected` when mismatch or missing on leased job; `http_status_for_app_error` maps lease RestError → 409; unit tests in `dispatch.rs` + `lease_tests`; `cargo test-ci`.
**PH-S111 ✅ (code):** `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` — optional renew interval override in `JobLeaseConfig::from_env()` (default `lease_ttl/3`, capped at TTL); HANDOFF §2a; unit tests; `cargo test-ci`.

**PH-S112 ✅ (e2e):** `e2e/tests/grid_job_lease.spec.ts` — `POST /api/v1/grid/envelope` Job + `source_peer_id` → ingest `leased` + `lease_owner`/`lease_epoch`/`lease_expires_at`; without peer → `scheduled` without lease; `e2e/package.json` `test:ci` includes `grid_job_lease`.
**PH-S116 ✅ (code):** `src/bin/poolai-worker.rs` — `LeaseRenewGuard` + `run_lease_renew_ticker` from `JobLeaseConfig.lease_renew_interval_secs` while task carries `job_id` + `lease_epoch`; wiremock unit tests (`lease_renew_ticker_fires_while_active`, epoch conflict stop); `cargo test --bin poolai-worker`.
**PH-S117 ✅ (e2e):** `e2e/tests/grid_result_lease.spec.ts` — grid Job ingest → leased; stale `lease_epoch` on Result → `409 lease_epoch_rejected`; matching epoch → `completed`; `test:ci` includes `grid_result_lease`.
**PH-S118 ✅ (e2e):** `jobs_lease.spec.ts` PH-S118 block — renew without acquire → `400`; expired TTL (`POOLAI_JOB_LEASE_TTL_SECS=2` on e2e stand) → `409 lease_expired`; wrong owner re-acquire → `409 lease_already_active`.
**PH-S119 ✅ (code):** `/ui/admin/jobs` — `#epoch` display; `title` tooltips on lease owner/epoch + column headers; i18n EN/UK; `admin.spec.ts` tooltip/`#42` assertions; `cargo test-ci`.
**PH-S120 ✅ (docs):** `docs/vision/manifest.json` — Solana cluster (`solana_concept`, `job_onchain`, `job_domain_events`, `crate_solana`, `solana_events`, `solana_sidecar`, `solana_program`); DIGEST § Solana modules + FM-033 crosslink; `SOLANA_ADAPTER_CONCEPT` → vision pointer; manifest rev **44**.
**PH-S121 ✅ (docs):** `POOLAI_GALAXY_GRID.md` §4.3.1.1 — worker lease renew vs `heartbeat-remote`; env `POOLAI_JOB_LEASE_*`; task payload `job_id` + `lease_epoch`; `LeaseRenewGuard` ticker contract (PH-S116); DIGEST row; manifest rev **45**.
**PH-S122 ✅ (docs):** `docs/openapi.yaml` — `GridResultBody.lease_epoch` (PH-S110 CAS); jobs lease acquire/renew/409 examples; grid envelope `409 lease_epoch_rejected`; gap audit ignores `#[cfg(test)]` routes; `poolai-vision-sync` auto-indexes git-tracked files into manifest; title **PoolAI Galaxy**; manifest rev **47**.
**PH-S125 ✅ (docs):** `docs/vision/` — **Eco** GPU mode (starfield off, no blur/glow on dense map); instant node select (manifest index cache, event delegation, no `renderMap` on click); Layers/Types filter dock in fullscreen; bottom toolbar layout (Sprint/Folders left, zoom right); manifest rev **49** · UI cache **v53**.
**PH-S123 ✅ (e2e):** `e2e/tests/grid_pricing.spec.ts` — `POOLAI_GALAXY_PRICING_FORCE_FALLBACK=1` via `stand.env` patch + restart: stable L2 snapshot, live provider skipped, `503 pricing_unavailable` without L2; `bin/e2e-playwright.sh` exports force-fallback/providers env; quoted JSON in `stand.env` for bash `source`.
**PH-S124 ✅ (docs):** [`OPENTELEMETRY_TRACING.md`](./OPENTELEMETRY_TRACING.md) — job lease span attribute contract (`job.lease.acquire` / `renew` / `reject`, `job.lease.*` attrs, reject codes); HANDOFF §2a cross-link; FM-038 → PH-S126 instrumentation.
**PH-S126 ✅ (code):** `src/observability/lease_trace.rs` — `trace_acquire_success` / `trace_renew_success` / `trace_lease_reject`; wired `JobStore` acquire/renew, scheduler bind, grid result CAS, PATCH CAS; `tests/observability_otel.rs`; `cargo test-ci` + `cargo test --test observability_otel --features otel`.
**PH-S127 ✅ (code):** `prometheus_export.rs` — `galaxy_pricing_fresh_served` / `stale_served` / `forced_fallback_total` gauges on `GET /metrics` (mirror oracle atomics); [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md); unit tests; `cargo test-ci`.
**PH-S128 ✅ (code):** `src/grid/galaxy_locality.rs` — `locality_score(worker, task)` pure fn (Galaxy §5.1–5.2), `rank_workers_by_locality` / `pick_best_worker_by_locality` scheduler stub; unit tests; no prefetch wire; `cargo test-ci`.
**PH-S129 ✅ (code):** `src/grid/dispatch.rs` — `SeedInventoryEntry` DTO + `plan_prefetch` / `noop_prefetch_hook` policy stub (Galaxy §5.5); unit tests; no live enqueue wire; `cargo test-ci`.
**PH-S130 ✅ (code):** `src/grid/galaxy_trust_score.rs` + `dispatch.rs` result path — `trust_score` 0–100 settlement gate stub (Galaxy §6.5): `PayoutEligible` / `PayoutHeld` / `NotApplicable`; optional `metrics.trust_score` on grid result; unit tests; no payout wire; `cargo test-ci`.
**PH-S131 ✅ (code):** `virtual_node_telegram_wallet_service.rs` + `POST /api/v1/virtual-nodes/telegram/wallet` — payout pubkey bind stub (Galaxy §3.2); OpenAPI sync; `virtual_node_telegram_binding_integration` contract tests; `poolai-openapi-gap-audit` 0; no on-chain wire; `cargo test-ci`.
**PH-S132 ✅ (docs):** `POOLAI_GALAXY_GRID.md` §8.1 — `network_profile` wire contract (`region`, `latency_ms_p50`, `bandwidth_mbps`, `egress_policy`, SmallWorld consumption); TBD #1 closed; DIGEST row; cross-link `galaxy_locality.rs` `LocalityNetworkProfile`.
**PH-S133 ✅ (e2e):** `e2e/tests/jobs_migrating.spec.ts` — Playwright PATCH `leased → migrating → executing` + `executing ↔ migrating` roundtrip (PH-S104 wire); `npm run test:ci` includes `jobs_migrating`; contract `jobs_patch_migrating_lifecycle_roundtrip` unchanged.
**PH-S134 ✅ (e2e):** `e2e/tests/protocol_middleware.spec.ts` — Playwright `POST /discovery/register-remote` with `X-PoolAI-Protocol` (1.2 → compat headers; 1.0 → 403 `protocol_unsupported`); `npm run test:ci` includes `protocol_middleware`; без змін compat matrix wire.
**PH-S138 ✅ (tests):** `tests/galaxy_locality_rank_integration.rs` — multi-worker fixture (`eu-primary` / `us-replica` / `ap-empty`); `rank_workers_by_locality` + `pick_best_worker_by_locality` ordering; tie-break + latency vs inventory cases; `cargo test --test galaxy_locality_rank_integration`.
**PH-S139 ✅ (e2e):** `e2e/tests/telegram_wallet.spec.ts` — Playwright `POST /api/v1/virtual-nodes/telegram/wallet` verified bind + invalid pubkey → 400 (PH-S131 wire); `npm run test:ci` includes `telegram_wallet`; без нового wallet API wire.
**PH-S140 ✅ (code):** `src/grid/galaxy_network_profile.rs` — parse `metadata.network_profile` on `POST /api/v1/discovery/register-remote` (object or JSON string); canonical JSON in peer metadata; `400` on invalid region/schema; `tests/discovery_network_profile_integration.rs`; `cargo test-ci`.
**PH-S141 ✅ (code):** `/ui/admin/jobs` — `migrating` status badge (`warning` class, i18n EN/UK, tooltip PH-S104); Playwright admin smoke (`admin.spec.ts` PH-S141); `cargo test-ci` + e2e.
**PH-S142 ✅ (code):** `src/grid/galaxy_verify_sampling.rs` — `POOLAI_GALAXY_VERIFY_BASE_SAMPLE_RATE` parser (`0.0..=1.0`, default `0.05`); `VerifySamplingConfig::from_env()`; unit tests; no live sampling wire; `cargo test-ci`.
**PH-S144 ✅ (tests):** legacy Playwright API-smoke (`jobs_lease`, `jobs_migrating`, `protocol_middleware`, `telegram_wallet`, `grid_pricing`, `grid_job_lease`, `grid_result_lease`) → Rust `tests/grid_pricing_integration.rs`, `grid_envelope_lease_integration.rs`, `protocol_middleware_integration.rs` + `jobs_api_contracts.rs` gaps; archived `e2e/archive/api-smoke/`; `e2e/package.json` `test:ci` browser + `jobs_raid` only; **rust_ratio 91.91%**; `cargo test-ci`.
**Vision ✅:** next **PH-S145**.
**Research replenish (2026-06-08):** §5.12 **2→10** — code-first + tests: E2E migrating/protocol (S133–S134), wallet GET + E2E (S135/S139), prefetch/trust/verify env stubs (S136/S137/S142), locality rank integration (S138), network_profile register-remote (S140), admin migrating badge (S141). Концепт-горизонт: Galaxy §8.2 settlement/telegram VM · §5.3/§6 Prometheus metrics — post-S142.
**PH-S113 ✅ (docs):** `docs/vision/` — L4 Lib roots + L5 Workspace; nodes `Cargo.toml`, `.cargo/config.toml`, `src/lib.rs`, `poolai-solana-adapter`.
**PH-S114 ✅ (docs):** Galaxy map pan/zoom — `#map-world` transform; wheel ~6%/крок (тачпад), кнопки 16%; drag pan; dblclick focus.
**PH-S115 ✅ (docs):** folder-colored edges + cluster layout (сітка за `src/*/`); **⊟ Folders** collapse (5+); **◎ Sprint** dim; manifest **rev 38**; `file_list.csv` + README/INDEX sync.
**Черга:** §5.12 **9** відкритих — **PH-S145** (наступний) … **PH-S150**. Rust ratio band — [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) · baseline **91.91%**.

**Роадмеп Galaxy Grid:** [`GALAXY_GRID_ROADMAP_2026-05-27.md`](./GALAXY_GRID_ROADMAP_2026-05-27.md) (PH-S65…S111 ✅).

## 1. Канонічний порядок документації та планів

Той самий список, що в кореневому [`README.md`](../../README.md) (*Documentation map*) і [`docs/README.md`](../README.md) (*Canonical reading order*), кроки **1–12**.

| Крок | Що читати |
|------|-----------|
| 0b | [`REPOSITORY_LAYOUT.md`](./REPOSITORY_LAYOUT.md) — `src/` vs `src/bin/` vs `bin/` vs `scripts/` vs `crates/`. |
| 1 | Кореневий [`README.md`](../../README.md) — швидкий старт, збірка, CI, карта доків. |
| 2 | [`INDEX_2026-03-17.md`](../INDEX_2026-03-17.md) — навігація по всьому `docs/`. |
| 3 | [`NEXT_STEPS_ARCHITECT_2026-03-17.md`](./NEXT_STEPS_ARCHITECT_2026-03-17.md) — **головний** план Rust Architect (P1–P6, TurboQuant). |
| 4 | **Цей файл** — [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md): гілка, git-push, зріз P2/P3, next steps. |
| 5 | Концепція: [`concept/poolAI_concept_root.txt`](../concept/poolAI_concept_root.txt), Grid/Memory/Job у `docs/concept/` та [`JOB_LAYER_CONCEPT_2026-03-17.md`](./JOB_LAYER_CONCEPT_2026-03-17.md). |
| 6 | Архітектура: [`ARCHITECTURE_REVIEW.md`](../ARCHITECTURE_REVIEW.md), [`ARCHITECTURE_BEST_PRACTICES.md`](../ARCHITECTURE_BEST_PRACTICES.md). |
| 7 | Продуктивність: [`performance/BENCHMARKS.md`](../performance/BENCHMARKS.md), [`performance/PROFILING.md`](../performance/PROFILING.md); **`poolai_health_load --json`** для baseline; опційно [`benchmarks.yml`](../../.github/workflows/benchmarks.yml). |
| 8 | CI: [`ci.yml`](../../.github/workflows/ci.yml). |
| 9 | Інвентар: [`file_list.csv`](../../file_list.csv) (оновлюй також `docs/catalog/` при зміні витягу); повний список: `git ls-files`. |
| 10 | Git push (Windows): [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md). |
| 11 | Витяг функціоналу: [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). |
| 12 | Керування функціоналом: [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) (**§5.1 — наступні кроки за FM-***); правило [`.cursor/rules/functionality-management.mdc`](../../.cursor/rules/functionality-management.mdc). |

Індекс планів у `docs/development/`: [`README.md`](./README.md). **Таксономія каталогу `docs/`:** [`../STRUCTURE.md`](../STRUCTURE.md). OpenAPI: [`docs/openapi.yaml`](../openapi.yaml). UI↔API: [`UI_QUALITY_AND_E2E_PLAN_2026-04-06.md`](./UI_QUALITY_AND_E2E_PLAN_2026-04-06.md). **Крок 11 / витяг функціоналу:** [`catalog/FUNCTIONALITY_DIGEST_2026-04-06.md`](../catalog/FUNCTIONALITY_DIGEST_2026-04-06.md). **Крок 12 / беклог:** [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) (**§5.1** — наступні кроки за FM-*). **Project skill (Cursor):** [`.cursor/skills/poolai-documentation/SKILL.md`](../../.cursor/skills/poolai-documentation/SKILL.md).

## 2a. Virtual node / Telegram env (FM-016+)

| Змінна | Де | Призначення |
|--------|-----|-------------|
| `POOLAI_COORDINATOR_URL` | worker | Base URL coordinator (без trailing `/`) |
| `POOLAI_PROTOCOL_VERSION` | worker | Galaxy wire protocol на register-remote (default `1.2`) |
| `POOLAI_BUILD_ID` | worker | Build id на register-remote (default `CARGO_PKG_VERSION`) |
| `POOLAI_COORDINATOR_PROTOCOL_VERSION` | coordinator | Coordinator protocol для compat matrix (default `1.2`) |
| `POOLAI_GALAXY_PRICE_CACHE_TTL_SECS` | coordinator | Pricing oracle fresh TTL (default `300`; `galaxy_pricing_oracle`, §4.2) |
| `POOLAI_GALAXY_PRICE_MAX_STALE_SECS` | coordinator | Pricing oracle stale window (default `3600`) |
| `POOLAI_GALAXY_PRICING_FORCE_FALLBACK` | coordinator | `1` — аварійний L2-only режим (`pricing_forced_fallback` log + metric; PH-S81) |
| `POOLAI_GALAXY_PRICING_FALLBACK_JSON` | coordinator | L2 fixed quote map by unit key (usd_micro JSON); PH-S75/S78 |
| `POOLAI_GALAXY_PRICING_PROVIDERS` | coordinator | JSON allow-list provider catalog (PH-S92); no live HTTP fetch |
| `POOLAI_JOB_LEASE_TTL_SECS` | coordinator | Default lease TTL seconds (default `90`; `JobLeaseConfig`, Galaxy §4.3.1; PH-S97) |
| `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS` | coordinator | Optional renew/heartbeat interval seconds (default `lease_ttl/3`, max `lease_ttl`; PH-S111) |
| `POOLAI_TELEGRAM_ID` | worker | Telegram user id → `POST .../telegram/bind` після register |
| `POOLAI_WORKER_CACHE_DIR` | worker | Локальний кеш probe-артефактів після успішного `raid_artifact_probe` |
| `POOLAI_VIRTUAL_NODE_DATA_DIR` | coordinator | Персистентні tasks/bindings (напр. `data/virtual_nodes`) |
| `POOLAI_JOB_DATA_DIR` | coordinator | Персистентні jobs (напр. `data/jobs`; default `jobs.json`) |
| `POOLAI_ONCHAIN_EVENTS_DIR` | coordinator | NDJSON `events.ndjson` для sidecar (`JobCompleted` / memory epics; PH-S38) |
| `POOLAI_JOB_STORE` | coordinator | `sqlite` → `jobs.db` (`--features job-store-sqlite`); `raid` → snapshot у RAID (`POOLAI_RAID_BASE_PATH` **до** першого `JobStore::global()`); інакше JSON (`POOLAI_JOB_DATA_DIR` / `jobs.json`) |
| `POOLAI_RAID_BASE_PATH` | coordinator | Каталог RAID-артефактів (обов’язково для `POOLAI_JOB_STORE=raid`; той самий шлях, що для `/api/v1/raid/*`) |
| `POOLAI_MEMORY_DATA_DIR` | coordinator | Персистентні memory shards (напр. `data/memory`, `shards.json`) |
| `POOLAI_MONITORING_DATA_DIR` | coordinator | Enterprise monitoring SQLite (`monitoring.db`: metrics, dashboards, alert_rules) |
| `POOLAI_SOLANA_CONFIG` | sidecar | Шлях до TOML (default: bundled `config/devnet.toml`) |
| `POOLAI_SOLANA_CLUSTER` | sidecar | `devnet` / `localnet` (mainnet rejected) |
| `POOLAI_SOLANA_MOCK_RPC` | sidecar | `1` — mock submit у stdout ack (`rpc` block); default **off** (FM-033 real RPC) |
| `POOLAI_SOLANA_KEYPAIR_PATH` | sidecar | Solana CLI JSON keypair для devnet `sendTransaction` |
| `POOLAI_SOLANA_PROGRAM_ID` | sidecar | Deployed `poolai-events` program id (інакше Memo fallback) |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | coordinator | OTLP HTTP collector URL (feature `otel`; export off if unset) |
| `OTEL_SERVICE_NAME` | coordinator | OTel `service.name` (default `poolai`) |
| *(OTel lease spans)* | coordinator | Span attrs contract: [`OPENTELEMETRY_TRACING.md`](./OPENTELEMETRY_TRACING.md) § Job lease spans (`job.lease.*`; PH-S124 docs, PH-S126 code) |
| *(build)* `prometheus` feature | coordinator | Enables `GET /metrics` Prometheus text scrape (FM-043; included in `cargo test-ci`) |
| `HTTPS_CERT_PATH` / `HTTPS_KEY_PATH` | coordinator | PEM paths when `https.enabled` (FM-044) |
| `HTTPS_CERT_RELOAD_SECS` | coordinator | Optional hot reload interval for TLS certificates |

**Стек (агенти):** Rust-only runtime — [`.cursor/rules/runtime-stack-policy.mdc`](../../.cursor/rules/runtime-stack-policy.mdc); `docs/STRUCTURE.md` §7. **Не** пропонувати Python для ML/API.
| `POOLAI_TELEGRAM_WEBHOOK_SECRET` | coordinator | Опційно: header `X-Telegram-Webhook-Secret` для webhook |
| `POOLAI_TELEGRAM_AUTH_MAX_AGE_SECS` | enterprise OAuth | Max вік `auth_date` для Telegram Login Widget (default 86400) |
| `TELEGRAM_BOT_TOKEN` | `poolai-telegram-bot` | Token від @BotFather |

Збірка бота: `cargo build --bin poolai-telegram-bot --features tgbot`. Запуск: `TELEGRAM_BOT_TOKEN=... POOLAI_COORDINATOR_URL=http://127.0.0.1:8080 poolai-telegram-bot`.

Секрети — лише в env на хості, не в репо.

## 2b. FM-003 dev stand (одна машина)

| Скрипт | Призначення |
|--------|-------------|
| `bin/run-lan-nodes.ps1` / `.sh` | Два `poolai` на 8080+8081 |
| `bin/run-virtual-node-dev.ps1` / `.sh` | Coordinator + `poolai-worker` |
| `bin/verify-dev-stand.ps1` / `.sh` | Health + discovery + pool join + bootstrap tasks (>=4) + ML pipeline demo (PH-S17); опційно `VERIFY_RAID_JOB_STORE=1` — RAID job persist після restart (PH-S54) |
| `bin/verify-lan-prep.ps1` / `.sh` | FM-027: dual-port or `POOLAI_NODE_*_URL` health + discovery peers |
| `bin/capture-p2b-single-host-metrics.ps1` / `.sh` | FM-028: `run-lan-nodes` + health_load ×2 + TQ01 snapshot → `data/lan-stand/metrics-fm028-*.json` |

Runbook: [`LAN_BENCHMARK_RUNBOOK.md`](../performance/LAN_BENCHMARK_RUNBOOK.md) §5–5.1. **Запуск усього проєкту:** [`RUN_LOCAL.md`](./RUN_LOCAL.md) (`bin/run-poolai.sh`).

## 2. Git push (Windows / Cursor)

- **Канонічна інструкція:** [`.cursor/commands/git-push.md`](../../.cursor/commands/git-push.md) — MSYS2 UCRT64 **зовнішній** термінал, `PATH` з `~/.cargo/bin`, `K8S_OPENAPI_ENABLED_VERSION=1.28` за потреби cloud-sdk, формат коміта з Summary.
- Не робити `git add -A` без потреби; не стаджити `data/audit/*.log.gz`.
- Старі одноразові нотатки `PUSH_*.md` перенесені в [`docs/archive/`](../archive/); актуальні проблеми — [`docs/troubleshooting/`](../troubleshooting/).

## 3. Що вже зроблено (орієнтир для нової сесії)

- **`src/services/`**: `raid_service`, **`raid_distributed_protocol_service`** (distributed RAID JSON protocol; тонкий `raid_distributed_handlers.rs`), `vm_service`, `library_service`, **`instance_service`** (`/api/v1/instance/*`, `/state`), **`chat_completion_service`** (`/v1/chat/completions` — тонкий `completions.rs`), **`system_service`** (status/health/metrics/models/GPU, login, config get/update), **`ui_service`** (теми/компоненти + enterprise-дашборди через `EnterpriseService`), **`discovery_service`**, **`topology_service`**, **`worker_pool_service`**, **`rewards_service`** (`/api/v1/rewards/*`), `enterprise_service`, `cloud_service`, `admin_service` + `GET /api/v1/admin/overview` (`src/network/api/admin.rs`). HTML **`GET /api/v1/status`** — модуль **`network/api/system_status_html.rs`** (не в `SystemService`).
- **RaidService (P2)**: крім list — `put_artifact`, `delete_artifact`, `quota`, `cluster_status`; DTO квоти/статусу в `raid_service.rs`; тонкі handlers у `src/network/api/raid.rs`.
- **ML pipeline (Stage 4.4)**: детерміновані Rust-бекенди для `Preprocessing`, `Training`, `Evaluation`, `Deployment` (`src/ml/pipeline.rs`).
- **TurboQuant (P2b, фаза 1)**: `src/ml/turboquant.rs` (формат `TQ01`), інтеграція в крок `Quantization` за конфігом; див. `docs/ml/TURBOQUANT_INTEGRATION.md`.
- **Priority 3 / FM-005 (HTTP-шар)** ✅: `json_errors.rs` — **`HttpAppError`**, **`IntoResponse`**; **`AppError::RestError`**. Покриття: **`api/*`**, **`raid*`** (**`raid_api_err`**), **`enterprise_api`**, **`authenticate_user`** / **`refresh_access_token`** / **`login`/`refresh` handlers**, **`check_permission`**, **`auth_middleware`** / **`permission_middleware`**.
- **P3 (auth / WS / rate limit)**: **`auth.rs`**, **`ws.rs`**, **`rate_limit.rs`** — той самий JSON-формат помилок (`src/network/json_errors.rs`); UI читає `error.message`. **`http_status_for_app_error`**, **`IntoResponse`** для **`AppError`** / **`HttpAppError`**. Приклад змішаного стилю: **`api/rewards.rs`** — частина GET → **`Result<Json<_>, AppError>`**, **`/rewards/progress/*`** → **`Result<_, HttpAppError>`** (**`ApiNotFound`** / **`NOT_FOUND`**).
- **Перевірка тестів (як CI)**: `K8S_OPENAPI_ENABLED_VERSION=1.28` + `cargo test-ci` (alias у `.cargo/config.toml`: `ml,enterprise,cloud,test-utils,job-store-sqlite,prometheus`). **Raft (PH-S04…S06, PH-S21):** `cargo test-raft-ci` — `raft_wire_integration` + `raft_multi_node_harness` + `raft_membership_log` (`--features raft,test-utils`). Інжектований `AppState`: `tests/appstate_http_injection_integration.rs`, `vm_api_contracts.rs`, `distributed_raid_wire_integration`. На Windows при OOM: `-j 1 -- --test-threads=1`.
- **Clippy (2026-04-10):** перед push доцільно прогнати ті самі команди, що в [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml): `cargo clippy --all-targets --no-default-features -- -D warnings`, `cargo clippy --all-targets --features jwt,https -- -D warnings`, і з `K8S_OPENAPI_ENABLED_VERSION=1.28` — `cargo clippy --all-targets --features cloud,cloud-sdk -- -D warnings`. Для змін у **enterprise** / UI — також `cargo clippy -p poolai --features enterprise -- -D warnings`. Код і `tests/*` вирівняні під ці матриці.
- **FM-012 ✅ (2026-05-16):** i18n UA/EN + Telegram OAuth hardening — [`oauth.rs`](../../src/network/enterprise_api/oauth.rs), [`security.rs`](../../src/enterprise/security.rs), [`i18n_core.js`](../../src/ui/i18n_core.js); unit-тести allowlist/expiry/RBAC.

## 4. Наступні кроки (канон: FM-* + Architect)

**PH-S03…S14 закрито** (лише **PH-S01/PH-S15** Deferred, **PH-S02/PH-S16** BLOCKED). **Єдине зведення FM** — [`catalog/FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) **§5.1**, **§5.9**, legacy **§5.8**, «не зроблено» **§5.3**.

| Порядок | Фокус | Стан |
|--------|--------|------|
| — | **PH-S24** Security ops | **✅** (rotation hooks + pen-test checklist) |
| — | **PH-S16** / **FM-003** LAN §4 | **BLOCKED** (2 хости) |
| — | **PH-S15** / **FM-041** Cloud SDK | **Deferred** |

**Закрито (2026-05-24–25):** **PH-S25…S34** — post-S24 maintenance (E2E, OpenAPI secrets, security/metrics, visual baseline script); **PH-S23** — Playwright admin flows; **PH-S22** topology WS; **PH-S21** Raft membership. **Не повторювати** PH-S03…S34; PH-S29 metrics test (`82d35fd3`).

**Промпт:** [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md).

## 5. Автономний режим (VDT → локальний CI → git push)

**Ролі:** людина (власник/креатив) · агент-оркестратор · субагенти `explore`/`shell`/`generalPurpose` — [`.cursor/rules/virtual-development-team.mdc`](../../.cursor/rules/virtual-development-team.mdc).

1. Старт: [`NEXT_SESSION_PROMPT.md`](./NEXT_SESSION_PROMPT.md) — **PH-S40** hardware VM або **PH-S48**; ops LAN **BLOCKED**; FM-041 Deferred.
2. Ітерація: `poolai-session-iteration.mdc` — S0, MSYS2 bash, `df -h /s`, **один PH-S***, staging/commit/push.
3. **Локальний CI (канон):** `cargo fmt` → `cargo test-ci`; за scope — `test-raft-ci`, `poolai-openapi-gap-audit`, `e2e` `test:ci`. **GitHub CI не блокує** ітерацію.
4. Оркестратор: `autonomous-orchestrator.mdc`; бенч — лише за scope спринту (`BENCHMARKS.md`, `poolai_health_load`).
5. **Не в обсязі:** FM-003 §4 LAN (2 хости); mainnet Solana; native Azure Compute SDK crate.
6. **Push:** MSYS2 UCRT64, [`git-push.md`](../../.cursor/commands/git-push.md); код у коміті → Summary + самарі в чат.
7. **Не в git:** `data/audit/*.log*`, `data/dev/`, `comitmsg/*.txt` (чернетки commit-msg; див. `comitmsg/README.md`), `bin/commit-*.sh`, `target/`.
