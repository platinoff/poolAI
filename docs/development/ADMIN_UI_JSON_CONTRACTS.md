# Адмін UI: мапінг JSON → екран (контракти)

Документ узгоджений з [UI_ADMIN_GAP_AND_UPGRADE_PLAN_2026-04-06.md](./UI_ADMIN_GAP_AND_UPGRADE_PLAN_2026-04-06.md) (фаза B). Джерело істини для шляхів — `src/network/`, для UI — `src/ui/admin/*.rs`.

## Загальні примітки

- Публічний REST: префікс **`/api/v1`**. Enterprise: **`/api/enterprise`** (потрібен збір/запуск з `--features enterprise`).
- Помилки часто у формі `{ "error": { "code", "message" }, "context"? }` (`json_errors.rs`). UI читає це через `apiErrorMessageFromBody` / `apiErrorDetailFromBody` у `admin_common.js`.

---

## Сторінки та поля

| Сторінка | Файл UI | Основні запити | Поля відповіді → UI |
|----------|---------|----------------|---------------------|
| Dashboard | `dashboard.rs` | `GET /api/v1/admin/overview` | `status`, `uptime_seconds` → огляд; `workers`, `workers_total`, `vm_instances`, `cpu_usage_percent`, `memory_usage_mb` → швидка статистика (`AdminOverview` у `admin_service.rs`). |
| | | `GET /api/enterprise/monitoring/alerts?…` | Масив `Alert`: `severity`, `metric`, `current_value` → список алертів. |
| | | `GET /api/enterprise/audit/events?…` | Масив `AuditEvent`: `timestamp`, `action` → активність. |
| | | `GET /api/enterprise/monitoring/metrics?…` | Масив точок: `metric`, `value` → графіки (очікується `cpu_usage`, `memory_usage`). |
| Workers | `workers.rs` | `GET /api/v1/workers` | `id` / `worker_id`, `is_healthy`, `total_requests_processed` → таблиця. |
| VM | `vm.rs` | `GET /api/v1/vm/instances` | `name`, `status` (рядок), `resources.cpu_cores`, `resources.memory_mb`, `id` → таблиця та дії. |
| Libraries | `libs.rs` | `GET /api/v1/libraries` | `name`, `version`, `metadata` (об’єкт; **`metadata.installed_at`** → статус «Installed» у UI); опційно `installed` у майбутніх DTO. 503 → `{ "error": … }`. |
| Instances | `instances.rs` | `GET /api/v1/instance` | `{ "instances": [ { instance_id, model_id, status, created_at, placement: { strategy, node_ids, … } } ] }`; 503 без `instance_manager`. |
| Topology | `topology.rs` | `GET /api/v1/topology`, `/nodes`, `/latency` | `node_count`, `latency_measurements`, `last_updated`; вузли: `nodes.{id}.available_gpu_memory_mb`, `total_gpu_memory_mb`, `available_cpu_cores`, `current_load`. |
| Config | `config.rs` | `GET /api/v1/config` | `system`, `gpu`, `pool`, `monitoring`, `version`, `health`, `https`; UI читає `config.system.name`, `log_level`, `max_workers`, … (потрібен `initialize_config` при старті). |
| Users | `users.rs` | `GET /api/v1/users` | `id`, `username`, `role`, `active`, `created_at` (масив). |
| RAID | `raid.rs` | `GET /api/v1/raid/artifacts`, snapshot | Масив артефактів: `id`, `name`, `stored_at`, `path` (`ArtifactRef`). |
| | | `GET /api/v1/raid/admin/status` | Обгортка **`{ "status": { "mode", "initialized", "active", "rebalancing_enabled" } }`**. |
| | | `GET …/metrics/burst`, `…/smallworld` | **`{ "metrics": { … } }`**: burst — `total_artifacts`, `artifacts_in_burst`, `base_replication_factor`, `max_replication_factor`; smallworld — `total_artifacts`, `total_nodes`, `avg_clustering_coefficient`, `target_clustering_coefficient`. |
| Tenants | `tenants.rs` | `/api/enterprise/tenants` | CRUD тенантів — див. enterprise tenants handler. |
| Security | `security.rs` | `/api/enterprise/security/...` | OAuth2 / SAML / політики — відповіді з `enterprise_api/security.rs`. |
| Audit | `audit.rs` | `GET /api/enterprise/audit/events?…` | `timestamp`, `level`, `user_id`, `action`, `resource_type`, `resource_id`, `result`. |
| Monitoring | `monitoring.rs` | `/api/enterprise/monitoring/...` | Алерти, метрики, правила — див. `enterprise_api/monitoring.rs`. |

---

## Sync Artifacts (RAID distributed)

`POST /api/v1/raid/distributed/artifacts/sync` приймає тіло **`ProtocolMessage`** (`src/raid/protocol.rs`):

- Обов’язкові поля верхнього рівня: `type` (`"sync_artifacts"`), `id`, `timestamp` (ISO-8601), `node_id`, `payload`.
- `payload` для sync: **`SyncArtifactsPayload`** — мінімально достатньо `{ "direction": "bidirectional" | "pull" | "push" }`; опційно `last_sync_timestamp`, `artifact_ids`.

Приклад для адмін-кнопки (узгоджено з `admin/raid.rs`):

```json
{
  "type": "sync_artifacts",
  "id": "uuid",
  "timestamp": "2026-04-07T12:00:00.000Z",
  "node_id": "ui-admin",
  "payload": { "direction": "bidirectional" }
}
```
