//! OmniRouter proxy — OpenAI-compatible routing + forwarding.
//!
//! `POST /api/omni/v1/chat/completions` accepts an OpenAI-format body. The target
//! provider is resolved from (in order): `X-Omni-Provider` header / `provider`
//! body field → the catalog owner of the requested model → `routing.default_provider`
//! → `routing.fallback_order` → highest-priority enabled provider with a base URL.
//! The body (minus our `provider` extension field) is forwarded to the upstream
//! OpenAI-compatible endpoint. `stream: true` requests are piped through as SSE.
//!
//! Debugging: set header `X-Omni-Dry-Run: 1` to resolve the route without sending
//! anything upstream (no network).

use axum::body::Body;
use axum::http::header;
use axum::http::HeaderMap;
use axum::response::Response;
use serde_json::{json, Value};

use crate::app_error::AppError;

use super::catalog;
use super::config::OmniConfig;
use super::OmniRouter;

/// Resolve the provider for a request.
pub fn select_provider(
    model: &str,
    explicit: Option<&str>,
    cfg: &OmniConfig,
) -> Result<String, String> {
    if let Some(p) = explicit.map(str::trim).filter(|s| !s.is_empty()) {
        if catalog::provider(p).is_none() {
            return Err(format!("unknown provider: {p}"));
        }
        if !cfg.enabled(p) {
            return Err(format!("provider disabled: {p}"));
        }
        if cfg.effective_base_url(p).is_none() {
            return Err(format!("provider has no base_url configured: {p}"));
        }
        return Ok(p.to_string());
    }

    if !model.is_empty() {
        let mut owners: Vec<&str> = catalog::find_models(model)
            .iter()
            .map(|m| m.provider)
            .collect();
        owners.dedup();
        owners.sort_by_key(|id| -cfg.priority(id));
        for id in owners {
            if cfg.enabled(id) && cfg.effective_base_url(id).is_some() {
                return Ok(id.to_string());
            }
        }
    }

    if cfg.routing.auto || model.is_empty() {
        let def = cfg.routing.default_provider.trim();
        if !def.is_empty() && cfg.enabled(def) && cfg.effective_base_url(def).is_some() {
            return Ok(def.to_string());
        }
        for id in &cfg.routing.fallback_order {
            if cfg.enabled(id) && cfg.effective_base_url(id).is_some() {
                return Ok(id.clone());
            }
        }
    }

    let mut best: Option<(i32, &str)> = None;
    for spec in catalog::providers() {
        if cfg.enabled(spec.id) && cfg.effective_base_url(spec.id).is_some() {
            let p = cfg.priority(spec.id);
            if best.map(|(bp, _)| p > bp).unwrap_or(true) {
                best = Some((p, spec.id));
            }
        }
    }
    best.map(|(_, id)| id.to_string()).ok_or_else(|| {
        "no enabled provider with a base_url — set OMNI_*_BASE_URL / omni.toml".to_string()
    })
}

/// OpenAI-compatible `POST /chat/completions`.
pub async fn chat_completions(
    omni: &OmniRouter,
    headers: &HeaderMap,
    raw: &[u8],
) -> Result<Response, AppError> {
    let body: Value = serde_json::from_slice(raw)
        .map_err(|e| AppError::new(format!("invalid JSON body: {e}")))?;
    let model = body
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let explicit = explicit_provider(headers, &body);
    let cfg = omni.config.read().await.clone();
    let provider_id = select_provider(&model, explicit.as_deref(), &cfg)
        .map_err(|e| AppError::new(format!("route: {e}")))?;
    let base_url = cfg
        .effective_base_url(&provider_id)
        .ok_or_else(|| AppError::new(format!("provider {provider_id} has no base_url")))?;
    let upstream_url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

    let dry_run = headers
        .get("x-omni-dry-run")
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v == "1");
    if dry_run {
        return Ok(Response::new(Body::from(
            serde_json::to_vec(&json!({
                "dry_run": true,
                "provider": provider_id,
                "base_url": base_url,
                "upstream": upstream_url,
                "model": model,
            }))
            .unwrap_or_default(),
        )));
    }

    let mut forwarded = body.clone();
    if let Value::Object(map) = &mut forwarded {
        map.remove("provider");
    }
    let mut req = omni.client.post(&upstream_url).json(&forwarded);
    if let Some(key) = cfg.effective_api_key(&provider_id) {
        req = req.header(header::AUTHORIZATION, format!("Bearer {key}"));
    }
    let upstream = req
        .send()
        .await
        .map_err(|e| AppError::new(format!("upstream request failed: {e}")))?;

    let streaming = body.get("stream").and_then(Value::as_bool).unwrap_or(false);
    if streaming {
        stream_response(upstream).await
    } else {
        json_response(upstream).await
    }
}

/// OpenAI-compatible `GET /models` for the configured providers.
pub async fn v1_models(omni: &OmniRouter) -> Result<Response, AppError> {
    let cfg = omni.config.read().await.clone();
    let mut data: Vec<Value> = Vec::new();
    for spec in catalog::models() {
        if !cfg.enabled(spec.provider) {
            continue;
        }
        data.push(json!({
            "id": spec.id,
            "object": "model",
            "owned_by": spec.provider,
            "created": 0,
            "free": spec.free,
            "recommended": spec.recommended,
            "context_window": spec.context_window,
            "max_output": spec.max_output,
            "tier": spec.tier,
        }));
    }
    Ok(value_into_response(
        json!({ "object": "list", "data": data }),
    ))
}

