# OpenTelemetry tracing (FM-038)

**Status:** HTTP spans via `tower-http` TraceLayer; OTLP export with feature `otel`. Job lease span attribute contract documented (PH-S124); instrumentation — PH-S126.

## HTTP middleware

Every request through `network::start_server` gets a `http.request` span (`http.method`, `http.route`, `otel.name`).

With feature **`otel`**, incoming W3C `traceparent` / `tracestate` headers are linked as parent context.

## Build

```bash
cargo build --release --features enterprise,ml,cloud,otel
```

Default CI / `cargo test-ci` builds **without** `otel` (no extra OTLP deps). OTel integration tests:

```bash
cargo test --test observability_otel --features otel
```

## Export (OTLP HTTP)

| Env | Default | Purpose |
|-----|---------|---------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | — | Collector base URL (e.g. `http://127.0.0.1:4318`); **required** to enable export |
| `OTEL_SERVICE_NAME` | `poolai` | `service.name` resource attribute |
| `RUST_LOG` / `RUST_LOG` via subscriber | `info` | `tracing` filter (unchanged) |

Example (Jaeger OTLP HTTP):

```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://127.0.0.1:4318
export OTEL_SERVICE_NAME=poolai
cargo run --features enterprise,ml,cloud,otel
```

Code: `src/observability/` (`tracing_init.rs`, `http_trace.rs`).

**Metrics (pull model):** FM-043 adds Prometheus text at `GET /metrics` (`feature = prometheus`) — see [`PROMETHEUS_METRICS.md`](./PROMETHEUS_METRICS.md). OTLP and Prometheus are complementary.

## Job lease spans (Galaxy §4.3.1, PH-S124 → PH-S126)

**Contract (PH-S124 ✅):** attribute names and outcomes for coordinator job lease acquire, renew, and reject paths. **Implementation (PH-S126):** `tracing` spans in `src/job/`, `src/observability/`, wired from HTTP handlers and internal call sites (`feature = otel`).

Lease spans are **child spans** of the active `http.request` span on API routes, or standalone spans when lease logic runs inside the scheduler or grid ingest (no HTTP parent).

### Span names

| Span name | When emitted | Primary call sites |
|-----------|--------------|-------------------|
| `job.lease.acquire` | Lease holder assigned or epoch bumped | `acquire_lease_on_record`, `maybe_acquire_lease_on_schedule`, `POST /api/v1/jobs/{id}/lease`, grid Job ingest (`schedule_with_grid_peer`) |
| `job.lease.renew` | Active lease TTL extended (epoch unchanged) | `renew_lease_on_record`, `POST /api/v1/jobs/{id}/lease/renew`, `poolai-worker` `LeaseRenewGuard` ticker |
| `job.lease.reject` | Acquire/renew/CAS validation fails before mutation | Same entry points as acquire/renew; also `PATCH /api/v1/jobs/{id}` optional `lease_epoch`, grid `Result` ingest `check_grid_result_lease_epoch` |

On **success**, set `job.lease.outcome = success` on the acquire/renew span (no separate reject span). On **failure**, end the operation span with `job.lease.outcome` ≠ `success` **or** emit `job.lease.reject` (PH-S126 picks one pattern; both carry the attributes below).

### Span attributes (`job.lease.*`)

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `job.id` | string | yes | Job identifier (`JobId` wire value) |
| `job.lease.operation` | string | yes | `acquire` \| `renew` \| `patch_cas` \| `grid_result_cas` |
| `job.lease.source` | string | yes | `api` \| `scheduler` \| `grid_ingest` \| `worker_client` |
| `job.lease.outcome` | string | yes | `success` \| `rejected` \| `expired` \| `already_active` \| `no_lease` |
| `job.lease.owner` | string | on acquire success / renew | Resolved holder (`lease_owner` or bound `worker_id` / `vm_id`) |
| `job.lease.epoch` | int | when known | Active epoch after success, or epoch on record at reject time |
| `job.lease.epoch.requested` | int | renew / CAS | Epoch from request body (`RenewJobLeaseRequest`, PATCH, `GridResultBody`) |
| `job.lease.expires_at` | string | on success | ISO-8601 UTC `lease_expires_at` after acquire/renew |
| `job.lease.ttl_secs` | int | acquire / renew | `JobLeaseConfig.lease_ttl_secs` (from `POOLAI_JOB_LEASE_TTL_SECS`) |
| `job.lease.reject.code` | string | on reject | REST `error.code` — see table below |
| `http.status_code` | int | API paths | HTTP status returned to client (inherits from parent `http.request` when applicable) |

