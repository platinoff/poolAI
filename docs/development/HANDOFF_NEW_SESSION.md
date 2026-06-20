# Передача контексту новій сесії (PoolAI)

**Оновлено:** 2026-06-20 (PH-S680…S689 ✅ · master backlog **321** PH-S690…S1010 · active **10** · vision **rev 268** · rust_ratio **94.73%** · Cursor **3.8.11** research ✅)

**Cursor 3.8.11 (2026-06-20):** post-update research — [`CURSOR_UPDATE_RESEARCH_2026-06-20.md`](./CURSOR_UPDATE_RESEARCH_2026-06-20.md); `poolai-vision-sync --check` ok; baseline rule оновлено. **Наступна сесія:** **`абракадабра`** (drain PH-S690…S699).

**Master backlog 321 (2026-06-20):** project scan «не зроблено» → [`PH_S_MASTER_BACKLOG_351.md`](./PH_S_MASTER_BACKLOG_351.md) (PH-S690…S1010) · FM **§5.14**. Активна §5.12: **PH-S690…S699** (band 4). **`абракадабра`** = drain 10 → promote наступні 10 (**33** сесій до S1010). **BLOCKED/Deferred** поза backlog: FM-003, FM-041.

**PH-S680…S689 ✅ (2026-06-20):** `GET /api/v1/grid/settlement-metrics` + `GET /api/v1/grid/trust-metrics`; `settlement_gate_depth_stub` (PH-S684); `parsePrometheusGauge` wasm glue (payout-batch); stand smoke settlement/trust API; `poolai-loc-audit` → `rust_ratio.json` **94.73%**; hold advisory `--min-ratio 0.95`; `galaxy_horizon_s680_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 268**.

**PH-S670…S679 ✅ (2026-06-20):** `GET /api/v1/grid/verification-metrics` + `GET /api/v1/grid/replay-metrics`; `verification_replay_depth_stub` (PH-S674); `parsePrometheusGauge` wasm glue (grid-verification); stand smoke verification/replay API; `poolai-loc-audit` → `rust_ratio.json` **94.72%**; hold advisory `--min-ratio 0.95`; `galaxy_horizon_s670_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 267**.

**PH-S660…S669 ✅ (2026-06-20):** ui-core UTC timestamp fix (`format_unix_timestamp_display_ph_s628`); ML metric URL encode (`cpu%2Eusage`); `cargo test -p poolai-ui-core` 0 failed; wasm-only `formatIsoDatetime` (drop `toLocaleString` in `src/ui/mod.rs`); network_profile heartbeat persist stub + `heartbeat_network_profile_persist_stub_ph_s664`; `poolai-loc-audit` → `rust_ratio.json` **94.70%**; hold advisory `--min-ratio 0.95`; `galaxy_horizon_s660_integration`; FM/HANDOFF/NEXT/STABLE/GALAXY sync; vision **rev 266**.

**PH-S650…S659 ✅ (2026-06-20):** ui-core warning cleanup (`table.rs`); `GALAXY_GRID_ROADMAP` sync до S640…S649; cursor sandbox temp cleanup + path restore; `poolai-vision-sync --check` (ok, rev 264); `cargo fmt --all`; `cargo test -p poolai-ui-core` rerun (3 pre-existing fails); FM/HANDOFF/NEXT/STABLE_STATE maintenance sync.

**PH-S640…S649 ✅ (2026-06-20):** replay resolved HTTP; verification replay record/history; checker enqueue HTTP; payout-eligible HTTP; settlement resolved HTTP; prefetch strict-mode HTTP; dashboard/updates-compat/jobs wasm-only; `galaxy_horizon_s640_integration`.

**PH-S630…S639 ✅ (2026-06-20):** mismatch trust delta persist; payout-batch cleared HTTP; prefetch seed-pull; replication executor; replay enqueue; heartbeat-unhealthy; topology/security/grid-pricing wasm; `galaxy_horizon_s630_integration`.

**PH-S620…S629 ✅ (2026-06-20):** verification trust delta persist; payout-held HTTP; elevated sampling HTTP; lease prefetch HTTP; hot-tier promote/evict HTTP; prefetch ingest/wait/complete metrics; shard_fetch_latency_ms_p50; raid formatBytes wasm-only; security datetime wasm; `galaxy_horizon_s620_integration`.

**PH-S610…S619 ✅ (2026-06-20):** stale-epoch trust delta; worker-unhealthy trust delta; hot-tier scheduling gate; re-migrate delta-fetch; access-weight prefetch order; replication hourly cap HTTP; payout-batch worker lamports; checker-timeout HTTP; raid formatBytes wasm; `galaxy_horizon_s610_integration`.

