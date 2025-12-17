//! UI module
//!
//! Concept alignment (planned in `poolAI_concept.txt`):
//! - Web dashboard (basic)
//! - UI components/themes/layouts (planned)

use crate::core::error::AppError;
use axum::{response::Html, routing::get, Router};
use tracing::info;

pub struct UiManager;

impl UiManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing UI module");
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AppError> {
        info!("Shutting down UI module");
        Ok(())
    }
}

pub fn create_ui_routes() -> Router {
    Router::new().route("/", get(dashboard_handler))
}

async fn dashboard_handler() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>PoolAI Dashboard</title>
  <style>
    body { font-family: Segoe UI, Arial, sans-serif; background:#0f1216; color:#e8e8e8; margin:0; }
    .wrap { max-width: 900px; margin: 48px auto; padding: 0 20px; }
    .card { background:#171b22; border:1px solid #262b36; border-radius:14px; padding: 22px; box-shadow: 0 12px 40px rgba(0,0,0,.25); }
    h1 { margin: 0 0 10px; color:#67e480; }
    a { color:#77c7ff; text-decoration:none; }
    a:hover { text-decoration:underline; }
    code { background:#0f1216; padding:2px 6px; border-radius:6px; border:1px solid #262b36; }
    .grid { display:grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-top: 16px; }
    .item { padding: 12px; border-radius: 12px; border:1px solid #262b36; background:#0f1216; }
    .muted { color:#a8b0bf; font-size: 0.95em; }
    @media (max-width: 700px) { .grid { grid-template-columns: 1fr; } }
  </style>
</head>
<body>
  <div class="wrap">
    <div class="card">
      <h1>PoolAI Dashboard</h1>
      <div class="muted">Minimal UI scaffold. Full dashboard components/themes are planned.</div>

      <div class="grid">
        <div class="item">
          <div><b>API</b></div>
          <div class="muted">REST endpoints: <code>/api/v1</code></div>
          <div style="margin-top:8px"><a href="/api/v1/status">Open status</a></div>
        </div>
        <div class="item">
          <div><b>Health</b></div>
          <div class="muted">Health check: <code>/api/v1/health</code></div>
          <div style="margin-top:8px"><a href="/api/v1/health">Open health</a></div>
        </div>
      </div>
    </div>
  </div>
</body>
</html>"#,
    )
}

pub async fn initialize() -> Result<(), AppError> {
    UiManager::new().initialize().await
}

pub async fn shutdown() -> Result<(), AppError> {
    UiManager::new().shutdown().await
}


