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
    extract::{Extension, Request, State},
    http::{header::ACCEPT, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};

use crate::core::config::{get_config, update_config, PoolAIConfig};
use crate::core::error::ErrorContext;
use crate::core::state::ApiContext;
use crate::network::api::common::{api_json_error, check_permission};
use crate::network::auth::{authenticate_user, AuthRequest, Claims};
use crate::network::ws::websocket_handler;
use crate::services::system_service::SystemService;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request as HttpRequest, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn status_handler_works_with_api_context() {
        let app_state = ApiContext::default();

        let app = create_system_routes().with_state(app_state);

        let response = app
            .oneshot(
                HttpRequest::builder()
                    .uri("/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
    }
}

/// Create system routes
pub fn create_system_routes() -> Router<ApiContext> {
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
            put(config_update_handler)
                .layer(middleware::from_fn(crate::network::auth::auth_middleware)),
        )
}

async fn status_handler(
    State(app_state): State<ApiContext>,
    req: Request<axum::body::Body>,
) -> Response {
    // Touch system state so that future extensions can use it without changing
    // the handler signature again.
    let _ = app_state.get_system_state();
    let status = SystemService::status_snapshot();
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
    /* Box-sizing для правильного позиціонування */
    *, *::before, *::after {{
      box-sizing: border-box;
    }}
    
    body {{ 
      font-family: 'Segoe UI', Arial, sans-serif; 
      background: var(--bg, #0f1216); 
      color: var(--text, #e8e8e8); 
      margin: 0; 
      padding: 0;
      transition: background-color 0.3s ease, color 0.3s ease;
    }}
    
    :root {{
      --bg: #0f1216;
      --surface: #171b22;
      --surface-secondary: #1e2329;
      --text: #e8e8e8;
      --text-muted: #a8b0bf;
      --primary: #67e480;
      --link: #77c7ff;
      --link-hover: #8bd5ff;
      --border: #262b36;
      --danger: #ff5555;
      --warning: #ffb86c;
      --info: #8be9fd;
    }}
    
    .wrap {{
      max-width: 1080px;
      margin: 28px auto;
      padding: 0 16px;
      width: 100%;
    }}
    
    .container {{ 
      max-width: 1080px; 
      margin: 28px auto; 
      background: var(--surface, #171b22); 
      border-radius: 14px; 
      box-shadow: 0 12px 40px rgba(0,0,0,.20); 
      padding: 32px; 
      border: 1px solid var(--border, #262b36);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }}
    
    .container:hover {{
      box-shadow: 0 16px 48px rgba(0,0,0,.25);
    }}
    
    h1 {{ 
      color: var(--primary, #67e480); 
      margin-bottom: 0.5em; 
      display: flex; 
      align-items: center; 
      gap: 12px;
      font-size: 24px;
      font-weight: 600;
    }}
    
    .logo {{ 
      width: 40px; 
      height: 40px; 
      vertical-align: middle;
      border-radius: 8px;
    }}
    
    .status {{ 
      font-size: 1.1em; 
      margin-bottom: 1.5em;
      padding: 16px;
      background: var(--bg, #0f1216);
      border-radius: 10px;
      border: 1px solid var(--border, #262b36);
    }}
    
    .status strong {{
      color: var(--text-muted, #a8b0bf);
      margin-right: 8px;
    }}
    
    .info-list {{ list-style: none; padding: 0; }}
    .info-list li {{ margin-bottom: 0.5em; }}
    
    a {{ 
      color: var(--link, #77c7ff); 
      text-decoration: none;
      transition: color 0.2s ease;
    }}
    a:hover {{ 
      color: var(--link-hover, #8bd5ff); 
      text-decoration: underline; 
    }}
    
    .links {{ margin-top: 2em; }}
    
    .badge {{ 
      display: inline-block; 
      background: var(--surface-secondary, #1e2329); 
      color: var(--primary, #67e480); 
      border-radius: 6px; 
      padding: 4px 10px; 
      font-size: 0.85em; 
      margin-left: 8px;
      font-weight: 600;
      border: 1px solid var(--border, #262b36);
    }}
    
    .footer {{ 
      margin-top: 2em; 
      color: var(--text-muted, #a8b0bf); 
      font-size: 0.95em; 
      text-align: center;
      padding-top: 20px;
      border-top: 1px solid var(--border, #262b36);
    }}
    
    .api-ref {{ 
      margin-top: 2em; 
      background: var(--bg, #0f1216); 
      border-radius: 12px; 
      padding: 24px; 
      border: 1px solid var(--border, #262b36);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    }}
    
    .api-ref:hover {{
      border-color: var(--primary, #67e480);
      box-shadow: 0 4px 16px rgba(103, 228, 128, 0.1);
    }}
    
    .api-ref h2 {{ 
      color: var(--info, #8be9fd); 
      margin-top: 0;
      margin-bottom: 1em;
      font-size: 1.3em;
      font-weight: 600;
    }}
    
    .api-ref code {{ 
      background: var(--surface-secondary, #1e2329); 
      color: var(--warning, #ffb86c); 
      border-radius: 6px; 
      padding: 3px 8px; 
      font-family: 'Fira Mono', 'Consolas', monospace;
      font-size: 0.9em;
      border: 1px solid var(--border, #262b36);
    }}
    
    .api-ref li {{ 
      margin-bottom: 0.6em;
      line-height: 1.6;
      transition: transform 0.2s ease;
    }}
    
    .api-ref li:hover {{
      transform: translateX(4px);
    }}
    
    .api-section {{ 
      margin-bottom: 2em; 
      padding-bottom: 1.5em; 
      border-bottom: 1px solid var(--border, #262b36);
      animation: fadeIn 0.4s ease-out;
    }}
    
    .api-section:last-child {{ border-bottom: none; }}
    
    .api-section h3 {{ 
      font-size: 1.15em; 
      margin-bottom: 0.8em;
      font-weight: 600;
    }}
    
    .api-ref ul {{ 
      margin-top: 0.8em;
      padding-left: 0;
    }}
    
    .security-info {{ 
      margin-top: 2em; 
      background: var(--surface-secondary, #1e2329); 
      border: 1px solid var(--warning, #ffb86c); 
      border-radius: 10px; 
      padding: 16px 20px;
      transition: all 0.3s ease;
    }}
    
    .security-info:hover {{
      border-color: var(--primary, #67e480);
      box-shadow: 0 4px 16px rgba(103, 228, 128, 0.1);
    }}
    
    .security-info strong {{ 
      color: var(--warning, #ffb86c);
      font-weight: 600;
    }}
    
    .curl-block {{ 
      background: var(--surface-secondary, #1e2329); 
      color: var(--text, #e8e8e8); 
      border-radius: 8px; 
      padding: 14px 18px; 
      font-size: 0.95em; 
      margin: 1.5em 0; 
      font-family: 'Fira Mono', 'Consolas', monospace;
      border: 1px solid var(--border, #262b36);
      transition: all 0.3s ease;
    }}
    
    .curl-block:hover {{
      border-color: var(--primary, #67e480);
      box-shadow: 0 2px 8px rgba(103, 228, 128, 0.1);
    }}
    
    .curl-block code {{
      background: transparent;
      border: none;
      padding: 0;
      color: var(--primary, #67e480);
    }}
    
    .doc-links {{ 
      margin-top: 2em;
      display: flex;
      flex-wrap: wrap;
      gap: 12px;
    }}
    
    .doc-links a {{ 
      margin-right: 0;
      padding: 8px 14px;
      background: var(--surface-secondary, #1e2329);
      border-radius: 8px;
      border: 1px solid var(--border, #262b36);
      transition: all 0.2s ease;
    }}
    
    .doc-links a:hover {{
      background: var(--surface, #171b22);
      border-color: var(--primary, #67e480);
      transform: translateY(-2px);
      box-shadow: 0 4px 12px rgba(103, 228, 128, 0.15);
      text-decoration: none;
    }}
    
    @keyframes fadeIn {{
      from {{ opacity: 0; transform: translateY(10px); }}
      to {{ opacity: 1; transform: translateY(0); }}
    }}
    
    /* Responsive Design */
    @media (max-width: 768px) {{
      .wrap {{ padding: 0 12px; margin: 16px auto; }}
      .container {{ padding: 20px; border-radius: 12px; }}
      .api-ref {{ padding: 16px; }}
      .doc-links {{ flex-direction: column; }}
      .doc-links a {{ width: 100%; text-align: center; }}
    }}
  </style>
</head>
<body>
  <div class='wrap'>
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
    Json(SystemService::metrics_snapshot())
}

async fn models_handler() -> impl IntoResponse {
    Json(SystemService::models_snapshot())
}

async fn gpu_info() -> impl IntoResponse {
    Json(SystemService::gpu_snapshot())
}

async fn health_handler() -> impl IntoResponse {
    Json(SystemService::health_snapshot())
}

async fn login_handler(
    State(ctx): State<ApiContext>,
    Json(auth_req): Json<AuthRequest>,
) -> impl IntoResponse {
    match authenticate_user(auth_req, ctx.user_manager.clone()).await {
        Ok(auth_response) => Json(auth_response).into_response(),
        Err((status, error)) => (status, error).into_response(),
    }
}

/// Get system configuration
async fn config_get_handler() -> impl IntoResponse {
    match get_config() {
        Ok(config) => Json(config).into_response(),
        Err(e) => {
            let (s, j) = api_json_error(
                "CONFIG_GET_FAILED",
                format!("Failed to get configuration: {}", e),
                Some(ErrorContext::new("config_get")),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            (s, j).into_response()
        }
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
        Err(e) => {
            let (s, j) = api_json_error(
                "CONFIG_UPDATE_FAILED",
                format!("Failed to update configuration: {}", e),
                Some(ErrorContext::new("config_update")),
                StatusCode::BAD_REQUEST,
            );
            (s, j).into_response()
        }
    }
}
