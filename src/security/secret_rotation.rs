//! Secret rotation registry and built-in reload hooks (PH-S24 / FM security ops).

use crate::core::error::AppError;
use crate::security::jwt_secrets;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

/// Supported rotatable secret kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretKind {
    Jwt,
    TlsCertificate,
    TelegramWebhook,
}

impl SecretKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SecretKind::Jwt => "jwt",
            SecretKind::TlsCertificate => "tls_certificate",
            SecretKind::TelegramWebhook => "telegram_webhook",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "jwt" => Some(SecretKind::Jwt),
            "tls_certificate" | "tls" => Some(SecretKind::TlsCertificate),
            "telegram_webhook" | "telegram" => Some(SecretKind::TelegramWebhook),
            _ => None,
        }
    }
}

type RotationHook = Arc<dyn Fn() -> Result<(), String> + Send + Sync>;

#[derive(Debug, Default)]
struct KindMeta {
    last_rotated_unix: Option<u64>,
    rotation_count: u64,
}

struct Registry {
    hooks: parking_lot::RwLock<HashMap<SecretKind, Vec<RotationHook>>>,
    meta: parking_lot::RwLock<HashMap<SecretKind, KindMeta>>,
}

impl Registry {
    fn new() -> Self {
        Self {
            hooks: parking_lot::RwLock::new(HashMap::new()),
            meta: parking_lot::RwLock::new(HashMap::new()),
        }
    }

    fn register(&self, kind: SecretKind, hook: RotationHook) {
        self.hooks.write().entry(kind).or_default().push(hook);
    }

    fn run(&self, kind: SecretKind) -> Result<RotationRunReport, AppError> {
        let hooks = self.hooks.read().get(&kind).cloned().unwrap_or_default();
        if hooks.is_empty() {
            return Err(AppError::ConfigError(format!(
                "no rotation hooks registered for {}",
                kind.as_str()
            )));
        }
        let mut hook_results = Vec::new();
        for (i, hook) in hooks.iter().enumerate() {
            match hook() {
                Ok(()) => hook_results.push(HookResult {
                    index: i,
                    ok: true,
                    message: None,
                }),
                Err(e) => {
                    hook_results.push(HookResult {
                        index: i,
                        ok: false,
                        message: Some(e),
                    });
                }
            }
        }
        let all_ok = hook_results.iter().all(|r| r.ok);
        if all_ok {
            let now = unix_now();
            let mut meta = self.meta.write();
            let entry = meta.entry(kind).or_default();
            entry.last_rotated_unix = Some(now);
            entry.rotation_count = entry.rotation_count.saturating_add(1);
            info!(kind = kind.as_str(), "secret rotation hooks completed");
        } else {
            warn!(kind = kind.as_str(), "secret rotation hooks had failures");
        }
        Ok(RotationRunReport {
            kind,
            hooks: hook_results,
            success: all_ok,
        })
    }

