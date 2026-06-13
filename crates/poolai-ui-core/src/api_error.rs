//! API error parsing — parity with `admin_common.js` (`apiErrorMessageFromBody`, `formatFetchError`).

use serde_json::Value;

/// Parsed API error detail (`apiErrorDetailFromBody`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApiErrorDetail {
    pub message: Option<String>,
    pub code: Option<String>,
    pub hint: Option<String>,
}

/// Default English hints (JS `poolaiT` fallbacks).
pub mod hints {
    pub const RAID_503: &str = "RAID manager is not initialized on this server.";
    pub const LIBRARY_503: &str = "Library subsystem may not be initialized.";
    pub const VM_503: &str = "VM manager may not be attached.";
    pub const GENERIC_503: &str = "A subsystem may still be starting or unavailable.";
    pub const FORBIDDEN_403: &str = "You may need Admin or Operator role, or sign in again.";
    pub const ENTERPRISE_404: &str =
        "Build and run the server with the enterprise feature for this API.";
}

/// Parse legacy flat `error` string or `{ error: { code, message } }`.
pub fn api_error_message_from_body(payload: &Value) -> Option<String> {
    match payload {
        Value::Object(map) => {
            if let Some(Value::String(s)) = map.get("error") {
                return Some(s.clone());
            }
            if let Some(Value::Object(err)) = map.get("error") {
                if let Some(Value::String(msg)) = err.get("message") {
                    return Some(msg.clone());
                }
            }
            map.get("message")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        }
        _ => None,
    }
}

/// Full detail including `error.code` and `context.hint`.
pub fn api_error_detail_from_body(payload: &Value) -> ApiErrorDetail {
    let message = api_error_message_from_body(payload);
    let mut code = None;
    let mut hint = None;
    if let Value::Object(map) = payload {
        if let Some(Value::Object(err)) = map.get("error") {
            if let Some(Value::String(c)) = err.get("code") {
                code = Some(c.clone());
            }
        }
        if let Some(Value::Object(ctx)) = map.get("context") {
            if let Some(Value::String(h)) = ctx.get("hint") {
                hint = Some(h.clone());
            }
        }
    }
    ApiErrorDetail {
        message,
        code,
        hint,
    }
}

/// Mirrors `hintFor503(code, message)` with English fallbacks.
pub fn hint_for_503(code: Option<&str>, message: &str) -> &'static str {
    if code == Some("RAID_MANAGER_UNAVAILABLE") {
        return hints::RAID_503;
    }
    let lower = message.to_lowercase();
    if lower.contains("library") {
        return hints::LIBRARY_503;
    }
    if lower.contains("vm") {
        return hints::VM_503;
    }
    hints::GENERIC_503
}

/// Mirrors `formatFetchError(status, url, payload)`.
pub fn format_fetch_error(status: u16, url: Option<&str>, payload: &Value) -> String {
    let detail = api_error_detail_from_body(payload);
    let base = detail
        .message
        .clone()
        .unwrap_or_else(|| format!("HTTP {status}"));
    let mut extra = detail.hint.unwrap_or_default();
    if status == 403 && extra.is_empty() {
        extra = hints::FORBIDDEN_403.to_string();
    }
    if status == 503 && extra.is_empty() {
        extra = hint_for_503(detail.code.as_deref(), &base).to_string();
    }
    if status == 404 && url.is_some_and(|u| u.contains("/api/enterprise")) && extra.is_empty() {
        extra = hints::ENTERPRISE_404.to_string();
    }
    if extra.is_empty() {
        base
    } else {
        format!("{base} — {extra}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_flat_and_structured_errors() {
        assert_eq!(
            api_error_message_from_body(&json!({"error": "boom"})),
            Some("boom".into())
        );
        assert_eq!(
            api_error_message_from_body(&json!({"error": {"code": "x", "message": "m"}})),
            Some("m".into())
        );
        let detail = api_error_detail_from_body(&json!({
            "error": {"code": "lease_epoch_rejected", "message": "epoch mismatch"},
            "context": {"hint": "retry acquire"}
        }));
        assert_eq!(detail.code.as_deref(), Some("lease_epoch_rejected"));
        assert_eq!(detail.hint.as_deref(), Some("retry acquire"));
    }

    #[test]
    fn format_fetch_error_status_hints() {
        let payload =
            json!({"error": {"code": "RAID_MANAGER_UNAVAILABLE", "message": "raid down"}});
        let msg = format_fetch_error(503, None, &payload);
        assert!(msg.contains("raid down"));
        assert!(msg.contains(hints::RAID_503));

        let msg403 = format_fetch_error(403, None, &json!({}));
        assert!(msg403.contains(hints::FORBIDDEN_403));

        let msg404 = format_fetch_error(404, Some("/api/enterprise/foo"), &json!({}));
        assert!(msg404.contains(hints::ENTERPRISE_404));
    }
}