Env driving TTL/renew interval: [`HANDOFF_NEW_SESSION.md`](./HANDOFF_NEW_SESSION.md) §2a (`POOLAI_JOB_LEASE_TTL_SECS`, `POOLAI_JOB_LEASE_RENEW_INTERVAL_SECS`). Wire reference: [`POOLAI_GALAXY_GRID.md`](../concept/POOLAI_GALAXY_GRID.md) §4.3.1.

### Reject codes (`job.lease.reject.code`)

| Code | HTTP | Trigger | Typical `job.lease.outcome` |
|------|------|---------|----------------------------|
| `lease_already_active` | 409 | `POST …/lease` while another holder has active lease (`reject_if_active`) | `already_active` |
| `lease_epoch_rejected` | 409 | Renew/PATCH/grid Result epoch ≠ active `lease_epoch` | `rejected` |
| `lease_expired` | 409 | Renew after `lease_expires_at` | `expired` |
| *(validation)* | 400 | Renew without prior acquire (`NoLeaseOnJob`) | `no_lease` |

Grid result CAS uses the same `lease_epoch_rejected` code (PH-S110). Worker renew client stops the ticker on 409 / `lease_epoch_rejected` / `lease_already_active` (PH-S116).

### Routes → span mapping

| Route / path | `job.lease.operation` | `job.lease.source` | Success span | Reject span |
|--------------|----------------------|-------------------|--------------|-------------|
| `POST /api/v1/jobs/{id}/lease` | `acquire` | `api` | `job.lease.acquire` | `job.lease.reject` |
| `POST /api/v1/jobs/{id}/lease/renew` | `renew` | `api` | `job.lease.renew` | `job.lease.reject` |
| `PATCH /api/v1/jobs/{id}` + `lease_epoch` | `patch_cas` | `api` | — (no lease mutation) | `job.lease.reject` on mismatch |
| Scheduler bind (`maybe_acquire_lease_on_schedule`) | `acquire` | `scheduler` | `job.lease.acquire` | — (no-op when active) |
| Grid Job ingest (`dispatch::schedule_with_grid_peer`) | `acquire` | `grid_ingest` | `job.lease.acquire` | — |
| Grid Result ingest (`check_grid_result_lease_epoch`) | `grid_result_cas` | `grid_ingest` | — | `job.lease.reject` |
| `poolai-worker` renew HTTP client | `renew` | `worker_client` | `job.lease.renew` | `job.lease.reject` |

### Example trace shape (API acquire → renew)

```
http.request  http.method=POST  http.route=/api/v1/jobs/{id}/lease
  └─ job.lease.acquire  job.id=…  job.lease.operation=acquire  job.lease.source=api
       job.lease.outcome=success  job.lease.owner=worker-a  job.lease.epoch=1
       job.lease.expires_at=2026-05-29T12:01:30Z  job.lease.ttl_secs=90

http.request  http.method=POST  http.route=/api/v1/jobs/{id}/lease/renew
  └─ job.lease.renew  job.id=…  job.lease.epoch=1  job.lease.epoch.requested=1
       job.lease.outcome=success  job.lease.expires_at=2026-05-29T12:02:00Z
```

Reject example (stale epoch on renew):

```
http.request  http.method=POST  http.route=/api/v1/jobs/{id}/lease/renew
  └─ job.lease.reject  job.lease.operation=renew  job.lease.outcome=rejected
       job.lease.epoch.requested=0  job.lease.epoch=1
       job.lease.reject.code=lease_epoch_rejected  http.status_code=409
```

### PH-S126 implementation notes

- Helper module: `src/observability/lease_trace.rs` (planned) — `#[cfg(feature = "otel")]` span builders; no-op stubs without `otel`.
- Tests: extend `tests/observability_otel.rs` — assert span names/attrs on acquire success + `lease_epoch_rejected` (in-memory subscriber or OTel SDK test exporter).
- Do **not** duplicate Prometheus lease counters here; FM-043 `/metrics` is separate (PH-S127 pricing oracle export).

**Last updated:** 2026-05-29 (PH-S124 lease span attrs contract; FM-038, Galaxy §4.3.1).
