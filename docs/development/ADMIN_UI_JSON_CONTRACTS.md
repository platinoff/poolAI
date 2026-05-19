# Адмін UI: мапінг JSON → екран (контракти)

**Оновлено:** 2026-05-19 (S26 — metrics, alert-rules, SAML, security policies; P1 ✅).

Документ узгоджений з [UI_ADMIN_GAP_AND_UPGRADE_PLAN_2026-04-06.md](./UI_ADMIN_GAP_AND_UPGRADE_PLAN_2026-04-06.md) (фаза B). Джерело істини для шляхів — `src/network/`, для UI — `src/ui/admin/*.rs`.

**Contract tests:** `tests/admin_ui_api_contracts.rs` (`cargo test-ci`, feature `enterprise`). Колонка **Тест** — ім’я `async fn` у файлі.

## Загальні примітки

- Публічний REST: префікс **`/api/v1`**. Enterprise: **`/api/enterprise`** (потрібен збір/запуск з `--features enterprise`).
- Помилки часто у формі `{ "error": { "code", "message" }, "context"? }` (`json_errors.rs`). UI читає це через `apiErrorMessageFromBody` / `apiErrorDetailFromBody` у `admin_common.js`.

---

## Сторінки та поля

| Сторінка | Файл UI | Основні запити | Поля відповіді → UI | Тест |
|----------|---------|----------------|---------------------|------|
| Dashboard | `dashboard.rs` | `GET /api/v1/admin/overview` | `status`, `uptime_seconds` → огляд; `workers`, `workers_total`, `vm_instances`, `cpu_usage_percent`, `memory_usage_mb` → швидка статистика (`AdminOverview` у `admin_service.rs`). | `admin_overview_includes_dashboard_keys` |
| | | `GET /api/enterprise/monitoring/alerts?…` | Масив `Alert`: `severity`, `metric`, `current_value`, `threshold`, `triggered_at`, `acknowledged` → список алертів. | `enterprise_monitoring_alerts_json_shape` |
| | | `GET /api/enterprise/audit/events?…` | Масив `AuditEvent`: `timestamp`, `action`, `level`, `result` → активність. | `enterprise_audit_events_json_shape` |
| | | `GET /api/enterprise/monitoring/metrics?…` | Масив точок: `metric`, `value`, `timestamp` → графіки (очікується `cpu_usage`, `memory_usage`). | `enterprise_monitoring_metrics_json_shape` |
| Workers | `workers.rs` | `GET /api/v1/workers` | `id` / `worker_id`, `is_healthy`, `total_requests_processed` → таблиця. | `workers_list_json_shape_for_admin` |
| VM | `vm.rs` | `GET /api/v1/vm/instances` | `name`, `status` (рядок), `resources.cpu_cores`, `resources.memory_mb`, `id` → таблиця та дії. | `vm_instances_*` |
| Libraries | `libs.rs` | `GET /api/v1/libraries` | `name`, `version`, `metadata` (об’єкт; **`metadata.installed_at`** → статус «Installed» у UI); опційно `installed` у майбутніх DTO. 503 → `{ "error": … }`. | `libraries_list_*` |
| Instances | `instances.rs` | `GET /api/v1/instance` | `{ "instances": [ { instance_id, model_id, status, created_at, placement: { strategy, node_ids, … } } ] }`; 503 без `instance_manager`. | `instance_list_*` |
| Topology | `topology.rs` | `GET /api/v1/topology`, `/nodes`, `/latency` | `node_count`, `latency_measurements`, `last_updated`; вузли: `nodes.{id}.available_gpu_memory_mb`, `total_gpu_memory_mb`, `available_cpu_cores`, `current_load`. | `topology_*` |
| Config | `config.rs` | `GET /api/v1/config` | `system`, `gpu`, `pool`, `monitoring`, `version`, `health`, `https`; UI читає `config.system.name`, `log_level`, `max_workers`, … (потрібен `initialize_config` при старті). | `config_get_json_shape_for_admin` |
| Users | `users.rs` | `GET /api/v1/users` | `id`, `username`, `role`, `active`, `created_at` (масив). | `users_list_json_shape_for_admin` |
| RAID | `raid.rs` | `GET /api/v1/raid/artifacts`, snapshot | Масив артефактів: `id`, `name`, `stored_at`, `path` (`ArtifactRef`). | `raid_artifacts_*` |
| | | `GET /api/v1/raid/admin/status` | Обгортка **`{ "status": { "mode", "initialized", "active", "rebalancing_enabled" } }`**. | `raid_admin_status_json_shape` |
| | | `GET …/metrics/burst`, `…/smallworld` | **`{ "metrics": { … } }`**: burst — `total_artifacts`, `artifacts_in_burst`, `base_replication_factor`, `max_replication_factor`; smallworld — `total_artifacts`, `total_nodes`, `avg_clustering_coefficient`, `target_clustering_coefficient`. | `raid_admin_burst_metrics_*`, `raid_admin_smallworld_metrics_*` |
| Tenants | `tenants.rs` | `GET /api/enterprise/tenants` | Масив `Tenant`: `id`, `name`, `config` (`active`, `max_workers`, `max_memory_mb`, …), `usage` (`workers`, `memory_mb`, …). | `enterprise_tenants_list_json_shape` |
| Security | `security.rs` | `GET /api/enterprise/security/oauth2/providers` | Масив: `name`, `enabled`, `config.client_id`, `config.authorization_url`, … | `enterprise_oauth2_providers_json_shape` |
| | | `GET /api/enterprise/security/saml/providers` | Масив: `name`, `enabled`, `config.entity_id`, `config.sso_url`, … | `enterprise_saml_providers_json_shape` |
| | | `GET /api/enterprise/security/policies` | Масив: `name`, `description`, `require_mfa`, `session_timeout`, `max_failed_attempts`. | `enterprise_security_policies_json_shape` |
| Audit | `audit.rs` | `GET /api/enterprise/audit/events?…` | `timestamp`, `level`, `user_id`, `action`, `resource_type`, `resource_id`, `result`. | `enterprise_audit_events_json_shape` |
| Monitoring | `monitoring.rs` | `GET /api/enterprise/monitoring/dashboards` | `id`, `name`, `description`, `metrics[]`, `is_public`, `created_at`. | `enterprise_monitoring_dashboards_json_shape` |
| | | `GET /api/enterprise/monitoring/alert-rules` | `name`, `metric`, `operator`, `threshold`, `severity`, `enabled`. | `enterprise_monitoring_alert_rules_json_shape` |
| | | alerts | Див. `enterprise_api/monitoring.rs`. | `enterprise_monitoring_alerts_json_shape` |

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
