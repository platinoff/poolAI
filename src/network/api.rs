// network/api.rs
use crate::libs::{get_global_manager, LibraryType};
use crate::network::auth::{auth_middleware, authenticate_user, AuthRequest, Claims};
use crate::network::ws::websocket_handler;
use crate::platform;
use crate::raid;
use crate::rewards::{get_reward_statistics, get_top_users, get_user_progress, get_user_rewards};
use crate::vm;
use axum::extract::Extension;
use axum::{
    http::header::ACCEPT,
    middleware,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Serialize;

use crate::network::raid_distributed_handlers::*;

#[derive(Serialize)]
struct StatusResponse {
    status: &'static str,
    version: &'static str,
    uptime: u64,
}

#[derive(Serialize)]
struct MetricsResponse {
    active_workers: u32,
    total_requests: u64,
    avg_response_time: f64,
}

#[derive(Serialize)]
struct ModelInfo {
    name: &'static str,
    status: &'static str,
    memory_usage: u64,
}

#[derive(Serialize)]
struct WorkerInfo {
    id: &'static str,
    status: &'static str,
    current_task: Option<&'static str>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: String,
    version: &'static str,
    uptime: u64,
    checks: HealthChecks,
}

#[derive(Serialize)]
struct HealthChecks {
    database: HealthCheck,
    memory: HealthCheck,
    workers: HealthCheck,
    gpu: HealthCheck,
}

#[derive(Serialize)]
struct HealthCheck {
    status: &'static str,
    message: String,
    response_time_ms: u64,
}

pub fn create_api_routes() -> Router {
    Router::new()
        .route("/status", get(status_handler))
        .route("/health", get(health_handler))
        .route("/login", post(login_handler))
        .route("/metrics", get(metrics_handler))
        .route("/models", get(models_handler))
        .route("/workers", get(workers_handler))
        .route("/gpu", get(gpu_info))
        .route("/ws/metrics", get(websocket_handler))
        .route("/rewards", get(rewards_handler))
        .route("/rewards/:user_id", get(user_rewards_handler))
        .route("/rewards/progress/:user_id", get(user_progress_handler))
        .route("/rewards/statistics", get(rewards_statistics_handler))
        .route("/rewards/top", get(top_users_handler))
        .route("/libraries", get(libraries_list_handler))
        .route("/libraries/:name", get(library_info_handler))
        // Write endpoints with auth middleware
        .route(
            "/libraries/:name/install",
            post(library_install_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/libraries/:name/uninstall",
            post(library_uninstall_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/libraries/:name/update",
            post(library_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/vm/instances", get(vm_instances_handler))
        .route(
            "/vm/instances",
            post(vm_instance_create_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/vm/instances/:id",
            put(vm_instance_update_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/vm/instances/:id",
            delete(vm_instance_delete_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/vm/instances/:id/start",
            post(vm_instance_start_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/vm/instances/:id/stop",
            post(vm_instance_stop_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route(
            "/vm/instances/:id/restart",
            post(vm_instance_restart_handler).layer(middleware::from_fn(auth_middleware)),
        )
        .route("/vm/instances/:id/health", get(vm_instance_health_handler))
        .route(
            "/vm/instances/:id/resources",
            get(vm_instance_resources_handler),
        )
        .route(
            "/vm/resource-limits-supported",
            get(vm_resource_limits_supported_handler),
        )
        .route("/raid/nodes", get(raid_nodes_handler))
        .route("/raid/artifacts", get(raid_artifacts_handler))
        .route("/raid/quota", get(raid_quota_handler))
        .route("/raid/events", get(raid_events_handler))
        .route("/raid/events/:artifact_id", get(raid_events_for_artifact_handler))
        .route("/raid/events/range", get(raid_events_range_handler))
        .route("/raid/snapshot", get(raid_snapshot_handler))
        .route("/raid/snapshot/create", post(raid_snapshot_create_handler).layer(middleware::from_fn(auth_middleware)))
        .route(
            "/raid/gc",
            post(raid_gc_handler).layer(middleware::from_fn(auth_middleware)),
        )
        // Distributed RAID Protocol endpoints
        .route(
            "/raid/distributed/artifacts/replicate",
            post(put_artifact_handler),
        )
        .route(
            "/raid/distributed/artifacts/get",
            post(get_artifact_handler),
        )
        .route(
            "/raid/distributed/artifacts/delete",
            post(delete_artifact_handler),
        )
        .route(
            "/raid/distributed/artifacts/sync",
            post(sync_artifacts_handler),
        )
        .route("/raid/distributed/health", post(health_check_handler))
        .route("/raid/distributed/cluster/join", post(join_cluster_handler))
        .route(
            "/raid/distributed/cluster/leave",
            post(leave_cluster_handler),
        )
}

async fn vm_instances_handler() -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let instances = manager.list_instances().await;
    Json(instances)
}

async fn vm_instance_resources_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager.get_instance_resource_usage(uuid).await {
        Ok(usage) => Json(usage).into_response(),
        Err(e) => {
            let error_response = serde_json::json!({
                "error": e.to_string()
            });
            (axum::http::StatusCode::NOT_FOUND, Json(error_response)).into_response()
        }
    }
}

async fn vm_resource_limits_supported_handler() -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let supported = manager.is_resource_limits_supported();
    Json(serde_json::json!({
        "supported": supported
    }))
}

// Helper function to check RBAC permissions
fn check_permission(
    claims: &Claims,
    required_permission: &str,
) -> Result<(), (axum::http::StatusCode, Json<serde_json::Value>)> {
    if !claims
        .permissions
        .contains(&required_permission.to_string())
    {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "Insufficient permissions",
                "required": required_permission,
                "user_permissions": claims.permissions
            })),
        ));
    }
    Ok(())
}

// VM instance write operations with RBAC

#[derive(serde::Deserialize)]
struct VmCreateRequest {
    name: String,
    resources: vm::VmResources,
    isolation: Option<vm::VmIsolation>,
}

async fn vm_instance_create_handler(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<VmCreateRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let isolation = payload.isolation.unwrap_or(vm::VmIsolation::ProcessSandbox);

    match manager
        .create_instance(payload.name, payload.resources, isolation)
        .await
    {
        Ok(instance) => Json(instance).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to create VM instance: {}", e)
            })),
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
struct VmUpdateRequest {
    name: Option<String>,
    resources: Option<vm::VmResources>,
    isolation: Option<vm::VmIsolation>,
}

async fn vm_instance_update_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<VmUpdateRequest>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager
        .update_instance(uuid, payload.name, payload.resources, payload.isolation)
        .await
    {
        Ok(instance) => Json(instance).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to update VM instance: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_delete_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check permission: delete:all or write:vm
    if let Err(err) =
        check_permission(&claims, "delete:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager.delete_instance(uuid).await {
        Ok(_) => Json(serde_json::json!({
            "message": format!("VM instance {} deleted successfully", id)
        }))
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to delete VM instance: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_start_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager.start_instance(uuid).await {
        Ok(_) => Json(serde_json::json!({
            "message": format!("VM instance {} started successfully", id)
        }))
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to start VM instance: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_stop_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager.stop_instance(uuid).await {
        Ok(_) => Json(serde_json::json!({
            "message": format!("VM instance {} stopped successfully", id)
        }))
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to stop VM instance: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_restart_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:vm
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:vm"))
    {
        return err.into_response();
    }

    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager.restart_instance(uuid).await {
        Ok(_) => Json(serde_json::json!({
            "message": format!("VM instance {} restarted successfully", id)
        }))
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to restart VM instance: {}", e)
            })),
        )
            .into_response(),
    }
}

async fn vm_instance_health_handler(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let manager = vm::get_global_manager();
    let uuid = match uuid::Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid UUID format"
                })),
            )
                .into_response();
        }
    };

    match manager.get_instance_health(uuid).await {
        Ok(Some(status)) => match status {
            crate::runtime::health::HealthStatus::Healthy => Json(serde_json::json!({
                "status": "healthy"
            }))
            .into_response(),
            crate::runtime::health::HealthStatus::Unhealthy(reason) => Json(serde_json::json!({
                "status": "unhealthy",
                "reason": reason
            }))
            .into_response(),
            crate::runtime::health::HealthStatus::Unknown => Json(serde_json::json!({
                "status": "unknown"
            }))
            .into_response(),
        },
        Ok(None) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Health check not registered for this instance"
            })),
        )
            .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("{}", e)
            })),
        )
            .into_response(),
    }
}

