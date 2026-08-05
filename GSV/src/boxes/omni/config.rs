//! OmniRouter config — durable `GSV/data/omni.toml` + env overrides.
//!
//! Per-provider tuning: `base_url`, `api_key`, `enabled`, `priority` (higher wins).
//! Secrets are never logged; the UI wire is redacted (key presence only). API keys
//! may instead be supplied via env `OMNI_<PROVIDER>_API_KEY` (and base URLs via
//! `OMNI_<PROVIDER>_BASE_URL`), which keeps them out of the toml file entirely.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::boxes::omni::catalog;

/// Router-level tuning.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RoutingConfig {
    /// Provider used when the model has no known owner and no explicit target.
    pub default_provider: String,
    /// Auto-resolve provider from the requested model when possible.
    pub auto: bool,
    /// Ordered fallback chain (provider ids).
    pub fallback_order: Vec<String>,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            default_provider: "openai".to_string(),
            auto: true,
            fallback_order: [
                "openai",
                "anthropic",
                "google",
                "minimax",
                "deepseek",
                "moonshot",
                "zai",
                "qwen",
                "openrouter",
            ]
            .iter()
            .map(|s| (*s).to_string())
            .collect(),
        }
    }
}

/// Per-provider tuning.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProviderConfig {
    /// Override for the catalog default base URL (empty = use default/env).
    pub base_url: Option<String>,
    /// API key (optional if `OMNI_<PROVIDER>_API_KEY` is set).
    pub api_key: Option<String>,
    /// Enabled for routing/proxy.
    pub enabled: bool,
    /// Routing priority (higher = preferred on ambiguous model routes).
    pub priority: i32,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            base_url: None,
            api_key: None,
            enabled: true,
            priority: 0,
        }
    }
}

/// Full router config (toml-backed).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct OmniConfig {
    pub routing: RoutingConfig,
    /// Per-provider overrides (keys = provider ids).
    pub provider: HashMap<String, ProviderConfig>,
}

impl OmniConfig {
    /// Load from `{data_dir}/omni.toml`, filling every catalog provider with
    /// defaults so the file is immediately tunable.
    pub fn load(data_dir: &Path) -> Self {
        let raw = std::fs::read_to_string(data_dir.join("omni.toml"));
        let mut cfg = raw
            .ok()
            .and_then(|s| toml::from_str::<OmniConfig>(&s).ok())
            .unwrap_or_default();
        for spec in catalog::providers() {
            cfg.provider.entry(spec.id.to_string()).or_default();
        }
        cfg
    }

    /// Persist to `{data_dir}/omni.toml`.
    pub fn save(&self, data_dir: &Path) -> Result<(), String> {
        std::fs::create_dir_all(data_dir).map_err(|e| format!("create data dir: {e}"))?;
        let raw = toml::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        std::fs::write(data_dir.join("omni.toml"), raw).map_err(|e| format!("write: {e}"))
    }

    /// Config row for a provider (defaults when absent).
    pub fn provider_config(&self, id: &str) -> ProviderConfig {
        self.provider.get(id).cloned().unwrap_or_default()
    }

    /// Enabled flag for a provider (default true).
    pub fn enabled(&self, id: &str) -> bool {
        self.provider.get(id).map(|p| p.enabled).unwrap_or(true)
    }

    /// Routing priority for a provider (default 0).
    pub fn priority(&self, id: &str) -> i32 {
        self.provider.get(id).map(|p| p.priority).unwrap_or(0)
    }

    /// Effective base URL: config override → `OMNI_{ID}_BASE_URL` → catalog.
    pub fn effective_base_url(&self, id: &str) -> Option<String> {
        let configured = self
            .provider
            .get(id)
            .and_then(|p| p.base_url.clone())
            .filter(|u| !u.trim().is_empty());
        if configured.is_some() {
            return configured;
        }
        let env = std::env::var(env_name(id, "BASE_URL"))
            .ok()
            .filter(|u| !u.trim().is_empty());
        if env.is_some() {
            return env;
        }
        catalog::provider(id)
            .and_then(|p| (!p.default_base_url.is_empty()).then(|| p.default_base_url.to_string()))
    }

    /// Effective API key: config override → `OMNI_{ID}_API_KEY` → `None`.
    pub fn effective_api_key(&self, id: &str) -> Option<String> {
        let configured = self
            .provider
            .get(id)
            .and_then(|p| p.api_key.clone())
            .filter(|k| !k.trim().is_empty());
        if configured.is_some() {
            return configured;
        }
        std::env::var(env_name(id, "API_KEY"))
            .ok()
            .filter(|k| !k.trim().is_empty())
    }

    /// Redacted JSON for the UI: never contains a raw key, only `key_set`.
    pub fn redacted(&self) -> Value {
        let mut providers = serde_json::Map::new();
        for spec in catalog::providers() {
            let pc = self.provider_config(spec.id);
            let key = self.effective_api_key(spec.id);
            providers.insert(
                spec.id.to_string(),
                json!({
                    "base_url": self.effective_base_url(spec.id).unwrap_or_default(),
                    "enabled": pc.enabled,
                    "priority": pc.priority,
                    "key_set": key.is_some(),
                }),
            );
        }
        json!({
            "routing": {
                "default_provider": self.routing.default_provider,
                "auto": self.routing.auto,
                "fallback_order": self.routing.fallback_order,
            },
            "provider": providers,
        })
    }

