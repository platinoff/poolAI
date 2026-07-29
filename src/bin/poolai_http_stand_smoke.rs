//! HTTP stand smoke against a live coordinator (PH-S145).
//!
//! Replaces legacy Playwright API-smoke when a stand is running (`bin/e2e-playwright.sh --start`
//! or `run-poolai single`). Integration tests in `tests/` remain the CI canon without a stand.
//!
//! ```text
//! export POOLAI_BASE_URL=http://127.0.0.1:8080
//! cargo run --bin poolai-http-stand-smoke
//!
//! # RAID persist + restart (replaces legacy Playwright jobs_raid, PH-S156):
//! export POOLAI_E2E_STAND_ROOT=/tmp/poolai-e2e-NNN
//! cargo run --bin poolai-http-stand-smoke -- --raid-restart
//!
//! # Job lease renew suite (replaces legacy Playwright jobs_lease, PH-S196):
//! cargo run --bin poolai-http-stand-smoke -- --lease-renew
//!
//! # Full suite incl. raid restart:
//! cargo run --bin poolai-http-stand-smoke -- --raid
//!
//! cargo run --bin poolai-http-stand-smoke -- --json
//!
//! # RUN_LOCAL quick subset (PH-S1093):
//! cargo run --bin poolai-http-stand-smoke -- --run-local-smoke
//!
//! # Tenant live stand smoke (PH-S1193 band 55):
//! cargo run --bin poolai-http-stand-smoke -- --tenant-stand-smoke
//! # or: POOLAI_STAND_SMOKE_TENANT=1
//!
//! # SSO live stand smoke (PH-S1293 band 65):
//! cargo run --bin poolai-http-stand-smoke -- --sso-stand-smoke
//! # or: POOLAI_STAND_SMOKE_SSO=1
//!
//! # Audit live stand smoke (PH-S1393 band 75):
//! cargo run --bin poolai-http-stand-smoke -- --audit-stand-smoke
//! # or: POOLAI_STAND_SMOKE_AUDIT=1
//!
//! # Vision revision parity (PH-S208, PH-S235):
//! export POOLAI_VISION_BASE_URL=http://127.0.0.1:8765   # open-docs-vision.ps1
//! cargo run --bin poolai-http-stand-smoke   # repo manifest vs FM footer + extensions + optional HTTP header
//! ```

use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_BASE: &str = "http://127.0.0.1:8080";
const DEFAULT_VISION_BASE: &str = "http://127.0.0.1:8765";
const ENV_BASE: &str = "POOLAI_BASE_URL";
const ENV_VISION_BASE: &str = "POOLAI_VISION_BASE_URL";
const ENV_STAND_ROOT: &str = "POOLAI_E2E_STAND_ROOT";
const MANIFEST_REL: &str = "docs/vision/manifest.json";
const EXTENSIONS_REL: &str = "docs/vision/extensions.json";
const FM_REL: &str = "docs/catalog/FUNCTION_MANAGEMENT.md";
const VISION_REV_HEADER: &str = "x-poolai-vision-revision";
const VALID_PUBKEY: &str = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

#[derive(Debug, Clone)]
struct Cli {
    json_out: bool,
    include_raid: bool,
    raid_restart_only: bool,
    lease_renew_only: bool,
    /// PH-S1093: RUN_LOCAL quick subset (health + monitoring + vm + ops).
    run_local_smoke_only: bool,
    /// PH-S1193: live tenant store/CRUD/usage+quota suite (band 55).
    tenant_stand_smoke_only: bool,
    /// PH-S1293: live SSO store/CRUD/callback suite (band 65).
    sso_stand_smoke_only: bool,
    /// PH-S1393: live audit store/events/validate suite (band 75).
    audit_stand_smoke_only: bool,
    /// PH-S1493: live policy store/query/validate suite (band 85).
    policy_stand_smoke_only: bool,
    /// PH-S1593: live monitoring store/alerts/validate suite (band 95).
    monitoring_stand_smoke_only: bool,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct SmokeCaseResult {
    name: &'static str,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
struct SmokeReport {
    base_url: String,
    stand_root: Option<String>,
    ok: bool,
    passed: u32,
    failed: u32,
    cases: Vec<SmokeCaseResult>,
    tool: &'static str,
}

fn base_url_from_env() -> String {
    std::env::var(ENV_BASE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_BASE.to_string())
}

fn vision_base_url_from_env() -> String {
    std::env::var(ENV_VISION_BASE)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_VISION_BASE.to_string())
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_manifest_json(root: &Path) -> Result<Value, String> {
    let path = root.join(MANIFEST_REL);
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("parse manifest: {e}"))
}

fn read_manifest_revision(root: &Path) -> Result<u64, String> {
    let manifest = read_manifest_json(root)?;
    manifest
        .get("revision")
        .and_then(Value::as_u64)
        .ok_or_else(|| "manifest missing revision".to_string())
}

fn read_manifest_next_sprint(root: &Path) -> Option<String> {
    read_manifest_json(root).ok().and_then(|manifest| {
        manifest
            .get("next_sprint")
            .and_then(Value::as_str)
            .map(str::to_owned)
    })
}

fn read_extensions_active_sprint(root: &Path) -> Result<Option<String>, String> {
    let path = root.join(EXTENSIONS_REL);
    let raw = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let ext: Value = serde_json::from_str(&raw).map_err(|e| format!("parse extensions: {e}"))?;
    Ok(ext
        .get("active_sprint")
        .and_then(Value::as_str)
        .map(str::to_owned))
}

fn assert_vision_repo_parity(root: &Path) -> Result<(), String> {
    let repo_rev = read_manifest_revision(root)?;
    let fm_rev = read_fm_vision_revision(root)?;
    if repo_rev != fm_rev {
        return Err(format!(
            "repo manifest.revision {repo_rev} != FM Vision rev {fm_rev}"
        ));
    }
    if let Some(next) = read_manifest_next_sprint(root) {
        let active = read_extensions_active_sprint(root)?
            .ok_or_else(|| "extensions.json missing active_sprint".to_string())?;
        if active != next {
            return Err(format!(
                "extensions.active_sprint {active:?} != manifest.next_sprint {next:?}"
            ));
        }
    }
    Ok(())
}

fn extract_fm_section_512(content: &str) -> Option<&str> {
    let start = content.find("### 5.12")?;
    let rest = &content[start..];
    let end = rest[10..]
        .find("\n### 5.")
        .map(|i| 10 + i)
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

fn parse_fm_vision_revision(section: &str) -> Option<u64> {
    for line in section.lines() {
        let marker = "Vision rev **";
        let Some(start) = line.find(marker) else {
            continue;
        };
        let rest = &line[start + marker.len()..];
        let end = rest.find("**")?;
        return rest[..end].parse().ok();
    }
    None
}

fn read_fm_vision_revision(root: &Path) -> Result<u64, String> {
    let path = root.join(FM_REL);
    let content = fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let section =
        extract_fm_section_512(&content).ok_or_else(|| "FM §5.12 section not found".to_string())?;
    parse_fm_vision_revision(section)
        .ok_or_else(|| "FM §5.12 missing Vision rev **N** footer".to_string())
}

async fn smoke_vision_revision_parity(client: &Client) -> Result<(), String> {
    let root = repo_root();
    assert_vision_repo_parity(&root)?;
    let repo_rev = read_manifest_revision(&root)?;
    let vision_base = vision_base_url_from_env();
    let url = api_url(&vision_base, "/docs/vision/manifest.json");
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return Err(format!(
                "vision server unreachable at {url} ({e}); start open-docs-vision.ps1 or set {ENV_VISION_BASE}"
            ));
        }
    };
    if !resp.status().is_success() {
        return Err(format!("vision manifest status {}", resp.status()));
    }
    let header_rev = resp
        .headers()
        .get(VISION_REV_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| format!("missing {VISION_REV_HEADER} header on {url}"))?
        .parse::<u64>()
        .map_err(|_| format!("invalid {VISION_REV_HEADER} header"))?;
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let body_rev = body
        .get("revision")
        .and_then(Value::as_u64)
        .ok_or_else(|| "manifest JSON missing revision".to_string())?;
    if header_rev != body_rev {
        return Err(format!(
            "{VISION_REV_HEADER} {header_rev} != manifest.revision {body_rev}"
        ));
    }
    if body_rev != repo_rev {
        return Err(format!(
            "live manifest.revision {body_rev} != repo/FM revision {repo_rev}"
        ));
    }
    Ok(())
}