**PH-S600…S609 ✅ (2026-06-19):** strict-locality HTTP 409; semantic_hash human-review; wallet rebind cooldown; p95 tail-latency penalty; topology/white-IP prefetch admission; RAID prefetch HTTP; re-migrate PATCH; fraud-proof HTTP; dashboard wasm slim; `galaxy_horizon_s600_integration`.

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
**PH-S145 ✅ (code):** `src/bin/poolai_http_stand_smoke.rs` — Rust HTTP stand smoke (`reqwest`, `POOLAI_BASE_URL`, optional `--raid` + `POOLAI_E2E_STAND_ROOT`); mirrors archived Playwright API smokes; [`RUN_LOCAL.md`](./RUN_LOCAL.md); `cargo test-ci`.
**PH-S146 ✅ (code):** `crates/poolai-ui-core` — shared admin validators/formatters (lease, pricing, api_error, format, validate, ml) з parity до `src/ui` JS; 16 unit tests; workspace member; `cargo test -p poolai-ui-core` + `cargo test-ci`.
**PH-S147 ✅ (code):** `crates/poolai-ui-wasm` — wasm32 POC (`formatUsdMicro`, `formatUnixSecs`, `leaseStateLabel`); `bash bin/build-ui-wasm.sh` → `src/ui/wasm/`; `.cargo/config.toml` wasm rustflags fix; [`RUN_LOCAL.md`](./RUN_LOCAL.md) + [`RUST_RATIO_STRATEGY_2026-06-13.md`](./RUST_RATIO_STRATEGY_2026-06-13.md) §2.
**PH-S148 ✅ (e2e/docs):** `e2e/package.json` `test:ci` — лише `smoke admin a11y visual jobs_raid` (browser-only; API specs archived у PH-S144); [`E2E_PLAYWRIGHT.md`](./E2E_PLAYWRIGHT.md) sync; `docs/vision/manifest.json` JSON fix (`poolai_loc_audit` node); `cargo run --bin poolai-loc-audit` **91.99%** ≥90%; archived api-smoke не повертаються.
**PH-S150 ✅ (ops/code):** `poolai-loc-audit` — CLI `--warn-below` / `--target` / `--stretch` / `--advisory` / `--min-ratio`; CI job `rust-ratio-audit` (`.github/workflows/ci.yml`); GitHub `::warning::` below 88%; `rust_ratio.json` **92.00%** ≥91%; stretch spirit **96%** documented.
**PH-S151 ✅ (code/e2e):** `/ui/admin/grid-pricing` → `poolai-ui-wasm` module (`formatUsdMicro`, `formatUnixSecs`); static `/ui/wasm/*` via `src/ui/wasm_static.rs`; JS fallback when wasm absent; `e2e-playwright.sh --start` builds wasm when `wasm-bindgen` present; Playwright admin smoke PH-S151.
**PH-S152 ✅ (code/e2e):** shared `POOLAI_UI_WASM_MODULE` in `src/ui/admin/mod.rs`; `/ui/admin/jobs` lease badges via wasm `leaseStateLabel` + thin JS fallback; grid-pricing migrated to `window.poolaiUiWasm`; Playwright admin smoke PH-S152; `cargo test-ci`.
**PH-S153 ✅ (code):** `crates/poolai-ui-core/src/table.rs` + wasm exports (`escapeHtml`, `apiError*`, `formatFetchError`, table HTML/CSV/JSON/sort/filter); `admin_common.js` **−426 LOC** (879 lines); theme/modal → `admin_theme.js` / `admin_modal_a11y.js`; wasm bootstrap on all admin pages; **rust_ratio 92.30%**.
**PH-S154 ✅ (code):** `crates/poolai-ui-core/src/i18n.rs` — admin jobs + grid-pricing EN/UK subset (48 keys); `admin_layout` injects `window.__poolaiAdminI18nRust` before `i18n_core.js`; `i18n_core.js` **−103 LOC** admin block; `jobs.rs` / `grid_pricing.rs` parity tests; **rust_ratio 92.15%** ≥91%.
**PH-S155 ✅ (code):** `crates/poolai-ui-core/src/ml.rs` — chart scale, ML step flatten, sparkline series, metric summary (PH-S43 keys); wasm exports (`parseMlNumeric`, `chartScale`, `flattenMlStepRows`, …); `admin_charts.js` canvas/SVG glue + thin wasm wrappers; **rust_ratio 92.19%** ≥91%.
**PH-S156 ✅ (code/e2e):** `poolai-http-stand-smoke --raid-restart` — POST job → `restart.sh` → GET persisted (заміна Playwright `jobs_raid`); `e2e/package.json` `test:ci` без `jobs_raid`; `bin/e2e-playwright.sh --start` запускає Rust stand smoke; [`E2E_PLAYWRIGHT.md`](./E2E_PLAYWRIGHT.md) sync.
**PH-S157 ✅ (code):** `src/pool/topology_graph.rs` — force layout + heatmap HTML у Rust; `GET /api/v1/topology/graph`; slim `topology_graph.js` (SVG paint only); **rust_ratio 92.33%**; `topology_graph.js` **−173 LOC**.
**PH-S158 ✅ (code):** `poolai-e2e-stand` — Rust stand start/restart/stop; slim `bin/e2e-playwright.sh`; **rust_ratio 92.42%**.
**PH-S159 ✅ (ops):** `poolai-loc-audit` — CI stretch gate warn **93%**, stretch **96%**; default `--warn-below 0.93`; FM replenish PH-S166…S169; **rust_ratio 92.42%** ≥91%.
**PH-S160 ✅ (code):** `poolai-ui-core/theme.rs` — `normalize_theme` + token map; `admin_layout` injects `window.__poolaiAdminThemesRust`; wasm `normalizeTheme`; `admin_theme.js` **−29 LOC** (DOM glue only); `cargo test-ci`.
**PH-S161 ✅ (code):** `poolai-ui-core/modal.rs` — focus-trap `trap_tab_action`, dynamic modal HTML; wasm `trapTabAction`; `__poolaiAdminModalRust` patch; slim `admin_modal_a11y.js`; `cargo test-ci`.
**PH-S162 ✅ (code):** `poolai-ui-core/i18n.rs` — auth + dashboard shell EN/UK (92 keys); `window.__poolaiAuthDashI18nRust` on login + dashboard layout + admin layout; slim `i18n_core.js` auth/dash block; `cargo test-ci`.
**PH-S163 ✅ (code):** `dispatch.rs` — `TrustScoreGateConfig::from_env()` on grid result ingest; `tests/galaxy_trust_metrics_integration.rs` HTTP envelope → `/metrics` scrape; `cargo test-ci`.
**PH-S164 ✅ (code):** `verify_sampling_middleware.rs` — `x-poolai-verify-base-sample-rate` on grid routes; `dispatch.rs` deterministic verify sample stub + counter; `tests/galaxy_verify_sampling_integration.rs`; `cargo test-ci`.
**PH-S165 ✅ (ops):** `poolai-loc-audit` — CI hold gate `--min-ratio 0.95 --advisory`; target **95%** (formal band top); stretch **96%** spirit; **rust_ratio 92.68%** ≥91%.
**PH-S166 ✅ (code):** `poolai-ui-core/design_tokens.rs` — structural CSS vars + admin default `:root` colors; `admin_layout` injects `admin_base_css()`; slim `design_tokens.css` / `admin_styles.css`; `cargo test-ci`.
**PH-S167 ✅ (code):** `galaxy_prefetch_metrics.rs` — counters on `plan_prefetch`; `/metrics` scrape via `refresh_galaxy_prefetch_gauges`; `tests/galaxy_prefetch_metrics_integration.rs`; `cargo test-ci`.
**PH-S168 ✅ (code):** `galaxy_pricing_oracle` — `galaxy_pricing_cache_age_seconds` gauge on L1 hit (`observe_l1_cache_age_secs`); `/metrics` via `refresh_galaxy_pricing_gauges`; `tests/galaxy_pricing_cache_age_integration.rs`; `cargo test-ci`.
**PH-S169 ✅ (code):** `galaxy_locality.rs` — `stale_network_profile_penalty` on missing/>24h `profile_age_secs`; wired into `locality_score` / `rank_workers_by_locality`; unit tests; `cargo test-ci`.
**PH-S170 ✅ (code):** `galaxy_settlement.rs` — `SettlementStatus::PendingVerification` from trust hold + verify sample on grid result ingest; unit + dispatch tests; `cargo test-ci`.
**PH-S171 ✅ (code):** `galaxy_replication.rs` — `replication_strict` 3-of-3 tier config + quorum stub; wired into grid job ingest `replication_tier`; unit tests; `cargo test-ci`.
**PH-S172 ✅ (code):** `galaxy_pricing_provider_metrics.rs` — catalog allow-list lookup/hit counters on `matching_entries`; `/metrics` via `refresh_galaxy_pricing_gauges`; `tests/galaxy_pricing_provider_catalog_metrics_integration.rs`; `cargo test-ci`.
**PH-S173 ✅ (code):** `galaxy_pricing_provider_metrics.rs` — `galaxy_pricing_provider_errors_total` on live provider HTTP fetch fail in `fetch_live_provider_quotes`; `/metrics` via `refresh_galaxy_pricing_gauges`; `tests/galaxy_pricing_provider_errors_integration.rs`; `cargo test-ci`.
**PH-S174 ✅ (code):** `galaxy_pricing_oracle.rs` — `galaxy_pricing_quote_usd_micro` gauge on last served quote in `try_quote`; `/metrics` via `refresh_galaxy_pricing_gauges`; `tests/galaxy_pricing_quote_usd_micro_integration.rs`; `cargo test-ci`.
**PH-S175 ✅ (code):** `galaxy_verification_metrics.rs` — `galaxy_verification_mismatch_total` on grid result `metrics.verification_verdict: mismatch`; `/metrics` via `refresh_galaxy_verification_gauges`; `tests/galaxy_verification_mismatch_integration.rs`; `cargo test-ci`.
**PH-S176 ✅ (code):** `galaxy_replay_metrics.rs` — `galaxy_replay_pending` gauge on grid result mismatch / replay flags; `/metrics` via `refresh_galaxy_verification_gauges`; `tests/galaxy_replay_pending_integration.rs`; `cargo test-ci`.
**PH-S177 ✅ (code):** `galaxy_verification_metrics.rs` — `galaxy_verification_sample_total` on edge sample stub + explicit `metrics.verification_sample`; `/metrics` via `refresh_galaxy_verification_gauges`; `tests/galaxy_verification_sample_integration.rs`; `cargo test-ci`.
**PH-S178 ✅ (code):** `galaxy_settlement_metrics.rs` — `galaxy_settlement_pending_verification_total` on grid result `PendingVerification`; `/metrics` via `refresh_galaxy_verification_gauges`; `tests/galaxy_settlement_pending_verification_integration.rs`; `cargo test-ci`.
**PH-S179 ✅ (code):** `galaxy_replication_metrics.rs` — `galaxy_replication_strict_total` on grid job ingest `replication_strict`; `/metrics` via `refresh_galaxy_replication_gauges`; `tests/galaxy_replication_strict_integration.rs`; `cargo test-ci`.
**PH-S180 ✅ (code):** `galaxy_verification_metrics.rs` — `galaxy_verification_match_total` on grid result `verification_verdict: match`; `/metrics` via `refresh_galaxy_verification_gauges`; `tests/galaxy_verification_match_integration.rs`; `cargo test-ci`.
**PH-S181 ✅ (code):** `galaxy_pricing_oracle.rs` — `galaxy_pricing_market_min_usd_micro` gauge on `try_quote`; `/metrics` via `refresh_galaxy_pricing_gauges`; `tests/galaxy_pricing_market_min_usd_micro_integration.rs`; `cargo test-ci`.
**PH-S182 ✅ (code):** `galaxy_trust_score.rs` — `galaxy_trust_score` gauge on grid result ingest (`observe_last_trust_score`); `/metrics` via `refresh_galaxy_trust_gauges`; extended `tests/galaxy_trust_metrics_integration.rs`; `cargo test-ci`.
**PH-S183 ✅ (code):** `galaxy_locality.rs` — `galaxy_shard_local_hit_ratio` gauge on `rank_workers_by_locality` top worker; `/metrics` via `refresh_galaxy_locality_gauges`; `tests/galaxy_shard_local_hit_ratio_integration.rs`; `cargo test-ci`.
**PH-S184 ✅ (code):** `galaxy_prefetch_metrics.rs` — `galaxy_prefetch_bytes_total` counter on `plan_prefetch`; `/metrics` via `refresh_galaxy_prefetch_gauges`; extended `tests/galaxy_prefetch_metrics_integration.rs`; `cargo test-ci`.
**PH-S188 ✅ (vision):** independent map layer/type filters; LAYERS/TYPES All/None; 3D stack decoupled; rev **125**.
**PH-S195 ✅ (code):** `GET /api/v1/grid/seed-inventory` — coordinator `SeedInventoryPeerSnapshot` stub; OpenAPI; `tests/grid_seed_inventory_integration.rs`; `cargo test-ci`.
**PH-S196 ✅ (code/e2e):** `poolai-http-stand-smoke --lease-renew` — acquire/renew/conflict/expired suite (заміна archived Playwright `jobs_lease`); slim default stand smoke; `bin/e2e-playwright.sh --start` gate; `cargo test-ci`.
**PH-S197 ✅ (code/ui):** `/ui/admin/updates-compat` — wasm `compatStatusLabel` / `protocolVersionLabel`; updates-compat i18n → `poolai-ui-core`; slim `i18n_core.js`; Playwright PH-S197 gate; `cargo test-ci`.
**PH-S199 ✅ (vision):** `docs/vision/` — Ms mode planes `pointer-events:none`; `elementsFromPoint` edge/node trace; click focus ~14px label; zoom-back stack (`←`); sidebar `scrollIntoView` + folder expand; rev **137**.
**PH-S200 ✅ (vision):** `docs/vision/feed.json` — RSS sprint ticker (`poolai-vision-sync` from FM §5.12); header marquee panel; click item → sprint queue; rev **139**.
**PH-S201 ✅ (ops):** `.cursor/hooks/post-push-ph-s-notify.sh` — `postToolUse` after successful `git push` + `PH-S*` in commit subject → VDT docs-sync `additional_context`; self-test `--self-test`.
**PH-S202 ✅ (vision):** `docs/vision/vision.js` — sprint queue card click → `pickMapNodeForSprint` + `focusMapNode`; `map-linked` / `queue-active` chips; rev **142**.
**PH-S203 ✅ (vision):** `docs/vision/vision.js` — Arrow keys cycle 1-hop manifest neighbors (`linkedMapNeighbors`); rev **144**.
**PH-S204 ✅ (vision):** `docs/vision/vision.js` — edge click → `edgeTraceNodeId` + endpoint select + `edge-click-active` trace; rev **146**.
**PH-S205 ✅ (ops):** `poolai-vision-sync --check` — manifest vs FM §5.12 drift gate; CI `vision-manifest-drift`; rev **147**.
**PH-S206 ✅ (vision):** `docs/vision/vision.js` — minimap `#minimap-selection-ring` + selected dot on dense map; viewport fill; rev **148**.
**PH-S207 ✅ (code/ui):** `admin.mon.*` + `admin.page.monitoring` i18n → `poolai-ui-core`; slim `i18n_core.js`; `cargo test-ci`.
**PH-S208 ✅ (code/tests):** `poolai-http-stand-smoke` — `vision_revision_parity` (FM/manifest + `X-PoolAI-Vision-Revision` header); `open-docs-vision.ps1` header; rev **150**.
**PH-S209 ✅ (vision):** `docs/vision/` — `:focus-visible` on map controls + filter chips; roving `tabindex` on SVG nodes; Enter/Space activate; arrow nav syncs focus; rev **151**.
**PH-S210 ✅ (code/tests):** `poolai-http-stand-smoke` — `grid_seed_inventory` case for `GET /api/v1/grid/seed-inventory` (PH-S195 stub); `cargo test-ci`.
**PH-S211 ✅ (code/ui):** `admin_layout_jobs` + `admin_jobs_patch` — jobs page slim Rust i18n; `admin.page.jobs` removed from `i18n_core.js`; `cargo test-ci`.
**PH-S212 ✅ (vision):** `docs/vision/` — `prefers-reduced-motion` + `map-fx-off` skip constellation glow/edge animation; rev **154**.
**PH-S213 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_prefetch_metrics` on live `/metrics` (PH-S184 counters); `cargo test-ci`.
**PH-S214 ✅ (code/ui):** `admin_raid_patch` + `admin_layout_raid` — raid page slim Rust i18n; `admin.raid.*` removed from `i18n_core.js`.
**PH-S215 ✅ (vision):** `docs/vision/` — `focusPanelToggle` on panel collapse/Esc; `aria-expanded` on collapse toggles; UI cache **v71**; rev **161**.
**PH-S216 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_pricing_forced_fallback_metrics` on live `/metrics`; rev **161**.
**PH-S217 ✅ (code/ui):** `admin_grid_pricing_patch` + `admin_layout_grid_pricing` — grid-pricing slim Rust i18n; `admin.page.gridPricing` removed from `i18n_core.js`.
**PH-S218 ✅ (vision):** `docs/vision/` — `#map-selection-live` aria-live region announces selected node label/layer/path; UI cache **v72**; rev **157**.
**PH-S219 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_trust_payout_metrics` on live `/metrics` (eligible/held/score gauges); `cargo test-ci`.
**PH-S220 ✅ (code/ui):** `admin_monitoring_patch` + `admin_layout_monitoring` — monitoring slim Rust i18n; removed from fat `admin_jobs_grid_patch`.
**PH-S221 ✅ (code/ui):** `admin_updates_compat_patch` + `admin_layout_updates_compat` — updates-compat slim Rust i18n; default layout patch jobs-only.
**PH-S222 ✅ (code/ui):** `admin_workers_patch` + `admin_layout_workers` — workers slim Rust i18n; `admin.wrk.*` removed from `i18n_core.js`.
**PH-S223 ✅ (code/ui):** `admin_libs_patch` + `admin_layout_libs` — libs slim Rust i18n; `admin.lib.*` removed from `i18n_core.js`.
**PH-S224 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_pricing_cache_age_metrics` on live `/metrics` (PH-S168 gauge).
**PH-S225 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_verification_metrics` on live `/metrics` (sample/match/mismatch/scheduled).
**PH-S226 ✅ (vision):** `docs/vision/vision.js` — `ensurePanelExpanded` for map/queue panels; RSS ticker map-linked → `focusSprintOnMap`; queue `aria-label` + `:focus-visible`; map select syncs `queue-active`; UI cache **v73**; rev **173**.
**PH-S227 ✅ (ops/vision):** `poolai-vision-sync --check` — manifest ↔ `.mdc` VDT rules cross-link drift; index `.mdc` + vision artifacts; rev **176**.
**PH-S228 ✅ (code/ui):** `admin_dashboard_patch` + `admin_layout_dashboard` — dashboard slim Rust i18n; `admin.dash.*` removed from `i18n_core.js`.
**PH-S229 ✅ (code/ui):** `admin_audit_patch` + `admin_layout_audit` — audit slim Rust i18n; `admin.audit.*` removed from `i18n_core.js`.
**PH-S230 ✅ (code/ui):** `admin_tenants_patch` + `admin_layout_tenants`; `admin.tenants.col.name` shim in jobs patch for `security.rs` until PH-S231.
**PH-S231 ✅ (code/ui):** `admin_security_patch` + `admin_layout_security`; `admin.sec.*` + `admin.page.security` removed from `i18n_core.js`; `admin.sec.col.name` owns table headers; tenants shim removed from jobs patch.
**PH-S232 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_replication_metrics` on live `/metrics` (`galaxy_replication_strict_total`; PH-S179).
**PH-S233 ✅ (vision):** `docs/vision/vision.js` — `bindMapLinkedSprintChip` + `aria-label` on links-panel sprint chips, RSS ticker, queue; UI **v74** / CSS **v70**; rev **182**.
**PH-S234 ✅ (code/ui):** `admin_topology_patch` + `admin_layout_topology`; `admin.topo.*` + `admin.page.topology` removed from `i18n_core.js`; topology page slim Rust i18n patch.
**PH-S235 ✅ (code/tests):** `poolai-http-stand-smoke` — `assert_vision_repo_parity` (manifest.revision vs FM `Vision rev **N**` + `extensions.active_sprint` vs `manifest.next_sprint`); unit tests `ph_s235`; extends PH-S208 live header check.
**PH-S236 ✅ (code/ui):** `admin_instances_patch` + `admin_layout_instances`; `admin.inst.*` + `admin.page.instances` removed from `i18n_core.js`.
**PH-S237 ✅ (code/ui):** `admin_vm_patch` + `admin_layout_vm`; `admin.vmadm.*` + `admin.page.vm` removed from `i18n_core.js`; modal `vm.*` keys remain in core.
**PH-S238 ✅ (code/ui):** `admin_users_patch` + `admin_layout_users`; `admin.usr.*` + `admin.page.users` removed from `i18n_core.js`; shared `admin.status.*` / `ui.*` remain in core.
**PH-S239 ✅ (code/ui):** `admin_config_patch` + `admin_layout_config`; `admin.cfg.*` + `admin.page.config` removed from `i18n_core.js`; shared `admin.status.*` / `ui.*` remain in core.
**PH-S240 ✅ (code/ui):** `admin_table_patch` injected on all admin layouts via `__poolaiAdminTableI18nRust`; `admin.table.*` removed from `i18n_core.js`.
**PH-S241 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_pricing_fresh_served_metrics` on live `/metrics` (PH-S127 gauge export); unit test `ph_s241`.
**PH-S242 ✅ (code/ui):** `ADMIN_NAV_*` merged into `auth_dash_shell_patch`; `admin.nav.*` removed from `i18n_core.js`; audit tests `ph_s242`.
**PH-S243 ✅ (code/ui):** `ADMIN_CHROME_*` merged into `auth_dash_shell_patch`; `admin.brand` / skip / lang / logout / browserSuffix removed from `i18n_core.js`; audit tests `ph_s243`.
**PH-S244 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_pricing_stale_served_metrics` on live `/metrics` (PH-S127 gauge export); unit test `ph_s244`.
**PH-S245 ✅ (code/ui):** `admin_status_patch` injected on all admin layouts via `__poolaiAdminStatusI18nRust`; `admin.status.*` / `admin.na` / `admin.btn.edit` removed from `i18n_core.js`; audit tests `ph_s245`.
**PH-S246 ✅ (code/ui):** `admin_err_patch` injected on all admin layouts via `__poolaiAdminErrI18nRust`; `err.hint*` / `err.insufficientAdmin` / `admin.accessRequired` removed from `i18n_core.js`; audit tests `ph_s246`.
**PH-S247 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_pricing_provider_metrics` on live `/metrics`; unit test `ph_s247`.
**PH-S248 ✅ (code/ui):** `vm_modal_patch` on admin + dashboard shells; `vm.*` removed from `i18n_core.js`; audit tests `ph_s248`.
**PH-S249 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_settlement_metrics` on live `/metrics`; unit test `ph_s249`.
**PH-S250 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_shard_local_hit_ratio_metrics` on live `/metrics`; unit test `ph_s250`.
**PH-S251 ✅ (docs):** GALAXY_GRID_ROADMAP + README + INDEX sprint zriz; §5.12 band S247…S252 closed.
**PH-S252 ✅ (code/ui):** `admin_ui_confirm_patch` on admin + dashboard shells; `ui.confirm*` glue removed from `i18n_core.js`; audit tests `ph_s252`.
**PH-S253 ✅ (code/tests):** `poolai-http-stand-smoke` — `galaxy_pricing_quote_market_metrics` on live `/metrics`; unit test `ph_s253`.
**PH-S254 ✅ (code/tests):** `galaxy_fee_split_applied_metrics` stand smoke; unit test `ph_s254`.
**PH-S255 ✅ (code/tests):** `galaxy_cross_region_egress_metrics` stand smoke; unit test `ph_s255`.
**PH-S256 ✅ (code/tests):** `galaxy_replay_pending_metrics` stand smoke; unit test `ph_s256`.
**PH-S257 ✅ (code/ui):** `workers.*` → `admin_workers_patch` + `workers_panel_patch`; audit tests `ph_s257`.
**PH-S258 ✅ (code/ui):** `home.*` → `admin_home_patch` on dashboard layout; audit test `ph_s258`.
**PH-S259 ✅ (code/ui):** `form.*` + residual `err.*` → `admin_form_patch` / extended `admin_err_patch`.
**PH-S260 ✅ (code/ui):** `ui.save`/`ui.search*`/`ui.retry*` → `admin_ui_toolbar_patch`; audit tests `ph_s260`.
**PH-S261 ✅ (docs):** INDEX/STABLE_STATE/GALAXY_ROADMAP/RUST_RATIO sprint zriz; §5.12 band S253…S262 closed.
**PH-S262 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` **94.23%**; hold 95% advisory; stretch 96% spirit.
**PH-S263 ✅ (code/ui):** `common.*` + residual `ui.*` → `admin_ui_common_patch`.
**PH-S264 ✅ (code/ui):** dashboard `libs.*` → `libs_panel_patch`.
**PH-S265 ✅ (code/ui):** dashboard `raid.*` → `raid_panel_patch`.
**PH-S266 ✅ (ops):** `i18n_core.js` STRINGS core **0** inline keys; `poolai-loc-audit` → **94.34%**.
**PH-S267 ✅ (docs):** INDEX/STABLE_STATE/FM/HANDOFF/NEXT canon sync.
**PH-S268 ✅ (docs):** GALAXY_GRID_ROADMAP §5.5 prefetch horizon pointer.
**PH-S269 ✅ (vision):** `feed.json` sprint zriz via `poolai-vision-sync`.
**PH-S270 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S271 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.34%** (advisory warn).
**PH-S272 ✅ (docs):** INDEX §7 + ratio pointer **94.34%**.
**PH-S273 ✅ (code/ui):** `admin_common.js` api-error wasm-first; removed `hintFor503` + JS `err.hint*` dup; audit test `ph_s273`.
**PH-S274 ✅ (code/ui):** `admin_dom` + wasm `adminLoadingHtml`/`adminInlineErrorHtml`; `adminShowLoading`/`adminShowInlineError` wasm-first glue.
**PH-S275 ✅ (code/ui):** `render_sparkline_html` + wasm `renderSparklineHtml`; `admin_charts.js` sparkline wasm-first.
**PH-S276 ✅ (code):** `ingest_job_prefetch_stub` + `required_shard_ids` on `GridJobBody`; grid job ingest calls `plan_prefetch`.
**PH-S277 ✅ (code):** `topology_graph.js` paint-only audit ≤100 LOC gate test `ph_s277`.
**PH-S278 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` **94.36%** (sprint PH-S278).
**PH-S279 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE_STATE canon sync.
**PH-S280 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S281 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.36%**.
**PH-S282 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.36%**.
**PH-S283 ✅ (code):** `enqueue_prefetch_hook` + `galaxy_prefetch_enqueue_total`; ingest uses enqueue stub.
**PH-S284 ✅ (code/ui):** `render_line_chart_html` + wasm; `admin_charts.js` line chart wasm-first.
**PH-S285 ✅ (code):** `ingest_job_locality_rank_stub` + `locality_workers_from_seed_snapshots`.
**PH-S286 ✅ (tests):** stand smoke `/metrics` includes `galaxy_prefetch_enqueue_total`.
**PH-S287 ✅ (code/ui):** `group_metrics_by_name` + wasm `groupMetricsByName`.
**PH-S288 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S288** (**94.36%**).
**PH-S289 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S290 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S291 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.36%**.
**PH-S292 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.36%**.
**PH-S293 ✅ (code):** `wait_prefetch_hook` + `galaxy_prefetch_wait_ms_total`; ingest prefetch path calls wait stub.
**PH-S294 ✅ (code/ui):** `render_metrics_chart_grid_html` + wasm `renderMetricsChartGridHtml`.
**PH-S295 ✅ (code):** `galaxy_locality_rank_ingest_total` on `ingest_job_locality_rank_stub`.
**PH-S296 ✅ (tests):** stand smoke `/metrics` includes wait + locality ingest counters.
**PH-S297 ✅ (code/ui):** `sanitize_chart_id` + wasm `sanitizeChartId`; charts JS wasm-first.
**PH-S298 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S298** (**94.37%**).
**PH-S299 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S300 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S301 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.37%**.
**PH-S302 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.37%**.
**PH-S303 ✅ (code):** `galaxy_prefetch_strict_mode_total` on strict locality prefetch plans.
**PH-S304 ✅ (code/ui):** `renderLineChartEmptyHtml` wasm; line chart empty wasm-first.
**PH-S305 ✅ (code):** `galaxy_locality_rank_miss_total` on locality rank miss.
**PH-S306 ✅ (tests):** stand smoke `/metrics` strict + complete + rank miss counters.
**PH-S307 ✅ (code):** `complete_prefetch_hook` + `galaxy_prefetch_complete_total`.
**PH-S308 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S308** (**94.38%**).
**PH-S309 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S310 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S311 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.38%**.
**PH-S312 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.38%**.
**PH-S313 ✅ (code):** `galaxy_prefetch_ingest_total` on `ingest_job_prefetch_stub`.
**PH-S314 ✅ (code/ui):** `buildMetricHistoryUrl` wasm; `poolaiFetchMetricHistory` wasm-first.
**PH-S315 ✅ (code):** `galaxy_locality_rank_empty_workers_total` on empty worker inventory.
**PH-S316 ✅ (tests):** stand smoke `/metrics` ingest + empty workers counters.
**PH-S317 ✅ (code/ui):** `buildMetricsWindowUrl` wasm; `poolaiFetchMetricsWindow` wasm-first.
**PH-S318 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S318** (**94.39%**).
**PH-S319 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S320 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S321 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.39%**.
**PH-S322 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.39%**.
**PH-S323 ✅ (code):** `galaxy_prefetch_skip_ingest_total` on empty `required_shard_ids`.
**PH-S324 ✅ (code/ui):** `buildMlPipelinesUrl` wasm; `poolaiFetchMlPipelines` wasm-first.
**PH-S325 ✅ (code):** `galaxy_locality_rank_skip_total` on empty shard list.
**PH-S326 ✅ (tests):** stand smoke `/metrics` skip ingest + rank skip counters.
**PH-S327 ✅ (code/ui):** `buildMlPipelineDemoUrl` wasm; `poolaiRunMlPipelineDemo` wasm-first.
**PH-S328 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S328** (**94.37%**).
**PH-S329 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S330 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S331 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.37%**.
**PH-S332 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.37%**.
**PH-S333 ✅ (code):** `galaxy_replay_pending_scheduled_total` on replay schedule path.
**PH-S334 ✅ (code/ui):** `buildMetricHistoryUrlWithHours` wasm; `poolaiFetchMetricHistory` wasm-first.
**PH-S335 ✅ (code):** `galaxy_replay_pending_resolved_total` on replay verdict path.
**PH-S336 ✅ (tests):** stand smoke `/metrics` replay scheduled + resolved counters.
**PH-S337 ✅ (code/ui):** `buildMetricsWindowUrlWithHours` wasm; `poolaiFetchMetricsWindow` wasm-first.
**PH-S338 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S338** (**94.37%**).
**PH-S339 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S340 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S341 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.37%**.
**PH-S342 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.37%**.
**PH-S343 ✅ (code):** `galaxy_verification_sample_completed_total` on match/mismatch verdict.
**PH-S344 ✅ (code/ui):** `buildMonitoringAlertsUrl` wasm; `poolaiFetchMonitoringAlerts` wasm-first.
**PH-S345 ✅ (code):** `galaxy_verification_sample_skipped_total` on edge NotSelected stub.
**PH-S346 ✅ (tests):** stand smoke `/metrics` verification completed + skipped counters.
**PH-S347 ✅ (code/ui):** `buildAlertRulesUrl` wasm; `poolaiFetchAlertRules` wasm-first.
**PH-S348 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint **PH-S348** (**94.35%**).
**PH-S349 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S350 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S351 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.35%**.
**PH-S352 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.35%**.
**PH-S353 ✅ (code/ui):** `buildMonitoringDashboardsUrl` + `buildMonitoringAlertAcknowledgeUrl` wasm; monitoring POST/ack wasm-first; `build-ui-wasm.sh` Windows cargo PATH.
**PH-S354 ✅ (code):** `galaxy_settlement_not_applicable_total` on grid result `NotApplicable` path.
**PH-S355 ✅ (code/ui):** `buildMonitoringActiveAlertsUrl` wasm; monitoring `acknowledged: false`.
**PH-S356 ✅ (code):** `galaxy_verification_sample_not_applicable_total` on local-origin verify stub.
**PH-S357 ✅ (tests):** stand smoke settlement + verify not-applicable `/metrics` shape.
**PH-S358 ✅ (code):** `admin_charts_*_wasm_first_ph_s353/355` glue tests.
**PH-S359 ✅ (ops):** `poolai-loc-audit` → **94.33%**.
**PH-S360 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S361 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S362 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.33%**.
**PH-S363 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.33%**.
**PH-S364 ✅ (code):** `galaxy_trust_payout_not_applicable_total` on local-origin trust gate path.
**PH-S365 ✅ (code/ui):** dashboard `buildMonitoringActiveAlertsUrl(5)` wasm-first.
**PH-S366 ✅ (code/ui):** `buildMonitoringMetricLatestUrl` wasm glue + `poolaiMonitoringMetricLatestUrl`.
**PH-S367 ✅ (tests):** stand smoke trust not-applicable `/metrics` shape.
**PH-S368 ✅ (code):** dashboard + metric latest wasm glue tests.
**PH-S369 ✅ (ops):** `poolai-loc-audit` → **94.32%**.
**PH-S370 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S371 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S372 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.32%**.
**PH-S373 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.32%**.
**PH-S374 ✅ (code):** `galaxy_trust_gate_min_threshold` gauge on `/metrics` (env `POOLAI_GALAXY_MIN_TRUST_PAYOUT`).
**PH-S375 ✅ (code/ui):** dashboard `buildAuditEventsUrl(10)` wasm-first.
**PH-S376 ✅ (code/ui):** `buildAdminOverviewUrl` wasm glue on dashboard load.
**PH-S377 ✅ (tests):** stand smoke trust gate min threshold `/metrics` shape.
**PH-S378 ✅ (code):** dashboard overview + audit wasm glue tests.
**PH-S379 ✅ (ops):** `poolai-loc-audit` → **94.33%**.
**PH-S380 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S381 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S382 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.33%**.
**PH-S383 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.33%**.
**PH-S384 ✅ (code):** `galaxy_trust_gate_default_score` gauge on `/metrics` (DEFAULT_TRUST_SCORE 50).
**PH-S385 ✅ (code/ui):** dashboard `formatUptime` wasm-first on overview.
**PH-S386 ✅ (code/ui):** `buildDashboardMetricsWindowUrl` wasm glue on metrics chart.
**PH-S387 ✅ (tests):** stand smoke trust gate default score `/metrics` shape.
**PH-S388 ✅ (code):** dashboard uptime + metrics window wasm glue tests.
**PH-S389 ✅ (ops):** `poolai-loc-audit` → **94.33%**.
**PH-S390 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S391 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S392 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.33%**.
**PH-S393 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.33%**.
**PH-S394 ✅ (code):** `galaxy_trust_gate_evaluations_total` gauge on grid result path.
**PH-S395 ✅ (code):** `galaxy_trust_default_score_applied_total` when trust_score omitted.
**PH-S396 ✅ (code/ui):** dashboard recent activity `formatIsoDatetime` wasm-first.
**PH-S397 ✅ (tests):** stand smoke trust gate evaluation counters `/metrics` shape.
**PH-S398 ✅ (code):** dashboard audit timestamp wasm glue tests.
**PH-S399 ✅ (ops):** `poolai-loc-audit` → **94.34%**.
**PH-S400 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S401 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S402 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.34%**.
**PH-S403 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.34%**.
**PH-S404 ✅ (code):** `galaxy_settlement_resolved_total` gauge on grid result path.
**PH-S405 ✅ (code):** `galaxy_trust_explicit_score_total` when trust_score explicit.
**PH-S406 ✅ (code/ui):** dashboard active alerts `alertSeverityBadgeClass` wasm-first.
**PH-S407 ✅ (tests):** stand smoke settlement resolved + explicit score `/metrics` shape.
**PH-S408 ✅ (code):** dashboard alert severity wasm glue tests.
**PH-S409 ✅ (ops):** `poolai-loc-audit` → **94.34%**.
**PH-S410 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S411 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S412 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.34%**.
**PH-S413 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.34%**.
**PH-S414 ✅ (code):** `galaxy_verification_sampling_evaluations_total` gauge on grid result path.
**PH-S415 ✅ (code):** `galaxy_replay_evaluations_total` gauge on grid result path.
**PH-S416 ✅ (code/ui):** dashboard `updateDashboardRefreshedAt` + `formatLocaleTimeHms` wasm-first.
**PH-S417 ✅ (tests):** stand smoke verify + replay evaluation `/metrics` shape.
**PH-S418 ✅ (code):** dashboard refreshed-at wasm glue tests.
**PH-S419 ✅ (ops):** `poolai-loc-audit` → **94.35%**.
**PH-S420 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync.
**PH-S421 ✅ (ops):** `poolai-vision-sync --check` green.
**PH-S422 ✅ (ops):** hold advisory `--min-ratio 0.95` snapshot **94.35%**.
**PH-S423 ✅ (docs):** INDEX §7 + rust_ratio pointer **94.35%**.
**PH-S424 ✅ (code):** `galaxy_prefetch_seed_pull_total` on `seed_pull_hook` / `complete_prefetch_hook` (Galaxy §5.5).
**PH-S425 ✅ (code):** `galaxy_prefetch_lease_acquired_total` + `lease_acquire_prefetch_stub` on `POST /jobs/{id}/lease`.
**PH-S426 ✅ (code):** `galaxy_replication_enqueue_total` on grid job ingest (`galaxy_replication_metrics`).
**PH-S427 ✅ (code):** `galaxy_settlement_payout_batch_total` on cleared settlement path.
**PH-S428 ✅ (code/ui):** dashboard quick-stats wasm — `formatPercent` + `formatMegabytes` (`poolai-ui-core` / `poolai-ui-wasm`).
**PH-S429 ✅ (tests):** `poolai-http-stand-smoke` — prefetch seed-pull / lease-acquired / replication enqueue / payout batch `/metrics` shape.
**PH-S430 ✅ (code):** admin dashboard quick-stats wasm glue tests (PH-S428 gates).
**PH-S431 ✅ (ops):** `poolai-loc-audit` → `rust_ratio.json` sprint zriz **94.35%**.
**PH-S432 ✅ (docs):** INDEX/HANDOFF/NEXT/STABLE/GALAXY canon sync band.
**PH-S433 ✅ (ops/docs):** `poolai-vision-sync` **rev 229** + INDEX §7 rust_ratio maintain.
**PH-S434 ✅ (code):** `resolve_seed_pull_shards` + `seed_pull_hook` inventory resolver (Galaxy §5.5).
**PH-S435 ✅ (code):** `replication_executor_hook` + `galaxy_replication_executor_enqueue_total` on grid job ingest.
**PH-S436 ✅ (code):** `PayoutBatchLedgerEntry` stub on cleared settlement path.
**PH-S437 ✅ (code):** `enqueue_verification_checker` + `galaxy_verification_checker_enqueue_total`.
**PH-S438 ✅ (code):** `record_replay_verification_enqueue` on mismatch replay path.
**PH-S439 ✅ (code):** `galaxy_capability_doc.rs` — signed capability document parse/validate stub.
**PH-S440 ✅ (tests):** `discovery_network_profile_integration` — `network_profile` retained across `heartbeat-remote`.
**PH-S441 ✅ (code/ui):** `buildMetricHistoryQuery` wasm; `admin_charts.js` wasm-first fetch path.
**PH-S442 ✅ (tests):** `poolai-http-stand-smoke` — executor/checker/replay enqueue `/metrics` shape (PH-S434…S438).
**PH-S443 ✅ (ops/docs):** `poolai-loc-audit` **94.36%**; FM/HANDOFF/NEXT; `poolai-vision-sync` **rev 230**.
**PH-S444 ✅ (code):** `fetch_seed_shards_hook` + `galaxy_prefetch_seed_fetch_*` on memory store lookup (Galaxy §5.5).
**PH-S445 ✅ (code):** `check_strict_locality_gate` → `locality_unsatisfied` + `galaxy_locality_unsatisfied_total` (Galaxy §5.6).
**PH-S446 ✅ (code):** `PrefetchTrigger::CoAccessGraph` + `plan_co_access_prefetch` + `galaxy_prefetch_co_access_total`.
**PH-S447 ✅ (code):** `GalaxyVerificationReplayRecord` on mismatch enqueue path (Galaxy §6.3).
**PH-S448 ✅ (code):** optional `capability_document` on `POST /api/v1/discovery/register-remote`; `discovery_capability_document_integration`.
**PH-S449 ✅ (code):** `poolai_protocol_negotiation_rejected_total` in protocol middleware + register-remote.
**PH-S450 ✅ (code/wasm):** `renderMlPipelineMetricsPanel` wasm-first; slim `admin_charts.js`.
**PH-S451 ✅ (tests):** `poolai-http-stand-smoke` — S444…S449 `/metrics` export shape.
**PH-S452 ✅ (ops/docs):** `rust_ratio.json` **94.37%**; FM/HANDOFF/NEXT canon.
**PH-S453 ✅ (ops):** `poolai-vision-sync` **rev 232** + `--check` green.
**PH-S454 ✅ (code):** `re_migrate_prefetch_stub` on Migrating→Leased PATCH; `galaxy_prefetch_re_migrate_total` (Galaxy §5.5).
**PH-S455 ✅ (code):** `POOLAI_GALAXY_VERIFY_ELEVATED_RATE` + `galaxy_verification_elevated_applied_total` post-mismatch (Galaxy §6.2).
**PH-S456 ✅ (code):** verification trust deltas `+10`/`-100` + `galaxy_trust_score_delta_total` (Galaxy §6.5).
**PH-S457 ✅ (code):** `POOLAI_GALAXY_REPLICATION_MAX_PER_HOUR` rate-limit gate + `galaxy_replication_rate_limited_total`.
**PH-S458 ✅ (code):** `galaxy_hot_promote_total` / `galaxy_hot_evict_total` on prefetch complete path (Galaxy §5.4).
**PH-S459 ✅ (code):** `galaxy_shard_access_total` + `galaxy_prefetch_queue_depth` telemetry stubs (Galaxy §5.3).
**PH-S460 ✅ (code/tests):** `GET /api/v1/grid/verification-replay` + `grid_verification_replay_integration`.
**PH-S461 ✅ (code/wasm):** `renderMonitoringAlertsPanel` wasm-first; slim `monitoring.rs`.
**PH-S462 ✅ (tests):** `poolai-http-stand-smoke` — S454…S460 `/metrics` + verification-replay smoke.
**PH-S463 ✅ (ops):** `rust_ratio.json` **94.38%**; FM/HANDOFF/NEXT; `poolai-vision-sync` **rev 235** + `--check`.
**PH-S464 ✅ (code):** prefetch bandwidth backpressure — `POOLAI_GALAXY_PREFETCH_MIN_BANDWIDTH_MBPS` + `galaxy_prefetch_backpressure_total`; `enqueue_prefetch_hook` gate; unit tests.
**PH-S465 ✅ (code):** RAID artifact prefetch fetch stub — `fetch_seed_shards_from_raid_hook` + `galaxy_prefetch_raid_fetch_*`; wired in `complete_prefetch_hook`.
**PH-S466 ✅ (code):** capability document ed25519 verify stub — `verify_capability_signature_stub` + dev fixture `tests/fixtures/capability/dev_pubkey.hex`.
**PH-S467 ✅ (code):** `GET /api/v1/grid/payout-batch` read API + `last_payout_batch_ledger_entry`; `tests/grid_payout_batch_integration.rs`; OpenAPI sync.
**PH-S468 ✅ (code):** `poolai_protocol_negotiation_accepted_total` on register-remote `CompatStatus::Accepted`; Prometheus export.
**PH-S469 ✅ (code):** `POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON` → `co_access_graph_from_env`; `plan_co_access_prefetch` env override; unit tests.
**PH-S470 ✅ (code/ui):** `renderMonitoringDashboardsPanel` wasm; slim `monitoring.rs` dashboards table; `poolai-ui-core` + `admin_charts.js`.
**PH-S471 ✅ (tests):** `tests/galaxy_horizon_s464_integration.rs` — `/metrics` shape PH-S464…S469 band.
**PH-S472 ✅ (tests):** `poolai-http-stand-smoke` — `galaxy_horizon_wire_s464_metrics` + `grid_payout_batch` smoke.
**PH-S473 ✅ (ops):** `rust_ratio.json` **94.39%**; FM/HANDOFF/NEXT; `poolai-vision-sync` **rev 235** + `--check`.
**PH-S474 ✅ (code):** prefetch `lan_only` cross-region egress guardrail — `POOLAI_GALAXY_COORDINATOR_REGION` / `POOLAI_GALAXY_PREFETCH_PEER_*` + `galaxy_prefetch_egress_blocked_total`; `prefetch_enqueue_blocked` gate; unit tests.
**PH-S475 ✅ (code):** `telegram_seat_service` — `POOLAI_TELEGRAM_SEAT_LIMIT`; register-remote **409** `seat_exhausted` for `origin=telegram_edge`; `tests/telegram_seat_integration.rs`.
**PH-S476 ✅ (code):** `POOLAI_GALAXY_CAPABILITY_VERIFY_PK_HEX` env override in `verify_capability_signature_stub` (PH-S466 extension).
**PH-S477 ✅ (code):** `GET /api/v1/grid/payout-batch/history?limit=N` — ring buffer ledger; OpenAPI + integration test.
**PH-S478 ✅ (code):** `GET /api/v1/grid/verification-replay/history?limit=N` — replay record ring buffer; OpenAPI + integration test.
**PH-S479 ✅ (code):** `fetch_seed_shards_from_peer_hook` + `galaxy_prefetch_peer_fetch_*` metrics; `coordinator_seed_inventory_snapshot` resolve path.
**PH-S480 ✅ (code/ui):** `renderWorkersPanel` wasm; slim `workers.rs` table; `poolai-ui-core` + `admin_charts.js`.
**PH-S481 ✅ (tests):** `tests/galaxy_horizon_s474_integration.rs` — `/metrics` + history API shape PH-S474…S479 band.
**PH-S482 ✅ (ops):** `poolai_http_stand_smoke` — S474 metrics + payout/replay history API smoke.
**PH-S483 ✅ (ops):** `rust_ratio.json` **94.39%**; FM/HANDOFF/NEXT; `poolai-vision-sync` **rev 236** + `--check`.
**PH-S484 ✅ (code):** live prefetch bytes pull — `galaxy_prefetch_pull_bytes_total` on `fetch_seed_shards_hook`; `/metrics` gauge; unit tests.
**PH-S485 ✅ (code):** locality rank → grid schedule bind — `ingest_job_locality_rank_stub` result passed to `schedule_with_grid_peer`; integration band.
**PH-S486 ✅ (code):** `POOLAI_TELEGRAM_SEAT_POLICY` (`bound_wallet_session`) + `compute_seat_limit`; extends PH-S475 flat cap; unit tests.
**PH-S487 ✅ (code):** `POOLAI_GALAXY_HOT_PROMOTE_THRESHOLD` gates `record_hot_promote` in `complete_prefetch_hook`; unit tests.
**PH-S488 ✅ (code):** `enqueue_verification_checker_task` + in-process task record on grid result sample path; unit tests.
**PH-S489 ✅ (code):** `galaxy_network_profile_store` — persist `network_profile` on register-remote; `POOLAI_GALAXY_NETWORK_PROFILE_DATA_DIR`; unit tests.
**PH-S490 ✅ (code/ui):** `renderInstancesPanel` wasm; slim `instances.rs`; `poolai-ui-core` + `admin_charts.js`.
**PH-S491 ✅ (tests):** `tests/galaxy_horizon_s484_integration.rs` — `/metrics` shape PH-S484…S489 band.
**PH-S492 ✅ (ops):** `poolai_http_stand_smoke` — `galaxy_horizon_wire_s484_metrics` (`galaxy_prefetch_pull_bytes_total`).
**PH-S493 ✅ (ops):** `rust_ratio.json` **94.41%**; FM/HANDOFF/NEXT; `poolai-vision-sync` **rev 237** + `--check`.
**PH-S494 ✅ (code):** `GET /api/v1/grid/verification-checker/tasks`; OpenAPI + `grid.rs` tests.
**PH-S495 ✅ (code):** `drain_verification_checker_task` on grid result match/mismatch wire.
**PH-S496 ✅ (code):** `galaxy_verification_checker_pending_total` on `/metrics`.
**PH-S497 ✅ (code):** `GET /api/v1/grid/network-profiles/{peer_id}` read API; OpenAPI sync.
**PH-S498 ✅ (code):** `register-remote` hydrates persisted `network_profile` when metadata absent.
**PH-S499 ✅ (code/ui):** `renderVmPanel` wasm; slim `vm.rs`; `poolai-ui-core` + `admin_charts.js`.
**PH-S500 ✅ (tests):** `tests/galaxy_horizon_s494_integration.rs` — `/metrics` + read APIs PH-S494…S499 band.
**PH-S501 ✅ (ops):** `poolai_http_stand_smoke` — checker tasks + network profile + `galaxy_verification_checker_pending_total`.
**PH-S502 ✅ (ops):** `rust_ratio.json` **94.41%**; FM/HANDOFF/NEXT.
**PH-S503 ✅ (ops):** `poolai-vision-sync` **rev 238** + `--check`.
**PH-S504 ✅ (code):** mandatory signed `capability_document` for `telegram_edge` on register-remote; `validate_telegram_edge_capability`.
**PH-S505 ✅ (code):** `GET /api/v1/grid/telegram-seats` — seat coordinator snapshot (Galaxy §3.1).
**PH-S506 ✅ (code):** `PUT /api/v1/grid/network-profiles/{peer_id}` — profile upsert + GET round-trip.
**PH-S507 ✅ (code):** `galaxy_worker_dto` on `GET /api/v1/discovery/virtual-nodes` (`galaxy` field).
**PH-S508 ✅ (ui):** workers admin — Galaxy virtual-nodes panel (origin badges, latency sort); wasm `renderGalaxyVirtualNodesPanel`.
**PH-S509 ✅ (code):** tgbot `/wallet` command → `POST …/telegram/wallet` via coordinator client.
**PH-S510 ✅ (code):** wallet rebind cooldown — `409 wallet_rebind_cooldown`; `POOLAI_TELEGRAM_WALLET_REBIND_COOLDOWN_SECS`.
**PH-S511 ✅ (code):** `evaluate_semantic_hash_verification` on grid result ingest (non_deterministic task_profile).
**PH-S512 ✅ (ui):** `/ui/admin/grid-verification` read-only checker tasks panel; wasm `renderGridVerificationPanel`.
**PH-S513 ✅ (ops):** `galaxy_horizon_s504_integration` + stand smoke (telegram-seats, network-profile PUT) + loc-audit + vision-sync.
**PH-S514 ✅ (code):** tgbot `/status` → `GET /api/v1/grid/telegram-seats` snapshot (`fetch_telegram_seats`).
**PH-S515 ✅ (code):** tgbot `/stop` → `DELETE …/telegram/bindings/{telegram_user_id}` unbind client.
**PH-S516 ✅ (code):** `galaxy_worker_dto` — `capabilities` + `seed_inventory` on virtual-nodes list.
**PH-S517 ✅ (ui):** `/ui/admin/telegram-seats` read-only panel; wasm `renderTelegramSeatsPanel`.
**PH-S518 ✅ (code):** `lease_failover.rs` — `POOLAI_JOB_MAX_MIGRATIONS_PER_JOB` + `fail_reason` on expired lease path.
**PH-S519 ✅ (code):** heartbeat-remote refreshes `network_profile.last_measured_at` on peer metadata.
**PH-S520 ✅ (code):** `POOLAI_ALLOWED_BUILD_IDS` allow-list → `403 build_id_rejected` on register-remote.
**PH-S521 ✅ (code):** `PayoutBatchLedgerEntry` fee-split lamports fields on cleared settlement ingest.
**PH-S522 ✅ (code):** `galaxy_worker_health` — consecutive miss → `galaxy_worker_unhealthy_total` metric.
**PH-S523 ✅ (ops):** `galaxy_horizon_s514_integration` + stand smoke + loc-audit + vision-sync.
**PH-S524 ✅ (code):** worker-unhealthy lease failover → `fail_reason=worker-unhealthy`; `jobs_worker_unhealthy_failover_integration`.
**PH-S525 ✅ (code):** scheduler/grid bind skips unhealthy peers; `jobs_scheduler_unhealthy_integration`.
**PH-S526 ✅ (code):** `POOLAI_JOB_MAX_TOTAL_RUNTIME_SECS` max wall-clock runtime cap; `lease_failover.rs` unit tests.
**PH-S527 ✅ (code):** signed capability `expires_at` enforcement; telegram_edge requires expiry (PH-S527).
**PH-S528 ✅ (code):** governance Prometheus gauges (`poolai_release_verify_*`, `poolai_update_notify_pending`); stand smoke.
**PH-S529 ✅ (code):** discovery startup hydrate persisted `network_profile`; `network_profile_hydrate_integration`.
**PH-S530 ✅ (code):** queue starvation failover (`POOLAI_JOB_QUEUE_STARVATION_SECS`, `JobRecord.leased_at`).
**PH-S531 ✅ (code):** `GET /api/v1/grid/payout-batch` → `settlement_mode: offline_batch`.
**PH-S532 ✅ (code/e2e):** `admin_charts.js` wasm-first slim — line/sparkline JS fallbacks removed.
**PH-S533 ✅ (ops):** `galaxy_horizon_s524_integration` + stand smoke + loc-audit + vision-sync.
**PH-S534 ✅ (code):** `submit_shadow_verification_checker_job` → JobStore `local_srv` shadow check on sample.
**PH-S535 ✅ (code):** `submit_replay_verification_job` on mismatch/replay_pending → JobStore `Verifying`.
**PH-S536 ✅ (code):** `enqueue_replication_executor_jobs` — M parallel strict-tier replication jobs.
**PH-S537 ✅ (code):** `fetch_seed_shards_from_peer_http` — peer seed-inventory HTTP pull prefetch.
**PH-S538 ✅ (code):** `PayoutBatchLedgerEntry.payout_pubkey` via `resolve_payout_pubkey` on Cleared.
**PH-S539 ✅ (code):** `emit_settlement_job_rewarded` — NDJSON `JobCompleted` + `payout_lamports` stub.
**PH-S540 ✅ (code):** `check_telegram_edge_capability_admission` — GPU jobs require probe history.
**PH-S541 ✅ (code):** `GalaxyWorkerLimits` cold-mining caps (`max_cpu_pct`, `max_ram_mb`, `max_disk_mb`).
**PH-S542 ✅ (code):** `evaluate_checker_timeout_policy` — retry then `VerificationInconclusive`.
**PH-S543 ✅ (ops):** `galaxy_horizon_s534_integration` + stand smoke + loc-audit + vision-sync.
**PH-S544 ✅ (vision UX):** feed ticker у header — однорядкова marquee + тонкий custom scroll rail (`docs/vision/`).
**PH-S545 ✅ (code):** `galaxy_replication_quorum_gate` — strict-tier executor digest quorum before Cleared; `tests/galaxy_replication_quorum_integration.rs`.
**PH-S546 ✅ (code):** `evaluate_strict_prefetch_timeout` — `prefetch-timeout` under strict_locality + `galaxy_prefetch_timeout_total`.
**PH-S547 ✅ (code):** `LeaseFailReason::CapacityPreemption` + `apply_capacity_preemption_failover` (`lease_failover.rs`).
**PH-S548 ✅ (code):** `rank_workers_by_locality` tie-break `queue_depth` + `pricing_usd_micro` (`galaxy_locality.rs`).
**PH-S549 ✅ (code):** `galaxy_update_policy` — `POOLAI_UPDATE_POLICY` notify tick; hook on `poolai-verify-release` success.
**PH-S550 ✅ (code):** `galaxy_settlement_mode` — `POOLAI_SETTLEMENT_ON_CHAIN=1` → payout-batch `on_chain` + pending.
**PH-S551 ✅ (docs):** Galaxy §8.2 TBD #2 — Telegram cold-mining MVP CPU/RAM/Disk + GPU migration pointer.
**PH-S552 ✅ (code):** `galaxy_trust_score_store` — JSON persist + register-remote metadata hydrate.
**PH-S553 ✅ (code/e2e):** `/ui/admin/payout-batch` read-only panel; unit test in `payout_batch.rs`.
**PH-S554 ✅ (tests):** `galaxy_horizon_s545_integration` close band + loc-audit + vision-sync.
**PH-S555 ✅ (vision UX):** Galaxy map 3D orbit — `map-scene-3d` perspective, **WASD** keys, center **touch pad**; layer stack sync; UI v77 / CSS v73; rev **246**.
**PH-S556 ✅ (vision UX):** True 3D layer projection — `applyMap3DProjection` + `MAP_LAYER_Z_STEP`; WASD W↑S↓A←D→; orbit pad bottom-center; UI v78 / CSS v74; rev **247**.
**PH-S557 ✅ (vision UX):** Gravity solar-system layout — folder mass hubs, multi-ring orbits, orphan rim stars; orbit 2× slower; planes 50% transparent; stack↔map sync; UI v79 / CSS v75; rev **248**.
**PH-S558 ✅ (code):** `GET /api/v1/grid/payout-batch` routing snapshot (`primary_dev_lamports`, `secondary_admin_lamports`, `payout_pubkey`); `tests/grid_payout_batch_integration.rs`; `cargo test-ci`.
**PH-S559 ✅ (code):** `POOLAI_WALLET_VERIFY_DEVNET=1` → devnet verify stub on telegram wallet bind; `tests/telegram_wallet_devnet_verify_integration.rs`.
**PH-S560 ✅ (code):** human-review settlement hold on non-deterministic semantic_hash mismatch; `galaxy_settlement_human_review_total`; dispatch path; unit + integration tests.
**PH-S561 ✅ (code):** `POOLAI_CAPABILITY_VERIFY_KEY` → HTTP 403 on invalid capability_document signature; `tests/discovery_capability_production_verify_integration.rs`.
**PH-S562 ✅ (code):** GPU passthrough gate — `gpu_passthrough` capability required for telegram_edge `inference:gpu`; `galaxy_capability_admission.rs`; unit tests.
**PH-S563 ✅ (code):** `galaxy_network_profile_stale_total` on locality rank stale profile; `/metrics` export; stand smoke.
**PH-S564 ✅ (code/e2e):** `poolai-ui-core/payout_batch.rs` + wasm `renderPayoutBatchPanelHtml`; slim `/ui/admin/payout-batch` JS; Playwright admin smoke.
**PH-S565 ✅ (e2e):** `e2e/tests/vision.spec.ts` solar layout smoke; `/ui/admin/payout-batch` in pa11y matrix (FM-019).
**PH-S566 ✅ (code):** `poolai-ui-core/topology.rs` label helpers + wasm exports; `topology_graph.rs` uses shared crate.
**PH-S567 ✅ (ops):** `tests/galaxy_horizon_s558_integration.rs` horizon close band S558–S566; vision rev **249**; loc-audit.
**PH-S568 ✅ (code):** `galaxy_settlement_onchain` mock RPC submit when `POOLAI_SETTLEMENT_ON_CHAIN=1`.
**PH-S569 ✅ (code):** checker-timeout inconclusive/retry gauges in `prometheus_export` + stand smoke.
**PH-S570 ✅ (code):** `GET /api/v1/grid/network-profiles` list persisted peer ids.
**PH-S571 ✅ (code):** `galaxy_fraud_proof` env hold + `galaxy_fraud_proof_pending_total`.
**PH-S572 ✅ (code):** `tee_attestation` on capability doc + `POOLAI_TEE_ATTEST_REQUIRED` gate.
**PH-S573 ✅ (code):** `POST /admin/security-advisories/{id}/acknowledge` + `poolai_advisory_acknowledged_total`.
**PH-S574 ✅ (tests):** `galaxy_prefetch_peer_http_integration` wiremock peer seed pull.
**PH-S575 ✅ (code):** `table_export_buttons_html` + `exportFilenameFromAria` wasm; slim `admin_common.js`.
**PH-S576 ✅ (code):** `POOLAI_PROTOCOL_SUNSET_MIN` → HTTP 426 on register-remote.
**PH-S577 ✅ (ops):** `galaxy_horizon_s568_integration` close band; vision rev **250**.
**PH-S579 ✅ (vision):** Galaxy map fit-all default zoom; ▶/⏸ auto-orbit (90% WASD) + auto fit; `vision2.webp`; FX tune (`body.vision-fx`); rev **257**.

**PH-S580 ✅ (code):** `galaxy_hot_tier_hit_ratio` gauge on `rank_workers_by_locality` top worker; `/metrics` via `refresh_galaxy_locality_gauges`; unit tests.
**PH-S581 ✅ (tests):** `poolai-http-stand-smoke` — `galaxy_hot_tier_hit_ratio_metrics` on live `/metrics`.
**PH-S582 ✅ (ui):** `/ui/admin/network-profiles` — list + per-peer GET panels; admin smoke.
**PH-S583 ✅ (code):** `heartbeat-remote` optional `metadata.network_profile` → persist; `discovery_network_profile_integration`.
**PH-S584 ✅ (ui):** `/ui/admin/seed-inventory` read-only panel; admin smoke.
**PH-S585 ✅ (e2e):** `e2e/tests/vision.spec.ts` — auto-orbit toggle + fit-all ⌂ smoke (PH-S579).
**PH-S586 ✅ (code/ui):** `GET /api/v1/admin/security-advisories` stub list + `/ui/admin/security-advisories` acknowledge UI.
**PH-S587 ✅ (ui):** `/ui/admin/updates-compat` — `POOLAI_UPDATE_POLICY` + `POOLAI_RELEASE_MANIFEST_URL` readout.
**PH-S588 ✅ (tests):** grid ingest + `POOLAI_GALAXY_CO_ACCESS_GRAPH_JSON` → `galaxy_prefetch_co_access_total`.
**PH-S589 ✅ (ops):** `galaxy_horizon_s580_integration` close band + loc-audit + vision-sync.
**PH-S590 ✅ (vision):** orbit pause RAF; ~30% WASD auto-orbit; `galaxy-bg` pointer-events; controls z-index; `vision.spec.ts` rotY play/pause.
**PH-S591 ✅ (code):** prefetch backpressure from persisted profile `bandwidth_mbps` (`with_prefetch_peer`).
**PH-S592 ✅ (code):** prefetch egress guardrail from profile `egress_policy` + `region`.
**PH-S593 ✅ (tests):** `POST /grid/envelope` GPU job → 403 `gpu_passthrough_required`.
**PH-S594 ✅ (tests):** register-remote TEE attestation required when `POOLAI_TEE_ATTEST_REQUIRED=1`.
**PH-S595 ✅ (code):** `POST /virtual-nodes/telegram/wallet/rebind-override` admin bearer + override metric.
**PH-S596 ✅ (ui):** `/ui/admin/network-profiles` PUT upsert form + `admin.spec.ts` smoke.
**PH-S597 ✅ (tests):** on-chain grid result complete HTTP → mock RPC ack.
**PH-S598 ✅ (a11y):** network-profiles, seed-inventory, security-advisories in `pa11y-ci.sh` + `a11y.spec.ts`.
**PH-S599 ✅ (ops):** `galaxy_horizon_s591_integration` close band + loc-audit + vision-sync `--check`.

**Rules ✅:** **`абракадабра`** — project scan → 10 PH-S* → drain → push; канон [`.cursor/rules/poolai-session-iteration.mdc`](../.cursor/rules/poolai-session-iteration.mdc).
**§5.12:** **0** відкритих — наступна сесія: **`абракадабра`** (project scan → +10 → drain).
**Vision ✅:** rev **259** · rust_ratio **94.67%** · hold **95%** advisory.

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