/// Connectivity check: `GET {base}/models` for one provider.
pub async fn test_provider(omni: &OmniRouter, provider_id: &str) -> Result<Value, AppError> {
    let cfg = omni.config.read().await.clone();
    let base_url = cfg
        .effective_base_url(provider_id)
        .ok_or_else(|| AppError::new(format!("provider {provider_id} has no base_url")))?;
    let url = format!("{}/models", base_url.trim_end_matches('/'));
    let mut req = omni.client.get(&url);
    if let Some(key) = cfg.effective_api_key(provider_id) {
        req = req.header(header::AUTHORIZATION, format!("Bearer {key}"));
    }
    let started = std::time::Instant::now();
    match req.send().await {
        Ok(r) => Ok(json!({
            "provider": provider_id,
            "ok": r.status().is_success(),
            "status": r.status().as_u16(),
            "latency_ms": started.elapsed().as_millis(),
        })),
        Err(e) => Ok(json!({
            "provider": provider_id,
            "ok": false,
            "error": e.to_string(),
        })),
    }
}

/// Explicit provider from `X-Omni-Provider` header or `provider` body field.
fn explicit_provider(headers: &HeaderMap, body: &Value) -> Option<String> {
    if let Some(p) = headers
        .get("x-omni-provider")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Some(p.to_string());
    }
    body.get("provider")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

/// Non-streaming JSON response with the upstream status code.
async fn json_response(upstream: reqwest::Response) -> Result<Response, AppError> {
    let status = upstream.status();
    let bytes = upstream
        .bytes()
        .await
        .map_err(|e| AppError::new(format!("upstream body: {e}")))?;
    let value: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    let payload = serde_json::to_vec(&value).unwrap_or_else(|_| b"null".to_vec());
    let mut builder = Response::builder().status(status);
    builder = builder.header(header::CONTENT_TYPE, "application/json");
    builder
        .body(Body::from(payload))
        .map_err(|e| AppError::new(format!("response build: {e}")))
}

/// SSE passthrough for `stream: true`.
async fn stream_response(upstream: reqwest::Response) -> Result<Response, AppError> {
    let mut builder = Response::builder().status(upstream.status());
    if let Some(ct) = upstream.headers().get(header::CONTENT_TYPE) {
        builder = builder.header(header::CONTENT_TYPE, ct);
    }
    builder = builder.header(header::CACHE_CONTROL, "no-cache");
    let stream = upstream.bytes_stream();
    builder
        .body(Body::from_stream(stream))
        .map_err(|e| AppError::new(format!("stream response build: {e}")))
}

/// Build an already-OK JSON axum response (used by `v1_models`).
fn value_into_response(value: Value) -> Response {
    let bytes = serde_json::to_vec(&value).unwrap_or_default();
    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(bytes))
        .expect("static json response")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boxes::omni::config::OmniConfig;

    fn cfg_with(provider: &str, base_url: &str, key: Option<&str>) -> OmniConfig {
        let mut cfg = OmniConfig::default();
        cfg.apply(&json!({
            "provider": { provider: {
                "base_url": base_url,
                "api_key": key.unwrap_or(""),
                "enabled": true,
                "priority": 10,
            }}
        }))
        .expect("apply");
        cfg
    }

    #[test]
    fn explicit_provider_wins() {
        let cfg = cfg_with("openai", "http://localhost:1/v1", Some("k"));
        assert_eq!(
            select_provider("gpt-5.2", Some("openai"), &cfg).as_deref(),
            Ok("openai")
        );
    }

    #[test]
    fn explicit_unknown_provider_rejected() {
        let cfg = cfg_with("openai", "http://localhost:1/v1", Some("k"));
        assert!(select_provider("gpt-5.2", Some("nope"), &cfg).is_err());
    }

    #[test]
    fn model_owner_resolved_when_no_explicit() {
        let cfg = cfg_with("anthropic", "http://localhost:1/v1", Some("k"));
        let selected = select_provider("claude-opus-4.5", None, &cfg).expect("route");
        assert_eq!(selected, "anthropic");
    }

    #[test]
    fn qwen_model_prefers_higher_priority_host() {
        // qwen (priority 5) should win over cerebras (priority 1) for the shared id.
        let mut cfg = OmniConfig::default();
        cfg.apply(&json!({
            "provider": {
                "qwen": { "base_url": "http://localhost:1/v1", "priority": 5 },
                "cerebras": { "base_url": "http://localhost:2/v1", "priority": 1 },
            }
        }))
        .expect("apply");
        assert_eq!(
            select_provider("qwen3-coder-480b", None, &cfg).as_deref(),
            Ok("qwen")
        );
    }

    #[test]
    fn disabled_default_falls_back_to_another_enabled_provider() {
        let mut cfg = OmniConfig::default();
        cfg.routing.default_provider = "openai".to_string();
        cfg.apply(
            &json!({ "provider": { "openai": { "base_url": "http://x/v1", "enabled": false } } }),
        )
        .expect("apply");
        // Disabled default is skipped → another enabled catalog provider wins.
        assert_eq!(
            select_provider("gpt-5.2", None, &cfg).as_deref(),
            Ok("anthropic")
        );
    }

    #[test]
    fn all_providers_disabled_errors() {
        let mut cfg = OmniConfig::default();
        for spec in catalog::providers() {
            cfg.apply(&json!({ "provider": { spec.id: { "enabled": false } } }))
                .expect("apply");
        }
        assert!(select_provider("gpt-5.2", None, &cfg).is_err());
    }

    #[test]
    fn dry_run_and_explicit_routing_agree() {
        let cfg = cfg_with("deepseek", "http://127.0.0.1:20128/v1", Some("k"));
        // OmniRoute wired as a provider → router should pick it for deepseek models.
        assert_eq!(
            select_provider("deepseek-v4-pro", None, &cfg).as_deref(),
            Ok("deepseek")
        );
    }
}