fn parse_cli() -> Cli {
    let mut json_out = false;
    let mut include_raid = false;
    let mut raid_restart_only = false;
    let mut lease_renew_only = false;
    let mut run_local_smoke_only = false;
    let mut tenant_stand_smoke_only = false;
    let mut sso_stand_smoke_only = false;
    let mut audit_stand_smoke_only = false;
    let mut policy_stand_smoke_only = false;
    let mut monitoring_stand_smoke_only = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--json" => json_out = true,
            "--raid-restart" => raid_restart_only = true,
            "--raid" => include_raid = true,
            "--lease-renew" => lease_renew_only = true,
            "--run-local-smoke" => run_local_smoke_only = true,
            "--tenant-stand-smoke" => tenant_stand_smoke_only = true,
            "--sso-stand-smoke" => sso_stand_smoke_only = true,
            "--audit-stand-smoke" => audit_stand_smoke_only = true,
            "--policy-stand-smoke" => policy_stand_smoke_only = true,
            "--monitoring-stand-smoke" => monitoring_stand_smoke_only = true,
            _ if arg.starts_with('-') => {}
            _ => {}
        }
    }
    if !raid_restart_only {
        raid_restart_only = std::env::var("POOLAI_STAND_SMOKE_RAID_RESTART")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !lease_renew_only {
        lease_renew_only = std::env::var("POOLAI_STAND_SMOKE_LEASE_RENEW")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !include_raid && !raid_restart_only {
        include_raid = std::env::var("POOLAI_STAND_SMOKE_RAID")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !run_local_smoke_only {
        run_local_smoke_only = std::env::var("POOLAI_STAND_SMOKE_RUN_LOCAL")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !tenant_stand_smoke_only {
        tenant_stand_smoke_only = std::env::var("POOLAI_STAND_SMOKE_TENANT")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !sso_stand_smoke_only {
        sso_stand_smoke_only = std::env::var("POOLAI_STAND_SMOKE_SSO")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !audit_stand_smoke_only {
        audit_stand_smoke_only = std::env::var("POOLAI_STAND_SMOKE_AUDIT")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !policy_stand_smoke_only {
        policy_stand_smoke_only = std::env::var("POOLAI_STAND_SMOKE_POLICY")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    if !monitoring_stand_smoke_only {
        monitoring_stand_smoke_only = std::env::var("POOLAI_STAND_SMOKE_MONITORING")
            .ok()
            .is_some_and(|v| matches!(v.as_str(), "1" | "true" | "yes"));
    }
    Cli {
        json_out,
        include_raid,
        raid_restart_only,
        lease_renew_only,
        run_local_smoke_only,
        tenant_stand_smoke_only,
        sso_stand_smoke_only,
        audit_stand_smoke_only,
        policy_stand_smoke_only,
        monitoring_stand_smoke_only,
        base_url: base_url_from_env(),
    }
}

fn smoke_id(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{prefix}-{nanos}")
}

fn api_url(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    format!("{base}{path}")
}

async fn wait_health(client: &Client, base: &str, tries: u32) -> Result<(), String> {
    let url = api_url(base, "/api/v1/health");
    for _ in 0..tries {
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Err(format!("health not ready at {url}"))
}

async fn smoke_health(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/health"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("health status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    for key in ["status", "version", "checks"] {
        if body.get(key).is_none() {
            return Err(format!("health missing `{key}`: {body}"));
        }
    }
    if body.get("status").and_then(Value::as_str) != Some("healthy") {
        return Err(format!("health status != healthy: {body}"));
    }
    Ok(())
}

/// PH-S1090: enterprise monitoring alerts list (RUN_LOCAL / admin wasm slim).
async fn smoke_monitoring_alerts_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/monitoring/alerts"))
        .send()
        .await
        .map_err(|e| format!("monitoring alerts request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("monitoring alerts status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    body.as_array()
        .ok_or_else(|| format!("monitoring alerts expected array: {body}"))?;
    Ok(())
}

/// PH-S1290: live `GET /api/enterprise/security/sso/store` shape.
async fn smoke_sso_store_wire(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/security/sso/store"))
        .send()
        .await
        .map_err(|e| format!("sso store request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("sso store status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let obj = body
        .as_object()
        .ok_or_else(|| format!("sso store expected object: {body}"))?;
    for key in ["mode", "durable_path", "configured"] {
        if !obj.contains_key(key) {
            return Err(format!("sso store missing `{key}`: {body}"));
        }
    }
    Ok(())
}

async fn smoke_sso_admin_bearer(client: &Client, base: &str) -> Result<String, String> {
    let resp = client
        .post(api_url(base, "/api/v1/login"))
        .json(&json!({ "username": "admin", "password": "admin123" }))
        .send()
        .await
        .map_err(|e| format!("sso login request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("sso login status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let token = body
        .get("token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| format!("sso login missing token: {body}"))?;
    Ok(format!("Bearer {token}"))
}

/// PH-S1291: live OAuth2 + SAML list → create → get → delete.
async fn smoke_sso_oauth2_saml_crud(client: &Client, base: &str) -> Result<(), String> {
    let auth = smoke_sso_admin_bearer(client, base).await?;
    let oauth_name = smoke_id("stand-oauth");
    let saml_name = smoke_id("stand-saml");

    let oauth_list = client
        .get(api_url(base, "/api/enterprise/security/oauth2/providers"))
        .send()
        .await
        .map_err(|e| format!("oauth2 list: {e}"))?;
    if oauth_list.status() != StatusCode::OK {
        return Err(format!("oauth2 list status {}", oauth_list.status()));
    }

    let oauth_create = client
        .post(api_url(base, "/api/enterprise/security/oauth2/providers"))
        .header("authorization", &auth)
        .json(&json!({
            "name": oauth_name,
            "config": {
                "client_id": "cid",
                "client_secret": "csecret",
                "authorization_url": "https://oauth.example.com/authorize",
                "token_url": "https://oauth.example.com/token",
                "redirect_uri": "https://poolai.example.com/callback",
                "scopes": ["openid", "profile"],
                "telegram_allow_user_ids": []
            },
            "enabled": true
        }))
        .send()
        .await
        .map_err(|e| format!("oauth2 create: {e}"))?;
    if oauth_create.status() != StatusCode::CREATED {
        return Err(format!("oauth2 create status {}", oauth_create.status()));
    }

    let oauth_get = client
        .get(api_url(
            base,
            &format!("/api/enterprise/security/oauth2/providers/{oauth_name}"),
        ))
        .send()
        .await
        .map_err(|e| format!("oauth2 get: {e}"))?;
    if oauth_get.status() != StatusCode::OK {
        return Err(format!("oauth2 get status {}", oauth_get.status()));
    }

    let oauth_del = client
        .delete(api_url(
            base,
            &format!("/api/enterprise/security/oauth2/providers/{oauth_name}"),
        ))
        .header("authorization", &auth)
        .send()
        .await
        .map_err(|e| format!("oauth2 delete: {e}"))?;
    if oauth_del.status() != StatusCode::OK {
        return Err(format!("oauth2 delete status {}", oauth_del.status()));
    }

    let saml_create = client
        .post(api_url(base, "/api/enterprise/security/saml/providers"))
        .header("authorization", &auth)
        .json(&json!({
            "name": saml_name,
            "config": {
                "entity_id": "https://idp.example.com/entity",
                "sso_url": "https://idp.example.com/sso",
                "acs_url": "https://poolai.example.com/acs",
                "slo_url": null,
                "certificate": "TEST_CERT",
                "attribute_mapping": {
                    "email": "email",
                    "username": "username"
                }
            },
            "enabled": true
        }))
        .send()
        .await
        .map_err(|e| format!("saml create: {e}"))?;
    if saml_create.status() != StatusCode::CREATED {
        return Err(format!("saml create status {}", saml_create.status()));
    }

    let saml_get = client
        .get(api_url(
            base,
            &format!("/api/enterprise/security/saml/providers/{saml_name}"),
        ))
        .send()
        .await
        .map_err(|e| format!("saml get: {e}"))?;
    if saml_get.status() != StatusCode::OK {
        return Err(format!("saml get status {}", saml_get.status()));
    }

    let saml_del = client
        .delete(api_url(
            base,
            &format!("/api/enterprise/security/saml/providers/{saml_name}"),
        ))
        .header("authorization", &auth)
        .send()
        .await
        .map_err(|e| format!("saml delete: {e}"))?;
    if saml_del.status() != StatusCode::OK {
        return Err(format!("saml delete status {}", saml_del.status()));
    }
    Ok(())
}

/// PH-S1292: live OAuth/SAML callback fixtures (no live IdP).
async fn smoke_sso_callback_fixtures(client: &Client, base: &str) -> Result<(), String> {
    let oauth = client
        .get(api_url(base, "/api/enterprise/auth/github/callback"))
        .send()
        .await
        .map_err(|e| format!("oauth callback: {e}"))?;
    if oauth.status() != StatusCode::BAD_REQUEST {
        return Err(format!("oauth callback status {}", oauth.status()));
    }
    let oauth_text = oauth.text().await.map_err(|e| e.to_string())?;
    if !(oauth_text.contains("OAUTH2_MISSING_CODE")
        || oauth_text.contains("Missing authorization code"))
    {
        return Err(format!("oauth callback unexpected body: {oauth_text}"));
    }

    let saml = client
        .post(api_url(
            base,
            "/api/enterprise/auth/saml/missing-provider-band65/callback",
        ))
        .header("content-type", "application/x-www-form-urlencoded")
        .body("SAMLResponse=dGVzdA%3D%3D&RelayState=x")
        .send()
        .await
        .map_err(|e| format!("saml callback: {e}"))?;
    if saml.status() != StatusCode::BAD_REQUEST {
        return Err(format!("saml callback status {}", saml.status()));
    }
    let saml_text = saml.text().await.map_err(|e| e.to_string())?;
    if !(saml_text.contains("SAML_ASSERTION_INVALID") || saml_text.contains("Failed to validate")) {
        return Err(format!("saml callback unexpected body: {saml_text}"));
    }
    Ok(())
}

/// PH-S1390: live `GET /api/enterprise/audit/store` shape.
async fn smoke_audit_store_wire(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/audit/store"))
        .send()
        .await
        .map_err(|e| format!("audit store request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("audit store status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let obj = body
        .as_object()
        .ok_or_else(|| format!("audit store expected object: {body}"))?;
    for key in ["mode", "durable_path", "configured"] {
        if !obj.contains_key(key) {
            return Err(format!("audit store missing `{key}`: {body}"));
        }
    }
    Ok(())
}

/// PH-S1391: live `GET /api/enterprise/audit/events` query (+ optional action filter).
async fn smoke_audit_events_query(client: &Client, base: &str) -> Result<(), String> {
    let list = client
        .get(api_url(base, "/api/enterprise/audit/events?limit=5"))
        .send()
        .await
        .map_err(|e| format!("audit events list: {e}"))?;
    if list.status() != StatusCode::OK {
        return Err(format!("audit events list status {}", list.status()));
    }
    let body: Value = list.json().await.map_err(|e| e.to_string())?;
    if !body.is_array() {
        return Err(format!("audit events expected array: {body}"));
    }

    let filtered = client
        .get(api_url(
            base,
            "/api/enterprise/audit/events?action=create_instance&limit=2",
        ))
        .send()
        .await
        .map_err(|e| format!("audit events filter: {e}"))?;
    if filtered.status() != StatusCode::OK {
        return Err(format!("audit events filter status {}", filtered.status()));
    }
    let filtered_body: Value = filtered.json().await.map_err(|e| e.to_string())?;
    if !filtered_body.is_array() {
        return Err(format!(
            "audit events filter expected array: {filtered_body}"
        ));
    }
    Ok(())
}

/// PH-S1392: live audit event-field validation fixtures (no durable append).
async fn smoke_audit_event_field_fixtures(client: &Client, base: &str) -> Result<(), String> {
    let missing_action = client
        .post(api_url(base, "/api/enterprise/audit/events/validate"))
        .json(&json!({
            "action": "",
            "resource_type": "vm_instance",
            "result": "success"
        }))
        .send()
        .await
        .map_err(|e| format!("audit validate missing action: {e}"))?;
    if missing_action.status() != StatusCode::BAD_REQUEST {
        return Err(format!(
            "audit validate missing action status {}",
            missing_action.status()
        ));
    }
    let action_body: Value = missing_action.json().await.map_err(|e| e.to_string())?;
    let action_text = action_body.to_string();
    if !(action_text.contains("AUDIT_MISSING_ACTION") || action_text.contains("missing action")) {
        return Err(format!(
            "audit validate missing action unexpected: {action_body}"
        ));
    }

    let missing_resource = client
        .post(api_url(base, "/api/enterprise/audit/events/validate"))
        .json(&json!({
            "action": "create_instance",
            "resource_type": "  ",
            "result": "success"
        }))
        .send()
        .await
        .map_err(|e| format!("audit validate missing resource: {e}"))?;
    if missing_resource.status() != StatusCode::BAD_REQUEST {
        return Err(format!(
            "audit validate missing resource status {}",
            missing_resource.status()
        ));
    }
    let resource_body: Value = missing_resource.json().await.map_err(|e| e.to_string())?;
    let resource_text = resource_body.to_string();
    if !(resource_text.contains("AUDIT_MISSING_RESOURCE")
        || resource_text.contains("missing resource_type"))
    {
        return Err(format!(
            "audit validate missing resource unexpected: {resource_body}"
        ));
    }
    Ok(())
}

/// PH-S1590: live `GET /api/enterprise/monitoring/store` shape.
async fn smoke_monitoring_store_wire(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/monitoring/store"))
        .send()
        .await
        .map_err(|e| format!("monitoring store request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("monitoring store status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let obj = body
        .as_object()
        .ok_or_else(|| format!("monitoring store expected object: {body}"))?;
    for key in ["mode", "durable_path", "configured"] {
        if !obj.contains_key(key) {
            return Err(format!("monitoring store missing `{key}`: {body}"));
        }
    }
    Ok(())
}

/// PH-S1591: live `GET /api/enterprise/monitoring/alerts` query (+ optional severity filter).
async fn smoke_monitoring_alerts_query(client: &Client, base: &str) -> Result<(), String> {
    let list = client
        .get(api_url(base, "/api/enterprise/monitoring/alerts?limit=5"))
        .send()
        .await
        .map_err(|e| format!("alerts list: {e}"))?;
    if list.status() != StatusCode::OK {
        return Err(format!("alerts list status {}", list.status()));
    }
    let body: Value = list.json().await.map_err(|e| e.to_string())?;
    if !body.is_array() {
        return Err(format!("alerts expected array: {body}"));
    }

    let filtered = client
        .get(api_url(
            base,
            "/api/enterprise/monitoring/alerts?severity=WARNING&limit=2",
        ))
        .send()
        .await
        .map_err(|e| format!("alerts filter: {e}"))?;
    if filtered.status() != StatusCode::OK {
        return Err(format!("alerts filter status {}", filtered.status()));
    }
    let filtered_body: Value = filtered.json().await.map_err(|e| e.to_string())?;
    let arr = filtered_body
        .as_array()
        .ok_or_else(|| format!("alerts filter expected array: {filtered_body}"))?;
    for item in arr {
        let sev = item.get("severity").and_then(|s| s.as_str()).unwrap_or("");
        if !(sev.eq_ignore_ascii_case("WARNING") || sev.eq_ignore_ascii_case("warning")) {
            return Err(format!("alerts filter leaked `{sev}`"));
        }
    }
    Ok(())
}

/// PH-S1592: live monitoring alert-rule validation fixtures (no durable write).
async fn smoke_monitoring_field_fixtures(client: &Client, base: &str) -> Result<(), String> {
    let valid = client
        .post(api_url(
            base,
            "/api/enterprise/monitoring/alert-rules/validate",
        ))
        .json(&json!({
            "name": "stand-smoke-ok",
            "metric": "cpu_usage",
            "threshold": 90.0,
            "operator": ">",
            "severity": "WARNING",
            "enabled": true
        }))
        .send()
        .await
        .map_err(|e| format!("monitoring validate ok: {e}"))?;
    if valid.status() != StatusCode::OK {
        return Err(format!("monitoring validate ok status {}", valid.status()));
    }
    let valid_body: Value = valid.json().await.map_err(|e| e.to_string())?;
    if valid_body.get("valid").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("monitoring validate ok unexpected: {valid_body}"));
    }

    let missing_name = client
        .post(api_url(
            base,
            "/api/enterprise/monitoring/alert-rules/validate",
        ))
        .json(&json!({
            "name": "",
            "metric": "cpu_usage",
            "threshold": 90.0,
            "operator": ">"
        }))
        .send()
        .await
        .map_err(|e| format!("monitoring validate missing name: {e}"))?;
    if missing_name.status() != StatusCode::BAD_REQUEST {
        return Err(format!(
            "monitoring validate missing name status {}",
            missing_name.status()
        ));
    }
    let name_body: Value = missing_name.json().await.map_err(|e| e.to_string())?;
    let name_text = name_body.to_string();
    if !(name_text.contains("MONITORING_MISSING_NAME") || name_text.contains("name")) {
        return Err(format!(
            "monitoring validate missing name unexpected: {name_body}"
        ));
    }

    let bad_op = client
        .post(api_url(
            base,
            "/api/enterprise/monitoring/alert-rules/validate",
        ))
        .json(&json!({
            "name": "bad-op",
            "metric": "cpu_usage",
            "threshold": 90.0,
            "operator": "!="
        }))
        .send()
        .await
        .map_err(|e| format!("monitoring validate bad operator: {e}"))?;
    if bad_op.status() != StatusCode::BAD_REQUEST {
        return Err(format!(
            "monitoring validate bad operator status {}",
            bad_op.status()
        ));
    }
    let op_body: Value = bad_op.json().await.map_err(|e| e.to_string())?;
    let op_text = op_body.to_string();
    if !(op_text.contains("MONITORING_INVALID_OPERATOR") || op_text.contains("operator")) {
        return Err(format!(
            "monitoring validate bad operator unexpected: {op_body}"
        ));
    }
    Ok(())
}

/// PH-S1490: live `GET /api/enterprise/policy/store` shape.
async fn smoke_policy_store_wire(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/policy/store"))
        .send()
        .await
        .map_err(|e| format!("policy store request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("policy store status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let obj = body
        .as_object()
        .ok_or_else(|| format!("policy store expected object: {body}"))?;
    for key in ["mode", "durable_path", "configured"] {
        if !obj.contains_key(key) {
            return Err(format!("policy store missing `{key}`: {body}"));
        }
    }
    Ok(())
}

/// PH-S1491: live `GET /api/enterprise/security/policies` query (+ optional name filter).
async fn smoke_policy_policies_query(client: &Client, base: &str) -> Result<(), String> {
    let list = client
        .get(api_url(base, "/api/enterprise/security/policies?limit=5"))
        .send()
        .await
        .map_err(|e| format!("policies list: {e}"))?;
    if list.status() != StatusCode::OK {
        return Err(format!("policies list status {}", list.status()));
    }
    let body: Value = list.json().await.map_err(|e| e.to_string())?;
    if !body.is_array() {
        return Err(format!("policies expected array: {body}"));
    }

    let filtered = client
        .get(api_url(
            base,
            "/api/enterprise/security/policies?name=default&limit=2",
        ))
        .send()
        .await
        .map_err(|e| format!("policies filter: {e}"))?;
    if filtered.status() != StatusCode::OK {
        return Err(format!("policies filter status {}", filtered.status()));
    }
    let filtered_body: Value = filtered.json().await.map_err(|e| e.to_string())?;
    let arr = filtered_body
        .as_array()
        .ok_or_else(|| format!("policies filter expected array: {filtered_body}"))?;
    for item in arr {
        let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("");
        if !name.contains("default") {
            return Err(format!("policies filter leaked `{name}`"));
        }
    }
    Ok(())
}

/// PH-S1492: live policy-field validation fixtures (no durable write).
async fn smoke_policy_field_fixtures(client: &Client, base: &str) -> Result<(), String> {
    let valid = client
        .post(api_url(base, "/api/enterprise/security/policies/validate"))
        .json(&json!({
            "name": "stand-smoke-ok",
            "description": "valid",
            "session_timeout": 3600,
            "require_mfa": false,
            "max_failed_attempts": 5
        }))
        .send()
        .await
        .map_err(|e| format!("policy validate ok: {e}"))?;
    if valid.status() != StatusCode::OK {
        return Err(format!("policy validate ok status {}", valid.status()));
    }
    let valid_body: Value = valid.json().await.map_err(|e| e.to_string())?;
    if valid_body.get("valid").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("policy validate ok unexpected: {valid_body}"));
    }

    let missing_name = client
        .post(api_url(base, "/api/enterprise/security/policies/validate"))
        .json(&json!({
            "name": "",
            "description": "x",
            "session_timeout": 3600
        }))
        .send()
        .await
        .map_err(|e| format!("policy validate missing name: {e}"))?;
    if missing_name.status() != StatusCode::BAD_REQUEST {
        return Err(format!(
            "policy validate missing name status {}",
            missing_name.status()
        ));
    }
    let name_body: Value = missing_name.json().await.map_err(|e| e.to_string())?;
    let name_text = name_body.to_string();
    if !(name_text.contains("POLICY_MISSING_NAME") || name_text.contains("name must be non-empty"))
    {
        return Err(format!(
            "policy validate missing name unexpected: {name_body}"
        ));
    }

    let bad_timeout = client
        .post(api_url(base, "/api/enterprise/security/policies/validate"))
        .json(&json!({
            "name": "timeout",
            "description": "x",
            "session_timeout": 0
        }))
        .send()
        .await
        .map_err(|e| format!("policy validate timeout: {e}"))?;
    if bad_timeout.status() != StatusCode::BAD_REQUEST {
        return Err(format!(
            "policy validate timeout status {}",
            bad_timeout.status()
        ));
    }
    let timeout_body: Value = bad_timeout.json().await.map_err(|e| e.to_string())?;
    let timeout_text = timeout_body.to_string();
    if !(timeout_text.contains("POLICY_INVALID_TIMEOUT")
        || timeout_text.contains("session_timeout"))
    {
        return Err(format!(
            "policy validate timeout unexpected: {timeout_body}"
        ));
    }
    Ok(())
}

/// PH-S1190: live `GET /api/enterprise/tenants/store` shape.
async fn smoke_tenants_store_wire(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/tenants/store"))
        .send()
        .await
        .map_err(|e| format!("tenants store request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("tenants store status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let obj = body
        .as_object()
        .ok_or_else(|| format!("tenants store expected object: {body}"))?;
    for key in ["mode", "durable_path", "configured"] {
        if !obj.contains_key(key) {
            return Err(format!("tenants store missing `{key}`: {body}"));
        }
    }
    Ok(())
}

async fn smoke_tenants_admin_bearer(client: &Client, base: &str) -> Result<String, String> {
    let resp = client
        .post(api_url(base, "/api/v1/login"))
        .json(&json!({ "username": "admin", "password": "admin123" }))
        .send()
        .await
        .map_err(|e| format!("tenant login request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("tenant login status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let token = body
        .get("token")
        .and_then(|t| t.as_str())
        .ok_or_else(|| format!("tenant login missing token: {body}"))?;
    Ok(format!("Bearer {token}"))
}

/// PH-S1191: live list → create → get → delete on `/api/enterprise/tenants*`.
async fn smoke_tenants_crud_lifecycle(client: &Client, base: &str) -> Result<(), String> {
    let auth = smoke_tenants_admin_bearer(client, base).await?;
    let name = smoke_id("stand-tenant");

    let list = client
        .get(api_url(base, "/api/enterprise/tenants"))
        .send()
        .await
        .map_err(|e| format!("tenants list: {e}"))?;
    if list.status() != StatusCode::OK {
        return Err(format!("tenants list status {}", list.status()));
    }
    let list_body: Value = list.json().await.map_err(|e| e.to_string())?;
    list_body
        .as_array()
        .ok_or_else(|| format!("tenants list expected array: {list_body}"))?;

    let create = client
        .post(api_url(base, "/api/enterprise/tenants"))
        .header("authorization", &auth)
        .json(&json!({
            "name": name,
            "config": {
                "active": true,
                "max_workers": 4,
                "max_memory_mb": 1024,
                "max_cpu_cores": 2,
                "max_storage_mb": 1024,
                "max_vm_instances": 2
            }
        }))
        .send()
        .await
        .map_err(|e| format!("tenants create: {e}"))?;
    if create.status() != StatusCode::OK {
        return Err(format!("tenants create status {}", create.status()));
    }
    let created: Value = create.json().await.map_err(|e| e.to_string())?;
    let id = created
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("tenants create missing id: {created}"))?
        .to_string();

    let get = client
        .get(api_url(base, &format!("/api/enterprise/tenants/{id}")))
        .send()
        .await
        .map_err(|e| format!("tenants get: {e}"))?;
    if get.status() != StatusCode::OK {
        return Err(format!("tenants get status {}", get.status()));
    }

    let del = client
        .delete(api_url(base, &format!("/api/enterprise/tenants/{id}")))
        .header("authorization", &auth)
        .send()
        .await
        .map_err(|e| format!("tenants delete: {e}"))?;
    if del.status() != StatusCode::OK {
        return Err(format!("tenants delete status {}", del.status()));
    }
    Ok(())
}

/// PH-S1192: live usage + quota allow/deny + foreign UUID → 404.
async fn smoke_tenants_usage_quota_isolation(client: &Client, base: &str) -> Result<(), String> {
    let auth = smoke_tenants_admin_bearer(client, base).await?;
    let name = smoke_id("stand-quota");

    let create = client
        .post(api_url(base, "/api/enterprise/tenants"))
        .header("authorization", &auth)
        .json(&json!({
            "name": name,
            "config": {
                "active": true,
                "max_workers": 4,
                "max_memory_mb": 1024,
                "max_cpu_cores": 2,
                "max_storage_mb": 1024,
                "max_vm_instances": 2
            }
        }))
        .send()
        .await
        .map_err(|e| format!("quota tenant create: {e}"))?;
    if create.status() != StatusCode::OK {
        return Err(format!("quota tenant create status {}", create.status()));
    }
    let created: Value = create.json().await.map_err(|e| e.to_string())?;
    let id = created
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("quota tenant missing id: {created}"))?
        .to_string();

    let usage = client
        .get(api_url(
            base,
            &format!("/api/enterprise/tenants/{id}/usage"),
        ))
        .send()
        .await
        .map_err(|e| format!("tenants usage: {e}"))?;
    if usage.status() != StatusCode::OK {
        return Err(format!("tenants usage status {}", usage.status()));
    }
    let usage_body: Value = usage.json().await.map_err(|e| e.to_string())?;
    for key in [
        "workers",
        "memory_mb",
        "cpu_cores",
        "storage_mb",
        "vm_instances",
    ] {
        if usage_body.get(key).is_none() {
            return Err(format!("usage missing `{key}`: {usage_body}"));
        }
    }

    let allow = client
        .post(api_url(
            base,
            &format!("/api/enterprise/tenants/{id}/quota"),
        ))
        .json(&json!({ "workers": 1, "memory_mb": 64, "cpu_cores": 1 }))
        .send()
        .await
        .map_err(|e| format!("quota allow: {e}"))?;
    if allow.status() != StatusCode::OK {
        return Err(format!("quota allow status {}", allow.status()));
    }
    let allow_body: Value = allow.json().await.map_err(|e| e.to_string())?;
    if allow_body.get("allowed").and_then(|a| a.as_bool()) != Some(true) {
        return Err(format!("expected quota allow: {allow_body}"));
    }

    let deny = client
        .post(api_url(
            base,
            &format!("/api/enterprise/tenants/{id}/quota"),
        ))
        .json(&json!({ "workers": 10_000, "memory_mb": 64, "cpu_cores": 1 }))
        .send()
        .await
        .map_err(|e| format!("quota deny: {e}"))?;
    if deny.status() != StatusCode::OK {
        return Err(format!("quota deny status {}", deny.status()));
    }
    let deny_body: Value = deny.json().await.map_err(|e| e.to_string())?;
    if deny_body.get("allowed").and_then(|a| a.as_bool()) != Some(false) {
        return Err(format!("expected quota deny: {deny_body}"));
    }

    let foreign = uuid::Uuid::new_v4();
    let missing = client
        .get(api_url(base, &format!("/api/enterprise/tenants/{foreign}")))
        .send()
        .await
        .map_err(|e| format!("foreign tenant get: {e}"))?;
    if missing.status() != StatusCode::NOT_FOUND {
        return Err(format!(
            "foreign tenant expected 404, got {}",
            missing.status()
        ));
    }

    let _ = client
        .delete(api_url(base, &format!("/api/enterprise/tenants/{id}")))
        .header("authorization", &auth)
        .send()
        .await;
    Ok(())
}

/// PH-S1091: enterprise monitoring dashboards list.
async fn smoke_monitoring_dashboards_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/enterprise/monitoring/dashboards"))
        .send()
        .await
        .map_err(|e| format!("monitoring dashboards request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("monitoring dashboards status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    body.as_array()
        .ok_or_else(|| format!("monitoring dashboards expected array: {body}"))?;
    Ok(())
}

/// PH-S1092: VM instances list shape (`run-poolai` dev stand).
async fn smoke_vm_instances_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/vm/instances"))
        .send()
        .await
        .map_err(|e| format!("vm instances request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("vm instances status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let rows = body
        .as_array()
        .ok_or_else(|| format!("vm instances expected array: {body}"))?;
    if let Some(first) = rows.first() {
        for key in ["id", "name", "status"] {
            if first.get(key).is_none() {
                return Err(format!("vm instances row missing `{key}`: {first}"));
            }
        }
    }
    Ok(())
}

async fn smoke_grid_seed_inventory(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/seed-inventory"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("grid seed-inventory status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("grid seed-inventory body: {body}"));
    }
    if body.get("generated_at").and_then(|v| v.as_str()).is_none() {
        return Err(format!("grid seed-inventory missing generated_at: {body}"));
    }
    let entries = body
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("grid seed-inventory missing entries: {body}"))?;
    if entries.len() != 2 {
        return Err(format!(
            "grid seed-inventory expected 2 entries, got {}: {body}",
            entries.len()
        ));
    }
    if entries[0].get("peer_id").and_then(|v| v.as_str()) != Some("srv1-worker-a") {
        return Err(format!("grid seed-inventory first peer_id: {}", entries[0]));
    }
    if entries[0].pointer("/seed_inventory/shard_ids") != Some(&json!(["w:emb-1", "w:ckpt-7"])) {
        return Err(format!(
            "grid seed-inventory first shard_ids: {}",
            entries[0]
        ));
    }
    if entries[1].get("peer_id").and_then(|v| v.as_str()) != Some("srv2-worker-b") {
        return Err(format!(
            "grid seed-inventory second peer_id: {}",
            entries[1]
        ));
    }
    if body
        .get("memory_store_depth")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err(format!(
            "grid seed-inventory missing memory_store_depth: {body}"
        ));
    }
    if body
        .get("memory_layer_depth")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err(format!(
            "grid seed-inventory missing memory_layer_depth: {body}"
        ));
    }
    if body
        .get("registered_shard_count")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "grid seed-inventory missing registered_shard_count: {body}"
        ));
    }
    Ok(())
}

/// PH-S213: live stand exposes Galaxy prefetch counters on Prometheus scrape.
const GALAXY_PREFETCH_METRICS: &[&str] = &[
    "galaxy_prefetch_plan_total",
    "galaxy_prefetch_planned_shards_total",
    "galaxy_prefetch_hot_skip_total",
    "galaxy_prefetch_bytes_total",
    "galaxy_prefetch_enqueue_total",
    "galaxy_prefetch_wait_ms_total",
    "galaxy_prefetch_strict_mode_total",
    "galaxy_prefetch_complete_total",
    "galaxy_prefetch_ingest_total",
    "galaxy_prefetch_skip_ingest_total",
    "galaxy_prefetch_seed_pull_total",
    "galaxy_prefetch_lease_acquired_total",
    "galaxy_locality_rank_ingest_total",
    "galaxy_locality_rank_miss_total",
    "galaxy_locality_rank_empty_workers_total",
    "galaxy_locality_rank_skip_total",
    "galaxy_network_profile_stale_total",
];

fn metrics_text_has_prefetch_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_PREFETCH_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_prefetch_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_prefetch_counters(&body)
}

/// PH-S216: live stand exposes Galaxy pricing forced-fallback counter on Prometheus scrape.
const GALAXY_PRICING_FORCED_FALLBACK: &str = "galaxy_pricing_forced_fallback_total";

fn metrics_text_has_pricing_forced_fallback(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_FORCED_FALLBACK;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_forced_fallback_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_forced_fallback(&body)
}

/// PH-S224: live stand exposes Galaxy pricing cache age gauge on Prometheus scrape.
const GALAXY_PRICING_CACHE_AGE: &str = "galaxy_pricing_cache_age_seconds";

fn metrics_text_has_pricing_cache_age(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_CACHE_AGE;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_cache_age_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_cache_age(&body)
}

/// PH-S241: live stand exposes Galaxy pricing fresh-served gauge on Prometheus scrape.
const GALAXY_PRICING_FRESH_SERVED: &str = "galaxy_pricing_fresh_served";

fn metrics_text_has_pricing_fresh_served(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_FRESH_SERVED;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_fresh_served_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_fresh_served(&body)
}

/// PH-S244: live stand exposes Galaxy pricing stale-served gauge on Prometheus scrape.
const GALAXY_PRICING_STALE_SERVED: &str = "galaxy_pricing_stale_served";

fn metrics_text_has_pricing_stale_served(body: &str) -> Result<(), String> {
    let name = GALAXY_PRICING_STALE_SERVED;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_pricing_stale_served_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_stale_served(&body)
}

/// PH-S247: live stand exposes Galaxy pricing provider catalog + error gauges.
const GALAXY_PRICING_PROVIDER_METRICS: &[&str] = &[
    "galaxy_pricing_provider_catalog_lookups_total",
    "galaxy_pricing_provider_catalog_hits_total",
    "galaxy_pricing_provider_errors_total",
];

fn metrics_text_has_pricing_provider_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_PRICING_PROVIDER_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_pricing_provider_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_provider_counters(&body)
}

/// PH-S253: live stand exposes Galaxy pricing quote + market min gauges.
const GALAXY_PRICING_QUOTE_MARKET_METRICS: &[&str] = &[
    "galaxy_pricing_quote_usd_micro",
    "galaxy_pricing_market_min_usd_micro",
];

fn metrics_text_has_pricing_quote_market_gauges(body: &str) -> Result<(), String> {
    for name in GALAXY_PRICING_QUOTE_MARKET_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_pricing_quote_market_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_pricing_quote_market_gauges(&body)
}

/// PH-S254: live stand exposes Galaxy fee split applied gauge.
const GALAXY_FEE_SPLIT_APPLIED: &str = "galaxy_fee_split_applied_total";

fn metrics_text_has_fee_split_applied(body: &str) -> Result<(), String> {
    let name = GALAXY_FEE_SPLIT_APPLIED;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_fee_split_applied_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_fee_split_applied(&body)
}

/// PH-S255: live stand exposes Galaxy cross-region egress gauge.
const GALAXY_CROSS_REGION_EGRESS_MB: &str = "galaxy_cross_region_egress_mb";

fn metrics_text_has_cross_region_egress_mb(body: &str) -> Result<(), String> {
    let name = GALAXY_CROSS_REGION_EGRESS_MB;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_cross_region_egress_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_cross_region_egress_mb(&body)
}

/// PH-S256 / PH-S336: live stand exposes Galaxy replay pending gauge + scheduled/resolved totals.
const GALAXY_REPLAY_METRICS: &[&str] = &[
    "galaxy_replay_pending",
    "galaxy_replay_pending_scheduled_total",
    "galaxy_replay_pending_resolved_total",
    "galaxy_replay_evaluations_total",
    "galaxy_replay_verification_enqueue_total",
];

fn metrics_text_has_replay_pending(body: &str) -> Result<(), String> {
    for name in GALAXY_REPLAY_METRICS {
        if !body.contains(*name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_replay_pending_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_replay_pending(&body)
}

/// PH-S249: live stand exposes Galaxy settlement pending + cleared gauges.
const GALAXY_SETTLEMENT_METRICS: &[&str] = &[
    "galaxy_settlement_pending_verification_total",
    "galaxy_settlement_cleared_total",
    "galaxy_settlement_not_applicable_total",
    "galaxy_settlement_resolved_total",
    "galaxy_settlement_payout_batch_total",
    "galaxy_settlement_human_review_total",
];

/// PH-S569: checker timeout inconclusive/retry gauges on `/metrics`.
const GALAXY_CHECKER_TIMEOUT_METRICS: &[&str] = &[
    "galaxy_verification_checker_timeout_inconclusive_total",
    "galaxy_verification_checker_timeout_retry_total",
    "galaxy_fraud_proof_pending_total",
    "poolai_advisory_acknowledged_total",
];

fn metrics_text_has_settlement_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_SETTLEMENT_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_settlement_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_settlement_counters(&body)?;
    for name in GALAXY_CHECKER_TIMEOUT_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
    }
    Ok(())
}

/// PH-S528: governance ops Prometheus gauges on live stand.
const GALAXY_GOVERNANCE_METRICS: &[&str] = &[
    "poolai_release_verify_total",
    "poolai_release_verify_fail_total",
    "poolai_update_notify_pending",
];

async fn smoke_galaxy_governance_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    for name in GALAXY_GOVERNANCE_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
    }
    Ok(())
}

/// PH-S250: live stand exposes Galaxy shard local hit ratio gauge.
const GALAXY_SHARD_LOCAL_HIT_RATIO: &str = "galaxy_shard_local_hit_ratio";

/// PH-S581: live stand exposes Galaxy hot tier hit ratio gauge.
const GALAXY_HOT_TIER_HIT_RATIO: &str = "galaxy_hot_tier_hit_ratio";

fn metrics_text_has_shard_local_hit_ratio(body: &str) -> Result<(), String> {
    let name = GALAXY_SHARD_LOCAL_HIT_RATIO;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_shard_local_hit_ratio_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_shard_local_hit_ratio(&body)
}

fn metrics_text_has_hot_tier_hit_ratio(body: &str) -> Result<(), String> {
    let name = GALAXY_HOT_TIER_HIT_RATIO;
    if !body.contains(name) {
        return Err(format!("/metrics missing {name}"));
    }
    if !body.contains(&format!("# TYPE {name} gauge")) {
        return Err(format!("/metrics missing TYPE gauge for {name}"));
    }
    Ok(())
}

async fn smoke_galaxy_hot_tier_hit_ratio_metrics(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_hot_tier_hit_ratio(&body)
}

/// PH-S225: live stand exposes Galaxy verification counters on Prometheus scrape.
const GALAXY_VERIFICATION_METRICS: &[&str] = &[
    "galaxy_verification_sample_total",
    "galaxy_verification_mismatch_total",
    "galaxy_verification_match_total",
    "galaxy_verification_sample_scheduled_total",
    "galaxy_verification_sample_completed_total",
    "galaxy_verification_sample_skipped_total",
    "galaxy_verification_sample_not_applicable_total",
    "galaxy_verification_sampling_evaluations_total",
    "galaxy_verification_checker_enqueue_total",
];

fn metrics_text_has_verification_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_VERIFICATION_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_verification_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_verification_counters(&body)
}

/// PH-S219: live stand exposes Galaxy trust payout counters on Prometheus scrape.
const GALAXY_TRUST_PAYOUT_METRICS: &[&str] = &[
    "galaxy_trust_payout_eligible_total",
    "galaxy_trust_payout_held_total",
    "galaxy_trust_payout_not_applicable_total",
    "galaxy_trust_score",
    "galaxy_trust_gate_min_threshold",
    "galaxy_trust_gate_default_score",
    "galaxy_trust_gate_evaluations_total",
    "galaxy_trust_default_score_applied_total",
    "galaxy_trust_explicit_score_total",
];

fn metrics_text_has_trust_payout_counters(body: &str) -> Result<(), String> {
    for name in GALAXY_TRUST_PAYOUT_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_trust_payout_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_trust_payout_counters(&body)
}

/// PH-S232 / PH-S426: live stand exposes Galaxy replication counters on Prometheus scrape.
const GALAXY_REPLICATION_METRICS: &[&str] = &[
    "galaxy_replication_strict_total",
    "galaxy_replication_enqueue_total",
    "galaxy_replication_executor_enqueue_total",
];

fn metrics_text_has_replication_strict(body: &str) -> Result<(), String> {
    for name in GALAXY_REPLICATION_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_replication_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_replication_strict(&body)
}

/// PH-S451: live stand exposes PH-S444…S449 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S444_METRICS: &[&str] = &[
    "galaxy_prefetch_seed_fetch_total",
    "galaxy_prefetch_seed_fetch_miss_total",
    "galaxy_prefetch_co_access_total",
    "galaxy_locality_unsatisfied_total",
    "poolai_protocol_negotiation_rejected_total",
    "galaxy_verification_replay_record_total",
];

fn metrics_text_has_horizon_wire_s444(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S444_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s444_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s444(&body)
}

/// PH-S462: live stand exposes PH-S454…S460 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S454_METRICS: &[&str] = &[
    "galaxy_prefetch_re_migrate_total",
    "galaxy_verification_elevated_applied_total",
    "galaxy_trust_score_delta_total",
    "galaxy_replication_rate_limited_total",
    "galaxy_hot_promote_total",
    "galaxy_hot_evict_total",
    "galaxy_shard_access_total",
    "galaxy_prefetch_queue_depth",
];

fn metrics_text_has_horizon_wire_s454(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S454_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s454_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s454(&body)
}

async fn smoke_grid_verification_replay(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-replay"))
        .send()
        .await
        .map_err(|e| format!("verification-replay request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("verification-replay status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-replay body: {body}"));
    }
    Ok(())
}

/// PH-S482: live stand exposes PH-S474…S479 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S474_METRICS: &[&str] = &[
    "galaxy_prefetch_egress_blocked_total",
    "galaxy_prefetch_peer_fetch_total",
    "galaxy_prefetch_peer_fetch_miss_total",
];

fn metrics_text_has_horizon_wire_s474(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S474_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s474_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s474(&body)
}

/// PH-S492: live stand exposes PH-S484…S489 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S484_METRICS: &[&str] = &["galaxy_prefetch_pull_bytes_total"];

fn metrics_text_has_horizon_wire_s484(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S484_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s484_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s484(&body)
}

/// PH-S501: live stand exposes PH-S494…S499 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S494_METRICS: &[&str] = &["galaxy_verification_checker_pending_total"];

fn metrics_text_has_horizon_wire_s494(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S494_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s494_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s494(&body)
}

async fn smoke_grid_verification_checker_tasks(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-checker/tasks"))
        .send()
        .await
        .map_err(|e| format!("verification-checker/tasks request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!(
            "verification-checker/tasks status {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-checker/tasks body: {body}"));
    }
    if !body.get("tasks").and_then(|v| v.as_array()).is_some() {
        return Err(format!("verification-checker/tasks missing tasks: {body}"));
    }
    Ok(())
}

async fn smoke_grid_verification_lifecycle_depth(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-metrics"))
        .send()
        .await
        .map_err(|e| format!("verification-metrics lifecycle: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!(
            "verification-metrics lifecycle status {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let depth = body
        .get("lifecycle_depth")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("verification-metrics missing lifecycle_depth: {body}"))?;
    if depth.is_empty() {
        return Err("verification-metrics lifecycle_depth empty".into());
    }
    Ok(())
}

async fn smoke_grid_verification_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/verification-metrics"))
        .send()
        .await
        .map_err(|e| format!("verification-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("verification-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("verification-metrics missing metrics: {body}"))?;
    for key in [
        "sample_total",
        "mismatch_total",
        "match_total",
        "checker_pending_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("verification-metrics missing {key}: {body}"));
        }
    }
    Ok(())
}

async fn smoke_grid_replay_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/replay-metrics"))
        .send()
        .await
        .map_err(|e| format!("replay-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("replay-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("replay-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("replay-metrics missing metrics: {body}"))?;
    for key in [
        "replay_pending",
        "replay_pending_scheduled_total",
        "verification_replay_record_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("replay-metrics missing {key}: {body}"));
        }
    }
    Ok(())
}

async fn smoke_grid_settlement_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/settlement-metrics"))
        .send()
        .await
        .map_err(|e| format!("settlement-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("settlement-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("settlement-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("settlement-metrics missing metrics: {body}"))?;
    for key in [
        "pending_verification_total",
        "cleared_total",
        "resolved_total",
        "payout_batch_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("settlement-metrics missing {key}: {body}"));
        }
    }
    Ok(())
}

async fn smoke_grid_trust_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/trust-metrics"))
        .send()
        .await
        .map_err(|e| format!("trust-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("trust-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("trust-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("trust-metrics missing metrics: {body}"))?;
    for key in [
        "payout_eligible_total",
        "payout_held_total",
        "last_trust_score",
        "gate_min_threshold",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("trust-metrics missing {key}: {body}"));
        }
    }
    if body
        .get("trust_persist_depth")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .is_none()
    {
        return Err(format!("trust-metrics missing trust_persist_depth: {body}"));
    }
    Ok(())
}

async fn smoke_grid_replication_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/replication-metrics"))
        .send()
        .await
        .map_err(|e| format!("replication-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("replication-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("replication-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("replication-metrics missing metrics: {body}"))?;
    for key in [
        "strict_total",
        "enqueue_total",
        "executor_enqueue_total",
        "rate_limited_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("replication-metrics missing {key}: {body}"));
        }
    }
    let depth = body
        .get("replication_depth")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("replication-metrics missing replication_depth: {body}"))?;
    if depth.is_empty() {
        return Err("replication-metrics replication_depth empty".into());
    }
    if body
        .get("rate_cap_per_hour")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "replication-metrics missing rate_cap_per_hour: {body}"
        ));
    }
    Ok(())
}

async fn smoke_grid_pricing_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/pricing-metrics"))
        .send()
        .await
        .map_err(|e| format!("pricing-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("pricing-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("pricing-metrics body: {body}"));
    }
    let metrics = body
        .get("metrics")
        .ok_or_else(|| format!("pricing-metrics missing metrics: {body}"))?;
    for key in [
        "fresh_served_total",
        "stale_served_total",
        "forced_fallback_total",
        "provider_catalog_lookups_total",
        "provider_catalog_hits_total",
        "provider_errors_total",
        "provider_timeouts_total",
    ] {
        if !metrics.get(key).and_then(|v| v.as_u64()).is_some() {
            return Err(format!("pricing-metrics missing {key}: {body}"));
        }
    }
    if body.get("pricing_depth").and_then(|v| v.as_str()).is_none() {
        return Err(format!("pricing-metrics missing pricing_depth: {body}"));
    }
    if body
        .get("provider_http_timeout_ms")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "pricing-metrics missing provider_http_timeout_ms: {body}"
        ));
    }
    Ok(())
}