async fn raid_nodes_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    let nodes = manager.list_nodes().await;
    Json(nodes)
}

async fn raid_artifacts_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    let artifacts = manager.list_artifacts().await;
    Json(artifacts)
}

#[derive(serde::Serialize)]
struct RaidQuotaResponse {
    total_size_bytes: u64,
    quota_bytes: Option<u64>,
    usage_percent: Option<f64>,
    artifact_count: usize,
}

async fn raid_quota_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    let total_size = manager.get_total_size().await.unwrap_or(0);
    let artifacts = manager.list_artifacts().await;
    let artifact_count = artifacts.len();

    // Get quota from config (would need to expose it from RaidManager)
    let quota_bytes = None; // TODO: expose from config

    let usage_percent = quota_bytes.map(|quota| {
        if quota > 0 {
            (total_size as f64 / quota as f64) * 100.0
        } else {
            0.0
        }
    });

    Json(RaidQuotaResponse {
        total_size_bytes: total_size,
        quota_bytes,
        usage_percent,
        artifact_count,
    })
}

#[derive(serde::Serialize)]
struct RaidGcResponse {
    removed_count: usize,
}

// Audit log API handlers

/// Get all events from the event store
async fn raid_events_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    
    if let Some(event_store) = manager.event_store() {
        match event_store.read().await.load_events().await {
            Ok(events) => Json(serde_json::json!({
                "events": events,
                "count": events.len()
            })).into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to load events: {}", e)
                })),
            ).into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Event store not available"
            })),
        ).into_response()
    }
}

