# Admin UI field audit (FM-040)

**Дата:** 2026-05-23 · **Статус:** ✅ закрито (контрактні тести + маніфест).

Джерела: `src/ui/admin/*.rs` (13 сторінок), `tests/admin_ui_api_contracts.rs`, [`ADMIN_UI_JSON_CONTRACTS.md`](./ADMIN_UI_JSON_CONTRACTS.md).

## Покриття сторінок

| Сторінка | UI файл | API (основні) | Contract test |
|----------|---------|---------------|---------------|
| Dashboard | `dashboard.rs` | `GET /admin/overview` | `admin_overview_includes_dashboard_keys` |
| Workers | `workers.rs` | `GET /workers` | `workers_list_json_shape_for_admin` |
| VM | `vm.rs` | `GET /vm/instances` | `vm_instances_*` |
| Libraries | `libs.rs` | `GET /libraries` | `libraries_list_*` (+ `metadata.installed_at`) |
| Instances | `instances.rs` | `GET /instance`, previews, `GET /instance/{id}` | `instance_list_*`, **`instance_previews_*`**, **`instance_get_*`** |
| Topology | `topology.rs` | `/topology`, `/nodes`, `/latency`, `/nodes/{id}` | `topology_*`, **`topology_latency_*`**, **`topology_node_detail_*`** |
| Config | `config.rs` | `GET /config` | `config_get_json_shape_for_admin` |
| Users | `users.rs` | `GET /users` | `users_list_json_shape_for_admin` |
| RAID | `raid.rs` | artifacts, admin status, burst/smallworld metrics | `raid_*` |
| Tenants | `tenants.rs` | `GET /enterprise/tenants` | `enterprise_tenants_list_json_shape` |
| Security | `security.rs` | oauth2/saml/policies | `enterprise_*_providers_json_shape`, `enterprise_security_policies_*` |
| Audit | `audit.rs` | `GET /enterprise/audit/events` | `enterprise_audit_events_json_shape` (+ `user_id`, `resource_id`) |
| Monitoring | `monitoring.rs` | dashboards, alert-rules, alerts, metrics | `enterprise_monitoring_*` |

## FM-040 — додані перевірки

- **Instances:** `GET /instance/previews` → `sharding`, `memory_delta_by_node`; `GET /instance/{id}` → `placement.strategy` (string), `node_ids`.
- **Topology:** вкладені поля вузла (`available_gpu_memory_mb`, `total_cpu_cores`, `current_load`); `GET /topology/latency` → `latency_matrix`; `GET /topology/nodes/{id}` → detail modal.
- **Libraries:** `metadata` як object; опційно `metadata.installed_at` (статус Installed у UI).
- **Audit:** `user_id`, `resource_id` у рядку таблиці audit page.

## Поза scope UI (окремі FM)

- **Jobs API** — `tests/jobs_api_contracts.rs` (FM-026); адмін-сторінки jobs немає.
- **Grid / Memory** — wire API без dedicated admin page.

## Перевірка

```bash
export K8S_OPENAPI_ENABLED_VERSION=1.28
cargo test --test admin_ui_api_contracts --features ml,enterprise,cloud,test-utils
cargo test-ci
```