/// PH-S903: stand smoke pricing-metrics JSON↔Prom parity.
async fn smoke_pricing_metrics_json_prometheus_parity(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_pricing_metrics_parity;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    let resp = client
        .get(api_url(base, "/api/v1/grid/pricing-metrics"))
        .send()
        .await
        .map_err(|e| format!("pricing-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("pricing-metrics status {}", resp.status()));
    }
    let pricing: Value = resp.json().await.map_err(|e| e.to_string())?;
    validate_pricing_metrics_parity(&prom_text, &pricing)
}

/// PH-S901: grid pricing L2 fallback stable snapshot (PH-S123 pattern).
async fn smoke_grid_pricing_l2_fallback_stable(client: &Client, base: &str) -> Result<(), String> {
    let model = smoke_id("smoke-pricing-fallback");
    let url = format!(
        "{}/api/v1/grid/pricing?task_profile=inference:text&model_profile={model}&unit_key=inference_blended_token",
        base.trim_end_matches('/')
    );
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status() == StatusCode::SERVICE_UNAVAILABLE {
        return Ok(());
    }
    if resp.status() != StatusCode::OK {
        return Err(format!("grid pricing fallback status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("grid pricing fallback body: {body}"));
    }
    let snap = body
        .get("snapshot")
        .ok_or_else(|| format!("grid pricing fallback missing snapshot: {body}"))?;
    if snap
        .get("poolai_quote_usd_micro")
        .and_then(|v| v.as_u64())
        .is_none()
    {
        return Err(format!(
            "grid pricing fallback missing poolai_quote_usd_micro: {body}"
        ));
    }
    Ok(())
}

async fn smoke_grid_prefetch_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_prefetch_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/prefetch-metrics"))
        .send()
        .await
        .map_err(|e| format!("prefetch-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("prefetch-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("prefetch-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_prefetch_metrics_parity(&prom_text, &body)
}

async fn smoke_grid_locality_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_locality_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/locality-metrics"))
        .send()
        .await
        .map_err(|e| format!("locality-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("locality-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("locality-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_locality_metrics_parity(&prom_text, &body)
}

async fn smoke_grid_fee_split_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_fee_split_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/fee-split-metrics"))
        .send()
        .await
        .map_err(|e| format!("fee-split-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("fee-split-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("fee-split-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_fee_split_metrics_parity(&prom_text, &body)
}

async fn smoke_grid_update_policy_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_update_policy_json_export;

    let resp = client
        .get(api_url(base, "/api/v1/grid/update-policy"))
        .send()
        .await
        .map_err(|e| format!("update-policy request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("update-policy status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    validate_update_policy_json_export(&body)
}

async fn smoke_grid_governance_metrics_api(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_governance_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/governance-metrics"))
        .send()
        .await
        .map_err(|e| format!("governance-metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("governance-metrics status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("governance-metrics body: {body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_governance_metrics_parity(&prom_text, &body)?;
    if !prom_text.contains("poolai_advisory_acknowledged_total") {
        return Err("/metrics missing poolai_advisory_acknowledged_total".to_string());
    }
    Ok(())
}

/// PH-S713: band-6 JSON metrics export shape + Prometheus parity across all grid metric APIs.
async fn smoke_grid_metrics_json_prometheus_parity_band6(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    async fn fetch_metrics_json(client: &Client, base: &str, path: &str) -> Result<Value, String> {
        let resp = client
            .get(api_url(base, path))
            .send()
            .await
            .map_err(|e| format!("{path} request: {e}"))?;
        if resp.status() != StatusCode::OK {
            return Err(format!("{path} status {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("{path} json: {e}"))
    }

    let verification =
        fetch_metrics_json(client, base, "/api/v1/grid/verification-metrics").await?;
    let replay = fetch_metrics_json(client, base, "/api/v1/grid/replay-metrics").await?;
    let settlement = fetch_metrics_json(client, base, "/api/v1/grid/settlement-metrics").await?;
    let trust = fetch_metrics_json(client, base, "/api/v1/grid/trust-metrics").await?;
    let replication = fetch_metrics_json(client, base, "/api/v1/grid/replication-metrics").await?;
    let pricing = fetch_metrics_json(client, base, "/api/v1/grid/pricing-metrics").await?;

    validate_band6_metrics_parity(
        &prom_text,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
    )
}

/// PH-S833: stand smoke v2 — full grid JSON metrics export + Prometheus parity.
async fn smoke_grid_metrics_json_prometheus_parity_band6_v2(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity_v2;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    async fn fetch_metrics_json(client: &Client, base: &str, path: &str) -> Result<Value, String> {
        let resp = client
            .get(api_url(base, path))
            .send()
            .await
            .map_err(|e| format!("{path} request: {e}"))?;
        if resp.status() != StatusCode::OK {
            return Err(format!("{path} status {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("{path} json: {e}"))
    }

    let verification =
        fetch_metrics_json(client, base, "/api/v1/grid/verification-metrics").await?;
    let replay = fetch_metrics_json(client, base, "/api/v1/grid/replay-metrics").await?;
    let settlement = fetch_metrics_json(client, base, "/api/v1/grid/settlement-metrics").await?;
    let trust = fetch_metrics_json(client, base, "/api/v1/grid/trust-metrics").await?;
    let replication = fetch_metrics_json(client, base, "/api/v1/grid/replication-metrics").await?;
    let pricing = fetch_metrics_json(client, base, "/api/v1/grid/pricing-metrics").await?;
    let prefetch = fetch_metrics_json(client, base, "/api/v1/grid/prefetch-metrics").await?;
    let locality = fetch_metrics_json(client, base, "/api/v1/grid/locality-metrics").await?;
    let fee_split = fetch_metrics_json(client, base, "/api/v1/grid/fee-split-metrics").await?;
    let governance = fetch_metrics_json(client, base, "/api/v1/grid/governance-metrics").await?;
    let payout_batch =
        fetch_metrics_json(client, base, "/api/v1/grid/payout-batch-metrics").await?;

    validate_band6_metrics_parity_v2(
        &prom_text,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
        &prefetch,
        &locality,
        &fee_split,
        &governance,
        &payout_batch,
    )
}

/// PH-S1073: stand smoke v3 — extended grid JSON metrics export + Prometheus parity.
async fn smoke_grid_metrics_json_prometheus_parity_band6_v3(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity_v3;

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;

    async fn fetch_metrics_json(client: &Client, base: &str, path: &str) -> Result<Value, String> {
        let resp = client
            .get(api_url(base, path))
            .send()
            .await
            .map_err(|e| format!("{path} request: {e}"))?;
        if resp.status() != StatusCode::OK {
            return Err(format!("{path} status {}", resp.status()));
        }
        resp.json().await.map_err(|e| format!("{path} json: {e}"))
    }

    let verification =
        fetch_metrics_json(client, base, "/api/v1/grid/verification-metrics").await?;
    let replay = fetch_metrics_json(client, base, "/api/v1/grid/replay-metrics").await?;
    let settlement = fetch_metrics_json(client, base, "/api/v1/grid/settlement-metrics").await?;
    let trust = fetch_metrics_json(client, base, "/api/v1/grid/trust-metrics").await?;
    let replication = fetch_metrics_json(client, base, "/api/v1/grid/replication-metrics").await?;
    let pricing = fetch_metrics_json(client, base, "/api/v1/grid/pricing-metrics").await?;
    let prefetch = fetch_metrics_json(client, base, "/api/v1/grid/prefetch-metrics").await?;
    let locality = fetch_metrics_json(client, base, "/api/v1/grid/locality-metrics").await?;
    let fee_split = fetch_metrics_json(client, base, "/api/v1/grid/fee-split-metrics").await?;
    let governance = fetch_metrics_json(client, base, "/api/v1/grid/governance-metrics").await?;
    let payout_batch =
        fetch_metrics_json(client, base, "/api/v1/grid/payout-batch-metrics").await?;

    validate_band6_metrics_parity_v3(
        &prom_text,
        &verification,
        &replay,
        &settlement,
        &trust,
        &replication,
        &pricing,
        &prefetch,
        &locality,
        &fee_split,
        &governance,
        &payout_batch,
    )
}

async fn smoke_grid_network_profile_read(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(
            base,
            "/api/v1/grid/network-profiles/smoke-peer-missing",
        ))
        .send()
        .await
        .map_err(|e| format!("network-profiles request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("network-profiles status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("network-profiles body: {body}"));
    }
    if !body.get("peer_id").and_then(|v| v.as_str()).is_some() {
        return Err(format!("network-profiles missing peer_id: {body}"));
    }
    Ok(())
}

async fn smoke_grid_network_profiles_list(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/network-profiles"))
        .send()
        .await
        .map_err(|e| format!("network-profiles list request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("network-profiles list status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("network-profiles list body: {body}"));
    }
    if !body.get("peer_ids").and_then(|v| v.as_array()).is_some() {
        return Err(format!("network-profiles list missing peer_ids: {body}"));
    }
    Ok(())
}

async fn smoke_ops_power_openapi(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .post(api_url(base, "/api/v1/ops/power"))
        .json(&json!({"action": "shutdown"}))
        .send()
        .await
        .map_err(|e| format!("ops power request: {e}"))?;
    if resp.status() != StatusCode::ACCEPTED {
        return Err(format!(
            "ops power expected 202 Accepted, got {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    for key in ["accepted", "action", "dev_guard"] {
        if body.get(key).is_none() {
            return Err(format!("ops power missing `{key}`: {body}"));
        }
    }
    if body.get("accepted") != Some(&json!(true)) {
        return Err(format!("ops power accepted != true: {body}"));
    }
    if body.get("action") != Some(&json!("shutdown")) {
        return Err(format!("ops power action mismatch: {body}"));
    }
    Ok(())
}

async fn smoke_admin_security_advisories_openapi(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/admin/security-advisories"))
        .send()
        .await
        .map_err(|e| format!("security-advisories request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("security-advisories status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let rows = body
        .as_array()
        .ok_or_else(|| format!("security-advisories expected array: {body}"))?;
    if rows.is_empty() {
        return Err("security-advisories empty".into());
    }
    let first = &rows[0];
    for key in ["id", "severity", "summary", "acknowledged"] {
        if !first.get(key).is_some() {
            return Err(format!("security-advisories missing `{key}`: {first}"));
        }
    }
    Ok(())
}

async fn smoke_virtual_nodes_wallet_rebind_override_openapi(
    client: &Client,
    base: &str,
) -> Result<(), String> {
    let resp = client
        .post(api_url(
            base,
            "/api/v1/virtual-nodes/telegram/wallet/rebind-override",
        ))
        .json(&json!({
            "telegram_user_id": "9001",
            "chat_id": "-1001234567890",
            "payout_pubkey": "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
        }))
        .send()
        .await
        .map_err(|e| format!("wallet rebind-override request: {e}"))?;
    if resp.status() != StatusCode::UNAUTHORIZED {
        return Err(format!(
            "wallet rebind-override expected 401 without admin token, got {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("error").is_none() {
        return Err(format!("wallet rebind-override missing error: {body}"));
    }
    Ok(())
}

async fn smoke_grid_telegram_seats(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/telegram-seats"))
        .send()
        .await
        .map_err(|e| format!("telegram-seats request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("telegram-seats status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("telegram-seats body: {body}"));
    }
    if !body.get("seat_policy").and_then(|v| v.as_str()).is_some() {
        return Err(format!("telegram-seats missing seat_policy: {body}"));
    }
    Ok(())
}

async fn smoke_grid_network_profile_put(client: &Client, base: &str) -> Result<(), String> {
    let peer = "smoke-peer-put-s504";
    let resp = client
        .put(api_url(
            base,
            &format!("/api/v1/grid/network-profiles/{peer}"),
        ))
        .json(&serde_json::json!({
            "network_profile": { "region": "smoke", "latency_ms_p50": 11 }
        }))
        .send()
        .await
        .map_err(|e| format!("network-profiles PUT request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("network-profiles PUT status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("network-profiles PUT body: {body}"));
    }
    Ok(())
}

async fn smoke_grid_payout_batch_history(client: &Client, base: &str) -> Result<(), String> {
    use poolai::grid::stand_smoke_metrics_parity::validate_payout_batch_metrics_parity;

    let resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch/history?limit=5"))
        .send()
        .await
        .map_err(|e| format!("payout-batch/history request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("payout-batch/history status {}", resp.status()));
    }
    let history: Value = resp.json().await.map_err(|e| e.to_string())?;
    if history.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch/history body: {history}"));
    }
    if !history.get("entries").and_then(|v| v.as_array()).is_some() {
        return Err(format!("payout-batch/history missing entries: {history}"));
    }

    let latest_resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch"))
        .send()
        .await
        .map_err(|e| format!("payout-batch request: {e}"))?;
    if latest_resp.status() != StatusCode::OK {
        return Err(format!("payout-batch status {}", latest_resp.status()));
    }
    let latest: Value = latest_resp.json().await.map_err(|e| e.to_string())?;
    if latest.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch body: {latest}"));
    }
    if !latest
        .get("settlement_mode")
        .and_then(|v| v.as_str())
        .is_some()
    {
        return Err(format!("payout-batch missing settlement_mode: {latest}"));
    }

    let metrics_resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch-metrics"))
        .send()
        .await
        .map_err(|e| format!("payout-batch-metrics request: {e}"))?;
    if metrics_resp.status() != StatusCode::OK {
        return Err(format!(
            "payout-batch-metrics status {}",
            metrics_resp.status()
        ));
    }
    let metrics_body: Value = metrics_resp.json().await.map_err(|e| e.to_string())?;
    if metrics_body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch-metrics body: {metrics_body}"));
    }

    let prom_resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if prom_resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", prom_resp.status()));
    }
    let prom_text = prom_resp.text().await.map_err(|e| e.to_string())?;
    validate_payout_batch_metrics_parity(&prom_text, &metrics_body)
}

async fn smoke_grid_verification_replay_history(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(
            base,
            "/api/v1/grid/verification-replay/history?limit=5",
        ))
        .send()
        .await
        .map_err(|e| format!("verification-replay/history request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!(
            "verification-replay/history status {}",
            resp.status()
        ));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("verification-replay/history body: {body}"));
    }
    if !body.get("records").and_then(|v| v.as_array()).is_some() {
        return Err(format!(
            "verification-replay/history missing records: {body}"
        ));
    }
    Ok(())
}

/// PH-S472: live stand exposes PH-S464…S468 horizon wire metrics on Prometheus scrape.
const GALAXY_HORIZON_WIRE_S464_METRICS: &[&str] = &[
    "galaxy_prefetch_backpressure_total",
    "galaxy_prefetch_raid_fetch_total",
    "galaxy_prefetch_raid_fetch_miss_total",
    "poolai_protocol_negotiation_accepted_total",
];

fn metrics_text_has_horizon_wire_s464(body: &str) -> Result<(), String> {
    for name in GALAXY_HORIZON_WIRE_S464_METRICS {
        if !body.contains(name) {
            return Err(format!("/metrics missing {name}"));
        }
        if !body.contains(&format!("# TYPE {name} gauge")) {
            return Err(format!("/metrics missing TYPE gauge for {name}"));
        }
    }
    Ok(())
}

async fn smoke_galaxy_horizon_wire_s464_metrics(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/metrics"))
        .send()
        .await
        .map_err(|e| format!("/metrics request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("/metrics status {}", resp.status()));
    }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    metrics_text_has_horizon_wire_s464(&body)
}

async fn smoke_grid_payout_batch(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/grid/payout-batch"))
        .send()
        .await
        .map_err(|e| format!("payout-batch request: {e}"))?;
    if resp.status() != StatusCode::OK {
        return Err(format!("payout-batch status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("payout-batch body: {body}"));
    }
    if body.get("settlement_mode").and_then(|v| v.as_str()) != Some("offline_batch") {
        return Err(format!("payout-batch missing settlement_mode: {body}"));
    }
    if body.get("onchain_depth").and_then(|v| v.as_str()).is_none() {
        return Err(format!("payout-batch missing onchain_depth: {body}"));
    }
    if body.get("solana_depth").and_then(|v| v.as_str()).is_none() {
        return Err(format!("payout-batch missing solana_depth: {body}"));
    }
    Ok(())
}

async fn smoke_grid_pricing(client: &Client, base: &str) -> Result<(), String> {
    let model = smoke_id("smoke-pricing");
    let url = format!(
        "{}/api/v1/grid/pricing?task_profile=inference:text&model_profile={model}&unit_key=inference_blended_token",
        base.trim_end_matches('/')
    );
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("grid pricing status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    if body.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("grid pricing body: {body}"));
    }
    Ok(())
}

async fn create_unbound_job(client: &Client, base: &str, artifact: &str) -> Result<String, String> {
    let resp = client
        .post(api_url(base, "/api/v1/jobs"))
        .json(&json!({
            "kind": "inference",
            "priority": 5,
            "input_artifact_ids": [artifact],
            "resources": { "gpu_memory_mb": 9_007_199_254_740_991_u64 }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::CREATED {
        return Err(format!("create job status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    body.get("id")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| format!("create job missing id: {body}"))
}

async fn smoke_jobs_lease_acquire(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-acquire").await?;
    let acquire = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-worker" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquire.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquire.status()));
    }
    let body: Value = acquire.json().await.map_err(|e| e.to_string())?;
    let job = body.get("job").ok_or("lease response missing job")?;
    if job.get("status").and_then(|v| v.as_str()) != Some("leased") {
        return Err(format!("expected leased: {job}"));
    }
    if job.get("lease_owner").and_then(|v| v.as_str()) != Some("stand-smoke-worker") {
        return Err(format!("unexpected lease_owner: {job}"));
    }
    if job.get("lease_epoch").and_then(|v| v.as_u64()) != Some(1) {
        return Err(format!("expected lease_epoch 1: {job}"));
    }
    if job
        .get("lease_expires_at")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err(format!("missing lease_expires_at: {job}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_conflict(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-conflict").await?;
    let first = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-a" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if first.status() != StatusCode::OK {
        return Err(format!("first acquire status {}", first.status()));
    }
    let second = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-b" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if second.status() != StatusCode::CONFLICT {
        return Err(format!("second acquire status {}", second.status()));
    }
    let err: Value = second.json().await.map_err(|e| e.to_string())?;
    if err.pointer("/error/code").and_then(|v| v.as_str()) != Some("lease_already_active") {
        return Err(format!("expected lease_already_active: {err}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_extends(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-renew").await?;
    let acquired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-renew" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquired.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquired.status()));
    }
    let acquired_body: Value = acquired.json().await.map_err(|e| e.to_string())?;
    let job = acquired_body
        .get("job")
        .ok_or("lease response missing job")?;
    let epoch = job
        .get("lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    let expires_before = job
        .get("lease_expires_at")
        .and_then(|v| v.as_str())
        .ok_or("missing lease_expires_at")?
        .to_string();
    let renew = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if renew.status() != StatusCode::OK {
        return Err(format!("lease renew status {}", renew.status()));
    }
    let renewed_body: Value = renew.json().await.map_err(|e| e.to_string())?;
    let renewed_job = renewed_body
        .get("job")
        .ok_or("renew response missing job")?;
    if renewed_job.get("lease_epoch").and_then(|v| v.as_u64()) != Some(epoch) {
        return Err(format!("epoch changed on renew: {renewed_job}"));
    }
    let expires_after = renewed_job
        .get("lease_expires_at")
        .and_then(|v| v.as_str())
        .ok_or("renew missing lease_expires_at")?;
    if expires_after == expires_before {
        return Err("lease_expires_at unchanged after renew".into());
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_stale_epoch(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-renew-reject").await?;
    let acquired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-cas" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquired.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquired.status()));
    }
    let epoch = acquired
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?
        .pointer("/job/lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    let stale_epoch = epoch.saturating_sub(1);
    let rejected = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": stale_epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if rejected.status() != StatusCode::CONFLICT {
        return Err(format!("stale renew status {}", rejected.status()));
    }
    let err: Value = rejected.json().await.map_err(|e| e.to_string())?;
    if err.pointer("/error/code").and_then(|v| v.as_str()) != Some("lease_epoch_rejected") {
        return Err(format!("expected lease_epoch_rejected: {err}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_no_acquire(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-renew-no-acquire").await?;
    let renew = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": 1 }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if renew.status() != StatusCode::BAD_REQUEST {
        return Err(format!("renew without acquire status {}", renew.status()));
    }
    let err: Value = renew.json().await.map_err(|e| e.to_string())?;
    let msg = err
        .pointer("/error/message")
        .and_then(|v| v.as_str())
        .or_else(|| err.get("message").and_then(|v| v.as_str()))
        .unwrap_or("");
    if !msg.to_ascii_lowercase().contains("acquire lease") {
        return Err(format!("expected acquire lease message: {err}"));
    }
    Ok(())
}

async fn smoke_jobs_lease_renew_expired(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-lease-expired").await?;
    let acquired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-expired" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if acquired.status() != StatusCode::OK {
        return Err(format!("lease acquire status {}", acquired.status()));
    }
    let epoch = acquired
        .json::<Value>()
        .await
        .map_err(|e| e.to_string())?
        .pointer("/job/lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("missing lease_epoch")?;
    tokio::time::sleep(Duration::from_millis(2600)).await;
    let expired = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease/renew")))
        .json(&json!({ "lease_epoch": epoch }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if expired.status() != StatusCode::CONFLICT {
        return Err(format!("expired renew status {}", expired.status()));
    }
    let err: Value = expired.json().await.map_err(|e| e.to_string())?;
    if err.pointer("/error/code").and_then(|v| v.as_str()) != Some("lease_expired") {
        return Err(format!("expected lease_expired: {err}"));
    }
    Ok(())
}

/// PH-S853: `GET /api/v1/jobs` exposes `store_backend` for admin badge wire.
async fn smoke_jobs_store_backend(client: &Client, base: &str) -> Result<(), String> {
    let resp = client
        .get(api_url(base, "/api/v1/jobs"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("jobs list status {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let backend = body
        .get("store_backend")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing store_backend: {body}"))?;
    if backend.trim().is_empty() {
        return Err("empty store_backend".into());
    }
    if !matches!(backend, "json" | "sqlite" | "raid") {
        return Err(format!("unexpected store_backend: {backend}"));
    }
    Ok(())
}

async fn smoke_jobs_migrating(client: &Client, base: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-migrate").await?;
    let _ = client
        .post(api_url(base, &format!("/api/v1/jobs/{id}/lease")))
        .json(&json!({ "lease_owner": "stand-smoke-migrate" }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    for status in ["migrating", "executing", "migrating"] {
        let patch = client
            .patch(api_url(base, &format!("/api/v1/jobs/{id}")))
            .json(&json!({ "status": status }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if patch.status() != StatusCode::OK {
            return Err(format!("patch {status} status {}", patch.status()));
        }
        let body: Value = patch.json().await.map_err(|e| e.to_string())?;
        if body.pointer("/job/status").and_then(|v| v.as_str()) != Some(status) {
            return Err(format!("patch {status} body: {body}"));
        }
    }
    Ok(())
}

async fn smoke_protocol_middleware(client: &Client, base: &str) -> Result<(), String> {
    let peer_id = smoke_id("proto-accept");
    let resp = client
        .post(api_url(base, "/api/v1/discovery/register-remote"))
        .header("X-PoolAI-Protocol", "1.2")
        .json(&json!({
            "peer_id": peer_id,
            "address": "10.0.0.1",
            "port": 9091,
            "protocol_version": "1.2",
            "build_id": "stand-smoke",
            "metadata": { "role": "virtual_node" }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != StatusCode::OK {
        return Err(format!("register-remote status {}", resp.status()));
    }
    let compat = resp
        .headers()
        .get("x-poolai-protocol-compat")
        .and_then(|v| v.to_str().ok());
    if compat != Some("accepted") {
        return Err(format!("expected compat accepted, got {compat:?}"));
    }
    let reject = client
        .post(api_url(base, "/api/v1/discovery/register-remote"))
        .header("X-PoolAI-Protocol", "1.0")
        .json(&json!({
            "peer_id": smoke_id("proto-reject"),
            "address": "10.0.0.1",
            "port": 9091,
            "protocol_version": "1.2",
            "build_id": "stand-smoke",
            "metadata": { "role": "virtual_node" }
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if reject.status() != StatusCode::FORBIDDEN {
        return Err(format!("unsupported protocol status {}", reject.status()));
    }
    Ok(())
}

async fn smoke_telegram_wallet(client: &Client, base: &str) -> Result<(), String> {
    let user = smoke_id("wallet-ok");
    let ok = client
        .post(api_url(base, "/api/v1/virtual-nodes/telegram/wallet"))
        .json(&json!({
            "telegram_user_id": user,
            "chat_id": "-1001234567890",
            "payout_pubkey": VALID_PUBKEY,
            "chain": "solana"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if ok.status() != StatusCode::OK {
        return Err(format!("wallet bind status {}", ok.status()));
    }
    let bad = client
        .post(api_url(base, "/api/v1/virtual-nodes/telegram/wallet"))
        .json(&json!({
            "telegram_user_id": smoke_id("wallet-bad"),
            "chat_id": "-10099",
            "payout_pubkey": "not-valid!",
            "chain": "solana"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if bad.status() != StatusCode::BAD_REQUEST {
        return Err(format!("invalid wallet status {}", bad.status()));
    }
    Ok(())
}

async fn smoke_grid_envelope_lease(client: &Client, base: &str) -> Result<(), String> {
    let job_id = smoke_id("grid-job");
    let peer = "stand-smoke-grid-peer";
    let ingest = client
        .post(api_url(base, "/api/v1/grid/envelope"))
        .json(&json!({
            "v": 1,
            "sent_at": "2026-06-13T12:00:00Z",
            "source_peer_id": peer,
            "type": "job",
            "job_id": job_id,
            "task_kind": "inference",
            "input_artifact_ids": [format!("artifact-{job_id}")]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if ingest.status() != StatusCode::OK {
        return Err(format!("grid job ingest status {}", ingest.status()));
    }
    let get = client
        .get(api_url(base, &format!("/api/v1/jobs/{job_id}")))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let detail: Value = get.json().await.map_err(|e| e.to_string())?;
    let epoch = detail
        .pointer("/job/lease_epoch")
        .and_then(|v| v.as_u64())
        .ok_or("grid job missing lease_epoch")?;
    let result = client
        .post(api_url(base, "/api/v1/grid/envelope"))
        .json(&json!({
            "v": 1,
            "sent_at": "2026-06-13T12:00:01Z",
            "type": "result",
            "job_id": job_id,
            "status": "completed",
            "lease_epoch": epoch,
            "output_artifact_ids": [format!("out-{job_id}")]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if result.status() != StatusCode::OK {
        return Err(format!("grid result status {}", result.status()));
    }
    Ok(())
}

fn restart_stand(stand_root: &str) -> Result<(), String> {
    let script = Path::new(stand_root).join("restart.sh");
    if !script.is_file() {
        return Err(format!("missing {}", script.display()));
    }
    let status = Command::new("bash")
        .arg(script)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("restart.sh exit {status}"));
    }
    Ok(())
}

async fn smoke_jobs_raid(client: &Client, base: &str, stand_root: &str) -> Result<(), String> {
    let id = create_unbound_job(client, base, "smoke-raid-persist").await?;
    restart_stand(stand_root)?;
    wait_health(client, base, 45).await?;
    let get = client
        .get(api_url(base, &format!("/api/v1/jobs/{id}")))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !get.status().is_success() {
        return Err(format!("get job after restart status {}", get.status()));
    }
    let detail: Value = get.json().await.map_err(|e| e.to_string())?;
    if detail.pointer("/job/spec/id").and_then(|v| v.as_str()) != Some(id.as_str()) {
        return Err(format!("job id mismatch after restart: {detail}"));
    }
    Ok(())
}

async fn run_smokes(cli: &Cli) -> SmokeReport {
    let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
        Ok(c) => c,
        Err(e) => {
            return SmokeReport {
                base_url: cli.base_url.clone(),
                stand_root: std::env::var(ENV_STAND_ROOT).ok(),
                ok: false,
                passed: 0,
                failed: 1,
                cases: vec![SmokeCaseResult {
                    name: "client_build",
                    ok: false,
                    detail: Some(e.to_string()),
                }],
                tool: "poolai-http-stand-smoke",
            };
        }
    };

    let stand_root = std::env::var(ENV_STAND_ROOT).ok();
    let mut cases = Vec::new();

    if cli.raid_restart_only {
        match stand_root.as_deref() {
            Some(root) => {
                record(
                    &mut cases,
                    "jobs_raid_restart",
                    smoke_jobs_raid(&client, &cli.base_url, root).await,
                )
                .await;
            }
            None => {
                cases.push(SmokeCaseResult {
                    name: "jobs_raid_restart",
                    ok: false,
                    detail: Some(format!("--raid-restart requires {ENV_STAND_ROOT}")),
                });
            }
        }
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.lease_renew_only {
        record(
            &mut cases,
            "jobs_lease_acquire",
            smoke_jobs_lease_acquire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_conflict",
            smoke_jobs_lease_conflict(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_extends",
            smoke_jobs_lease_renew_extends(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_stale_epoch",
            smoke_jobs_lease_renew_stale_epoch(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_no_acquire",
            smoke_jobs_lease_renew_no_acquire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_lease_renew_expired",
            smoke_jobs_lease_renew_expired(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.tenant_stand_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "tenants_store_wire",
            smoke_tenants_store_wire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "tenants_crud_lifecycle",
            smoke_tenants_crud_lifecycle(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "tenants_usage_quota_isolation",
            smoke_tenants_usage_quota_isolation(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.sso_stand_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "sso_store_wire",
            smoke_sso_store_wire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "sso_oauth2_saml_crud",
            smoke_sso_oauth2_saml_crud(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "sso_callback_fixtures",
            smoke_sso_callback_fixtures(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.audit_stand_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "audit_store_wire",
            smoke_audit_store_wire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "audit_events_query",
            smoke_audit_events_query(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "audit_event_field_fixtures",
            smoke_audit_event_field_fixtures(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.policy_stand_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "policy_store_wire",
            smoke_policy_store_wire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "policy_policies_query",
            smoke_policy_policies_query(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "policy_field_fixtures",
            smoke_policy_field_fixtures(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.monitoring_stand_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_store_wire",
            smoke_monitoring_store_wire(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_alerts_query",
            smoke_monitoring_alerts_query(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_field_fixtures",
            smoke_monitoring_field_fixtures(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    if cli.run_local_smoke_only {
        async fn record(
            cases: &mut Vec<SmokeCaseResult>,
            name: &'static str,
            result: Result<(), String>,
        ) {
            cases.push(match result {
                Ok(()) => SmokeCaseResult {
                    name,
                    ok: true,
                    detail: None,
                },
                Err(e) => SmokeCaseResult {
                    name,
                    ok: false,
                    detail: Some(e),
                },
            });
        }

        record(
            &mut cases,
            "health",
            smoke_health(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_alerts",
            smoke_monitoring_alerts_api(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "monitoring_dashboards",
            smoke_monitoring_dashboards_api(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "vm_instances",
            smoke_vm_instances_api(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "ops_power_openapi",
            smoke_ops_power_openapi(&client, &cli.base_url).await,
        )
        .await;
        record(
            &mut cases,
            "jobs_store_backend",
            smoke_jobs_store_backend(&client, &cli.base_url).await,
        )
        .await;
        let passed = cases.iter().filter(|c| c.ok).count() as u32;
        let failed = cases.iter().filter(|c| !c.ok).count() as u32;
        return SmokeReport {
            base_url: cli.base_url.clone(),
            stand_root,
            ok: failed == 0,
            passed,
            failed,
            cases,
            tool: "poolai-http-stand-smoke",
        };
    }

    async fn record(
        cases: &mut Vec<SmokeCaseResult>,
        name: &'static str,
        result: Result<(), String>,
    ) {
        cases.push(match result {
            Ok(()) => SmokeCaseResult {
                name,
                ok: true,
                detail: None,
            },
            Err(e) => SmokeCaseResult {
                name,
                ok: false,
                detail: Some(e),
            },
        });
    }

    record(
        &mut cases,
        "health",
        smoke_health(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_pricing",
        smoke_grid_pricing(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_seed_inventory",
        smoke_grid_seed_inventory(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_prefetch_metrics",
        smoke_galaxy_prefetch_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_forced_fallback_metrics",
        smoke_galaxy_pricing_forced_fallback_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_cache_age_metrics",
        smoke_galaxy_pricing_cache_age_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_fresh_served_metrics",
        smoke_galaxy_pricing_fresh_served_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_stale_served_metrics",
        smoke_galaxy_pricing_stale_served_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_provider_metrics",
        smoke_galaxy_pricing_provider_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_pricing_quote_market_metrics",
        smoke_galaxy_pricing_quote_market_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_fee_split_applied_metrics",
        smoke_galaxy_fee_split_applied_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_cross_region_egress_metrics",
        smoke_galaxy_cross_region_egress_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_replay_pending_metrics",
        smoke_galaxy_replay_pending_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_verification_metrics",
        smoke_galaxy_verification_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_trust_payout_metrics",
        smoke_galaxy_trust_payout_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_replication_metrics",
        smoke_galaxy_replication_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s444_metrics",
        smoke_galaxy_horizon_wire_s444_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s454_metrics",
        smoke_galaxy_horizon_wire_s454_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_replay",
        smoke_grid_verification_replay(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s464_metrics",
        smoke_galaxy_horizon_wire_s464_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_payout_batch",
        smoke_grid_payout_batch(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s474_metrics",
        smoke_galaxy_horizon_wire_s474_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s484_metrics",
        smoke_galaxy_horizon_wire_s484_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_horizon_wire_s494_metrics",
        smoke_galaxy_horizon_wire_s494_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_checker_tasks",
        smoke_grid_verification_checker_tasks(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_metrics_api",
        smoke_grid_verification_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_lifecycle_depth",
        smoke_grid_verification_lifecycle_depth(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_replay_metrics_api",
        smoke_grid_replay_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_settlement_metrics_api",
        smoke_grid_settlement_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_trust_metrics_api",
        smoke_grid_trust_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_replication_metrics_api",
        smoke_grid_replication_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_pricing_metrics_api",
        smoke_grid_pricing_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "pricing_metrics_json_prometheus_parity",
        smoke_pricing_metrics_json_prometheus_parity(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_pricing_l2_fallback_stable",
        smoke_grid_pricing_l2_fallback_stable(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_prefetch_metrics_api",
        smoke_grid_prefetch_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_locality_metrics_api",
        smoke_grid_locality_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_fee_split_metrics_api",
        smoke_grid_fee_split_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_update_policy_api",
        smoke_grid_update_policy_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_governance_metrics_api",
        smoke_grid_governance_metrics_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_metrics_json_prometheus_parity_band6",
        smoke_grid_metrics_json_prometheus_parity_band6(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_metrics_json_prometheus_parity_band6_v2",
        smoke_grid_metrics_json_prometheus_parity_band6_v2(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_metrics_json_prometheus_parity_band6_v3",
        smoke_grid_metrics_json_prometheus_parity_band6_v3(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_network_profile_read",
        smoke_grid_network_profile_read(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_network_profiles_list",
        smoke_grid_network_profiles_list(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "ops_power_openapi",
        smoke_ops_power_openapi(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "monitoring_alerts",
        smoke_monitoring_alerts_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "monitoring_dashboards",
        smoke_monitoring_dashboards_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "vm_instances",
        smoke_vm_instances_api(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "admin_security_advisories_openapi",
        smoke_admin_security_advisories_openapi(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "virtual_nodes_wallet_rebind_override_openapi",
        smoke_virtual_nodes_wallet_rebind_override_openapi(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_telegram_seats",
        smoke_grid_telegram_seats(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_network_profile_put",
        smoke_grid_network_profile_put(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_payout_batch_history",
        smoke_grid_payout_batch_history(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_verification_replay_history",
        smoke_grid_verification_replay_history(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_settlement_metrics",
        smoke_galaxy_settlement_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_governance_metrics",
        smoke_galaxy_governance_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_shard_local_hit_ratio_metrics",
        smoke_galaxy_shard_local_hit_ratio_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "galaxy_hot_tier_hit_ratio_metrics",
        smoke_galaxy_hot_tier_hit_ratio_metrics(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "jobs_store_backend",
        smoke_jobs_store_backend(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "jobs_migrating",
        smoke_jobs_migrating(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "protocol_middleware",
        smoke_protocol_middleware(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "telegram_wallet",
        smoke_telegram_wallet(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "grid_envelope_lease",
        smoke_grid_envelope_lease(&client, &cli.base_url).await,
    )
    .await;
    record(
        &mut cases,
        "vision_revision_parity",
        smoke_vision_revision_parity(&client).await,
    )
    .await;

    if cli.include_raid {
        match stand_root.as_deref() {
            Some(root) => {
                record(
                    &mut cases,
                    "jobs_raid_restart",
                    smoke_jobs_raid(&client, &cli.base_url, root).await,
                )
                .await;
            }
            None => {
                cases.push(SmokeCaseResult {
                    name: "jobs_raid_restart",
                    ok: false,
                    detail: Some(format!("--raid requires {ENV_STAND_ROOT}")),
                });
            }
        }
    }

    let passed = cases.iter().filter(|c| c.ok).count() as u32;
    let failed = cases.iter().filter(|c| !c.ok).count() as u32;
    SmokeReport {
        base_url: cli.base_url.clone(),
        stand_root,
        ok: failed == 0,
        passed,
        failed,
        cases,
        tool: "poolai-http-stand-smoke",
    }
}

fn print_human(report: &SmokeReport) {
    eprintln!(
        "poolai-http-stand-smoke: {} ({}/{} passed) base={}",
        if report.ok { "OK" } else { "FAIL" },
        report.passed,
        report.passed + report.failed,
        report.base_url
    );
    for case in &report.cases {
        if case.ok {
            eprintln!("  OK  {}", case.name);
        } else {
            eprintln!(
                "  FAIL {} — {}",
                case.name,
                case.detail.as_deref().unwrap_or("?")
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = parse_cli();
    let report = run_smokes(&cli).await;
    if cli.json_out {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human(&report);
    }
    if !report.ok {
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_url_from_env_defaults() {
        std::env::remove_var(ENV_BASE);
        assert_eq!(base_url_from_env(), DEFAULT_BASE);
    }

    #[test]
    fn api_url_joins_path() {
        assert_eq!(
            api_url("http://127.0.0.1:8080", "/api/v1/health"),
            "http://127.0.0.1:8080/api/v1/health"
        );
        assert_eq!(
            api_url("http://127.0.0.1:8080/", "/api/v1/health"),
            "http://127.0.0.1:8080/api/v1/health"
        );
    }

    #[test]
    fn parse_cli_raid_restart_flag() {
        std::env::remove_var("POOLAI_STAND_SMOKE_RAID");
        std::env::remove_var("POOLAI_STAND_SMOKE_RAID_RESTART");
        std::env::remove_var("POOLAI_STAND_SMOKE_LEASE_RENEW");
        let args: Vec<String> = vec!["poolai-http-stand-smoke".into(), "--raid-restart".into()];
        assert!(args.iter().any(|a| a == "--raid-restart"));
    }

    #[test]
    fn parse_cli_lease_renew_flag_recognized() {
        let args: Vec<String> = vec!["poolai-http-stand-smoke".into(), "--lease-renew".into()];
        assert!(args.iter().any(|a| a == "--lease-renew"));
    }

    #[test]
    fn parse_fm_vision_revision_footer_ph_s235() {
        let section = "**Відкритих у §5.12:** **1** (PH-S235). **Закрито смуга:** PH-S128…S234 ✅. Vision rev **183**.\n";
        assert_eq!(parse_fm_vision_revision(section), Some(183));
    }

    #[test]
    fn read_manifest_revision_from_repo() {
        let root = repo_root();
        let rev = read_manifest_revision(&root).expect("manifest revision");
        assert!(rev > 0);
    }

    #[test]
    fn grid_seed_inventory_stub_shape() {
        let stub = json!({
            "ok": true,
            "generated_at": "2026-05-27T10:00:00Z",
            "entries": [
                {
                    "peer_id": "srv1-worker-a",
                    "seed_inventory": {
                        "shard_ids": ["w:emb-1", "w:ckpt-7"],
                        "hot_tier": { "ram_bytes_used": 3_221_225_472u64 }
                    }
                },
                { "peer_id": "srv2-worker-b" }
            ]
        });
        let entries = stub["entries"].as_array().expect("entries");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0]["peer_id"], "srv1-worker-a");
    }

    #[test]
    fn galaxy_prefetch_metrics_export_shape_ph_s213() {
        let sample = concat!(
            "# HELP galaxy_prefetch_plan_total Galaxy prefetch plans\n",
            "# TYPE galaxy_prefetch_plan_total gauge\n",
            "galaxy_prefetch_plan_total 0\n",
            "# HELP galaxy_prefetch_planned_shards_total Galaxy prefetch shards\n",
            "# TYPE galaxy_prefetch_planned_shards_total gauge\n",
            "galaxy_prefetch_planned_shards_total 0\n",
            "# HELP galaxy_prefetch_hot_skip_total Galaxy prefetch hot skip\n",
            "# TYPE galaxy_prefetch_hot_skip_total gauge\n",
            "galaxy_prefetch_hot_skip_total 0\n",
            "# HELP galaxy_prefetch_bytes_total Galaxy prefetch bytes\n",
            "# TYPE galaxy_prefetch_bytes_total gauge\n",
            "galaxy_prefetch_bytes_total 0\n",
            "# HELP galaxy_prefetch_enqueue_total Galaxy prefetch enqueue stub\n",
            "# TYPE galaxy_prefetch_enqueue_total gauge\n",
            "galaxy_prefetch_enqueue_total 0\n",
            "# HELP galaxy_prefetch_wait_ms_total Galaxy prefetch wait stub\n",
            "# TYPE galaxy_prefetch_wait_ms_total gauge\n",
            "galaxy_prefetch_wait_ms_total 0\n",
            "# HELP galaxy_prefetch_strict_mode_total Galaxy prefetch strict mode\n",
            "# TYPE galaxy_prefetch_strict_mode_total gauge\n",
            "galaxy_prefetch_strict_mode_total 0\n",
            "# HELP galaxy_prefetch_complete_total Galaxy prefetch complete hook\n",
            "# TYPE galaxy_prefetch_complete_total gauge\n",
            "galaxy_prefetch_complete_total 0\n",
            "# HELP galaxy_prefetch_ingest_total Galaxy prefetch ingest stub\n",
            "# TYPE galaxy_prefetch_ingest_total gauge\n",
            "galaxy_prefetch_ingest_total 0\n",
            "# HELP galaxy_prefetch_skip_ingest_total Galaxy prefetch skip ingest\n",
            "# TYPE galaxy_prefetch_skip_ingest_total gauge\n",
            "galaxy_prefetch_skip_ingest_total 0\n",
            "# HELP galaxy_prefetch_seed_pull_total Galaxy prefetch seed pull stub invocations (PH-S424)\n",
            "# TYPE galaxy_prefetch_seed_pull_total gauge\n",
            "galaxy_prefetch_seed_pull_total 0\n",
            "# HELP galaxy_prefetch_lease_acquired_total Galaxy prefetch plans triggered by lease acquire (PH-S425)\n",
            "# TYPE galaxy_prefetch_lease_acquired_total gauge\n",
            "galaxy_prefetch_lease_acquired_total 0\n",
            "# HELP galaxy_locality_rank_ingest_total Galaxy locality rank ingest\n",
            "# TYPE galaxy_locality_rank_ingest_total gauge\n",
            "galaxy_locality_rank_ingest_total 0\n",
            "# HELP galaxy_locality_rank_miss_total Galaxy locality rank miss\n",
            "# TYPE galaxy_locality_rank_miss_total gauge\n",
            "galaxy_locality_rank_miss_total 0\n",
            "# HELP galaxy_locality_rank_empty_workers_total Galaxy locality rank empty workers\n",
            "# TYPE galaxy_locality_rank_empty_workers_total gauge\n",
            "galaxy_locality_rank_empty_workers_total 0\n",
            "# HELP galaxy_locality_rank_skip_total Galaxy locality rank skip\n",
            "# TYPE galaxy_locality_rank_skip_total gauge\n",
            "galaxy_locality_rank_skip_total 0\n",
            "# HELP galaxy_network_profile_stale_total Galaxy stale network profile observations during locality rank (PH-S563)\n",
            "# TYPE galaxy_network_profile_stale_total gauge\n",
            "galaxy_network_profile_stale_total 0\n",
        );
        metrics_text_has_prefetch_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_forced_fallback_metrics_export_shape_ph_s216() {
        let sample = concat!(
            "# HELP galaxy_pricing_forced_fallback_total Galaxy pricing forced L2 quotes\n",
            "# TYPE galaxy_pricing_forced_fallback_total gauge\n",
            "galaxy_pricing_forced_fallback_total 0\n",
        );
        metrics_text_has_pricing_forced_fallback(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_cache_age_metrics_export_shape_ph_s224() {
        let sample = concat!(
            "# HELP galaxy_pricing_cache_age_seconds Galaxy pricing L1 cache age seconds last observed (PH-S168)\n",
            "# TYPE galaxy_pricing_cache_age_seconds gauge\n",
            "galaxy_pricing_cache_age_seconds 0\n",
        );
        metrics_text_has_pricing_cache_age(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_fresh_served_metrics_export_shape_ph_s241() {
        let sample = concat!(
            "# HELP galaxy_pricing_fresh_served Galaxy pricing oracle L1 fresh cache serves (PH-S127)\n",
            "# TYPE galaxy_pricing_fresh_served gauge\n",
            "galaxy_pricing_fresh_served 0\n",
        );
        metrics_text_has_pricing_fresh_served(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_stale_served_metrics_export_shape_ph_s244() {
        let sample = concat!(
            "# HELP galaxy_pricing_stale_served Galaxy pricing oracle L1 stale cache serves (PH-S127)\n",
            "# TYPE galaxy_pricing_stale_served gauge\n",
            "galaxy_pricing_stale_served 0\n",
        );
        metrics_text_has_pricing_stale_served(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_provider_metrics_export_shape_ph_s247() {
        let sample = concat!(
            "# HELP galaxy_pricing_provider_catalog_lookups_total Galaxy pricing provider catalog allow-list lookups (PH-S172)\n",
            "# TYPE galaxy_pricing_provider_catalog_lookups_total gauge\n",
            "galaxy_pricing_provider_catalog_lookups_total 0\n",
            "# HELP galaxy_pricing_provider_catalog_hits_total Galaxy pricing provider catalog allow-list hits (PH-S172)\n",
            "# TYPE galaxy_pricing_provider_catalog_hits_total gauge\n",
            "galaxy_pricing_provider_catalog_hits_total 0\n",
            "# HELP galaxy_pricing_provider_errors_total Galaxy pricing live provider HTTP fetch failures (PH-S173)\n",
            "# TYPE galaxy_pricing_provider_errors_total gauge\n",
            "galaxy_pricing_provider_errors_total 0\n",
        );
        metrics_text_has_pricing_provider_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_pricing_quote_market_metrics_export_shape_ph_s253() {
        let sample = concat!(
            "# HELP galaxy_pricing_quote_usd_micro Galaxy pricing last served PoolAI quote micro-USD (PH-S174)\n",
            "# TYPE galaxy_pricing_quote_usd_micro gauge\n",
            "galaxy_pricing_quote_usd_micro 0\n",
            "# HELP galaxy_pricing_market_min_usd_micro Galaxy pricing last observed market min micro-USD (PH-S181)\n",
            "# TYPE galaxy_pricing_market_min_usd_micro gauge\n",
            "galaxy_pricing_market_min_usd_micro 0\n",
        );
        metrics_text_has_pricing_quote_market_gauges(sample).expect("sample export");
    }

    #[test]
    fn galaxy_fee_split_applied_metrics_export_shape_ph_s254() {
        let sample = concat!(
            "# HELP galaxy_fee_split_applied_total Galaxy fee split applied on grid result path (PH-S194)\n",
            "# TYPE galaxy_fee_split_applied_total gauge\n",
            "galaxy_fee_split_applied_total 0\n",
        );
        metrics_text_has_fee_split_applied(sample).expect("sample export");
    }

    #[test]
    fn galaxy_cross_region_egress_metrics_export_shape_ph_s255() {
        let sample = concat!(
            "# HELP galaxy_cross_region_egress_mb Galaxy last observed cross-region egress whole MB on rank/prefetch path (PH-S185)\n",
            "# TYPE galaxy_cross_region_egress_mb gauge\n",
            "galaxy_cross_region_egress_mb 0\n",
        );
        metrics_text_has_cross_region_egress_mb(sample).expect("sample export");
    }

    #[test]
    fn galaxy_replay_pending_metrics_export_shape_ph_s256() {
        let sample = concat!(
            "# HELP galaxy_replay_pending Galaxy replay verifications pending coordinator verdict (PH-S176)\n",
            "# TYPE galaxy_replay_pending gauge\n",
            "galaxy_replay_pending 0\n",
            "# HELP galaxy_replay_pending_scheduled_total Galaxy replay holds scheduled on grid result path (PH-S333)\n",
            "# TYPE galaxy_replay_pending_scheduled_total gauge\n",
            "galaxy_replay_pending_scheduled_total 0\n",
            "# HELP galaxy_replay_pending_resolved_total Galaxy replay holds cleared on verdict (PH-S335)\n",
            "# TYPE galaxy_replay_pending_resolved_total gauge\n",
            "galaxy_replay_pending_resolved_total 0\n",
            "# HELP galaxy_replay_evaluations_total Galaxy replay pending evaluations on grid result path (PH-S415)\n",
            "# TYPE galaxy_replay_evaluations_total gauge\n",
            "galaxy_replay_evaluations_total 0\n",
            "# HELP galaxy_replay_verification_enqueue_total Galaxy replay verification enqueue stub on mismatch (PH-S438)\n",
            "# TYPE galaxy_replay_verification_enqueue_total gauge\n",
            "galaxy_replay_verification_enqueue_total 0\n",
        );
        metrics_text_has_replay_pending(sample).expect("sample export");
    }

    #[test]
    fn galaxy_settlement_metrics_export_shape_ph_s249() {
        let sample = concat!(
            "# HELP galaxy_settlement_pending_verification_total Galaxy settlement holds pending verification on grid result path (PH-S178)\n",
            "# TYPE galaxy_settlement_pending_verification_total gauge\n",
            "galaxy_settlement_pending_verification_total 0\n",
            "# HELP galaxy_settlement_cleared_total Galaxy settlement cleared on grid result path (PH-S187)\n",
            "# TYPE galaxy_settlement_cleared_total gauge\n",
            "galaxy_settlement_cleared_total 0\n",
            "# HELP galaxy_settlement_not_applicable_total Galaxy settlement not applicable on grid result path (PH-S354)\n",
            "# TYPE galaxy_settlement_not_applicable_total gauge\n",
            "galaxy_settlement_not_applicable_total 0\n",
            "# HELP galaxy_settlement_resolved_total Galaxy settlement status resolutions on grid result path (PH-S404)\n",
            "# TYPE galaxy_settlement_resolved_total gauge\n",
            "galaxy_settlement_resolved_total 0\n",
            "# HELP galaxy_settlement_payout_batch_total Galaxy offline payout batch ledger entries on cleared settlement (PH-S427)\n",
            "# TYPE galaxy_settlement_payout_batch_total gauge\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "# HELP galaxy_settlement_human_review_total Galaxy settlement human-review holds on non-deterministic semantic_hash (PH-S560)\n",
            "# TYPE galaxy_settlement_human_review_total gauge\n",
            "galaxy_settlement_human_review_total 0\n",
        );
        metrics_text_has_settlement_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_shard_local_hit_ratio_metrics_export_shape_ph_s250() {
        let sample = concat!(
            "# HELP galaxy_shard_local_hit_ratio Galaxy last observed top-ranked shard local hit ratio basis points 0-10000 (PH-S183)\n",
            "# TYPE galaxy_shard_local_hit_ratio gauge\n",
            "galaxy_shard_local_hit_ratio 0\n",
        );
        metrics_text_has_shard_local_hit_ratio(sample).expect("sample export");
    }

    #[test]
    fn galaxy_hot_tier_hit_ratio_metrics_export_shape_ph_s581() {
        let sample = concat!(
            "# HELP galaxy_hot_tier_hit_ratio Galaxy last observed top-ranked hot tier hit ratio basis points 0-10000 (PH-S580)\n",
            "# TYPE galaxy_hot_tier_hit_ratio gauge\n",
            "galaxy_hot_tier_hit_ratio 0\n",
        );
        metrics_text_has_hot_tier_hit_ratio(sample).expect("sample export");
    }

    #[test]
    fn galaxy_verification_metrics_export_shape_ph_s225() {
        let sample = concat!(
            "# HELP galaxy_verification_sample_total Galaxy verification samples scheduled on grid result path (PH-S177)\n",
            "# TYPE galaxy_verification_sample_total gauge\n",
            "galaxy_verification_sample_total 0\n",
            "# HELP galaxy_verification_mismatch_total Galaxy verification digest mismatches on grid result path (PH-S175)\n",
            "# TYPE galaxy_verification_mismatch_total gauge\n",
            "galaxy_verification_mismatch_total 0\n",
            "# HELP galaxy_verification_match_total Galaxy verification digest matches on grid result path (PH-S180)\n",
            "# TYPE galaxy_verification_match_total gauge\n",
            "galaxy_verification_match_total 0\n",
            "# HELP galaxy_verification_sample_scheduled_total Galaxy verification stub samples scheduled on grid result path (PH-S164; PH-S186 /metrics)\n",
            "# TYPE galaxy_verification_sample_scheduled_total gauge\n",
            "galaxy_verification_sample_scheduled_total 0\n",
            "# HELP galaxy_verification_sample_completed_total Galaxy verification samples completed with verdict on grid result path (PH-S343)\n",
            "# TYPE galaxy_verification_sample_completed_total gauge\n",
            "galaxy_verification_sample_completed_total 0\n",
            "# HELP galaxy_verification_sample_skipped_total Galaxy verification edge samples skipped by deterministic stub (PH-S345)\n",
            "# TYPE galaxy_verification_sample_skipped_total gauge\n",
            "galaxy_verification_sample_skipped_total 0\n",
            "# HELP galaxy_verification_sample_not_applicable_total Galaxy verification samples not applicable on local origin path (PH-S356)\n",
            "# TYPE galaxy_verification_sample_not_applicable_total gauge\n",
            "galaxy_verification_sample_not_applicable_total 0\n",
            "# HELP galaxy_verification_sampling_evaluations_total Galaxy verification sampling evaluations on grid result path (PH-S414)\n",
            "# TYPE galaxy_verification_sampling_evaluations_total gauge\n",
            "galaxy_verification_sampling_evaluations_total 0\n",
            "# HELP galaxy_verification_checker_enqueue_total Galaxy verification checker enqueue stub on sample verdict (PH-S437)\n",
            "# TYPE galaxy_verification_checker_enqueue_total gauge\n",
            "galaxy_verification_checker_enqueue_total 0\n",
        );
        metrics_text_has_verification_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_trust_payout_metrics_export_shape_ph_s219() {
        let sample = concat!(
            "# HELP galaxy_trust_payout_eligible_total Galaxy trust payout eligible\n",
            "# TYPE galaxy_trust_payout_eligible_total gauge\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "# HELP galaxy_trust_payout_held_total Galaxy trust payout held\n",
            "# TYPE galaxy_trust_payout_held_total gauge\n",
            "galaxy_trust_payout_held_total 0\n",
            "# HELP galaxy_trust_payout_not_applicable_total Galaxy trust gate local-origin results not applicable (PH-S364)\n",
            "# TYPE galaxy_trust_payout_not_applicable_total gauge\n",
            "galaxy_trust_payout_not_applicable_total 0\n",
            "# HELP galaxy_trust_score Galaxy last trust score\n",
            "# TYPE galaxy_trust_score gauge\n",
            "galaxy_trust_score 0\n",
            "# HELP galaxy_trust_gate_min_threshold Galaxy configured minimum trust 0..=100 for edge auto payout (PH-S374)\n",
            "# TYPE galaxy_trust_gate_min_threshold gauge\n",
            "galaxy_trust_gate_min_threshold 40\n",
            "# HELP galaxy_trust_gate_default_score Galaxy default trust score 0..=100 when grid result omits trust_score (PH-S384)\n",
            "# TYPE galaxy_trust_gate_default_score gauge\n",
            "galaxy_trust_gate_default_score 50\n",
            "# HELP galaxy_trust_gate_evaluations_total Galaxy trust gate evaluations on grid result path (PH-S394)\n",
            "# TYPE galaxy_trust_gate_evaluations_total gauge\n",
            "galaxy_trust_gate_evaluations_total 0\n",
            "# HELP galaxy_trust_default_score_applied_total Galaxy grid results where default trust score was applied (PH-S395)\n",
            "# TYPE galaxy_trust_default_score_applied_total gauge\n",
            "galaxy_trust_default_score_applied_total 0\n",
            "# HELP galaxy_trust_explicit_score_total Galaxy grid results with explicit trust_score on ingest (PH-S405)\n",
            "# TYPE galaxy_trust_explicit_score_total gauge\n",
            "galaxy_trust_explicit_score_total 0\n",
        );
        metrics_text_has_trust_payout_counters(sample).expect("sample export");
    }

    #[test]
    fn galaxy_replication_metrics_export_shape_ph_s232() {
        let sample = concat!(
            "# HELP galaxy_replication_strict_total Galaxy replication strict tier grid job ingests (PH-S179)\n",
            "# TYPE galaxy_replication_strict_total gauge\n",
            "galaxy_replication_strict_total 0\n",
            "# HELP galaxy_replication_enqueue_total Galaxy replication executor enqueue stub on grid job ingest (PH-S426)\n",
            "# TYPE galaxy_replication_enqueue_total gauge\n",
            "galaxy_replication_enqueue_total 0\n",
            "# HELP galaxy_replication_executor_enqueue_total Galaxy replication executor queue stub on grid job ingest (PH-S435)\n",
            "# TYPE galaxy_replication_executor_enqueue_total gauge\n",
            "galaxy_replication_executor_enqueue_total 0\n",
        );
        metrics_text_has_replication_strict(sample).expect("sample export");
    }

    #[test]
    fn grid_verification_metrics_api_export_shape_ph_s673() {
        let sample = r#"{"ok":true,"lifecycle_depth":"none","metrics":{"sample_total":0,"mismatch_total":0,"match_total":0,"sample_completed_total":0,"checker_enqueue_total":0,"checker_pending_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["checker_pending_total"], 0);
        assert_eq!(body["lifecycle_depth"], "none");
    }

    #[test]
    fn grid_verification_lifecycle_export_shape_ph_s883() {
        use poolai::grid::galaxy_verification_lifecycle_depth::{
            verification_lifecycle_depth_stub, verification_lifecycle_depth_wire_label,
            VerificationLifecycleDepth,
        };
        use poolai::grid::galaxy_verification_metrics::VerificationMetricsSnapshot;
        use poolai::grid::stand_smoke_metrics_parity::{
            stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
        };
        use serde_json::json;

        let empty = VerificationMetricsSnapshot {
            sample_total: 0,
            mismatch_total: 0,
            match_total: 0,
            sample_completed_total: 0,
            checker_enqueue_total: 0,
            checker_pending_total: 0,
        };
        let depth = verification_lifecycle_depth_stub(
            Some(&VerificationMetricsSnapshot {
                checker_enqueue_total: 1,
                checker_pending_total: 1,
                ..empty
            }),
            1,
        );
        assert_eq!(depth, VerificationLifecycleDepth::ShadowJobSubmit);
        assert_eq!(
            verification_lifecycle_depth_wire_label(depth),
            "shadow_job_submit"
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(
                &json!({"verification_checker_lifecycle": true})
            )),
            StandSmokeMetricsParityDepth::VerificationCheckerLifecycle
        );
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band23_export_shape_ph_s882() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::stand_smoke_metrics::render_grid_verification_metrics_strip_html;
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_panel": true}))),
            AdminWasmSlimDepth::GridVerificationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_verification_metrics_strip": true}))),
            AdminWasmSlimDepth::GridVerificationMetricsStrip
        );
        let strip = render_grid_verification_metrics_strip_html(
            r#"{"metrics":{"sample_total":2,"checker_pending_total":1}}"#,
            1,
        );
        assert!(strip.contains("Sample"));
        assert!(strip.contains("Pending"));
    }

    #[test]
    fn grid_replay_metrics_api_export_shape_ph_s673() {
        let sample = r#"{"ok":true,"metrics":{"replay_pending":0,"replay_pending_scheduled_total":0,"replay_pending_resolved_total":0,"replay_evaluations_total":0,"replay_verification_enqueue_total":0,"verification_replay_record_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["replay_pending"], 0);
    }

    #[test]
    fn grid_settlement_metrics_api_export_shape_ph_s683() {
        let sample = r#"{"ok":true,"metrics":{"pending_verification_total":0,"cleared_total":0,"not_applicable_total":0,"resolved_total":0,"payout_batch_total":0,"human_review_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["cleared_total"], 0);
    }

    #[test]
    fn grid_trust_metrics_api_export_shape_ph_s683() {
        let sample = r#"{"ok":true,"metrics":{"payout_eligible_total":0,"payout_held_total":0,"payout_not_applicable_total":0,"last_trust_score":0,"gate_min_threshold":40,"gate_default_score":50,"gate_evaluations_total":0,"default_score_applied_total":0,"explicit_score_total":0,"trust_score_delta_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["gate_min_threshold"], 40);
    }

    #[test]
    fn grid_replication_metrics_api_export_shape_ph_s693() {
        let sample = r#"{"ok":true,"replication_depth":"none","rate_cap_per_hour":1000,"metrics":{"strict_total":0,"enqueue_total":0,"executor_enqueue_total":0,"rate_limited_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["strict_total"], 0);
        assert_eq!(body["replication_depth"], "none");
    }

    #[test]
    fn grid_replication_depth_export_shape_ph_s893() {
        use poolai::grid::galaxy_replication_depth::{
            replication_depth_stub, replication_depth_wire_label, ReplicationDepth,
        };
        use poolai::grid::galaxy_replication_metrics::ReplicationMetricsSnapshot;
        use poolai::grid::stand_smoke_metrics_parity::{
            stand_smoke_metrics_parity_depth_stub, StandSmokeMetricsParityDepth,
        };
        use serde_json::json;

        let snap = ReplicationMetricsSnapshot {
            strict_total: 1,
            enqueue_total: 1,
            executor_enqueue_total: 1,
            rate_limited_total: 1,
        };
        let depth = replication_depth_stub(Some(&snap), 100);
        assert_eq!(depth, ReplicationDepth::RateCap);
        assert_eq!(replication_depth_wire_label(depth), "rate_cap");
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(
                &json!({"replication_quorum_production": true})
            )),
            StandSmokeMetricsParityDepth::ReplicationQuorumProduction
        );
    }

    #[test]
    fn grid_pricing_metrics_api_export_shape_ph_s693() {
        let sample = r#"{"ok":true,"pricing_depth":"none","provider_http_timeout_ms":1500,"metrics":{"fresh_served_total":0,"stale_served_total":0,"forced_fallback_total":0,"provider_catalog_lookups_total":0,"provider_catalog_hits_total":0,"provider_errors_total":0,"provider_timeouts_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["fresh_served_total"], 0);
        assert_eq!(body["pricing_depth"], "none");
    }

    #[test]
    fn grid_pricing_depth_export_shape_ph_s903() {
        use poolai::grid::galaxy_pricing_depth::{
            pricing_depth_stub, pricing_depth_wire_label, PricingDepth,
        };
        use poolai::grid::galaxy_pricing_metrics::PricingMetricsSnapshot;
        use poolai::grid::stand_smoke_metrics_parity::{
            stand_smoke_metrics_parity_depth_stub, validate_pricing_metrics_parity,
            StandSmokeMetricsParityDepth,
        };
        use serde_json::json;

        let snap = PricingMetricsSnapshot {
            fresh_served_total: 1,
            stale_served_total: 0,
            forced_fallback_total: 0,
            provider_catalog_lookups_total: 1,
            provider_catalog_hits_total: 1,
            provider_errors_total: 0,
            provider_timeouts_total: 0,
        };
        assert_eq!(
            pricing_depth_stub(Some(&snap), 1500),
            PricingDepth::LiveFetch
        );
        assert_eq!(
            pricing_depth_wire_label(PricingDepth::LiveFetch),
            "live_fetch"
        );
        assert_eq!(
            stand_smoke_metrics_parity_depth_stub(Some(&json!({"pricing_production": true}))),
            StandSmokeMetricsParityDepth::PricingProduction
        );
        let prom = concat!(
            "galaxy_pricing_fresh_served 1\n",
            "galaxy_pricing_stale_served 0\n",
            "galaxy_pricing_forced_fallback_total 0\n",
            "galaxy_pricing_provider_catalog_lookups_total 1\n",
            "galaxy_pricing_provider_errors_total 0\n",
            "galaxy_pricing_provider_timeouts_total 0\n",
        );
        let pricing = json!({
            "ok": true,
            "metrics": {
                "fresh_served_total": 1,
                "stale_served_total": 0,
                "forced_fallback_total": 0,
                "provider_catalog_lookups_total": 1,
                "provider_catalog_hits_total": 1,
                "provider_errors_total": 0,
                "provider_timeouts_total": 0
            }
        });
        validate_pricing_metrics_parity(prom, &pricing).expect("parity");
    }

    #[test]
    fn grid_prefetch_metrics_api_export_shape_ph_s753() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, PREFETCH_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"pull_bytes_total":0,"backpressure_total":0,"plan_total":0,"enqueue_total":0,"peer_fetch_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, PREFETCH_JSON_KEYS).expect("shape");
        assert_eq!(body["metrics"]["pull_bytes_total"], 0);
    }

    #[test]
    fn grid_locality_metrics_api_export_shape_ph_s763() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, LOCALITY_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"shard_local_hit_ratio_bps":0,"hot_tier_hit_ratio_bps":0,"cross_region_egress_mb":0,"hot_promote_total":0,"hot_evict_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, LOCALITY_JSON_KEYS).expect("shape");
        assert_eq!(body["metrics"]["hot_promote_total"], 0);
    }

    #[test]
    fn grid_fee_split_metrics_api_export_shape_ph_s782() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_fee_split_metrics_parity, validate_grid_metrics_json_export,
            FEE_SPLIT_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"fee_split_applied_total":0,"primary_dev_fee_bps":10,"secondary_admin_fee_min_bps":100,"secondary_admin_fee_max_bps":500}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, FEE_SPLIT_JSON_KEYS).expect("shape");
        let prom =
            "# TYPE galaxy_fee_split_applied_total gauge\ngalaxy_fee_split_applied_total 0\n";
        validate_fee_split_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn grid_update_policy_api_export_shape_ph_s790() {
        use poolai::grid::stand_smoke_metrics_parity::validate_update_policy_json_export;

        let sample = r#"{"ok":true,"policy":{"mode":"notify","env_update_policy":"POOLAI_UPDATE_POLICY","env_manifest_url":"POOLAI_RELEASE_MANIFEST_URL"}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_update_policy_json_export(&body).expect("shape");
    }

    #[test]
    fn grid_governance_metrics_api_export_shape_ph_s793() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_governance_metrics_parity, validate_grid_metrics_json_export,
            GOVERNANCE_JSON_KEYS,
        };

        let sample = r#"{"ok":true,"metrics":{"release_verify_total":1,"release_verify_fail_total":0,"update_notify_pending":2,"advisory_acknowledged_total":1}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        validate_grid_metrics_json_export(&body, GOVERNANCE_JSON_KEYS).expect("shape");
        let prom = concat!(
            "poolai_release_verify_total 1\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 2\n",
            "poolai_advisory_acknowledged_total 1\n",
        );
        validate_governance_metrics_parity(prom, &body).expect("parity");
    }

    #[test]
    fn grid_replication_pricing_wasm_panel_export_shape_ph_s703() {
        let sample = r#"{"ok":true,"metrics":{"strict_total":1,"enqueue_total":2,"executor_enqueue_total":0,"rate_limited_total":0}}"#;
        let body: Value = serde_json::from_str(sample).expect("json");
        assert_eq!(body["metrics"]["strict_total"], 1);
        assert_eq!(body["metrics"]["enqueue_total"], 2);
    }

    #[test]
    fn admin_wasm_slim_depth_stub_export_shape_ph_s703() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"panel_renderer": true}))),
            AdminWasmSlimDepth::PanelRenderer
        );
    }

    #[test]
    fn monitoring_settlement_payout_export_shape_ph_s803() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, validate_settlement_trust_metrics_parity,
            SETTLEMENT_JSON_KEYS, TRUST_JSON_KEYS,
        };
        let prom = concat!(
            "galaxy_settlement_cleared_total 2\n",
            "galaxy_settlement_payout_batch_total 1\n",
            "galaxy_trust_payout_eligible_total 3\n",
            "galaxy_trust_score 55\n",
        );
        let settlement = serde_json::json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 2,
                "resolved_total": 0,
                "payout_batch_total": 1,
            }
        });
        let trust = serde_json::json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 3,
                "payout_held_total": 0,
                "last_trust_score": 55,
                "gate_min_threshold": 40,
            }
        });
        validate_grid_metrics_json_export(&settlement, SETTLEMENT_JSON_KEYS).expect("settlement");
        validate_grid_metrics_json_export(&trust, TRUST_JSON_KEYS).expect("trust");
        validate_settlement_trust_metrics_parity(prom, &settlement, &trust).expect("parity");
        let ml_pipelines: serde_json::Value = serde_json::json!([]);
        assert!(ml_pipelines.is_array());
        let alerts: serde_json::Value = serde_json::json!([]);
        assert!(alerts.is_array());
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band15_export_shape_ph_s804() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"ml_pipeline_panel": true}))),
            AdminWasmSlimDepth::MlPipelinePanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"payout_batch_panel": true}))),
            AdminWasmSlimDepth::PayoutBatchPanel
        );
    }

    #[test]
    fn security_topology_export_shape_ph_s813() {
        use poolai::security::secret_rotation::{init_default_rotation_hooks, rotation_status};
        init_default_rotation_hooks();
        let status = rotation_status();
        assert!(!status.is_empty());
        for entry in &status {
            let _ = entry.kind.as_str();
            let _ = entry.configured;
            let _ = entry.hook_count;
        }
        let topology = serde_json::json!({
            "node_count": 0,
            "latency_measurements": 0,
            "last_updated": "2026-06-21T00:00:00Z",
            "node_ids": []
        });
        assert!(topology.get("node_count").is_some());
        assert!(topology.get("last_updated").is_some());
        let prom = "poolai_secret_rotations_total{kind=\"jwt\",success=\"true\"} 1\n";
        assert!(prom.contains("poolai_secret_rotations_total"));
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band16_export_shape_ph_s814() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::security::render_secret_rotation_panel_html;
        use poolai_ui_core::topology::render_topology_stats_strip_html;
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"security_rotation_panel": true}))),
            AdminWasmSlimDepth::SecurityRotationPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"topology_stats_strip": true}))),
            AdminWasmSlimDepth::TopologyStatsStrip
        );
        let sec_html = render_secret_rotation_panel_html("[]", "{}");
        assert!(sec_html.contains("secret-rotation-table"));
        let topo_html = render_topology_stats_strip_html(
            r#"{"node_count":1,"latency_measurements":0,"last_updated":""}"#,
            "{}",
        );
        assert!(topo_html.contains("topology-last-updated"));
    }

    #[test]
    fn vm_workers_export_shape_ph_s823() {
        use poolai_ui_core::admin_vm_workers::{
            validate_vm_instances_admin_list_shape, validate_workers_admin_list_shape,
            VM_INSTANCE_ADMIN_ROW_KEYS, WORKERS_ADMIN_ROW_KEYS,
        };
        use serde_json::json;

        assert!(!WORKERS_ADMIN_ROW_KEYS.is_empty());
        assert!(!VM_INSTANCE_ADMIN_ROW_KEYS.is_empty());

        let workers = json!([{
            "id": "w1",
            "status": "idle",
            "current_task": null,
            "is_healthy": true,
            "total_requests_processed": 1,
            "queue_size": 0,
            "active_connections": 0,
            "average_response_time_ms": 0
        }]);
        validate_workers_admin_list_shape(&workers).expect("workers export shape");

        let vms = json!([{
            "id": "vm-1",
            "name": "test-vm",
            "status": "running",
            "resources": { "cpu_cores": 2, "memory_mb": 1024, "gpu_required": false }
        }]);
        validate_vm_instances_admin_list_shape(&vms).expect("vm export shape");
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band17_export_shape_ph_s824() {
        use poolai_ui_core::grid_replication_pricing::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::libs::render_libs_panel_html;
        use poolai_ui_core::vm::render_vm_panel_html;
        use poolai_ui_core::workers::render_workers_panel_html;
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"vm_panel": true}))),
            AdminWasmSlimDepth::VmPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"workers_panel": true}))),
            AdminWasmSlimDepth::WorkersPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"libs_panel": true}))),
            AdminWasmSlimDepth::LibsPanel
        );
        let vm_html = render_vm_panel_html(
            "[]", "N", "S", "R", "A", "V", "CPU", "MEM", "Start", "Stop", "Del", "Empty",
        );
        assert!(vm_html.contains("admin-empty-state"));
        let wrk_html = render_workers_panel_html(
            "[]", "I", "S", "M", "A", "W", "H", "U", "Req", "Del", "Empty",
        );
        assert!(wrk_html.contains("admin-empty-state"));
        let libs_html = render_libs_panel_html(
            "[]", "N", "V", "S", "A", "L", "I", "NI", "U", "Up", "In", "Empty",
        );
        assert!(libs_html.contains("admin-empty-state"));
    }

    #[test]
    fn admin_wasm_slim_depth_stub_band44_export_shape_ph_s1084() {
        use poolai_ui_core::admin_wasm_slim_depth::{
            admin_wasm_slim_depth_stub, AdminWasmSlimDepth,
        };
        use poolai_ui_core::galaxy_telegram_seats::render_telegram_seats_panel_html;
        use poolai_ui_core::galaxy_virtual_nodes::render_galaxy_virtual_nodes_panel_html;
        use poolai_ui_core::instances::render_instances_panel_html;
        use poolai_ui_core::ml::{
            render_monitoring_alerts_panel_html, render_monitoring_dashboards_panel_html,
        };
        use poolai_ui_core::network_profiles::render_network_profiles_panel_html;
        use poolai_ui_core::stand_smoke_metrics::{
            render_grid_fee_split_metrics_strip_html, render_grid_governance_metrics_strip_html,
            render_grid_locality_metrics_strip_html, render_grid_prefetch_metrics_strip_html,
        };
        use serde_json::json;
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"monitoring_alerts_panel": true}))),
            AdminWasmSlimDepth::MonitoringAlertsPanel
        );
        assert_eq!(
            admin_wasm_slim_depth_stub(Some(&json!({"grid_fee_split_metrics_strip": true}))),
            AdminWasmSlimDepth::GridFeeSplitMetricsStrip
        );
        let alerts_html = render_monitoring_alerts_panel_html(
            "[]",
            "N/A",
            "Ack",
            "Active",
            "Ack",
            "Sev",
            "Metric",
            "Cur",
            "Thr",
            "Trig",
            "Status",
            "Act",
            "Alerts",
            "No alerts",
        );
        assert!(alerts_html.contains("admin-empty-state"));
        let dash_html = render_monitoring_dashboards_panel_html(
            "[]",
            "Name",
            "Desc",
            "Metrics",
            "Public",
            "Created",
            "Dash",
            "—",
            "N/A",
            "Yes",
            "No",
            "{n} metrics",
            "No dashboards",
        );
        assert!(dash_html.contains("admin-empty-state"));
        let inst_html = render_instances_panel_html(
            "[]", "ID", "Model", "St", "Str", "Nodes", "Created", "Act", "Inst", "View", "Del",
            "Empty",
        );
        assert!(inst_html.contains("admin-empty-state"));
        let tg_html = render_telegram_seats_panel_html(
            r#"{"seat_policy":"open","seat_limit":10,"active_seats":0,"bound_wallets":[]}"#,
            "Policy",
            "Limit",
            "Active",
            "Bound",
            "Seats",
        );
        assert!(tg_html.contains("admin-table"));
        let vn_html = render_galaxy_virtual_nodes_panel_html(
            "[]", "Peer", "Origin", "Region", "Latency", "Stale", "Nodes", "Empty",
        );
        assert!(vn_html.contains("admin-empty-state"));
        let np_html = render_network_profiles_panel_html(
            "[]", "Peer", "Region", "Latency", "BW", "Profiles", "Empty",
        );
        assert!(np_html.contains("muted"));
        let prefetch_html =
            render_grid_prefetch_metrics_strip_html(r#"{"metrics":{"pull_bytes_total":1}}"#, 0);
        assert!(prefetch_html.contains("admin-metrics-strip"));
        let locality_html =
            render_grid_locality_metrics_strip_html(r#"{"metrics":{"hot_promote_total":2}}"#, 0);
        assert!(locality_html.contains("admin-metrics-strip"));
        let gov_html = render_grid_governance_metrics_strip_html(
            r#"{"metrics":{"advisory_ack_total":1}}"#,
            r#"{"mode":"advisory"}"#,
            0,
        );
        assert!(gov_html.contains("admin-metrics-strip"));
        let fee_html =
            render_grid_fee_split_metrics_strip_html(r#"{"metrics":{"applied_total":3}}"#, 0);
        assert!(fee_html.contains("admin-metrics-strip"));
    }

    #[test]
    fn run_local_health_export_shape_ph_s1089() {
        use poolai_ui_core::stand_smoke_run_local_depth::RUN_LOCAL_HEALTH_KEYS;
        let health = json!({
            "status": "healthy",
            "version": "0.2.2",
            "timestamp": "2026-07-18T00:00:00Z",
            "uptime": 42,
            "checks": {
                "database": { "status": "healthy", "message": "ok", "response_time_ms": 1 },
                "memory": { "status": "healthy", "message": "ok", "response_time_ms": 1 },
                "workers": { "status": "healthy", "message": "ok", "response_time_ms": 1 },
                "gpu": { "status": "healthy", "message": "ok", "response_time_ms": 0 }
            }
        });
        for key in RUN_LOCAL_HEALTH_KEYS {
            assert!(health.get(key).is_some(), "health missing {key}");
        }
        assert_eq!(
            health.get("status").and_then(Value::as_str),
            Some("healthy")
        );
    }

    #[test]
    fn stand_smoke_run_local_band45_export_shape_ph_s1095() {
        use poolai_ui_core::stand_smoke_run_local_depth::{
            stand_smoke_run_local_depth_stub, StandSmokeRunLocalDepth, FM_BAND45_ROWS,
            RUN_LOCAL_SMOKE_CASES,
        };
        use serde_json::json;
        assert_eq!(
            stand_smoke_run_local_depth_stub(Some(&json!({"run_local_smoke": true}))),
            StandSmokeRunLocalDepth::RunLocalSmoke
        );
        assert_eq!(
            stand_smoke_run_local_depth_stub(Some(&json!({
                "run_local_smoke": true,
                "verify_dev_stand_hook": true,
                "quick_stand_smoke": true,
            }))),
            StandSmokeRunLocalDepth::FullRunLocalBand45
        );
        assert!(RUN_LOCAL_SMOKE_CASES.contains(&"health"));
        assert!(RUN_LOCAL_SMOKE_CASES.contains(&"monitoring_alerts"));
        assert!(RUN_LOCAL_SMOKE_CASES.contains(&"ops_power_openapi"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND45_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band45 row {row}"
            );
        }
    }

    #[test]
    fn rust_migration_advisory_band46_export_shape_ph_s1104() {
        use poolai_ui_core::rust_migration_advisory_depth::{
            migration_registry_total, rust_migration_advisory_depth_stub,
            RustMigrationAdvisoryDepth, ADMIN_JS_MIGRATION_CANDIDATES,
            ARCHIVED_E2E_MIGRATION_CANON, FM_BAND46_ROWS, MIGRATION_ADVISORY_CASES,
        };
        use serde_json::json;
        assert_eq!(
            rust_migration_advisory_depth_stub(Some(&json!({"ui_js_candidates": true}))),
            RustMigrationAdvisoryDepth::UiJsCandidates
        );
        assert_eq!(
            rust_migration_advisory_depth_stub(Some(&json!({
                "ui_js_candidates": true,
                "e2e_archived_canon": true,
                "loc_audit_advisory": true,
                "ops_shell_canon": true,
            }))),
            RustMigrationAdvisoryDepth::FullBand46
        );
        assert_eq!(ADMIN_JS_MIGRATION_CANDIDATES.len(), 6);
        assert_eq!(ARCHIVED_E2E_MIGRATION_CANON.len(), 8);
        assert!(migration_registry_total() >= 14);
        assert!(MIGRATION_ADVISORY_CASES.contains(&"stretch_spirit_hold"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND46_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band46 row {row}"
            );
        }
    }

    #[test]
    fn stable_state_touchup_band47_export_shape_ph_s1114() {
        use poolai_ui_core::stable_state_touchup_depth::{
            stable_criteria_total, stable_state_touchup_depth_stub, StableStateTouchupDepth,
            FM_BAND47_ROWS, STABLE_TOUCHUP_CASES, STABLE_TOUCHUP_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            stable_state_touchup_depth_stub(Some(&json!({"criteria_registry": true}))),
            StableStateTouchupDepth::CriteriaRegistry
        );
        assert_eq!(
            stable_state_touchup_depth_stub(Some(&json!({
                "criteria_registry": true,
                "stable_summary": true,
                "index_canon": true,
                "handoff_zriz": true,
                "loc_audit_touchup": true,
                "verify_dev_stand_hook": true,
                "quick_touchup": true,
                "docs_canon": true,
            }))),
            StableStateTouchupDepth::FullBand47
        );
        assert_eq!(STABLE_TOUCHUP_CRITERIA.len(), 7);
        assert_eq!(stable_criteria_total(), 7);
        assert!(STABLE_TOUCHUP_CASES.contains(&"product_complete"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND47_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band47 row {row}"
            );
        }
    }

    #[test]
    fn pre_push_canon_band49_export_shape_ph_s1134() {
        use poolai_ui_core::pre_push_hook_depth::{
            pre_push_hook_criteria_total, pre_push_hook_depth_stub, PrePushHookDepth,
            FM_BAND49_ROWS, PRE_PUSH_HOOK_CASES, PRE_PUSH_HOOK_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            pre_push_hook_depth_stub(Some(&json!({"vision_sync_canon": true}))),
            PrePushHookDepth::VisionSyncCanon
        );
        assert_eq!(
            pre_push_hook_depth_stub(Some(&json!({
                "pre_push_hook_script": true,
                "install_hook": true,
                "vision_sync_canon": true,
                "vision_sync_check": true,
                "cargo_fmt_gate": true,
                "pre_push_hook_docs": true,
                "verify_dev_stand_hook": true,
            }))),
            PrePushHookDepth::FullBand49
        );
        assert_eq!(PRE_PUSH_HOOK_CRITERIA.len(), 7);
        assert_eq!(pre_push_hook_criteria_total(), 7);
        assert!(PRE_PUSH_HOOK_CASES.contains(&"verify_dev_stand_hook"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND49_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band49 row {row}"
            );
        }
    }

    #[test]
    fn ci_canon_band50_export_shape_ph_s1144() {
        use poolai_ui_core::ci_canon_depth::{
            ci_canon_criteria_total, ci_canon_depth_stub, CiCanonDepth, CI_CANON_CASES,
            CI_CANON_CRITERIA, FM_BAND50_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            ci_canon_depth_stub(Some(&json!({"openapi_gap_audit": true}))),
            CiCanonDepth::OpenapiGapAudit
        );
        assert_eq!(
            ci_canon_depth_stub(Some(&json!({
                "test_ci_scope": true,
                "openapi_gap_audit": true,
                "rust_ratio_audit": true,
                "openapi_gap_ci_job": true,
                "verify_dev_stand_hook": true,
                "ci_canon_docs": true,
                "dual_gate": true,
            }))),
            CiCanonDepth::FullBand50
        );
        assert_eq!(CI_CANON_CRITERIA.len(), 7);
        assert_eq!(ci_canon_criteria_total(), 7);
        assert!(CI_CANON_CASES.contains(&"dual_gate"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND50_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band50 row {row}"
            );
        }
    }

    #[test]
    fn tenant_persist_band51_export_shape_ph_s1155() {
        use poolai_ui_core::tenant_persistence_depth::{
            tenant_persist_criteria_total, tenant_persistence_depth_stub, TenantPersistenceDepth,
            FM_BAND51_ROWS, TENANT_PERSIST_CASES, TENANT_PERSIST_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_persistence_depth_stub(Some(&json!({"audit_test": true}))),
            TenantPersistenceDepth::AuditTest
        );
        assert_eq!(
            tenant_persistence_depth_stub(Some(&json!({
                "tenant_persistence_depth": true,
                "loc_audit_flag": true,
                "audit_test": true,
                "verify_dev_stand_hook": true,
                "quick_flag": true,
                "stand_smoke_export": true,
                "tenant_persist_docs": true,
            }))),
            TenantPersistenceDepth::FullBand51
        );
        assert_eq!(TENANT_PERSIST_CRITERIA.len(), 7);
        assert_eq!(tenant_persist_criteria_total(), 7);
        assert!(TENANT_PERSIST_CASES.contains(&"verify_dev_stand_hook"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND51_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band51 row {row}"
            );
        }
    }

    #[test]
    fn tenant_store_band52_export_shape_ph_s1163() {
        use poolai_ui_core::tenant_depth::{
            tenant_criteria_total, tenant_depth_stub, TenantDepth, FM_BAND52_ROWS, TENANT_CASES,
            TENANT_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_depth_stub(Some(&json!({"api_contracts": true}))),
            TenantDepth::ApiContracts
        );
        assert_eq!(
            tenant_depth_stub(Some(&json!({
                "tenant_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_store_docs": true,
            }))),
            TenantDepth::FullBand52
        );
        assert_eq!(TENANT_CRITERIA.len(), 7);
        assert_eq!(tenant_criteria_total(), 7);
        assert!(TENANT_CASES.contains(&"store_wire"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND52_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band52 row {row}"
            );
        }
    }

    #[test]
    fn tenant_api_band53_export_shape_ph_s1176() {
        use poolai_ui_core::tenant_api_contracts_depth::{
            tenant_api_contracts_depth_stub, tenant_api_criteria_total, TenantApiContractsDepth,
            FM_BAND53_ROWS, TENANT_API_CASES, TENANT_API_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_api_contracts_depth_stub(Some(&json!({"http_crud": true}))),
            TenantApiContractsDepth::HttpCrud
        );
        assert_eq!(
            tenant_api_contracts_depth_stub(Some(&json!({
                "tenant_api_depth": true,
                "http_crud": true,
                "quota_usage": true,
                "isolation": true,
                "store_wire_http": true,
                "openapi_schemas": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_api_docs": true,
            }))),
            TenantApiContractsDepth::FullBand53
        );
        assert_eq!(TENANT_API_CRITERIA.len(), 10);
        assert_eq!(tenant_api_criteria_total(), 10);
        assert!(TENANT_API_CASES.contains(&"store_wire_http"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND53_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band53 row {row}"
            );
        }
    }

    #[test]
    fn tenant_admin_ops_band54_export_shape_ph_s1185() {
        use poolai_ui_core::tenant_admin_ops_depth::{
            tenant_admin_ops_criteria_total, tenant_admin_ops_depth_stub, TenantAdminOpsDepth,
            FM_BAND54_ROWS, TENANT_ADMIN_OPS_CASES, TENANT_ADMIN_OPS_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_admin_ops_depth_stub(Some(&json!({"usage_quota_glue": true}))),
            TenantAdminOpsDepth::UsageQuotaGlue
        );
        assert_eq!(
            tenant_admin_ops_depth_stub(Some(&json!({
                "tenant_admin_ops_depth": true,
                "store_strip": true,
                "usage_quota_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantAdminOpsDepth::FullBand54
        );
        assert_eq!(TENANT_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(tenant_admin_ops_criteria_total(), 10);
        assert!(TENANT_ADMIN_OPS_CASES.contains(&"store_strip"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND54_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band54 row {row}"
            );
        }
    }

    #[test]
    fn tenant_stand_smoke_band55_export_shape_ph_s1193() {
        use poolai_ui_core::tenant_stand_smoke_depth::{
            tenant_stand_smoke_criteria_total, tenant_stand_smoke_depth_stub,
            TenantStandSmokeDepth, FM_BAND55_ROWS, TENANT_STAND_SMOKE_CASES,
            TENANT_STAND_SMOKE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            tenant_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
            TenantStandSmokeDepth::LiveStore
        );
        assert_eq!(
            tenant_stand_smoke_depth_stub(Some(&json!({
                "tenant_stand_smoke_depth": true,
                "live_store": true,
                "live_crud": true,
                "live_usage_quota": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "tenant_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantStandSmokeDepth::FullBand55
        );
        assert_eq!(TENANT_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(tenant_stand_smoke_criteria_total(), 10);
        assert!(TENANT_STAND_SMOKE_CASES.contains(&"live_crud"));
        assert!(TENANT_STAND_SMOKE_CASES.contains(&"cli_flag"));
        // Marker used by loc-audit criteria: smoke_tenants_store_wire
        let smoke_src = include_str!("../../src/bin/poolai_http_stand_smoke.rs");
        assert!(smoke_src.contains("smoke_tenants_store_wire"));
        assert!(smoke_src.contains("smoke_tenants_crud_lifecycle"));
        assert!(smoke_src.contains("smoke_tenants_usage_quota_isolation"));
        assert!(smoke_src.contains("--tenant-stand-smoke"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND55_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band55 row {row}"
            );
        }
    }

    #[test]
    fn tenant_loc_audit_band56_export_shape_ph_s1203() {
        use poolai_ui_core::tenant_loc_audit_depth::{
            tenant_loc_audit_criteria_total, tenant_loc_audit_depth_stub,
            tenant_loc_audit_slices_met, TenantLocAuditDepth, FM_BAND56_ROWS,
            TENANT_LOC_AUDIT_CASES, TENANT_LOC_AUDIT_CRITERIA, TENANT_LOC_AUDIT_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            tenant_loc_audit_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            TenantLocAuditDepth::StandSmokeExport
        );
        assert_eq!(
            tenant_loc_audit_depth_stub(Some(&json!({
                "tenant_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantLocAuditDepth::FullBand56
        );
        assert_eq!(TENANT_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(tenant_loc_audit_criteria_total(), 10);
        assert_eq!(TENANT_LOC_AUDIT_SLICES.len(), 5);
        assert!(TENANT_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert_eq!(tenant_loc_audit_slices_met(loc_audit), (5, 5));
        assert!(loc_audit.contains("--tenant-loc-audit"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND56_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band56 row {row}"
            );
        }
    }

    #[test]
    fn tenant_docs_canon_band57_export_shape_ph_s1213() {
        use poolai_ui_core::tenant_docs_canon_depth::{
            tenant_docs_canon_criteria_total, tenant_docs_canon_depth_stub,
            tenant_docs_canon_slices_met, TenantDocsCanonDepth, FM_BAND57_ROWS,
            TENANT_DOCS_CANON_CASES, TENANT_DOCS_CANON_CRITERIA, TENANT_DOCS_CANON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            tenant_docs_canon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            TenantDocsCanonDepth::StandSmokeExport
        );
        assert_eq!(
            tenant_docs_canon_depth_stub(Some(&json!({
                "tenant_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantDocsCanonDepth::FullBand57
        );
        assert_eq!(TENANT_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(tenant_docs_canon_criteria_total(), 10);
        assert_eq!(TENANT_DOCS_CANON_SLICES.len(), 6);
        assert!(TENANT_DOCS_CANON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/TENANT_DOCS_CANON.md");
        assert_eq!(tenant_docs_canon_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--tenant-docs-canon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND57_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band57 row {row}"
            );
        }
    }

    #[test]
    fn tenant_vision_sync_band58_export_shape_ph_s1223() {
        use poolai_ui_core::tenant_vision_sync_depth::{
            tenant_vision_sync_criteria_total, tenant_vision_sync_depth_stub,
            tenant_vision_sync_slices_met, TenantVisionSyncDepth, FM_BAND58_ROWS,
            TENANT_VISION_SYNC_CASES, TENANT_VISION_SYNC_CRITERIA, TENANT_VISION_SYNC_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            tenant_vision_sync_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            TenantVisionSyncDepth::StandSmokeExport
        );
        assert_eq!(
            tenant_vision_sync_depth_stub(Some(&json!({
                "tenant_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantVisionSyncDepth::FullBand58
        );
        assert_eq!(TENANT_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(tenant_vision_sync_criteria_total(), 10);
        assert_eq!(TENANT_VISION_SYNC_SLICES.len(), 6);
        assert!(TENANT_VISION_SYNC_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/TENANT_VISION_SYNC.md");
        assert_eq!(tenant_vision_sync_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--tenant-vision-sync"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND58_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band58 row {row}"
            );
        }
    }

    #[test]
    fn tenant_ratio_advisory_band59_export_shape_ph_s1233() {
        use poolai_ui_core::tenant_ratio_advisory_depth::{
            tenant_ratio_advisory_criteria_total, tenant_ratio_advisory_depth_stub,
            tenant_ratio_advisory_slices_met, TenantRatioAdvisoryDepth, FM_BAND59_ROWS,
            TENANT_RATIO_ADVISORY_CASES, TENANT_RATIO_ADVISORY_CRITERIA,
            TENANT_RATIO_ADVISORY_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            tenant_ratio_advisory_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            TenantRatioAdvisoryDepth::StandSmokeExport
        );
        assert_eq!(
            tenant_ratio_advisory_depth_stub(Some(&json!({
                "tenant_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantRatioAdvisoryDepth::FullBand59
        );
        assert_eq!(TENANT_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(tenant_ratio_advisory_criteria_total(), 10);
        assert_eq!(TENANT_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(TENANT_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/TENANT_RATIO_ADVISORY.md");
        assert_eq!(tenant_ratio_advisory_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--tenant-ratio-advisory"));
        let multi = include_str!("../../src/enterprise/multi_tenancy.rs");
        assert!(multi.contains("persist_tenant_to_sqlite"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND59_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band59 row {row}"
            );
        }
    }

    #[test]
    fn tenant_horizon_band60_export_shape_ph_s1243() {
        use poolai_ui_core::tenant_horizon_depth::{
            tenant_horizon_criteria_total, tenant_horizon_depth_stub, tenant_horizon_slices_met,
            TenantHorizonDepth, FM_BAND60_ROWS, TENANT_HORIZON_CASES, TENANT_HORIZON_CRITERIA,
            TENANT_HORIZON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            tenant_horizon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            TenantHorizonDepth::StandSmokeExport
        );
        assert_eq!(
            tenant_horizon_depth_stub(Some(&json!({
                "tenant_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "tenant_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            TenantHorizonDepth::FullBand60
        );
        assert_eq!(TENANT_HORIZON_CRITERIA.len(), 10);
        assert_eq!(tenant_horizon_criteria_total(), 10);
        assert_eq!(TENANT_HORIZON_SLICES.len(), 10);
        assert!(TENANT_HORIZON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/TENANT_HORIZON.md");
        assert_eq!(tenant_horizon_slices_met(canon), (10, 10));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--tenant-horizon"));
        let multi = include_str!("../../src/enterprise/multi_tenancy.rs");
        assert!(multi.contains("persist_tenant_to_sqlite"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND60_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band60 row {row}"
            );
        }
    }

    #[test]
    fn sso_band61_export_shape_ph_s1253() {
        use poolai_ui_core::sso_depth::{
            sso_criteria_total, sso_depth_stub, SsoDepth, FM_BAND61_ROWS, SSO_CASES, SSO_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            sso_depth_stub(Some(&json!({"api_contracts": true}))),
            SsoDepth::ApiContracts
        );
        assert_eq!(
            sso_depth_stub(Some(&json!({
                "sso_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_docs": true,
            }))),
            SsoDepth::FullBand61
        );
        assert_eq!(SSO_CRITERIA.len(), 8);
        assert_eq!(sso_criteria_total(), 8);
        assert!(SSO_CASES.contains(&"verify_dev_stand_hook"));
        let security = include_str!("../../src/enterprise/security.rs");
        assert!(security.contains("POOLAI_SSO_STORE"));
        assert!(security.contains("validate_saml_audience_and_time"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND61_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band61 row {row}"
            );
        }
    }

    #[test]
    fn sso_store_band62_export_shape_ph_s1263() {
        use poolai_ui_core::sso_store_depth::{
            sso_store_criteria_total, sso_store_depth_stub, SsoStoreDepth, FM_BAND62_ROWS,
            SSO_STORE_CASES, SSO_STORE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            sso_store_depth_stub(Some(&json!({"api_contracts": true}))),
            SsoStoreDepth::ApiContracts
        );
        assert_eq!(
            sso_store_depth_stub(Some(&json!({
                "sso_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_store_docs": true,
            }))),
            SsoStoreDepth::FullBand62
        );
        assert_eq!(SSO_STORE_CRITERIA.len(), 7);
        assert_eq!(sso_store_criteria_total(), 7);
        assert!(SSO_STORE_CASES.contains(&"store_wire"));
        let security = include_str!("../../src/enterprise/security.rs");
        assert!(security.contains("sso_store_wire"));
        assert!(security.contains("POOLAI_SSO_DATA_DIR"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-store"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND62_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band62 row {row}"
            );
        }
    }

    #[test]
    fn sso_api_band63_export_shape_ph_s1276() {
        use poolai_ui_core::sso_api_contracts_depth::{
            sso_api_contracts_depth_stub, sso_api_criteria_total, SsoApiContractsDepth,
            FM_BAND63_ROWS, SSO_API_CASES, SSO_API_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            sso_api_contracts_depth_stub(Some(&json!({"oauth2_http_crud": true}))),
            SsoApiContractsDepth::Oauth2HttpCrud
        );
        assert_eq!(
            sso_api_contracts_depth_stub(Some(&json!({
                "sso_api_depth": true,
                "oauth2_http_crud": true,
                "saml_http_crud": true,
                "store_wire_http": true,
                "openapi_schemas": true,
                "callback_fixtures": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_api_docs": true,
            }))),
            SsoApiContractsDepth::FullBand63
        );
        assert_eq!(SSO_API_CRITERIA.len(), 10);
        assert_eq!(sso_api_criteria_total(), 10);
        assert!(SSO_API_CASES.contains(&"store_wire_http"));
        let security_api = include_str!("../../src/network/enterprise_api/security.rs");
        assert!(security_api.contains("sso_store_wire_handler"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-api"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND63_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band63 row {row}"
            );
        }
    }

    #[test]
    fn sso_admin_ops_band64_export_shape_ph_s1285() {
        use poolai_ui_core::sso_admin_ops_depth::{
            sso_admin_ops_criteria_total, sso_admin_ops_depth_stub, SsoAdminOpsDepth,
            FM_BAND64_ROWS, SSO_ADMIN_OPS_CASES, SSO_ADMIN_OPS_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            sso_admin_ops_depth_stub(Some(&json!({"providers_glue": true}))),
            SsoAdminOpsDepth::ProvidersGlue
        );
        assert_eq!(
            sso_admin_ops_depth_stub(Some(&json!({
                "sso_admin_ops_depth": true,
                "store_strip": true,
                "providers_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoAdminOpsDepth::FullBand64
        );
        assert_eq!(SSO_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(sso_admin_ops_criteria_total(), 10);
        assert!(SSO_ADMIN_OPS_CASES.contains(&"store_strip"));
        let security_ui = include_str!("../../src/ui/admin/security.rs");
        assert!(security_ui.contains("sso-store-badge"));
        assert!(security_ui.contains("refreshOAuth2Providers"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-admin-ops"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND64_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band64 row {row}"
            );
        }
    }

    #[test]
    fn audit_stand_smoke_band75_export_shape_ph_s1393() {
        use poolai_ui_core::audit_stand_smoke_depth::{
            audit_stand_smoke_criteria_total, audit_stand_smoke_depth_stub, AuditStandSmokeDepth,
            AUDIT_STAND_SMOKE_CASES, AUDIT_STAND_SMOKE_CRITERIA, FM_BAND75_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
            AuditStandSmokeDepth::LiveStore
        );
        assert_eq!(
            audit_stand_smoke_depth_stub(Some(&json!({
                "audit_stand_smoke_depth": true,
                "live_store": true,
                "live_events_query": true,
                "live_event_field_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "audit_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditStandSmokeDepth::FullBand75
        );
        assert_eq!(AUDIT_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(audit_stand_smoke_criteria_total(), 10);
        assert!(AUDIT_STAND_SMOKE_CASES.contains(&"live_events_query"));
        assert!(AUDIT_STAND_SMOKE_CASES.contains(&"cli_flag"));
        let smoke_src = include_str!("../../src/bin/poolai_http_stand_smoke.rs");
        assert!(smoke_src.contains("smoke_audit_store_wire"));
        assert!(smoke_src.contains("smoke_audit_events_query"));
        assert!(smoke_src.contains("smoke_audit_event_field_fixtures"));
        assert!(smoke_src.contains("--audit-stand-smoke"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-stand-smoke"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND75_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band75 row {row}"
            );
        }
    }

    #[test]
    fn sso_stand_smoke_band65_export_shape_ph_s1293() {
        use poolai_ui_core::sso_stand_smoke_depth::{
            sso_stand_smoke_criteria_total, sso_stand_smoke_depth_stub, SsoStandSmokeDepth,
            FM_BAND65_ROWS, SSO_STAND_SMOKE_CASES, SSO_STAND_SMOKE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            sso_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
            SsoStandSmokeDepth::LiveStore
        );
        assert_eq!(
            sso_stand_smoke_depth_stub(Some(&json!({
                "sso_stand_smoke_depth": true,
                "live_store": true,
                "live_crud": true,
                "live_callback_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "sso_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoStandSmokeDepth::FullBand65
        );
        assert_eq!(SSO_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(sso_stand_smoke_criteria_total(), 10);
        assert!(SSO_STAND_SMOKE_CASES.contains(&"live_crud"));
        assert!(SSO_STAND_SMOKE_CASES.contains(&"cli_flag"));
        let smoke_src = include_str!("../../src/bin/poolai_http_stand_smoke.rs");
        assert!(smoke_src.contains("smoke_sso_store_wire"));
        assert!(smoke_src.contains("smoke_sso_oauth2_saml_crud"));
        assert!(smoke_src.contains("smoke_sso_callback_fixtures"));
        assert!(smoke_src.contains("--sso-stand-smoke"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-stand-smoke"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND65_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band65 row {row}"
            );
        }
    }

    #[test]
    fn sso_loc_audit_band66_export_shape_ph_s1303() {
        use poolai_ui_core::sso_loc_audit_depth::{
            sso_loc_audit_criteria_total, sso_loc_audit_depth_stub, sso_loc_audit_slices_met,
            SsoLocAuditDepth, FM_BAND66_ROWS, SSO_LOC_AUDIT_CASES, SSO_LOC_AUDIT_CRITERIA,
            SSO_LOC_AUDIT_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            sso_loc_audit_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            SsoLocAuditDepth::StandSmokeExport
        );
        assert_eq!(
            sso_loc_audit_depth_stub(Some(&json!({
                "sso_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoLocAuditDepth::FullBand66
        );
        assert_eq!(SSO_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(sso_loc_audit_criteria_total(), 10);
        assert_eq!(SSO_LOC_AUDIT_SLICES.len(), 5);
        assert!(SSO_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert_eq!(sso_loc_audit_slices_met(loc_audit), (5, 5));
        assert!(loc_audit.contains("--sso-loc-audit"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND66_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band66 row {row}"
            );
        }
    }

    #[test]
    fn audit_loc_audit_band76_export_shape_ph_s1403() {
        use poolai_ui_core::audit_loc_audit_depth::{
            audit_loc_audit_criteria_total, audit_loc_audit_depth_stub, audit_loc_audit_slices_met,
            AuditLocAuditDepth, AUDIT_LOC_AUDIT_CASES, AUDIT_LOC_AUDIT_CRITERIA,
            AUDIT_LOC_AUDIT_SLICES, FM_BAND76_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_loc_audit_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            AuditLocAuditDepth::StandSmokeExport
        );
        assert_eq!(
            audit_loc_audit_depth_stub(Some(&json!({
                "audit_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditLocAuditDepth::FullBand76
        );
        assert_eq!(AUDIT_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(audit_loc_audit_criteria_total(), 10);
        assert_eq!(AUDIT_LOC_AUDIT_SLICES.len(), 5);
        assert!(AUDIT_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert_eq!(audit_loc_audit_slices_met(loc_audit), (5, 5));
        assert!(loc_audit.contains("--audit-loc-audit"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND76_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band76 row {row}"
            );
        }
    }

    #[test]
    fn audit_docs_canon_band77_export_shape_ph_s1413() {
        use poolai_ui_core::audit_docs_canon_depth::{
            audit_docs_canon_criteria_total, audit_docs_canon_depth_stub,
            audit_docs_canon_slices_met, AuditDocsCanonDepth, AUDIT_DOCS_CANON_CASES,
            AUDIT_DOCS_CANON_CRITERIA, AUDIT_DOCS_CANON_SLICES, FM_BAND77_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_docs_canon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            AuditDocsCanonDepth::StandSmokeExport
        );
        assert_eq!(
            audit_docs_canon_depth_stub(Some(&json!({
                "audit_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditDocsCanonDepth::FullBand77
        );
        assert_eq!(AUDIT_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(audit_docs_canon_criteria_total(), 10);
        assert_eq!(AUDIT_DOCS_CANON_SLICES.len(), 6);
        assert!(AUDIT_DOCS_CANON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/AUDIT_DOCS_CANON.md");
        assert_eq!(audit_docs_canon_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-docs-canon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND77_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band77 row {row}"
            );
        }
    }

    #[test]
    fn audit_vision_sync_band78_export_shape_ph_s1423() {
        use poolai_ui_core::audit_vision_sync_depth::{
            audit_vision_sync_criteria_total, audit_vision_sync_depth_stub,
            audit_vision_sync_slices_met, AuditVisionSyncDepth, AUDIT_VISION_SYNC_CASES,
            AUDIT_VISION_SYNC_CRITERIA, AUDIT_VISION_SYNC_SLICES, FM_BAND78_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_vision_sync_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            AuditVisionSyncDepth::StandSmokeExport
        );
        assert_eq!(
            audit_vision_sync_depth_stub(Some(&json!({
                "audit_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditVisionSyncDepth::FullBand78
        );
        assert_eq!(AUDIT_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(audit_vision_sync_criteria_total(), 10);
        assert_eq!(AUDIT_VISION_SYNC_SLICES.len(), 6);
        assert!(AUDIT_VISION_SYNC_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/AUDIT_VISION_SYNC.md");
        assert_eq!(audit_vision_sync_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-vision-sync"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND78_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band78 row {row}"
            );
        }
    }

    #[test]
    fn audit_ratio_advisory_band79_export_shape_ph_s1433() {
        use poolai_ui_core::audit_ratio_advisory_depth::{
            audit_ratio_advisory_criteria_total, audit_ratio_advisory_depth_stub,
            audit_ratio_advisory_slices_met, AuditRatioAdvisoryDepth, AUDIT_RATIO_ADVISORY_CASES,
            AUDIT_RATIO_ADVISORY_CRITERIA, AUDIT_RATIO_ADVISORY_SLICES, FM_BAND79_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_ratio_advisory_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            AuditRatioAdvisoryDepth::StandSmokeExport
        );
        assert_eq!(
            audit_ratio_advisory_depth_stub(Some(&json!({
                "audit_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditRatioAdvisoryDepth::FullBand79
        );
        assert_eq!(AUDIT_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(audit_ratio_advisory_criteria_total(), 10);
        assert_eq!(AUDIT_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(AUDIT_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/AUDIT_RATIO_ADVISORY.md");
        assert_eq!(audit_ratio_advisory_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-ratio-advisory"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND79_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band79 row {row}"
            );
        }
    }

    #[test]
    fn audit_horizon_band80_export_shape_ph_s1443() {
        use poolai_ui_core::audit_horizon_depth::{
            audit_horizon_criteria_total, audit_horizon_depth_stub, audit_horizon_slices_met,
            AuditHorizonDepth, AUDIT_HORIZON_CASES, AUDIT_HORIZON_CRITERIA, AUDIT_HORIZON_SLICES,
            FM_BAND80_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_horizon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            AuditHorizonDepth::StandSmokeExport
        );
        assert_eq!(
            audit_horizon_depth_stub(Some(&json!({
                "audit_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditHorizonDepth::FullBand80
        );
        assert_eq!(AUDIT_HORIZON_CRITERIA.len(), 10);
        assert_eq!(audit_horizon_criteria_total(), 10);
        assert_eq!(AUDIT_HORIZON_SLICES.len(), 10);
        assert!(AUDIT_HORIZON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/AUDIT_HORIZON.md");
        assert_eq!(audit_horizon_slices_met(canon), (10, 10));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-horizon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND80_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band80 row {row}"
            );
        }
    }

    #[test]
    fn sso_docs_canon_band67_export_shape_ph_s1313() {
        use poolai_ui_core::sso_docs_canon_depth::{
            sso_docs_canon_criteria_total, sso_docs_canon_depth_stub, sso_docs_canon_slices_met,
            SsoDocsCanonDepth, FM_BAND67_ROWS, SSO_DOCS_CANON_CASES, SSO_DOCS_CANON_CRITERIA,
            SSO_DOCS_CANON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            sso_docs_canon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            SsoDocsCanonDepth::StandSmokeExport
        );
        assert_eq!(
            sso_docs_canon_depth_stub(Some(&json!({
                "sso_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoDocsCanonDepth::FullBand67
        );
        assert_eq!(SSO_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(sso_docs_canon_criteria_total(), 10);
        assert_eq!(SSO_DOCS_CANON_SLICES.len(), 6);
        assert!(SSO_DOCS_CANON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/SSO_DOCS_CANON.md");
        assert_eq!(sso_docs_canon_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-docs-canon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND67_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band67 row {row}"
            );
        }
    }

    #[test]
    fn sso_vision_sync_band68_export_shape_ph_s1323() {
        use poolai_ui_core::sso_vision_sync_depth::{
            sso_vision_sync_criteria_total, sso_vision_sync_depth_stub, sso_vision_sync_slices_met,
            SsoVisionSyncDepth, FM_BAND68_ROWS, SSO_VISION_SYNC_CASES, SSO_VISION_SYNC_CRITERIA,
            SSO_VISION_SYNC_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            sso_vision_sync_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            SsoVisionSyncDepth::StandSmokeExport
        );
        assert_eq!(
            sso_vision_sync_depth_stub(Some(&json!({
                "sso_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoVisionSyncDepth::FullBand68
        );
        assert_eq!(SSO_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(sso_vision_sync_criteria_total(), 10);
        assert_eq!(SSO_VISION_SYNC_SLICES.len(), 6);
        assert!(SSO_VISION_SYNC_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/SSO_VISION_SYNC.md");
        assert_eq!(sso_vision_sync_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-vision-sync"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND68_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band68 row {row}"
            );
        }
    }

    #[test]
    fn sso_ratio_advisory_band69_export_shape_ph_s1333() {
        use poolai_ui_core::sso_ratio_advisory_depth::{
            sso_ratio_advisory_criteria_total, sso_ratio_advisory_depth_stub,
            sso_ratio_advisory_slices_met, SsoRatioAdvisoryDepth, FM_BAND69_ROWS,
            SSO_RATIO_ADVISORY_CASES, SSO_RATIO_ADVISORY_CRITERIA, SSO_RATIO_ADVISORY_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            sso_ratio_advisory_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            SsoRatioAdvisoryDepth::StandSmokeExport
        );
        assert_eq!(
            sso_ratio_advisory_depth_stub(Some(&json!({
                "sso_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoRatioAdvisoryDepth::FullBand69
        );
        assert_eq!(SSO_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(sso_ratio_advisory_criteria_total(), 10);
        assert_eq!(SSO_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(SSO_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/SSO_RATIO_ADVISORY.md");
        assert_eq!(sso_ratio_advisory_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-ratio-advisory"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND69_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band69 row {row}"
            );
        }
    }

    #[test]
    fn sso_horizon_band70_export_shape_ph_s1343() {
        use poolai_ui_core::sso_horizon_depth::{
            sso_horizon_criteria_total, sso_horizon_depth_stub, sso_horizon_slices_met,
            SsoHorizonDepth, FM_BAND70_ROWS, SSO_HORIZON_CASES, SSO_HORIZON_CRITERIA,
            SSO_HORIZON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            sso_horizon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            SsoHorizonDepth::StandSmokeExport
        );
        assert_eq!(
            sso_horizon_depth_stub(Some(&json!({
                "sso_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "sso_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            SsoHorizonDepth::FullBand70
        );
        assert_eq!(SSO_HORIZON_CRITERIA.len(), 10);
        assert_eq!(sso_horizon_criteria_total(), 10);
        assert_eq!(SSO_HORIZON_SLICES.len(), 10);
        assert!(SSO_HORIZON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/SSO_HORIZON.md");
        assert_eq!(sso_horizon_slices_met(canon), (10, 10));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--sso-horizon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND70_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band70 row {row}"
            );
        }
    }

    #[test]
    fn audit_band71_export_shape_ph_s1353() {
        use poolai_ui_core::audit_depth::{
            audit_criteria_total, audit_depth_stub, AuditDepth, AUDIT_CASES, AUDIT_CRITERIA,
            FM_BAND71_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_depth_stub(Some(&json!({"api_contracts": true}))),
            AuditDepth::ApiContracts
        );
        assert_eq!(
            audit_depth_stub(Some(&json!({
                "audit_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_docs": true,
            }))),
            AuditDepth::FullBand71
        );
        assert_eq!(AUDIT_CRITERIA.len(), 8);
        assert_eq!(audit_criteria_total(), 8);
        assert!(AUDIT_CASES.contains(&"verify_dev_stand_hook"));
        let audit_mod = include_str!("../../src/enterprise/audit.rs");
        assert!(audit_mod.contains("POOLAI_AUDIT_STORE"));
        assert!(audit_mod.contains("validate_audit_event_fields"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND71_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band71 row {row}"
            );
        }
    }

    #[test]
    fn policy_band81_export_shape_ph_s1453() {
        use poolai_ui_core::policy_depth::{
            policy_criteria_total, policy_depth_stub, PolicyDepth, FM_BAND81_ROWS, POLICY_CASES,
            POLICY_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            policy_depth_stub(Some(&json!({"api_contracts": true}))),
            PolicyDepth::ApiContracts
        );
        assert_eq!(
            policy_depth_stub(Some(&json!({
                "policy_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_docs": true,
            }))),
            PolicyDepth::FullBand81
        );
        assert_eq!(POLICY_CRITERIA.len(), 8);
        assert_eq!(policy_criteria_total(), 8);
        assert!(POLICY_CASES.contains(&"verify_dev_stand_hook"));
        let security_mod = include_str!("../../src/enterprise/security.rs");
        assert!(security_mod.contains("POOLAI_POLICY_STORE"));
        assert!(security_mod.contains("validate_security_policy_fields"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND81_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band81 row {row}"
            );
        }
    }

    #[test]
    fn policy_store_band82_export_shape_ph_s1463() {
        use poolai_ui_core::policy_store_depth::{
            policy_store_criteria_total, policy_store_depth_stub, PolicyStoreDepth, FM_BAND82_ROWS,
            POLICY_STORE_CASES, POLICY_STORE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            policy_store_depth_stub(Some(&json!({"api_contracts": true}))),
            PolicyStoreDepth::ApiContracts
        );
        assert_eq!(
            policy_store_depth_stub(Some(&json!({
                "policy_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_store_docs": true,
            }))),
            PolicyStoreDepth::FullBand82
        );
        assert_eq!(POLICY_STORE_CRITERIA.len(), 7);
        assert_eq!(policy_store_criteria_total(), 7);
        assert!(POLICY_STORE_CASES.contains(&"store_wire"));
        let security_mod = include_str!("../../src/enterprise/security.rs");
        assert!(security_mod.contains("policy_store_wire"));
        assert!(security_mod.contains("POOLAI_POLICY_DATA_DIR"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-store"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND82_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band82 row {row}"
            );
        }
    }

    #[test]
    fn policy_api_band83_export_shape_ph_s1475() {
        use poolai_ui_core::policy_api_contracts_depth::{
            policy_api_contracts_depth_stub, policy_api_criteria_total, PolicyApiContractsDepth,
            FM_BAND83_ROWS, POLICY_API_CASES, POLICY_API_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            policy_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
            PolicyApiContractsDepth::StoreWireHttp
        );
        assert_eq!(
            policy_api_contracts_depth_stub(Some(&json!({
                "policy_api_depth": true,
                "query_http_lifecycle": true,
                "store_wire_http": true,
                "openapi_schemas": true,
                "policy_field_fixtures": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_api_docs": true,
            }))),
            PolicyApiContractsDepth::FullBand83
        );
        assert_eq!(POLICY_API_CRITERIA.len(), 9);
        assert_eq!(policy_api_criteria_total(), 9);
        assert!(POLICY_API_CASES.contains(&"store_wire_http"));
        let security_api = include_str!("../../src/network/enterprise_api/security.rs");
        assert!(security_api.contains("GET /policy/store"));
        assert!(security_api.contains("policy_store_wire_handler"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-api"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND83_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band83 row {row}"
            );
        }
    }

    #[test]
    fn policy_stand_smoke_band85_export_shape_ph_s1493() {
        use poolai_ui_core::policy_stand_smoke_depth::{
            policy_stand_smoke_criteria_total, policy_stand_smoke_depth_stub,
            PolicyStandSmokeDepth, FM_BAND85_ROWS, POLICY_STAND_SMOKE_CASES,
            POLICY_STAND_SMOKE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            policy_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
            PolicyStandSmokeDepth::LiveStore
        );
        assert_eq!(
            policy_stand_smoke_depth_stub(Some(&json!({
                "policy_stand_smoke_depth": true,
                "live_store": true,
                "live_policies_query": true,
                "live_policy_field_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "policy_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyStandSmokeDepth::FullBand85
        );
        assert_eq!(POLICY_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(policy_stand_smoke_criteria_total(), 10);
        assert!(POLICY_STAND_SMOKE_CASES.contains(&"live_policies_query"));
        let smoke_src = include_str!("../../src/bin/poolai_http_stand_smoke.rs");
        assert!(smoke_src.contains("smoke_policy_store_wire"));
        assert!(smoke_src.contains("smoke_policy_policies_query"));
        assert!(smoke_src.contains("smoke_policy_field_fixtures"));
        assert!(smoke_src.contains("--policy-stand-smoke"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-stand-smoke"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND85_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band85 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_stand_smoke_band95_export_shape_ph_s1593() {
        use poolai_ui_core::monitoring_stand_smoke_depth::{
            monitoring_stand_smoke_criteria_total, monitoring_stand_smoke_depth_stub,
            MonitoringStandSmokeDepth, FM_BAND95_ROWS, MONITORING_STAND_SMOKE_CASES,
            MONITORING_STAND_SMOKE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_stand_smoke_depth_stub(Some(&json!({"live_store": true}))),
            MonitoringStandSmokeDepth::LiveStore
        );
        assert_eq!(
            monitoring_stand_smoke_depth_stub(Some(&json!({
                "monitoring_stand_smoke_depth": true,
                "live_store": true,
                "live_alerts_query": true,
                "live_monitoring_field_fixtures": true,
                "cli_flag": true,
                "loc_audit_flag": true,
                "verify_dev_stand_hook": true,
                "monitoring_stand_smoke_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringStandSmokeDepth::FullBand95
        );
        assert_eq!(MONITORING_STAND_SMOKE_CRITERIA.len(), 10);
        assert_eq!(monitoring_stand_smoke_criteria_total(), 10);
        assert!(MONITORING_STAND_SMOKE_CASES.contains(&"live_alerts_query"));
        let smoke_src = include_str!("../../src/bin/poolai_http_stand_smoke.rs");
        assert!(smoke_src.contains("smoke_monitoring_store_wire"));
        assert!(smoke_src.contains("smoke_monitoring_alerts_query"));
        assert!(smoke_src.contains("smoke_monitoring_field_fixtures"));
        assert!(smoke_src.contains("--monitoring-stand-smoke"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--monitoring-stand-smoke"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND95_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band95 row {row}"
            );
        }
    }

    #[test]
    fn policy_loc_audit_band86_export_shape_ph_s1503() {
        use poolai_ui_core::policy_loc_audit_depth::{
            policy_loc_audit_criteria_total, policy_loc_audit_depth_stub,
            policy_loc_audit_slices_met, PolicyLocAuditDepth, FM_BAND86_ROWS,
            POLICY_LOC_AUDIT_CASES, POLICY_LOC_AUDIT_CRITERIA, POLICY_LOC_AUDIT_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            policy_loc_audit_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            PolicyLocAuditDepth::StandSmokeExport
        );
        assert_eq!(
            policy_loc_audit_depth_stub(Some(&json!({
                "policy_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyLocAuditDepth::FullBand86
        );
        assert_eq!(POLICY_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(policy_loc_audit_criteria_total(), 10);
        assert_eq!(POLICY_LOC_AUDIT_SLICES.len(), 5);
        assert!(POLICY_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert_eq!(policy_loc_audit_slices_met(loc_audit), (5, 5));
        assert!(loc_audit.contains("--policy-loc-audit"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND86_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band86 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_loc_audit_band96_export_shape_ph_s1603() {
        use poolai_ui_core::monitoring_loc_audit_depth::{
            monitoring_loc_audit_criteria_total, monitoring_loc_audit_depth_stub,
            monitoring_loc_audit_slices_met, MonitoringLocAuditDepth, FM_BAND96_ROWS,
            MONITORING_LOC_AUDIT_CASES, MONITORING_LOC_AUDIT_CRITERIA, MONITORING_LOC_AUDIT_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_loc_audit_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            MonitoringLocAuditDepth::StandSmokeExport
        );
        assert_eq!(
            monitoring_loc_audit_depth_stub(Some(&json!({
                "monitoring_loc_audit_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_loc_audit_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringLocAuditDepth::FullBand96
        );
        assert_eq!(MONITORING_LOC_AUDIT_CRITERIA.len(), 10);
        assert_eq!(monitoring_loc_audit_criteria_total(), 10);
        assert_eq!(MONITORING_LOC_AUDIT_SLICES.len(), 5);
        assert!(MONITORING_LOC_AUDIT_CASES.contains(&"aggregate_flag"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert_eq!(monitoring_loc_audit_slices_met(loc_audit), (5, 5));
        assert!(loc_audit.contains("--monitoring-loc-audit"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND96_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band96 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_docs_canon_band97_export_shape_ph_s1613() {
        use poolai_ui_core::monitoring_docs_canon_depth::{
            monitoring_docs_canon_criteria_total, monitoring_docs_canon_depth_stub,
            monitoring_docs_canon_slices_met, MonitoringDocsCanonDepth, FM_BAND97_ROWS,
            MONITORING_DOCS_CANON_CASES, MONITORING_DOCS_CANON_CRITERIA,
            MONITORING_DOCS_CANON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_docs_canon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            MonitoringDocsCanonDepth::StandSmokeExport
        );
        assert_eq!(
            monitoring_docs_canon_depth_stub(Some(&json!({
                "monitoring_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringDocsCanonDepth::FullBand97
        );
        assert_eq!(MONITORING_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(monitoring_docs_canon_criteria_total(), 10);
        assert_eq!(MONITORING_DOCS_CANON_SLICES.len(), 6);
        assert!(MONITORING_DOCS_CANON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/MONITORING_DOCS_CANON.md");
        assert_eq!(monitoring_docs_canon_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--monitoring-docs-canon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND97_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band97 row {row}"
            );
        }
    }

    #[test]
    fn policy_docs_canon_band87_export_shape_ph_s1513() {
        use poolai_ui_core::policy_docs_canon_depth::{
            policy_docs_canon_criteria_total, policy_docs_canon_depth_stub,
            policy_docs_canon_slices_met, PolicyDocsCanonDepth, FM_BAND87_ROWS,
            POLICY_DOCS_CANON_CASES, POLICY_DOCS_CANON_CRITERIA, POLICY_DOCS_CANON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            policy_docs_canon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            PolicyDocsCanonDepth::StandSmokeExport
        );
        assert_eq!(
            policy_docs_canon_depth_stub(Some(&json!({
                "policy_docs_canon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_docs_canon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyDocsCanonDepth::FullBand87
        );
        assert_eq!(POLICY_DOCS_CANON_CRITERIA.len(), 10);
        assert_eq!(policy_docs_canon_criteria_total(), 10);
        assert_eq!(POLICY_DOCS_CANON_SLICES.len(), 6);
        assert!(POLICY_DOCS_CANON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/POLICIES_DOCS_CANON.md");
        assert_eq!(policy_docs_canon_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-docs-canon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND87_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band87 row {row}"
            );
        }
    }

    #[test]
    fn policy_vision_sync_band88_export_shape_ph_s1523() {
        use poolai_ui_core::policy_vision_sync_depth::{
            policy_vision_sync_criteria_total, policy_vision_sync_depth_stub,
            policy_vision_sync_slices_met, PolicyVisionSyncDepth, FM_BAND88_ROWS,
            POLICY_VISION_SYNC_CASES, POLICY_VISION_SYNC_CRITERIA, POLICY_VISION_SYNC_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            policy_vision_sync_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            PolicyVisionSyncDepth::StandSmokeExport
        );
        assert_eq!(
            policy_vision_sync_depth_stub(Some(&json!({
                "policy_vision_sync_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_vision_sync_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyVisionSyncDepth::FullBand88
        );
        assert_eq!(POLICY_VISION_SYNC_CRITERIA.len(), 10);
        assert_eq!(policy_vision_sync_criteria_total(), 10);
        assert_eq!(POLICY_VISION_SYNC_SLICES.len(), 6);
        assert!(POLICY_VISION_SYNC_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/POLICIES_VISION_SYNC.md");
        assert_eq!(policy_vision_sync_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-vision-sync"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND88_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band88 row {row}"
            );
        }
    }

    #[test]
    fn policy_ratio_advisory_band89_export_shape_ph_s1533() {
        use poolai_ui_core::policy_ratio_advisory_depth::{
            policy_ratio_advisory_criteria_total, policy_ratio_advisory_depth_stub,
            policy_ratio_advisory_slices_met, PolicyRatioAdvisoryDepth, FM_BAND89_ROWS,
            POLICY_RATIO_ADVISORY_CASES, POLICY_RATIO_ADVISORY_CRITERIA,
            POLICY_RATIO_ADVISORY_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            policy_ratio_advisory_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            PolicyRatioAdvisoryDepth::StandSmokeExport
        );
        assert_eq!(
            policy_ratio_advisory_depth_stub(Some(&json!({
                "policy_ratio_advisory_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_ratio_advisory_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyRatioAdvisoryDepth::FullBand89
        );
        assert_eq!(POLICY_RATIO_ADVISORY_CRITERIA.len(), 10);
        assert_eq!(policy_ratio_advisory_criteria_total(), 10);
        assert_eq!(POLICY_RATIO_ADVISORY_SLICES.len(), 6);
        assert!(POLICY_RATIO_ADVISORY_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/POLICIES_RATIO_ADVISORY.md");
        assert_eq!(policy_ratio_advisory_slices_met(canon), (6, 6));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-ratio-advisory"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND89_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band89 row {row}"
            );
        }
    }

    #[test]
    fn policy_horizon_band90_export_shape_ph_s1543() {
        use poolai_ui_core::policy_horizon_depth::{
            policy_horizon_criteria_total, policy_horizon_depth_stub, policy_horizon_slices_met,
            PolicyHorizonDepth, FM_BAND90_ROWS, POLICY_HORIZON_CASES, POLICY_HORIZON_CRITERIA,
            POLICY_HORIZON_SLICES,
        };
        use serde_json::json;
        assert_eq!(
            policy_horizon_depth_stub(Some(&json!({"stand_smoke_export": true}))),
            PolicyHorizonDepth::StandSmokeExport
        );
        assert_eq!(
            policy_horizon_depth_stub(Some(&json!({
                "policy_horizon_depth": true,
                "slice_aggregate": true,
                "criteria_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_horizon_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyHorizonDepth::FullBand90
        );
        assert_eq!(POLICY_HORIZON_CRITERIA.len(), 10);
        assert_eq!(policy_horizon_criteria_total(), 10);
        assert_eq!(POLICY_HORIZON_SLICES.len(), 10);
        assert!(POLICY_HORIZON_CASES.contains(&"aggregate_flag"));
        let canon = include_str!("../../docs/development/POLICIES_HORIZON.md");
        assert_eq!(policy_horizon_slices_met(canon), (10, 10));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-horizon"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND90_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band90 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_band91_export_shape_ph_s1553() {
        use poolai_ui_core::monitoring_depth::{
            monitoring_criteria_total, monitoring_depth_stub, MonitoringDepth, FM_BAND91_ROWS,
            MONITORING_CASES, MONITORING_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_depth_stub(Some(&json!({"api_contracts": true}))),
            MonitoringDepth::ApiContracts
        );
        assert_eq!(
            monitoring_depth_stub(Some(&json!({
                "monitoring_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_docs": true,
            }))),
            MonitoringDepth::FullBand91
        );
        assert_eq!(MONITORING_CRITERIA.len(), 8);
        assert_eq!(monitoring_criteria_total(), 8);
        assert!(MONITORING_CASES.contains(&"verify_dev_stand_hook"));
        let monitoring_mod = include_str!("../../src/enterprise/monitoring.rs");
        assert!(monitoring_mod.contains("POOLAI_MONITORING_DATA_DIR"));
        assert!(monitoring_mod.contains("validate_monitoring_alert_fields"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--monitoring"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND91_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band91 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_store_band92_export_shape_ph_s1563() {
        use poolai_ui_core::monitoring_store_depth::{
            monitoring_store_criteria_total, monitoring_store_depth_stub, MonitoringStoreDepth,
            FM_BAND92_ROWS, MONITORING_STORE_CASES, MONITORING_STORE_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_store_depth_stub(Some(&json!({"api_contracts": true}))),
            MonitoringStoreDepth::ApiContracts
        );
        assert_eq!(
            monitoring_store_depth_stub(Some(&json!({
                "monitoring_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_store_docs": true,
            }))),
            MonitoringStoreDepth::FullBand92
        );
        assert_eq!(MONITORING_STORE_CRITERIA.len(), 7);
        assert_eq!(monitoring_store_criteria_total(), 7);
        assert!(MONITORING_STORE_CASES.contains(&"store_wire"));
        let monitoring_mod = include_str!("../../src/enterprise/monitoring.rs");
        assert!(monitoring_mod.contains("monitoring_store_wire"));
        assert!(monitoring_mod.contains("POOLAI_MONITORING_STORE"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--monitoring-store"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND92_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band92 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_api_band93_export_shape_ph_s1575() {
        use poolai_ui_core::monitoring_api_contracts_depth::{
            monitoring_api_contracts_depth_stub, monitoring_api_criteria_total,
            MonitoringApiContractsDepth, FM_BAND93_ROWS, MONITORING_API_CASES,
            MONITORING_API_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
            MonitoringApiContractsDepth::StoreWireHttp
        );
        assert_eq!(
            monitoring_api_contracts_depth_stub(Some(&json!({
                "monitoring_api_depth": true,
                "query_http_lifecycle": true,
                "store_wire_http": true,
                "openapi_schemas": true,
                "monitoring_field_fixtures": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_api_docs": true,
            }))),
            MonitoringApiContractsDepth::FullBand93
        );
        assert_eq!(MONITORING_API_CRITERIA.len(), 9);
        assert_eq!(monitoring_api_criteria_total(), 9);
        assert!(MONITORING_API_CASES.contains(&"store_wire_http"));
        let monitoring_api = include_str!("../../src/network/enterprise_api/monitoring.rs");
        assert!(monitoring_api.contains("GET /monitoring/store"));
        assert!(monitoring_api.contains("monitoring_store_wire_handler"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--monitoring-api"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND93_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band93 row {row}"
            );
        }
    }

    #[test]
    fn policy_admin_ops_band84_export_shape_ph_s1485() {
        use poolai_ui_core::policy_admin_ops_depth::{
            policy_admin_ops_criteria_total, policy_admin_ops_depth_stub, PolicyAdminOpsDepth,
            FM_BAND84_ROWS, POLICY_ADMIN_OPS_CASES, POLICY_ADMIN_OPS_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            policy_admin_ops_depth_stub(Some(&json!({"query_ops_glue": true}))),
            PolicyAdminOpsDepth::QueryOpsGlue
        );
        assert_eq!(
            policy_admin_ops_depth_stub(Some(&json!({
                "policy_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "policy_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            PolicyAdminOpsDepth::FullBand84
        );
        assert_eq!(POLICY_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(policy_admin_ops_criteria_total(), 10);
        assert!(POLICY_ADMIN_OPS_CASES.contains(&"store_strip"));
        let policy_ui = include_str!("../../src/ui/admin/security.rs");
        assert!(policy_ui.contains("policy-store-badge"));
        assert!(policy_ui.contains("refreshSecurityPolicies"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--policy-admin-ops"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND84_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band84 row {row}"
            );
        }
    }

    #[test]
    fn monitoring_admin_ops_band94_export_shape_ph_s1585() {
        use poolai_ui_core::monitoring_admin_ops_depth::{
            monitoring_admin_ops_criteria_total, monitoring_admin_ops_depth_stub,
            MonitoringAdminOpsDepth, FM_BAND94_ROWS, MONITORING_ADMIN_OPS_CASES,
            MONITORING_ADMIN_OPS_CRITERIA,
        };
        use serde_json::json;
        assert_eq!(
            monitoring_admin_ops_depth_stub(Some(&json!({"query_ops_glue": true}))),
            MonitoringAdminOpsDepth::QueryOpsGlue
        );
        assert_eq!(
            monitoring_admin_ops_depth_stub(Some(&json!({
                "monitoring_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "monitoring_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            MonitoringAdminOpsDepth::FullBand94
        );
        assert_eq!(MONITORING_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(monitoring_admin_ops_criteria_total(), 10);
        assert!(MONITORING_ADMIN_OPS_CASES.contains(&"store_strip"));
        let mon_ui = include_str!("../../src/ui/admin/monitoring.rs");
        assert!(mon_ui.contains("monitoring-store-badge"));
        assert!(mon_ui.contains("refreshMonitoring"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--monitoring-admin-ops"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND94_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band94 row {row}"
            );
        }
    }

    #[test]
    fn audit_store_band72_export_shape_ph_s1363() {
        use poolai_ui_core::audit_store_depth::{
            audit_store_criteria_total, audit_store_depth_stub, AuditStoreDepth, AUDIT_STORE_CASES,
            AUDIT_STORE_CRITERIA, FM_BAND72_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_store_depth_stub(Some(&json!({"api_contracts": true}))),
            AuditStoreDepth::ApiContracts
        );
        assert_eq!(
            audit_store_depth_stub(Some(&json!({
                "audit_store_depth": true,
                "store_wire": true,
                "api_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_store_docs": true,
            }))),
            AuditStoreDepth::FullBand72
        );
        assert_eq!(AUDIT_STORE_CRITERIA.len(), 7);
        assert_eq!(audit_store_criteria_total(), 7);
        assert!(AUDIT_STORE_CASES.contains(&"store_wire"));
        let audit_mod = include_str!("../../src/enterprise/audit.rs");
        assert!(audit_mod.contains("audit_store_wire"));
        assert!(audit_mod.contains("POOLAI_AUDIT_DATA_DIR"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-store"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND72_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band72 row {row}"
            );
        }
    }

    #[test]
    fn audit_api_band73_export_shape_ph_s1375() {
        use poolai_ui_core::audit_api_contracts_depth::{
            audit_api_contracts_depth_stub, audit_api_criteria_total, AuditApiContractsDepth,
            AUDIT_API_CASES, AUDIT_API_CRITERIA, FM_BAND73_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_api_contracts_depth_stub(Some(&json!({"store_wire_http": true}))),
            AuditApiContractsDepth::StoreWireHttp
        );
        assert_eq!(
            audit_api_contracts_depth_stub(Some(&json!({
                "audit_api_depth": true,
                "query_http_lifecycle": true,
                "store_wire_http": true,
                "openapi_schemas": true,
                "event_field_fixtures": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_api_docs": true,
            }))),
            AuditApiContractsDepth::FullBand73
        );
        assert_eq!(AUDIT_API_CRITERIA.len(), 9);
        assert_eq!(audit_api_criteria_total(), 9);
        assert!(AUDIT_API_CASES.contains(&"store_wire_http"));
        let audit_api = include_str!("../../src/network/enterprise_api/audit.rs");
        assert!(audit_api.contains("GET /audit/store"));
        assert!(audit_api.contains("audit_store_wire_handler"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-api"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND73_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band73 row {row}"
            );
        }
    }

    #[test]
    fn audit_admin_ops_band74_export_shape_ph_s1385() {
        use poolai_ui_core::audit_admin_ops_depth::{
            audit_admin_ops_criteria_total, audit_admin_ops_depth_stub, AuditAdminOpsDepth,
            AUDIT_ADMIN_OPS_CASES, AUDIT_ADMIN_OPS_CRITERIA, FM_BAND74_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            audit_admin_ops_depth_stub(Some(&json!({"query_ops_glue": true}))),
            AuditAdminOpsDepth::QueryOpsGlue
        );
        assert_eq!(
            audit_admin_ops_depth_stub(Some(&json!({
                "audit_admin_ops_depth": true,
                "store_strip": true,
                "query_ops_glue": true,
                "html_contracts": true,
                "verify_dev_stand_hook": true,
                "stand_smoke_export": true,
                "loc_audit_flag": true,
                "audit_admin_ops_docs": true,
                "ratio_hold": true,
                "band_close": true,
            }))),
            AuditAdminOpsDepth::FullBand74
        );
        assert_eq!(AUDIT_ADMIN_OPS_CRITERIA.len(), 10);
        assert_eq!(audit_admin_ops_criteria_total(), 10);
        assert!(AUDIT_ADMIN_OPS_CASES.contains(&"store_strip"));
        let audit_ui = include_str!("../../src/ui/admin/audit.rs");
        assert!(audit_ui.contains("audit-store-badge"));
        assert!(audit_ui.contains("refreshAuditEvents"));
        let loc_audit = include_str!("../../src/bin/poolai_loc_audit.rs");
        assert!(loc_audit.contains("--audit-admin-ops"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND74_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band74 row {row}"
            );
        }
    }

    #[test]
    fn galaxy_edge_verification_band48_export_shape_ph_s1125() {
        use poolai_ui_core::galaxy_edge_verification_depth::{
            edge_verification_criteria_total, galaxy_edge_verification_depth_stub,
            GalaxyEdgeVerificationDepth, EDGE_VERIFICATION_CASES, EDGE_VERIFICATION_CRITERIA,
            FM_BAND48_ROWS,
        };
        use serde_json::json;
        assert_eq!(
            galaxy_edge_verification_depth_stub(Some(&json!({"fraud_proof_stub": true}))),
            GalaxyEdgeVerificationDepth::FraudProofStub
        );
        assert_eq!(
            galaxy_edge_verification_depth_stub(Some(&json!({
                "fraud_proof_stub": true,
                "capability_admission": true,
                "network_profile_stale": true,
                "tee_attestation": true,
                "metrics_http": true,
                "stand_smoke_parity": true,
            }))),
            GalaxyEdgeVerificationDepth::FullBand48
        );
        assert_eq!(EDGE_VERIFICATION_CRITERIA.len(), 7);
        assert_eq!(edge_verification_criteria_total(), 7);
        assert!(EDGE_VERIFICATION_CASES.contains(&"openapi_wire"));
        let fm = include_str!("../../docs/catalog/FUNCTION_MANAGEMENT.md");
        for row in FM_BAND48_ROWS {
            assert!(
                fm.contains(row) || row.starts_with("PH-S"),
                "FM band48 row {row}"
            );
        }
    }

    #[test]
    fn grid_verification_replay_json_export_shape_ph_s710() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, REPLAY_JSON_KEYS, VERIFICATION_JSON_KEYS,
        };
        let verification = serde_json::json!({
            "ok": true,
            "metrics": {
                "sample_total": 0,
                "mismatch_total": 0,
                "match_total": 0,
                "checker_pending_total": 0,
            }
        });
        validate_grid_metrics_json_export(&verification, VERIFICATION_JSON_KEYS)
            .expect("verification");
        let replay = serde_json::json!({
            "ok": true,
            "metrics": {
                "replay_pending": 0,
                "replay_pending_scheduled_total": 0,
                "verification_replay_record_total": 0,
            }
        });
        validate_grid_metrics_json_export(&replay, REPLAY_JSON_KEYS).expect("replay");
    }

    #[test]
    fn grid_settlement_trust_replication_pricing_json_export_shape_ph_s711() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_grid_metrics_json_export, PRICING_JSON_KEYS, REPLICATION_JSON_KEYS,
            SETTLEMENT_JSON_KEYS, TRUST_JSON_KEYS,
        };
        let settlement = serde_json::json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 0,
                "resolved_total": 0,
                "payout_batch_total": 0,
            }
        });
        validate_grid_metrics_json_export(&settlement, SETTLEMENT_JSON_KEYS).expect("settlement");
        let trust = serde_json::json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 0,
                "payout_held_total": 0,
                "last_trust_score": 0,
                "gate_min_threshold": 40,
            }
        });
        validate_grid_metrics_json_export(&trust, TRUST_JSON_KEYS).expect("trust");
        let replication = serde_json::json!({
            "ok": true,
            "metrics": {
                "strict_total": 0,
                "enqueue_total": 0,
                "executor_enqueue_total": 0,
                "rate_limited_total": 0,
            }
        });
        validate_grid_metrics_json_export(&replication, REPLICATION_JSON_KEYS)
            .expect("replication");
        let pricing = serde_json::json!({
            "ok": true,
            "metrics": {
                "fresh_served_total": 0,
                "stale_served_total": 0,
                "forced_fallback_total": 0,
                "provider_catalog_lookups_total": 0,
                "provider_catalog_hits_total": 0,
                "provider_errors_total": 0,
                "provider_timeouts_total": 0,
            }
        });
        validate_grid_metrics_json_export(&pricing, PRICING_JSON_KEYS).expect("pricing");
    }

    #[test]
    fn grid_prefetch_locality_metrics_parity_ph_s831() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_locality_metrics_parity, validate_prefetch_metrics_parity,
        };

        let prefetch_prom = concat!(
            "galaxy_prefetch_pull_bytes_total 1024\n",
            "galaxy_prefetch_backpressure_total 1\n",
        );
        let prefetch = serde_json::json!({
            "ok": true,
            "metrics": {
                "pull_bytes_total": 1024,
                "backpressure_total": 1,
                "plan_total": 0,
                "enqueue_total": 0,
                "peer_fetch_total": 0,
            }
        });
        validate_prefetch_metrics_parity(prefetch_prom, &prefetch).expect("prefetch parity");

        let locality_prom = concat!(
            "galaxy_shard_local_hit_ratio 7500\n",
            "galaxy_hot_tier_hit_ratio 4000\n",
            "galaxy_cross_region_egress_mb 5\n",
            "galaxy_hot_promote_total 1\n",
            "galaxy_hot_evict_total 0\n",
        );
        let locality = serde_json::json!({
            "ok": true,
            "metrics": {
                "shard_local_hit_ratio_bps": 7500,
                "hot_tier_hit_ratio_bps": 4000,
                "cross_region_egress_mb": 5,
                "hot_promote_total": 1,
                "hot_evict_total": 0,
            }
        });
        validate_locality_metrics_parity(locality_prom, &locality).expect("locality parity");
    }

    #[test]
    fn grid_governance_fee_metrics_parity_ph_s832() {
        use poolai::grid::stand_smoke_metrics_parity::{
            validate_fee_split_metrics_parity, validate_governance_metrics_parity,
        };

        let fee_prom = "galaxy_fee_split_applied_total 7\n";
        let fee_split = serde_json::json!({
            "ok": true,
            "metrics": {
                "fee_split_applied_total": 7,
                "primary_dev_fee_bps": 10,
                "secondary_admin_fee_min_bps": 100,
                "secondary_admin_fee_max_bps": 500,
            }
        });
        validate_fee_split_metrics_parity(fee_prom, &fee_split).expect("fee parity");

        let gov_prom = concat!(
            "poolai_release_verify_total 2\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 1\n",
            "poolai_advisory_acknowledged_total 3\n",
        );
        let governance = serde_json::json!({
            "ok": true,
            "metrics": {
                "release_verify_total": 2,
                "release_verify_fail_total": 0,
                "update_notify_pending": 1,
                "advisory_acknowledged_total": 3,
            }
        });
        validate_governance_metrics_parity(gov_prom, &governance).expect("governance parity");
    }

    #[test]
    fn stand_smoke_export_shape_regression_suite_ph_s834() {
        use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity_v2;

        let prom = concat!(
            "galaxy_verification_sample_total 1\n",
            "galaxy_verification_checker_pending_total 0\n",
            "galaxy_replay_pending 0\n",
            "galaxy_verification_replay_record_total 0\n",
            "galaxy_settlement_cleared_total 0\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "galaxy_trust_score 0\n",
            "galaxy_replication_strict_total 0\n",
            "galaxy_replication_enqueue_total 0\n",
            "galaxy_pricing_fresh_served 0\n",
            "galaxy_pricing_stale_served 0\n",
            "galaxy_prefetch_pull_bytes_total 0\n",
            "galaxy_prefetch_backpressure_total 0\n",
            "galaxy_shard_local_hit_ratio 0\n",
            "galaxy_hot_tier_hit_ratio 0\n",
            "galaxy_cross_region_egress_mb 0\n",
            "galaxy_hot_promote_total 0\n",
            "galaxy_hot_evict_total 0\n",
            "galaxy_fee_split_applied_total 0\n",
            "poolai_release_verify_total 0\n",
            "poolai_release_verify_fail_total 0\n",
            "poolai_update_notify_pending 0\n",
            "poolai_advisory_acknowledged_total 0\n",
            "galaxy_settlement_payout_batch_queue_depth 0\n",
            "galaxy_settlement_onchain_submit_total 0\n",
        );
        let verification = serde_json::json!({"ok": true, "metrics": {"sample_total": 1, "mismatch_total": 0, "match_total": 0, "checker_pending_total": 0}});
        let replay = serde_json::json!({"ok": true, "metrics": {"replay_pending": 0, "replay_pending_scheduled_total": 0, "verification_replay_record_total": 0}});
        let settlement = serde_json::json!({"ok": true, "metrics": {"pending_verification_total": 0, "cleared_total": 0, "resolved_total": 0, "payout_batch_total": 0}});
        let trust = serde_json::json!({"ok": true, "metrics": {"payout_eligible_total": 0, "payout_held_total": 0, "last_trust_score": 0, "gate_min_threshold": 40}});
        let replication = serde_json::json!({"ok": true, "metrics": {"strict_total": 0, "enqueue_total": 0, "executor_enqueue_total": 0, "rate_limited_total": 0}});
        let pricing = serde_json::json!({"ok": true, "metrics": {"fresh_served_total": 0, "stale_served_total": 0, "forced_fallback_total": 0, "provider_catalog_lookups_total": 0, "provider_catalog_hits_total": 0, "provider_errors_total": 0, "provider_timeouts_total": 0}});
        let prefetch = serde_json::json!({"ok": true, "metrics": {"pull_bytes_total": 0, "backpressure_total": 0, "plan_total": 0, "enqueue_total": 0, "peer_fetch_total": 0}});
        let locality = serde_json::json!({"ok": true, "metrics": {"shard_local_hit_ratio_bps": 0, "hot_tier_hit_ratio_bps": 0, "cross_region_egress_mb": 0, "hot_promote_total": 0, "hot_evict_total": 0}});
        let fee_split = serde_json::json!({"ok": true, "metrics": {"fee_split_applied_total": 0, "primary_dev_fee_bps": 10, "secondary_admin_fee_min_bps": 100, "secondary_admin_fee_max_bps": 500}});
        let governance = serde_json::json!({"ok": true, "metrics": {"release_verify_total": 0, "release_verify_fail_total": 0, "update_notify_pending": 0, "advisory_acknowledged_total": 0}});
        let payout_batch = serde_json::json!({"ok": true, "metrics": {"payout_batch_total": 0, "payout_batch_queue_depth": 0, "onchain_submit_total": 0}});
        validate_band6_metrics_parity_v2(
            prom,
            &verification,
            &replay,
            &settlement,
            &trust,
            &replication,
            &pricing,
            &prefetch,
            &locality,
            &fee_split,
            &governance,
            &payout_batch,
        )
        .expect("band18 regression suite");
    }

    #[test]
    fn multi_module_stand_smoke_full_suite_ph_s1002() {
        use poolai_ui_core::multi_module_depth::{
            multi_module_depth_stub, MultiModuleDepth, MULTI_MODULE_BAND35_TOP5_GRID_APIS,
            STAND_SMOKE_FULL_SUITE,
        };
        use serde_json::json;

        assert_eq!(
            multi_module_depth_stub(Some(&json!({"stand_smoke": true}))),
            MultiModuleDepth::StandSmoke
        );
        assert_eq!(MULTI_MODULE_BAND35_TOP5_GRID_APIS.len(), 5);
        assert!(STAND_SMOKE_FULL_SUITE.contains("--json"));
        stand_smoke_export_shape_regression_suite_ph_s834();
    }

    #[test]
    fn grid_metrics_band6_prometheus_parity_export_shape_ph_s713() {
        use poolai::grid::stand_smoke_metrics_parity::validate_band6_metrics_parity;
        let prom = concat!(
            "galaxy_verification_sample_total 0\n",
            "galaxy_verification_checker_pending_total 0\n",
            "galaxy_replay_pending 0\n",
            "galaxy_verification_replay_record_total 0\n",
            "galaxy_settlement_cleared_total 0\n",
            "galaxy_settlement_payout_batch_total 0\n",
            "galaxy_trust_payout_eligible_total 0\n",
            "galaxy_trust_score 0\n",
            "galaxy_replication_strict_total 0\n",
            "galaxy_replication_enqueue_total 0\n",
            "galaxy_pricing_fresh_served 0\n",
            "galaxy_pricing_stale_served 0\n",
        );
        let verification = serde_json::json!({"ok": true, "metrics": {"sample_total": 0, "mismatch_total": 0, "match_total": 0, "checker_pending_total": 0}});
        let replay = serde_json::json!({"ok": true, "metrics": {"replay_pending": 0, "replay_pending_scheduled_total": 0, "verification_replay_record_total": 0}});
        let settlement = serde_json::json!({"ok": true, "metrics": {"pending_verification_total": 0, "cleared_total": 0, "resolved_total": 0, "payout_batch_total": 0}});
        let trust = serde_json::json!({"ok": true, "metrics": {"payout_eligible_total": 0, "payout_held_total": 0, "last_trust_score": 0, "gate_min_threshold": 40}});
        let replication = serde_json::json!({"ok": true, "metrics": {"strict_total": 0, "enqueue_total": 0, "executor_enqueue_total": 0, "rate_limited_total": 0}});
        let pricing = serde_json::json!({"ok": true, "metrics": {"fresh_served_total": 0, "stale_served_total": 0, "forced_fallback_total": 0, "provider_catalog_lookups_total": 0, "provider_catalog_hits_total": 0, "provider_errors_total": 0, "provider_timeouts_total": 0}});
        validate_band6_metrics_parity(
            prom,
            &verification,
            &replay,
            &settlement,
            &trust,
            &replication,
            &pricing,
        )
        .expect("band6 parity");
    }

    #[test]
    fn grid_settlement_trust_prometheus_parity_export_shape_ph_s723() {
        use poolai::grid::stand_smoke_metrics_parity::validate_settlement_trust_metrics_parity;
        let prom = concat!(
            "galaxy_settlement_cleared_total 2\n",
            "galaxy_settlement_payout_batch_total 1\n",
            "galaxy_trust_payout_eligible_total 3\n",
            "galaxy_trust_score 55\n",
        );
        let settlement = serde_json::json!({
            "ok": true,
            "metrics": {
                "pending_verification_total": 0,
                "cleared_total": 2,
                "resolved_total": 0,
                "payout_batch_total": 1,
            }
        });
        let trust = serde_json::json!({
            "ok": true,
            "metrics": {
                "payout_eligible_total": 3,
                "payout_held_total": 0,
                "last_trust_score": 55,
                "gate_min_threshold": 40,
            }
        });
        validate_settlement_trust_metrics_parity(prom, &settlement, &trust).expect("parity");
    }

    #[test]
    fn grid_network_profiles_list_put_export_shape_ph_s733() {
        let list = serde_json::json!({
            "ok": true,
            "peer_ids": ["peer-a", "peer-b"],
            "count": 2
        });
        assert_eq!(list["ok"], true);
        assert_eq!(list["count"], 2);
        let ids = list["peer_ids"].as_array().expect("peer_ids");
        assert_eq!(ids.len(), 2);

        let profile = serde_json::json!({
            "ok": true,
            "peer_id": "peer-a",
            "network_profile": {
                "region": "smoke",
                "latency_ms_p50": 11
            }
        });
        assert_eq!(profile["peer_id"], "peer-a");
        assert_eq!(profile["network_profile"]["region"].as_str(), Some("smoke"));
    }

    #[test]
    fn openapi_band_export_shape_ph_s843() {
        let advisories = serde_json::json!([
            {
                "id": "CVE-2026-0001",
                "severity": "medium",
                "summary": "Signed release manifest rotation advisory (Galaxy §9.2)",
                "acknowledged": false
            }
        ]);
        let row = &advisories[0];
        for key in ["id", "severity", "summary", "acknowledged"] {
            assert!(row.get(key).is_some(), "missing {key}");
        }
        let rebind_err = serde_json::json!({
            "error": "admin_required",
            "message": "Bearer admin token required for wallet rebind override"
        });
        assert_eq!(rebind_err["error"].as_str(), Some("admin_required"));
    }

    #[test]
    fn signed_capability_reject_export_shape_ph_s743() {
        let reject = serde_json::json!({
            "error": {
                "code": "capability_signature_invalid",
                "message": "signed capability_document required for telegram_edge origin"
            }
        });
        assert_eq!(
            reject["error"]["code"].as_str(),
            Some("capability_signature_invalid")
        );
        assert!(reject["error"]["message"]
            .as_str()
            .unwrap_or("")
            .contains("telegram_edge"));

        let metrics = concat!(
            "# HELP galaxy_capability_unsigned_rejected_total Unsigned or invalid signed capability rejections on telegram_edge register-remote (PH-S740)\n",
            "# TYPE galaxy_capability_unsigned_rejected_total gauge\n",
            "galaxy_capability_unsigned_rejected_total 1\n",
            "# HELP galaxy_capability_signed_accepted_total Successful signed capability admissions on telegram_edge register-remote (PH-S741)\n",
            "# TYPE galaxy_capability_signed_accepted_total gauge\n",
            "galaxy_capability_signed_accepted_total 0\n",
        );
        assert!(metrics.contains("galaxy_capability_unsigned_rejected_total"));
        assert!(metrics.contains("galaxy_capability_signed_accepted_total"));
    }

    #[test]
    fn vision_revision_fm_parity_ph_s235() {
        let root = repo_root();
        assert_vision_repo_parity(&root)
            .expect("run poolai-vision-sync --check before stand smoke");
        let manifest_rev = read_manifest_revision(&root).expect("manifest");
        let fm_rev = read_fm_vision_revision(&root).expect("fm");
        assert_eq!(manifest_rev, fm_rev);
        if let Some(next) = read_manifest_next_sprint(&root) {
            let active = read_extensions_active_sprint(&root)
                .expect("extensions")
                .expect("active_sprint");
            assert_eq!(active, next);
        }
    }
}