/// Get events for a specific artifact
async fn raid_events_for_artifact_handler(
    axum::extract::Path(artifact_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let manager = raid::get_global_manager();
    
    if let Some(event_store) = manager.event_store() {
        match event_store.read().await.get_events_for_artifact(&artifact_id).await {
            Ok(events) => Json(serde_json::json!({
                "artifact_id": artifact_id,
                "events": events,
                "count": events.len()
            })).into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to load events: {}", e)
                })),
            ).into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Event store not available"
            })),
        ).into_response()
    }
}

/// Get events in a time range
#[derive(serde::Deserialize)]
struct EventsRangeQuery {
    start: Option<String>, // ISO 8601 timestamp
    end: Option<String>,   // ISO 8601 timestamp
}

async fn raid_events_range_handler(
    axum::extract::Query(params): axum::extract::Query<EventsRangeQuery>,
) -> impl IntoResponse {
    use chrono::DateTime;
    
    let manager = raid::get_global_manager();
    
    if let Some(event_store) = manager.event_store() {
        let start = params.start
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(7));
        
        let end = params.end
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);
        
        match event_store.read().await.get_events_in_range(start, end).await {
            Ok(events) => Json(serde_json::json!({
                "start": start.to_rfc3339(),
                "end": end.to_rfc3339(),
                "events": events,
                "count": events.len()
            })).into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to load events: {}", e)
                })),
            ).into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Event store not available"
            })),
        ).into_response()
    }
}

/// Get current snapshot
async fn raid_snapshot_handler() -> impl IntoResponse {
    let manager = raid::get_global_manager();
    
    if let Some(event_store) = manager.event_store() {
        match event_store.read().await.load_snapshot().await {
            Ok(Some(snapshot)) => Json(snapshot).into_response(),
            Ok(None) => (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "No snapshot available"
                })),
            ).into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to load snapshot: {}", e)
                })),
            ).into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Event store not available"
            })),
        ).into_response()
    }
}

/// Create a new snapshot
async fn raid_snapshot_create_handler(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) = check_permission(&claims, "write:all")
        .or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }
    
    let manager = raid::get_global_manager();
    
    match manager.create_snapshot().await {
        Ok(_) => Json(serde_json::json!({
            "status": "success",
            "message": "Snapshot created successfully"
        })).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to create snapshot: {}", e)
            })),
        ).into_response(),
    }
}

