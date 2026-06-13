//! wasm32 exports for admin grid-pricing panel helpers (PH-S147).
//!
//! Wraps [`poolai_ui_core`] formatters so the same logic runs in the browser via WASM.

use chrono::{DateTime, Utc};
use poolai_ui_core::api_error::{api_error_detail_from_body, format_fetch_error};
use poolai_ui_core::format::escape_html;
use poolai_ui_core::lease::lease_state;
use poolai_ui_core::pricing::{format_unix_secs, format_usd_micro};
use poolai_ui_core::table::{
    build_csv, build_json_export, compare_sort_values, empty_state_html, form_field_html,
    highlight_query_html, render_table_html, row_matches_query,
};
use serde_json::Value;
use wasm_bindgen::prelude::*;

/// Grid pricing: `formatUsdMicro(usdMicro)` — parity with admin `grid_pricing.rs`.
#[wasm_bindgen(js_name = formatUsdMicro)]
pub fn format_usd_micro_wasm(usd_micro: f64) -> String {
    format_usd_micro(Some(usd_micro))
}

/// Grid pricing: `formatUnixSecs(secs)` — parity with admin `grid_pricing.rs`.
#[wasm_bindgen(js_name = formatUnixSecs)]
pub fn format_unix_secs_wasm(secs: f64) -> String {
    format_unix_secs(Some(secs))
}

/// Jobs lease badge: returns `"active"`, `"expired"`, or `"none"`.
#[wasm_bindgen(js_name = leaseStateLabel)]
pub fn lease_state_label_wasm(expires_at: &str, now_rfc3339: &str) -> String {
    let now = parse_rfc3339_utc(now_rfc3339).unwrap_or_else(fallback_now_utc);
    let state = lease_state(empty_as_none(expires_at), now);
    state.as_str().to_string()
}

#[wasm_bindgen(js_name = escapeHtml)]
pub fn escape_html_wasm(s: &str) -> String {
    escape_html(s)
}

#[wasm_bindgen(js_name = apiErrorMessageFromBody)]
pub fn api_error_message_from_body_wasm(payload_json: &str) -> String {
    let payload: Value = serde_json::from_str(payload_json).unwrap_or(Value::Null);
    poolai_ui_core::api_error::api_error_message_from_body(&payload).unwrap_or_default()
}

#[wasm_bindgen(js_name = apiErrorDetailFromBody)]
pub fn api_error_detail_from_body_wasm(payload_json: &str) -> JsValue {
    let payload: Value = serde_json::from_str(payload_json).unwrap_or(Value::Null);
    let detail = api_error_detail_from_body(&payload);
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "message": detail.message,
        "code": detail.code,
        "hint": detail.hint,
    }))
    .unwrap_or(JsValue::NULL)
}

#[wasm_bindgen(js_name = formatFetchError)]
pub fn format_fetch_error_wasm(status: u16, url: &str, payload_json: &str) -> String {
    let payload: Value = serde_json::from_str(payload_json).unwrap_or(Value::Null);
    let url_opt = if url.is_empty() { None } else { Some(url) };
    format_fetch_error(status, url_opt, &payload)
}

#[wasm_bindgen(js_name = emptyStateHtml)]
pub fn empty_state_html_wasm(message: &str, hint: &str, icon: &str, action_html: &str) -> String {
    empty_state_html(
        message,
        empty_as_none(hint),
        if icon.is_empty() { "📋" } else { icon },
        empty_as_none(action_html),
    )
}

#[wasm_bindgen(js_name = renderTableHtml)]
pub fn render_table_html_wasm(headers_json: &str, rows_json: &str, options_json: &str) -> String {
    render_table_html(headers_json, rows_json, options_json)
}

#[wasm_bindgen(js_name = formFieldHtml)]
pub fn form_field_html_wasm(spec_json: &str, generated_id: &str) -> String {
    form_field_html(spec_json, generated_id)
}

#[wasm_bindgen(js_name = buildTableCsv)]
pub fn build_table_csv_wasm(headers_json: &str, rows_json: &str) -> String {
    let headers: Vec<String> = serde_json::from_str(headers_json).unwrap_or_default();
    let rows: Vec<Vec<String>> = serde_json::from_str(rows_json).unwrap_or_default();
    build_csv(&headers, &rows)
}

#[wasm_bindgen(js_name = buildTableJson)]
pub fn build_table_json_wasm(headers_json: &str, rows_json: &str) -> String {
    let headers: Vec<String> = serde_json::from_str(headers_json).unwrap_or_default();
    let rows: Vec<Vec<String>> = serde_json::from_str(rows_json).unwrap_or_default();
    build_json_export(&headers, &rows)
}

#[wasm_bindgen(js_name = compareSortValues)]
pub fn compare_sort_values_wasm(a: &str, b: &str, numeric: bool, ascending: bool) -> i32 {
    compare_sort_values(a, b, numeric, ascending)
}

#[wasm_bindgen(js_name = rowMatchesQuery)]
pub fn row_matches_query_wasm(row_text: &str, query: &str) -> bool {
    row_matches_query(row_text, query)
}

#[wasm_bindgen(js_name = highlightQueryHtml)]
pub fn highlight_query_html_wasm(original: &str, query: &str) -> String {
    highlight_query_html(original, query)
}

/// POC version string for smoke checks in browser devtools.
#[wasm_bindgen(js_name = poolaiUiWasmVersion)]
pub fn poolai_ui_wasm_version() -> String {
    "poolai-ui-wasm/0.1.0-ph-s153".to_string()
}

fn empty_as_none(s: &str) -> Option<&str> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}

fn parse_rfc3339_utc(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

#[cfg(not(target_arch = "wasm32"))]
fn fallback_now_utc() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(target_arch = "wasm32")]
fn fallback_now_utc() -> DateTime<Utc> {
    DateTime::from_timestamp(0, 0).expect("unix epoch")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use poolai_ui_core::lease::LeaseDisplayState;

    #[test]
    fn wasm_wrappers_match_core() {
        assert_eq!(format_usd_micro_wasm(450_000.0), "0.450000 USD");
        assert_eq!(
            format_unix_secs_wasm(1_718_280_000.0),
            "2024-06-13T12:00:00Z"
        );
        assert_eq!(escape_html_wasm("a<b>"), "a&lt;b&gt;");
    }

    #[test]
    fn lease_state_label_active() {
        let now = Utc.with_ymd_and_hms(2026, 6, 13, 12, 0, 0).unwrap();
        let label = lease_state_label_wasm("2026-06-13T13:00:00Z", &now.to_rfc3339());
        assert_eq!(label, LeaseDisplayState::Active.as_str());
    }

    #[test]
    fn table_csv_wasm_roundtrip() {
        let csv = build_table_csv_wasm(r#"["H"]"#, r#"[["x,y"]]"#);
        assert!(csv.contains("\"x,y\""));
    }
}