    /// Apply a partial JSON patch (from `POST /api/omni/config`): accepts
    /// `{ routing: {...}, provider: { <id>: { base_url?, api_key?, enabled?,
    /// priority? } } }`. Keys are written to the toml; empty strings clear them.
    pub fn apply(&mut self, patch: &Value) -> Result<(), String> {
        let obj = patch
            .as_object()
            .ok_or_else(|| "config patch must be an object".to_string())?;
        if let Some(r) = obj.get("routing").and_then(Value::as_object) {
            if let Some(dp) = r.get("default_provider").and_then(Value::as_str) {
                if !dp.trim().is_empty() && catalog::provider(dp.trim()).is_none() {
                    return Err(format!("unknown default provider: {dp}"));
                }
                self.routing.default_provider = dp.trim().to_string();
            }
            if let Some(a) = r.get("auto").and_then(Value::as_bool) {
                self.routing.auto = a;
            }
            if let Some(fo) = r.get("fallback_order").and_then(Value::as_array) {
                self.routing.fallback_order = fo
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect();
            }
        }
        if let Some(provs) = obj.get("provider").and_then(Value::as_object) {
            for (id, val) in provs {
                if catalog::provider(id).is_none() {
                    return Err(format!("unknown provider: {id}"));
                }
                let v = val
                    .as_object()
                    .ok_or_else(|| format!("provider {id} patch must be an object"))?;
                let entry = self.provider.entry(id.clone()).or_default();
                if let Some(b) = v.get("base_url").and_then(Value::as_str) {
                    entry.base_url = (!b.trim().is_empty()).then(|| b.trim().to_string());
                }
                if let Some(k) = v.get("api_key").and_then(Value::as_str) {
                    entry.api_key = (!k.trim().is_empty()).then(|| k.trim().to_string());
                }
                if let Some(e) = v.get("enabled").and_then(Value::as_bool) {
                    entry.enabled = e;
                }
                if let Some(p) = v.get("priority").and_then(Value::as_i64) {
                    entry.priority =
                        i32::try_from(p).map_err(|_| format!("priority overflow: {p}"))?;
                }
            }
        }
        Ok(())
    }
}

/// `openai` → `OMNI_OPENAI_...` (dashes → underscores).
fn env_name(provider_id: &str, suffix: &str) -> String {
    format!(
        "OMNI_{}_{}",
        provider_id.to_uppercase().replace('-', "_"),
        suffix
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tmp() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("gsv-omni-config-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn load_defaults_fill_all_catalog_providers() {
        let cfg = OmniConfig::load(&tmp());
        assert_eq!(cfg.provider.len(), catalog::providers().len());
        assert!(cfg.routing.auto);
        assert_eq!(
            cfg.effective_base_url("openai").as_deref(),
            Some("https://api.openai.com/v1")
        );
        // No key configured → None.
        assert_eq!(cfg.effective_api_key("openai"), None);
    }

    #[test]
    fn save_roundtrip_preserves_tuning() {
        let dir = tmp();
        let mut cfg = OmniConfig::load(&dir);
        cfg.provider
            .entry("openai".to_string())
            .or_default()
            .priority = 500;
        cfg.save(&dir).expect("save");
        let reloaded = OmniConfig::load(&dir);
        assert_eq!(reloaded.priority("openai"), 500);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn apply_patch_sets_and_clears_fields() {
        let mut cfg = OmniConfig::default();
        cfg.apply(&json!({
            "routing": { "default_provider": "deepseek", "auto": false },
            "provider": {
                "deepseek": { "base_url": "http://127.0.0.1:20128/v1", "api_key": "sk-test", "priority": 99 },
                "openai": { "base_url": "" },
            }
        }))
        .expect("apply");
        assert_eq!(cfg.routing.default_provider, "deepseek");
        assert!(!cfg.routing.auto);
        assert_eq!(
            cfg.effective_base_url("deepseek").as_deref(),
            Some("http://127.0.0.1:20128/v1")
        );
        assert_eq!(
            cfg.effective_api_key("deepseek").as_deref(),
            Some("sk-test")
        );
        assert_eq!(cfg.priority("deepseek"), 99);
        // base_url "" clears the override → falls back to catalog default.
        assert_eq!(
            cfg.effective_base_url("openai").as_deref(),
            Some("https://api.openai.com/v1")
        );
    }

    #[test]
    fn apply_patch_rejects_unknown_provider() {
        let mut cfg = OmniConfig::default();
        assert!(cfg
            .apply(&json!({ "provider": { "nope": { "enabled": true } } }))
            .is_err());
    }

    #[test]
    fn redacted_never_contains_raw_keys() {
        let mut cfg = OmniConfig::default();
        cfg.apply(&json!({ "provider": { "openai": { "api_key": "sk-super-secret" } } }))
            .expect("apply");
        let red = cfg.redacted().to_string();
        assert!(!red.contains("sk-super-secret"));
        assert!(red.contains("key_set"));
        assert!(red.contains("\"key_set\":true"));
    }
}
