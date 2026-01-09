//! System API endpoints
//!
//! Provides system-level endpoints:
//! - Status
//! - Health checks
//! - Metrics
//! - Authentication (login)
//! - Models
//! - GPU information

use axum::{
    extract::{Extension, Request},
    http::{header::ACCEPT, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use serde::Serialize;

use crate::core::config::{get_config, update_config, PoolAIConfig};
use crate::network::api::common::check_permission;
use crate::network::auth::{authenticate_user, AuthRequest, Claims};
use crate::network::ws::websocket_handler;
use crate::platform;

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

/// Create system routes
pub fn create_system_routes() -> Router {
    Router::new()
        .route("/status", get(status_handler))
        .route("/health", get(health_handler))
        .route("/login", axum::routing::post(login_handler))
        .route("/metrics", get(metrics_handler))
        .route("/models", get(models_handler))
        .route("/gpu", get(gpu_info))
        .route("/ws/metrics", get(websocket_handler))
        .route("/config", get(config_get_handler))
        .route(
            "/config",
            put(config_update_handler).layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
}

async fn status_handler(req: Request<axum::body::Body>) -> Response {
    let uptime = crate::version::get_uptime_seconds();
    let status = StatusResponse {
        status: "running",
        version: "0.1.0",
        uptime,
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
    .api-section {{ margin-bottom: 1.5em; padding-bottom: 1em; border-bottom: 1px solid #44475a; }}
    .api-section:last-child {{ border-bottom: none; }}
    .api-section h3 {{ font-size: 1.1em; margin-bottom: 0.5em; }}
    .api-ref ul {{ margin-top: 0.5em; }}
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
      <strong>Security:</strong> HTTPS <span style='color:#50fa7b'>enabled</span>, JWT <span style='color:#50fa7b'>enabled</span>, CORS <span style='color:#50fa7b'>enabled</span>
      <br><span style='font-size:0.95em'>Self-signed certificate for dev. <b>Never commit private keys to git!</b></span>
      <br><span style='font-size:0.9em; color:#6272a4'>🔐 = Requires authentication | ✨ = New feature | 🎁 = Rewards system</span>
    </div>
    <div class='api-ref'>
      <h2>API Reference <span style='font-size:0.7em; color:#6272a4'>(67+ endpoints)</span></h2>
      <div class='api-section'>
        <h3 style='color:#8be9fd; margin-top:0;'>System</h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/status</code> — Server status (HTML/JSON)</li>
          <li><b>GET</b> <code>/api/v1/health</code> — Health check <span style='color:#50fa7b'>✨</span></li>
          <li><b>POST</b> <code>/api/v1/login</code> — Authentication <span style='color:#50fa7b'>🔐</span></li>
          <li><b>GET</b> <code>/api/v1/metrics</code> — Metrics</li>
          <li><b>GET</b> <code>/api/v1/models</code> — Models</li>
          <li><b>GET</b> <code>/api/v1/gpu</code> — GPU Info</li>
          <li><b>WS</b> <code>/ws/metrics</code> — Live metrics (WebSocket) <span style='color:#50fa7b'>✨</span></li>
        </ul>
      </div>
      <div class='api-section'>
        <h3 style='color:#8be9fd;'>Workers</h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/workers</code> — List workers</li>
          <li><b>POST</b> <code>/api/v1/workers</code> — Create worker <span style='color:#50fa7b'>🔐</span></li>
          <li><b>DELETE</b> <code>/api/v1/workers/:id</code> — Delete worker <span style='color:#50fa7b'>🔐</span></li>
        </ul>
      </div>
      <div class='api-section'>
        <h3 style='color:#8be9fd;'>Rewards</h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/rewards</code> — Rewards system <span style='color:#ffb86c'>🎁</span></li>
          <li><b>GET</b> <code>/api/v1/rewards/:user_id</code> — User rewards</li>
          <li><b>GET</b> <code>/api/v1/rewards/progress/:user_id</code> — User progress</li>
          <li><b>GET</b> <code>/api/v1/rewards/statistics</code> — Rewards statistics</li>
          <li><b>GET</b> <code>/api/v1/rewards/top</code> — Top users</li>
        </ul>
      </div>
      <div class='api-section'>
        <h3 style='color:#8be9fd;'>VM Management</h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/vm/instances</code> — List VM instances</li>
          <li><b>POST</b> <code>/api/v1/vm/instances</code> — Create VM <span style='color:#50fa7b'>🔐</span></li>
          <li><b>PUT</b> <code>/api/v1/vm/instances/:id</code> — Update VM <span style='color:#50fa7b'>🔐</span></li>
          <li><b>DELETE</b> <code>/api/v1/vm/instances/:id</code> — Delete VM <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/vm/instances/:id/start</code> — Start VM <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/vm/instances/:id/stop</code> — Stop VM <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/vm/instances/:id/restart</code> — Restart VM <span style='color:#50fa7b'>🔐</span></li>
        </ul>
      </div>
      <div class='api-section'>
        <h3 style='color:#8be9fd;'>RAID Storage</h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/raid/nodes</code> — List nodes</li>
          <li><b>GET</b> <code>/api/v1/raid/artifacts</code> — List artifacts</li>
          <li><b>POST</b> <code>/api/v1/raid/artifacts</code> — Create artifact <span style='color:#50fa7b'>🔐</span></li>
          <li><b>DELETE</b> <code>/api/v1/raid/artifacts/:id</code> — Delete artifact <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/raid/snapshot/create</code> — Create snapshot <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/raid/snapshot/restore</code> — Restore snapshot <span style='color:#50fa7b'>🔐</span></li>
        </ul>
      </div>
      <div class='api-section'>
        <h3 style='color:#8be9fd;'>Libraries</h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/libraries</code> — List libraries</li>
          <li><b>GET</b> <code>/api/v1/libraries/:name</code> — Library info</li>
          <li><b>POST</b> <code>/api/v1/libraries/:name/install</code> — Install library <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/libraries/:name/uninstall</code> — Uninstall library <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/libraries/upload</code> — Upload library <span style='color:#50fa7b'>🔐</span></li>
        </ul>
      </div>
      <div class='api-section'>
        <h3 style='color:#8be9fd;'>Enterprise <span style='font-size:0.8em; color:#6272a4'>(requires enterprise feature)</span></h3>
        <ul>
          <li><b>GET</b> <code>/api/v1/users</code> — List users <span style='color:#50fa7b'>🔐</span></li>
          <li><b>POST</b> <code>/api/v1/users</code> — Create user <span style='color:#50fa7b'>🔐</span></li>
          <li><b>GET</b> <code>/api/v1/tenants</code> — List tenants <span style='color:#50fa7b'>🔐</span></li>
          <li><b>GET</b> <code>/api/v1/audit/events</code> — Audit events <span style='color:#50fa7b'>🔐</span></li>
          <li><b>GET</b> <code>/api/v1/monitoring/dashboards</code> — Monitoring dashboards <span style='color:#50fa7b'>🔐</span></li>
        </ul>
      </div>
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
            StatusCode::OK,
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

    // Get actual uptime from version module
    let uptime = crate::version::get_uptime_seconds();

    let health_response = HealthResponse {
        status: "healthy",
        timestamp: Utc::now().to_rfc3339(),
        version: "0.1.0",
        uptime,
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

/// Get system configuration
async fn config_get_handler() -> impl IntoResponse {
    match get_config() {
        Ok(config) => Json(config).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to get configuration: {}", e)
            })),
        )
            .into_response(),
    }
}

/// Update system configuration
async fn config_update_handler(
    Extension(claims): Extension<Claims>,
    Json(config): Json<PoolAIConfig>,
) -> impl IntoResponse {
    // Check permission: admin:all
    if let Err(err) = check_permission(&claims, "admin:all") {
        return err.into_response();
    }

    match update_config(config) {
        Ok(()) => Json(serde_json::json!({
            "message": "Configuration updated successfully"
        }))
        .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Failed to update configuration: {}", e)
            })),
        )
            .into_response(),
    }
}
