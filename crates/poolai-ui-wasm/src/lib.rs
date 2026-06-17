//! wasm32 exports for admin grid-pricing panel helpers (PH-S147).
//!
//! Wraps [`poolai_ui_core`] formatters so the same logic runs in the browser via WASM.

use chrono::{DateTime, Utc};
use poolai_ui_core::admin_dom::{admin_inline_error_html, admin_loading_html};
use poolai_ui_core::api_error::{api_error_detail_from_body, format_fetch_error};
use poolai_ui_core::format::escape_html;
use poolai_ui_core::lease::lease_state;
use poolai_ui_core::ml::{
    chart_scale, collect_ml_sparkline_series, flatten_ml_step_rows, format_ml_metric_summary,
    metric_point_values, parse_ml_numeric, render_sparkline_html,
};
use poolai_ui_core::modal::{admin_dynamic_modal_html, trap_tab_action, MODAL_FOCUSABLE_SELECTOR};
use poolai_ui_core::pricing::{format_unix_secs, format_usd_micro};
use poolai_ui_core::table::{
    build_csv, build_json_export, compare_sort_values, empty_state_html, escape_regex,
    form_field_html, highlight_query_html, render_table_html, row_matches_query,
};
use poolai_ui_core::theme::normalize_theme;
use poolai_ui_core::updates_compat::{compat_status_label, protocol_version_label};
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

/// Admin theme: maps stored name to `dark` | `light` | `high-contrast`.
#[wasm_bindgen(js_name = normalizeTheme)]
pub fn normalize_theme_wasm(name: &str) -> String {
    normalize_theme(name).to_string()
}

/// Modal focus-trap tab action: `none` | `first` | `last` | `root`.
#[wasm_bindgen(js_name = trapTabAction)]
pub fn trap_tab_action_wasm(
    shift_key: bool,
    focusable_count: u32,
    active_inside: bool,
    active_is_first: bool,
    active_is_last: bool,
) -> String {
    trap_tab_action(
        shift_key,
        focusable_count as usize,
        active_inside,
        active_is_first,
        active_is_last,
    )
    .as_str()
    .to_string()
}

#[wasm_bindgen(js_name = modalFocusableSelector)]
pub fn modal_focusable_selector_wasm() -> String {
    MODAL_FOCUSABLE_SELECTOR.to_string()
}

#[wasm_bindgen(js_name = adminDynamicModalHtml)]
pub fn admin_dynamic_modal_html_wasm() -> String {
    admin_dynamic_modal_html()
}

/// Jobs lease badge: returns `"active"`, `"expired"`, or `"none"`.
#[wasm_bindgen(js_name = leaseStateLabel)]
pub fn lease_state_label_wasm(expires_at: &str, now_rfc3339: &str) -> String {
    let now = parse_rfc3339_utc(now_rfc3339).unwrap_or_else(fallback_now_utc);
    let state = lease_state(empty_as_none(expires_at), now);
    state.as_str().to_string()
}

/// Updates & compatibility: human label for `compat_status` wire value.
#[wasm_bindgen(js_name = compatStatusLabel)]
pub fn compat_status_label_wasm(status: &str) -> String {
    compat_status_label(status).to_string()
}

/// Updates & compatibility: normalize protocol version for admin display.
#[wasm_bindgen(js_name = protocolVersionLabel)]
pub fn protocol_version_label_wasm(raw: &str) -> String {
    protocol_version_label(raw)
}

#[wasm_bindgen(js_name = escapeHtml)]
pub fn escape_html_wasm(s: &str) -> String {
    escape_html(s)
}

#[wasm_bindgen(js_name = escapeRegex)]
pub fn escape_regex_wasm(s: &str) -> String {
    escape_regex(s)
}

#[wasm_bindgen(js_name = formatIsoDatetime)]
pub fn format_iso_datetime_wasm(iso: &str) -> String {
    poolai_ui_core::format::format_iso_datetime_display(empty_as_none(iso))
}

