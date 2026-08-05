//! OmniRouter box — Rust AI proxy/router tuned to the "AI providers" sheet.
//!
//! Box 9: a router for the AI providers listed in the Aug 2026 "AI providers by
//! opencode" spreadsheet (recommended: GPT 5.2, GPT 5.1 Codex, Claude Opus 4.5,
//! Claude Sonnet 4.5, MiniMax M2.1, Gemini 3 Pro; plus Chinese and free hosts).
//! It exposes a catalog + config (tunable per provider) and an OpenAI-compatible
//! proxy (`/api/omni/v1/chat/completions`) that forwards to the resolved upstream
//! — which can itself be an OmniRoute instance (e.g. base_url → `http://127.0.0.1:20128/v1`).
//!
//! Endpoints:
//! - `GET /api/omni` — overview wire (providers, models, recommended, routing)
//! - `GET /api/omni/config` · `POST /api/omni/config` — read (redacted) / tune
//! - `GET /api/omni/v1/models` — OpenAI-compatible model list
//! - `POST /api/omni/v1/chat/completions` — OpenAI-compatible proxy
//! - `POST /api/omni/test { provider }` — connectivity check

pub mod catalog;
pub mod config;
pub mod proxy;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde::Serialize;
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::vision;

pub use catalog::{ModelSpec, ProviderSpec};
pub use config::{OmniConfig, ProviderConfig, RoutingConfig};
pub use proxy::select_provider;

/// Canonical box name.
pub const OMNI_ROUTER_NAME: &str = "OmniRouter";

/// Shared OmniRouter runtime: durable config + HTTP client.
#[derive(Clone)]
pub struct OmniRouter {
    /// Durable data dir (`GSV/data/`).
    pub data_dir: Arc<PathBuf>,
    /// Outbound HTTP client for upstream requests.
    pub client: reqwest::Client,
    /// Tuned config (toml-backed, lock-guarded).
    pub config: Arc<RwLock<OmniConfig>>,
}

impl OmniRouter {
    /// Build a router from the durable config at `data_dir`.
    pub fn new(data_dir: &Path) -> Self {
        let config = OmniConfig::load(data_dir);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            data_dir: Arc::new(data_dir.to_path_buf()),
            client,
            config: Arc::new(RwLock::new(config)),
        }
    }

    /// Persist the current config (best effort; logs on failure).
    pub fn persist(&self) {
        let cfg = self
            .config
            .try_read()
            .map(|c| c.clone())
            .unwrap_or_default();
        if let Err(e) = cfg.save(&self.data_dir) {
            tracing::warn!(error = %e, "omni config save failed");
        }
    }
}

/// One provider row in the `/api/omni` wire.
#[derive(Debug, Clone, Serialize)]
pub struct ProviderWire {
    pub id: String,
    pub name: String,
    pub region: String,
    pub free: bool,
    pub base_url: String,
    pub enabled: bool,
    pub priority: i32,
    pub key_set: bool,
    pub notes: String,
}

/// One model row in the `/api/omni` wire.
#[derive(Debug, Clone, Serialize)]
pub struct ModelWire {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_window: Option<u32>,
    pub max_output: Option<u32>,
    pub free: bool,
    pub recommended: bool,
    pub tier: String,
}

/// `/api/omni` overview wire.
#[derive(Debug, Clone, Serialize)]
pub struct OmniWire {
    pub name: &'static str,
    pub providers: Vec<ProviderWire>,
    pub models: Vec<ModelWire>,
    pub recommended: Vec<ModelWire>,
    pub routing: Value,
    pub config_path: String,
    pub generated_at: String,
}

/// Build the overview wire from the current router config.
pub async fn wire(omni: &OmniRouter) -> OmniWire {
    let cfg = omni.config.read().await.clone();
    let providers: Vec<ProviderWire> = catalog::providers()
        .iter()
        .map(|p| {
            let pc = cfg.provider_config(p.id);
            ProviderWire {
                id: p.id.to_string(),
                name: p.name.to_string(),
                region: p.region.to_string(),
                free: p.free,
                base_url: cfg.effective_base_url(p.id).unwrap_or_default(),
                enabled: pc.enabled,
                priority: pc.priority,
                key_set: cfg.effective_api_key(p.id).is_some(),
                notes: p.notes.to_string(),
            }
        })
        .collect();
    let models: Vec<ModelWire> = catalog::models().iter().map(model_wire).collect();
    let recommended = catalog::recommended_models()
        .iter()
        .map(|m| model_wire(m))
        .collect();
    OmniWire {
        name: OMNI_ROUTER_NAME,
        providers,
        models,
        recommended,
        routing: json!({
            "default_provider": cfg.routing.default_provider,
            "auto": cfg.routing.auto,
            "fallback_order": cfg.routing.fallback_order,
        }),
        config_path: omni.data_dir.join("omni.toml").display().to_string(),
        generated_at: vision::rfc3339_now(),
    }
}

fn model_wire(m: &ModelSpec) -> ModelWire {
    ModelWire {
        id: m.id.to_string(),
        name: m.name.to_string(),
        provider: m.provider.to_string(),
        context_window: m.context_window,
        max_output: m.max_output,
        free: m.free,
        recommended: m.recommended,
        tier: m.tier.to_string(),
    }
}