async fn raid_gc_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    // Check permission: write:all or write:raid
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:raid"))
    {
        return err.into_response();
    }
    let manager = raid::get_global_manager();
    match manager.gc_old_artifacts().await {
        Ok(removed) => Json(RaidGcResponse {
            removed_count: removed,
        })
        .into_response(),
        Err(e) => {
            let error_response = serde_json::json!({
                "error": format!("GC failed: {}", e)
            });
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_response),
            )
                .into_response()
        }
    }
}

async fn status_handler(req: axum::http::Request<axum::body::Body>) -> Response {
    let status = StatusResponse {
        status: "running",
        version: "0.1.0",
        uptime: 3600,
    };
    // Check the Accept header
    let want_html = req
        .headers()
        .get(ACCEPT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("text/html"))
        .unwrap_or(false);
    if want_html {
        let html = format!(
            r#"
<!DOCTYPE html>
<html lang='en'>
<head>
  <meta charset='UTF-8'>
  <meta name='viewport' content='width=device-width, initial-scale=1.0'>
  <title>PoolAI Status</title>
  <style>
    body {{ font-family: 'Segoe UI', Arial, sans-serif; background: #181c20; color: #f8f8f2; margin: 0; padding: 0; }}
    .container {{ max-width: 700px; margin: 40px auto; background: #23272e; border-radius: 12px; box-shadow: 0 4px 24px #0008; padding: 32px; }}
    h1 {{ color: #50fa7b; margin-bottom: 0.5em; display: flex; align-items: center; gap: 12px; }}
    .logo {{ width: 40px; height: 40px; vertical-align: middle; }}
    .status {{ font-size: 1.2em; margin-bottom: 1em; }}
    .info-list {{ list-style: none; padding: 0; }}
    .info-list li {{ margin-bottom: 0.5em; }}
    a {{ color: #8be9fd; text-decoration: none; }}
    a:hover {{ text-decoration: underline; }}
    .links {{ margin-top: 2em; }}
    .badge {{ display: inline-block; background: #44475a; color: #f1fa8c; border-radius: 6px; padding: 2px 8px; font-size: 0.9em; margin-left: 8px; }}
    .footer {{ margin-top: 2em; color: #6272a4; font-size: 0.95em; text-align: center; }}
    .api-ref {{ margin-top: 2em; background: #181c20; border-radius: 8px; padding: 18px 20px; border: 1px solid #44475a; }}
    .api-ref h2 {{ color: #8be9fd; margin-top: 0; }}
    .api-ref code {{ background: #282a36; color: #f1fa8c; border-radius: 4px; padding: 2px 6px; }}
    .api-ref li {{ margin-bottom: 0.4em; }}
    .security-info {{ margin-top: 2em; background: #23272e; border: 1px solid #44475a; border-radius: 8px; padding: 14px 18px; }}
    .security-info strong {{ color: #f1fa8c; }}
    .curl-block {{ background: #282a36; color: #f8f8f2; border-radius: 6px; padding: 10px 14px; font-size: 0.98em; margin: 1em 0; font-family: 'Fira Mono', 'Consolas', monospace; }}
    .doc-links {{ margin-top: 1.5em; }}
    .doc-links a {{ margin-right: 18px; }}
  </style>
</head>
<body>
  <div class='container'>
    <h1>
      <img class='logo' src='https://raw.githubusercontent.com/platinoff/poolAI/Bolvanka-Beta-v1--stage2-https/docs/poolai_logo.svg' alt='PoolAI Logo' onerror="this.style.display='none'"/>
      PoolAI Status <span class='badge'>API v1</span>
    </h1>
    <div class='status'>
      <strong>Status:</strong> <span style='color:#50fa7b'>{status}</span><br>
      <strong>Version:</strong> {version}<br>
      <strong>Uptime:</strong> {uptime} seconds
    </div>
    <div class='security-info'>
      <strong>Security:</strong> HTTPS <span style='color:#50fa7b'>enabled</span>, JWT <span style='color:#ffb86c'>planned</span>, CORS <span style='color:#50fa7b'>enabled</span>
      <br><span style='font-size:0.95em'>Self-signed certificate for dev. <b>Never commit private keys to git!</b></span>
    </div>
    <div class='api-ref'>
      <h2>API Reference</h2>
      <ul>
        <li><b>GET</b> <code>/api/v1/status</code> — Server status (HTML/JSON)</li>
        <li><b>GET</b> <code>/api/v1/health</code> — Health check <span style='color:#50fa7b'>✨ NEW!</span></li>
        <li><b>POST</b> <code>/api/v1/login</code> — Authentication <span style='color:#50fa7b'>🔐 NEW!</span></li>
        <li><b>GET</b> <code>/api/v1/metrics</code> — Metrics</li>
        <li><b>GET</b> <code>/api/v1/models</code> — Models</li>
        <li><b>GET</b> <code>/api/v1/gpu</code> — GPU Info</li>
                 <li><b>GET</b> <code>/api/v1/workers</code> — Workers</li>
         <li><b>WS</b> <code>/ws/metrics</code> — Live metrics (WebSocket) <span style='color:#50fa7b'>✨ NEW!</span></li>
         <li><b>GET</b> <code>/api/v1/rewards</code> — Rewards system <span style='color:#ffb86c'>🎁 NEW!</span></li>
         <li><b>GET</b> <code>/api/v1/rewards/:user_id</code> — User rewards</li>
         <li><b>GET</b> <code>/api/v1/rewards/progress/:user_id</code> — User progress</li>
         <li><b>GET</b> <code>/api/v1/rewards/statistics</code> — Rewards statistics</li>
         <li><b>GET</b> <code>/api/v1/rewards/top</code> — Top users</li>
      </ul>
      <div class='curl-block'>
        <b>Example (curl):</b><br>
        <code>curl -k https://localhost:8080/api/v1/status</code>
      </div>
    </div>
    <div class='doc-links'>
      <a href='https://github.com/platinoff/poolAI' target='_blank'>GitHub</a>
      <a href='https://github.com/platinoff/poolAI/tree/Bolvanka-Beta-v1--stage2-https' target='_blank'>Stage2+HTTPS branch</a>
      <a href='https://github.com/platinoff/poolAI/blob/Bolvanka-Beta-v1--stage2-https/README.md' target='_blank'>Docs (EN)</a>
      <a href='https://github.com/platinoff/poolAI/blob/Bolvanka-Beta-v1--stage2-https/../README.md' target='_blank'>Docs (UA)</a>
      <a href='https://github.com/platinoff/poolAI/blob/Bolvanka-Beta-v1--stage2-https/docs/SECURITY.md' target='_blank'>Security</a>
      <a href='https://github.com/platinoff/poolAI/issues' target='_blank'>Support</a>
    </div>
    <div class='footer'>
      <p>PoolAI — AI Mining Pool Management System<br>
      <span style='font-size:0.9em'>Madevinc corp, 2025</span></p>
    </div>
  </div>
</body>
</html>
        "#,
            status = status.status,
            version = status.version,
            uptime = status.uptime
        );
        (
            axum::http::StatusCode::OK,
            [("content-type", "text/html; charset=utf-8")],
            html,
        )
            .into_response()
    } else {
        Json(status).into_response()
    }
}

async fn metrics_handler() -> impl IntoResponse {
    let metrics = MetricsResponse {
        active_workers: 5,
        total_requests: 1234,
        avg_response_time: 0.045,
    };
    Json(metrics)
}

async fn models_handler() -> impl IntoResponse {
    let models = vec![
        ModelInfo {
            name: "llama-2-7b",
            status: "loaded",
            memory_usage: 8192,
        },
        ModelInfo {
            name: "gpt-3.5-turbo",
            status: "available",
            memory_usage: 4096,
        },
    ];
    Json(models)
}

async fn workers_handler() -> impl IntoResponse {
    let workers = vec![
        WorkerInfo {
            id: "worker-1",
            status: "busy",
            current_task: Some("text-generation"),
        },
        WorkerInfo {
            id: "worker-2",
            status: "idle",
            current_task: None,
        },
    ];
    Json(workers)
}

async fn gpu_info() -> impl IntoResponse {
    let info = platform::get_gpu_info();
    Json(info)
}

async fn health_handler() -> impl IntoResponse {
    use chrono::Utc;

    let start_time = std::time::Instant::now();

    // Simulated system health checks
    let health_checks = HealthChecks {
        database: HealthCheck {
            status: "healthy",
            message: "Database connection OK".to_string(),
            response_time_ms: 5,
        },
        memory: HealthCheck {
            status: "healthy",
            message: "Memory usage: 45%".to_string(),
            response_time_ms: 2,
        },
        workers: HealthCheck {
            status: "healthy",
            message: "8/8 workers active".to_string(),
            response_time_ms: 3,
        },
        gpu: HealthCheck {
            status: "healthy",
            message: "GPU temperature: 65°C".to_string(),
            response_time_ms: 8,
        },
    };

    let _response_time = start_time.elapsed().as_millis() as u64;

    let health_response = HealthResponse {
        status: "healthy",
        timestamp: Utc::now().to_rfc3339(),
        version: "0.1.0",
        uptime: 3600, // TODO: Actual uptime
        checks: health_checks,
    };

    Json(health_response)
}

async fn login_handler(Json(auth_req): Json<AuthRequest>) -> impl IntoResponse {
    match authenticate_user(auth_req).await {
        Ok(auth_response) => Json(auth_response).into_response(),
        Err((status, error)) => (status, error).into_response(),
    }
}

// Retrieve all rewards
async fn rewards_handler() -> impl IntoResponse {
    let rewards = get_reward_statistics().await;
    Json(rewards)
}

// Retrieve rewards for a specific user
async fn user_rewards_handler(
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let rewards = get_user_rewards(&user_id).await;
    Json(rewards)
}

// Retrieve user progress
async fn user_progress_handler(
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let progress = get_user_progress(&user_id).await;
    match progress {
        Some(progress) => Json(progress).into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "User not found"
            })),
        )
            .into_response(),
    }
}

// Retrieve rewards statistics
async fn rewards_statistics_handler() -> impl IntoResponse {
    let stats = get_reward_statistics().await;
    Json(stats)
}

// Retrieve top users
async fn top_users_handler() -> impl IntoResponse {
    let top_users = get_top_users(10).await;
    Json(top_users)
}

// Library management handlers

// List all installed libraries
async fn libraries_list_handler() -> impl IntoResponse {
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        let libraries = manager.list_libraries().await;
        Json(libraries).into_response()
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Library manager not initialized"
            })),
        )
            .into_response()
    }
}

// Get library information
async fn library_info_handler(
    axum::extract::Path(name): axum::extract::Path<String>,
) -> impl IntoResponse {
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager.get_library(&name).await {
            Some(lib) => Json(lib).into_response(),
            None => (
                axum::http::StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Library {} not found", name)
                })),
            )
                .into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Library manager not initialized"
            })),
        )
            .into_response()
    }
}

// Install library
async fn library_install_handler(
    axum::extract::Path(name): axum::extract::Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let version = payload
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("latest");

    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager
            .install_library(&name, version, LibraryType::ModelLibrary)
            .await
        {
            Ok(lib) => Json(lib).into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to install library: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Library manager not initialized"
            })),
        )
            .into_response()
    }
}

// Uninstall library (with RBAC check)
async fn library_uninstall_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager.uninstall_library(&name).await {
            Ok(_) => Json(serde_json::json!({
                "message": format!("Library {} uninstalled successfully", name)
            }))
            .into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to uninstall library: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Library manager not initialized"
            })),
        )
            .into_response()
    }
}

// Update library (with RBAC check)
async fn library_update_handler(
    Extension(claims): Extension<Claims>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Check permission: write:all or write:libs
    if let Err(err) =
        check_permission(&claims, "write:all").or_else(|_| check_permission(&claims, "write:libs"))
    {
        return err.into_response();
    }
    if let Some(manager) = get_global_manager() {
        let manager = manager.read().await;
        match manager.update_library(&name).await {
            Ok(lib) => Json(lib).into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to update library: {}", e)
                })),
            )
                .into_response(),
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Library manager not initialized"
            })),
        )
            .into_response()
    }
}