    fn status(&self) -> Vec<RotationStatusEntry> {
        let hooks = self.hooks.read();
        let meta = self.meta.read();
        let kinds = [
            SecretKind::Jwt,
            SecretKind::TlsCertificate,
            SecretKind::TelegramWebhook,
        ];
        kinds
            .into_iter()
            .map(|kind| {
                let m = meta.get(&kind);
                RotationStatusEntry {
                    kind,
                    configured: is_kind_configured(kind),
                    last_rotated_unix: m.and_then(|x| x.last_rotated_unix),
                    rotation_count: m.map(|x| x.rotation_count).unwrap_or(0),
                    grace_active: kind == SecretKind::Jwt
                        && jwt_secrets::jwt_store().read().grace_active(),
                    hook_count: hooks.get(&kind).map(|v| v.len()).unwrap_or(0),
                }
            })
            .collect()
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn is_kind_configured(kind: SecretKind) -> bool {
    match kind {
        SecretKind::Jwt => std::env::var("POOLAI_JWT_SECRET").is_ok(),
        SecretKind::TlsCertificate => {
            std::env::var("HTTPS_CERT_PATH").is_ok() || std::env::var("HTTPS_KEY_PATH").is_ok()
        }
        SecretKind::TelegramWebhook => std::env::var("POOLAI_TELEGRAM_WEBHOOK_SECRET").is_ok(),
    }
}

static REGISTRY: OnceLock<Arc<Registry>> = OnceLock::new();
static DEFAULT_HOOKS: OnceLock<()> = OnceLock::new();

fn registry() -> &'static Arc<Registry> {
    REGISTRY.get_or_init(|| Arc::new(Registry::new()))
}

/// Register a custom rotation hook for a secret kind.
pub fn register_rotation_hook<F>(kind: SecretKind, hook: F)
where
    F: Fn() -> Result<(), String> + Send + Sync + 'static,
{
    registry().register(kind, Arc::new(hook));
}

/// Register TLS certificate reload (called from HTTPS server startup).
pub fn register_tls_reload_hook<F>(hook: F)
where
    F: Fn() -> Result<(), String> + Send + Sync + 'static,
{
    registry().register(SecretKind::TlsCertificate, Arc::new(hook));
}

fn hook_reload_jwt() -> Result<(), String> {
    jwt_secrets::reload_jwt_from_env()
}

fn hook_reload_telegram_webhook() -> Result<(), String> {
    if std::env::var("POOLAI_TELEGRAM_WEBHOOK_SECRET").is_err() {
        return Err("POOLAI_TELEGRAM_WEBHOOK_SECRET not set".into());
    }
    Ok(())
}

/// Register built-in hooks (JWT env reload, Telegram env presence, TLS if registered).
pub fn init_default_rotation_hooks() {
    DEFAULT_HOOKS.get_or_init(|| {
        register_rotation_hook(SecretKind::Jwt, hook_reload_jwt);
        register_rotation_hook(SecretKind::TelegramWebhook, hook_reload_telegram_webhook);
    });
}

/// Run all hooks for a secret kind.
pub fn run_rotation(kind: SecretKind) -> Result<RotationRunReport, AppError> {
    init_default_rotation_hooks();
    registry().run(kind)
}

/// Status for admin/ops (no secret values).
pub fn rotation_status() -> Vec<RotationStatusEntry> {
    init_default_rotation_hooks();
    registry().status()
}

#[derive(Debug, Clone, Serialize)]
pub struct RotationStatusEntry {
    pub kind: SecretKind,
    pub configured: bool,
    pub last_rotated_unix: Option<u64>,
    pub rotation_count: u64,
    pub grace_active: bool,
    pub hook_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RotationRunReport {
    pub kind: SecretKind,
    pub success: bool,
    pub hooks: Vec<HookResult>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HookResult {
    pub index: usize,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Background poll: reload JWT secrets when `POOLAI_SECRET_ROTATION_POLL_SECS` is set.
pub fn spawn_jwt_env_poll_if_configured() {
    let Some(secs) = std::env::var("POOLAI_SECRET_ROTATION_POLL_SECS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|&n| n > 0)
    else {
        return;
    };
    init_default_rotation_hooks();
    info!(
        interval_secs = secs,
        "JWT secret env poll enabled (POOLAI_SECRET_ROTATION_POLL_SECS)"
    );
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(secs));
        interval.tick().await;
        loop {
            interval.tick().await;
            if let Err(e) = hook_reload_jwt() {
                warn!(error = %e, "JWT secret env poll reload failed");
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_kind_parse_roundtrip() {
        assert_eq!(SecretKind::from_str("jwt"), Some(SecretKind::Jwt));
        assert_eq!(
            SecretKind::from_str("tls_certificate"),
            Some(SecretKind::TlsCertificate)
        );
    }

    #[test]
    fn jwt_rotation_hook_updates_store() {
        init_default_rotation_hooks();
        let before = jwt_secrets::jwt_store().read().loaded_at_unix;
        run_rotation(SecretKind::Jwt).expect("jwt rotation");
        let after = jwt_secrets::jwt_store().read().loaded_at_unix;
        assert!(after >= before);
    }
}