#[wasm_bindgen(js_name = formatLocaleTimeHms)]
pub fn format_locale_time_hms_wasm(now_rfc3339: &str) -> String {
    poolai_ui_core::format::format_locale_time_hms(empty_as_none(now_rfc3339))
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

#[wasm_bindgen(js_name = adminLoadingHtml)]
pub fn admin_loading_html_wasm(text: &str) -> String {
    admin_loading_html(text)
}

#[wasm_bindgen(js_name = adminInlineErrorHtml)]
pub fn admin_inline_error_html_wasm(message: &str) -> String {
    admin_inline_error_html(message)
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

/// ML charts: `poolaiParseMlNumeric(val)`.
#[wasm_bindgen(js_name = parseMlNumeric)]
pub fn parse_ml_numeric_wasm(value: &str) -> Option<f64> {
    parse_ml_numeric(empty_as_none(value))
}

/// ML charts: `poolaiFormatMlMetricSummary(output)`.
#[wasm_bindgen(js_name = formatMlMetricSummary)]
pub fn format_ml_metric_summary_wasm(output_json: &str) -> String {
    let output: Value = serde_json::from_str(output_json).unwrap_or(Value::Null);
    format_ml_metric_summary(&output)
}

/// ML charts: `poolaiMetricPointValues(data)`.
#[wasm_bindgen(js_name = metricPointValues)]
pub fn metric_point_values_wasm(data_json: &str) -> JsValue {
    let data: Vec<Value> = serde_json::from_str(data_json).unwrap_or_default();
    serde_wasm_bindgen::to_value(&metric_point_values(&data)).unwrap_or(JsValue::NULL)
}

/// ML charts: `poolaiChartScale(values, width, height, padding)`.
#[wasm_bindgen(js_name = chartScale)]
pub fn chart_scale_wasm(values_json: &str, width: f64, height: f64, padding: f64) -> JsValue {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    serde_wasm_bindgen::to_value(&chart_scale(&values, width, height, padding))
        .unwrap_or(JsValue::NULL)
}

/// ML charts: `poolaiFlattenMlStepRows(pipelines)`.
#[wasm_bindgen(js_name = flattenMlStepRows)]
pub fn flatten_ml_step_rows_wasm(pipelines_json: &str) -> JsValue {
    let pipelines: Vec<Value> = serde_json::from_str(pipelines_json).unwrap_or_default();
    serde_wasm_bindgen::to_value(&flatten_ml_step_rows(&pipelines)).unwrap_or(JsValue::NULL)
}

/// ML charts: `poolaiCollectMlSparklineSeries(rows)`.
#[wasm_bindgen(js_name = collectMlSparklineSeries)]
pub fn collect_ml_sparkline_series_wasm(rows_json: &str) -> JsValue {
    let rows: Vec<poolai_ui_core::ml::MlStepRow> =
        serde_json::from_str(rows_json).unwrap_or_default();
    serde_wasm_bindgen::to_value(&collect_ml_sparkline_series(&rows)).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen(js_name = renderSparklineHtml)]
pub fn render_sparkline_html_wasm(
    label: &str,
    values_json: &str,
    width: f64,
    height: f64,
    avg_label: &str,
) -> String {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    render_sparkline_html(label, &values, width, height, avg_label)
}

/// POC version string for smoke checks in browser devtools.
#[wasm_bindgen(js_name = poolaiUiWasmVersion)]
pub fn poolai_ui_wasm_version() -> String {
    "poolai-ui-wasm/0.1.0-ph-s193".to_string()
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
        assert!(admin_loading_html_wasm("x").contains("muted"));
        assert!(admin_inline_error_html_wasm("err").contains("admin-fetch-error"));
    }

    #[test]
    fn trap_tab_action_wasm_matches_core() {
        assert_eq!(trap_tab_action_wasm(false, 2, true, false, true), "first");
        assert_eq!(trap_tab_action_wasm(true, 0, false, false, false), "root");
    }

    #[test]
    fn modal_wasm_html_has_dynamic_id() {
        assert!(admin_dynamic_modal_html_wasm().contains("adminDynamicModal"));
    }

    #[test]
    fn escape_regex_wasm_matches_core() {
        assert_eq!(escape_regex_wasm("a.b*"), r"a\.b\*");
    }

    #[test]
    fn format_iso_datetime_wasm_matches_core() {
        assert_eq!(
            format_iso_datetime_wasm("2026-06-15T12:00:00Z"),
            "2026-06-15 12:00:00 UTC"
        );
    }

    #[test]
    fn poolai_ui_wasm_version_ph_s193() {
        assert!(poolai_ui_wasm_version().contains("ph-s193"));
    }

    #[test]
    fn normalize_theme_wasm_matches_core() {
        assert_eq!(normalize_theme_wasm("light"), "light");
        assert_eq!(normalize_theme_wasm("high-contrast"), "high-contrast");
        assert_eq!(normalize_theme_wasm("unknown"), "dark");
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

    #[test]
    fn ml_metric_summary_wasm() {
        assert_eq!(
            format_ml_metric_summary_wasm(r#"{"accuracy":"0.9","status":"ok"}"#),
            "accuracy=0.9"
        );
    }

    #[test]
    fn chart_scale_core_matches_wasm_shape() {
        let scale = poolai_ui_core::ml::chart_scale(&[1.0, 3.0, 2.0], 100.0, 50.0, 4.0);
        assert_eq!(scale.points.len(), 3);
        assert!(scale.polyline.contains(','));
    }
}
