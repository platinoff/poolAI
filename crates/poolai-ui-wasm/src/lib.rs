//! wasm32 exports for admin grid-pricing panel helpers (PH-S147).
//!
//! Wraps [`poolai_ui_core`] formatters so the same logic runs in the browser via WASM.

use chrono::{DateTime, Utc};
use poolai_ui_core::admin_dom::{admin_inline_error_html, admin_loading_html};
use poolai_ui_core::api_error::{api_error_detail_from_body, format_fetch_error};
use poolai_ui_core::format::{
    alert_severity_badge_class, escape_html, format_bytes, format_latency_ms, format_load_fraction,
    format_megabytes, format_percent, format_rotation_kind, format_topology_timestamp,
    format_unix_timestamp_display, format_uptime,
};
use poolai_ui_core::galaxy_telegram_seats::render_telegram_seats_panel_html;
use poolai_ui_core::galaxy_virtual_nodes::render_galaxy_virtual_nodes_panel_html;
use poolai_ui_core::grid_replication_pricing::render_grid_replication_pricing_panel_html;
use poolai_ui_core::grid_verification::render_grid_verification_panel_html;
use poolai_ui_core::instances::render_instances_panel_html;
use poolai_ui_core::jobs::render_jobs_store_badge_html;
use poolai_ui_core::lease::lease_state;
use poolai_ui_core::libs::render_libs_panel_html;
use poolai_ui_core::memory::{format_seed_inventory_ram_bytes, render_memory_seed_meta_strip_html};
use poolai_ui_core::ml::{
    build_admin_overview_url, build_alert_rules_url, build_audit_events_url,
    build_dashboard_metrics_window_url, build_metric_history_query, build_metric_history_url,
    build_metric_history_url_with_hours, build_metrics_window_url,
    build_metrics_window_url_with_hours, build_ml_pipeline_demo_url, build_ml_pipelines_url,
    build_monitoring_active_alerts_url, build_monitoring_alert_acknowledge_url,
    build_monitoring_alerts_url, build_monitoring_dashboards_url,
    build_monitoring_metric_latest_url, chart_scale, collect_ml_sparkline_series,
    flatten_ml_step_rows, format_ml_metric_summary, group_metrics_by_name, metric_point_values,
    parse_ml_numeric, render_line_chart_empty_html, render_line_chart_html,
    render_metrics_chart_grid_html, render_ml_pipeline_metrics_panel_html,
    render_monitoring_alerts_panel_html, render_monitoring_dashboards_panel_html,
    render_sparkline_html, sanitize_chart_id,
};
use poolai_ui_core::modal::{admin_dynamic_modal_html, trap_tab_action, MODAL_FOCUSABLE_SELECTOR};
use poolai_ui_core::network_profiles::render_network_profiles_panel_html;
use poolai_ui_core::payout_batch::{
    render_payout_batch_history_strip_html, render_payout_batch_panel_html,
};
use poolai_ui_core::pricing::{format_unix_secs, format_usd_micro};
use poolai_ui_core::prometheus::parse_prometheus_gauge;
use poolai_ui_core::security::render_secret_rotation_panel_html;
use poolai_ui_core::stand_smoke_metrics::{
    render_grid_fee_split_metrics_strip_html, render_grid_governance_metrics_strip_html,
    render_grid_locality_metrics_strip_html, render_grid_prefetch_metrics_strip_html,
    render_grid_settlement_trust_metrics_strip_html, render_grid_verification_metrics_strip_html,
};
use poolai_ui_core::table::{
    build_csv, build_json_export, compare_sort_values, empty_state_html, escape_regex,
    export_filename_from_aria, form_field_html, highlight_query_html, render_table_html,
    row_matches_query, table_export_buttons_html,
};
use poolai_ui_core::theme::normalize_theme;
use poolai_ui_core::topology::{
    render_topology_stats_strip_html, short_topology_node_id, topology_hub_label,
};
use poolai_ui_core::updates_compat::{compat_status_label, protocol_version_label};
use poolai_ui_core::vm::render_vm_panel_html;
use poolai_ui_core::workers::render_workers_panel_html;
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

/// Jobs store backend badge HTML (PH-S852).
#[wasm_bindgen(js_name = renderJobsStoreBadgeHtml)]
pub fn render_jobs_store_badge_html_wasm(
    backend: &str,
    store_label: &str,
    store_hint: &str,
    backend_display: &str,
) -> String {
    render_jobs_store_badge_html(backend, store_label, store_hint, backend_display)
}

/// Memory / seed inventory meta strip HTML (PH-S862).
#[wasm_bindgen(js_name = renderMemorySeedMetaStripHtml)]
pub fn render_memory_seed_meta_strip_html_wasm(
    memory_persist: bool,
    registered_shard_count: u32,
    memory_store_depth: &str,
    memory_layer_depth: &str,
    persist_label: &str,
    shards_label: &str,
) -> String {
    render_memory_seed_meta_strip_html(
        memory_persist,
        registered_shard_count,
        memory_store_depth,
        memory_layer_depth,
        persist_label,
        shards_label,
    )
}

/// Seed inventory hot-tier RAM bytes cell (PH-S862).
#[wasm_bindgen(js_name = formatSeedInventoryRamBytes)]
pub fn format_seed_inventory_ram_bytes_wasm(ram_bytes: Option<f64>) -> String {
    format_seed_inventory_ram_bytes(ram_bytes.map(|v| v as u64))
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

#[wasm_bindgen(js_name = exportFilenameFromAria)]
pub fn export_filename_from_aria_wasm(aria_label: &str, extension: &str) -> String {
    export_filename_from_aria(aria_label, extension)
}

#[wasm_bindgen(js_name = tableExportButtonsHtml)]
pub fn table_export_buttons_html_wasm(
    export_csv_label: &str,
    export_json_label: &str,
    csv_aria: &str,
    json_aria: &str,
) -> String {
    table_export_buttons_html(export_csv_label, export_json_label, csv_aria, json_aria)
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

#[wasm_bindgen(js_name = renderLineChartHtml)]
pub fn render_line_chart_html_wasm(
    metric_name: &str,
    values_json: &str,
    width: f64,
    height: f64,
    padding: f64,
    points_label: &str,
    stat_min_lbl: &str,
    stat_max_lbl: &str,
    stat_avg_lbl: &str,
) -> String {
    let values: Vec<f64> = serde_json::from_str(values_json).unwrap_or_default();
    render_line_chart_html(
        metric_name,
        &values,
        width,
        height,
        padding,
        points_label,
        stat_min_lbl,
        stat_max_lbl,
        stat_avg_lbl,
    )
}

#[wasm_bindgen(js_name = renderLineChartEmptyHtml)]
pub fn render_line_chart_empty_html_wasm(no_data_label: &str) -> String {
    render_line_chart_empty_html(no_data_label)
}

#[wasm_bindgen(js_name = buildMetricHistoryQuery)]
pub fn build_metric_history_query_wasm(
    metric_name: &str,
    start_time: &str,
    end_time: &str,
    limit: u32,
) -> String {
    build_metric_history_query(metric_name, start_time, end_time, limit)
}

#[wasm_bindgen(js_name = buildMetricHistoryUrl)]
pub fn build_metric_history_url_wasm(
    metric_name: &str,
    start_time: &str,
    end_time: &str,
    limit: u32,
) -> String {
    build_metric_history_url(metric_name, start_time, end_time, limit)
}

#[wasm_bindgen(js_name = buildMetricHistoryUrlWithHours)]
pub fn build_metric_history_url_with_hours_wasm(
    metric_name: &str,
    hours: u32,
    limit: u32,
    now_rfc3339: &str,
) -> String {
    build_metric_history_url_with_hours(metric_name, hours, limit, now_rfc3339)
}

#[wasm_bindgen(js_name = buildMetricsWindowUrl)]
pub fn build_metrics_window_url_wasm(start_time: &str, end_time: &str, limit: u32) -> String {
    build_metrics_window_url(start_time, end_time, limit)
}

#[wasm_bindgen(js_name = buildMetricsWindowUrlWithHours)]
pub fn build_metrics_window_url_with_hours_wasm(
    hours: u32,
    limit: u32,
    now_rfc3339: &str,
) -> String {
    build_metrics_window_url_with_hours(hours, limit, now_rfc3339)
}

#[wasm_bindgen(js_name = buildMlPipelinesUrl)]
pub fn build_ml_pipelines_url_wasm() -> String {
    build_ml_pipelines_url()
}

#[wasm_bindgen(js_name = buildMlPipelineDemoUrl)]
pub fn build_ml_pipeline_demo_url_wasm() -> String {
    build_ml_pipeline_demo_url()
}

#[wasm_bindgen(js_name = renderMlPipelineMetricsPanel)]
pub fn render_ml_pipeline_metrics_panel_wasm(
    pipelines_json: &str,
    title: &str,
    empty_message: &str,
    empty_hint: &str,
    columns_json: &str,
    avg_label: &str,
) -> String {
    render_ml_pipeline_metrics_panel_html(
        pipelines_json,
        title,
        empty_message,
        empty_hint,
        columns_json,
        avg_label,
    )
}

#[wasm_bindgen(js_name = renderMonitoringAlertsPanel)]
pub fn render_monitoring_alerts_panel_wasm(
    alerts_json: &str,
    na_label: &str,
    ack_label: &str,
    active_label: &str,
    ack_btn_label: &str,
    col_severity: &str,
    col_metric: &str,
    col_current: &str,
    col_threshold: &str,
    col_triggered: &str,
    col_status: &str,
    col_actions: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    render_monitoring_alerts_panel_html(
        alerts_json,
        na_label,
        ack_label,
        active_label,
        ack_btn_label,
        col_severity,
        col_metric,
        col_current,
        col_threshold,
        col_triggered,
        col_status,
        col_actions,
        table_aria,
        empty_message,
    )
}

#[wasm_bindgen(js_name = renderWorkersPanel)]
pub fn render_workers_panel_wasm(
    workers_json: &str,
    col_id: &str,
    col_status: &str,
    col_metrics: &str,
    col_actions: &str,
    table_aria: &str,
    healthy_label: &str,
    unhealthy_label: &str,
    req_label: &str,
    delete_label: &str,
    empty_message: &str,
) -> String {
    render_workers_panel_html(
        workers_json,
        col_id,
        col_status,
        col_metrics,
        col_actions,
        table_aria,
        healthy_label,
        unhealthy_label,
        req_label,
        delete_label,
        empty_message,
    )
}

#[wasm_bindgen(js_name = renderInstancesPanel)]
pub fn render_instances_panel_wasm(
    instances_json: &str,
    col_instance_id: &str,
    col_model_id: &str,
    col_status: &str,
    col_strategy: &str,
    col_nodes: &str,
    col_created: &str,
    col_actions: &str,
    table_aria: &str,
    view_label: &str,
    delete_label: &str,
    empty_message: &str,
) -> String {
    render_instances_panel_html(
        instances_json,
        col_instance_id,
        col_model_id,
        col_status,
        col_strategy,
        col_nodes,
        col_created,
        col_actions,
        table_aria,
        view_label,
        delete_label,
        empty_message,
    )
}

#[wasm_bindgen(js_name = renderVmPanel)]
pub fn render_vm_panel_wasm(
    instances_json: &str,
    col_name: &str,
    col_status: &str,
    col_resources: &str,
    col_actions: &str,
    table_aria: &str,
    res_cpu_label: &str,
    res_mem_label: &str,
    start_label: &str,
    stop_label: &str,
    delete_label: &str,
    empty_message: &str,
) -> String {
    render_vm_panel_html(
        instances_json,
        col_name,
        col_status,
        col_resources,
        col_actions,
        table_aria,
        res_cpu_label,
        res_mem_label,
        start_label,
        stop_label,
        delete_label,
        empty_message,
    )
}

/// Admin libraries panel HTML (PH-S821).
#[wasm_bindgen(js_name = renderLibsPanel)]
pub fn render_libs_panel_wasm(
    libs_json: &str,
    col_name: &str,
    col_version: &str,
    col_status: &str,
    col_actions: &str,
    table_aria: &str,
    installed_label: &str,
    not_installed_label: &str,
    uninstall_label: &str,
    update_label: &str,
    install_label: &str,
    empty_message: &str,
) -> String {
    render_libs_panel_html(
        libs_json,
        col_name,
        col_version,
        col_status,
        col_actions,
        table_aria,
        installed_label,
        not_installed_label,
        uninstall_label,
        update_label,
        install_label,
        empty_message,
    )
}

#[wasm_bindgen(js_name = renderGalaxyVirtualNodesPanel)]
pub fn render_galaxy_virtual_nodes_panel_wasm(
    nodes_json: &str,
    col_peer: &str,
    col_origin: &str,
    col_region: &str,
    col_latency: &str,
    col_stale: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    render_galaxy_virtual_nodes_panel_html(
        nodes_json,
        col_peer,
        col_origin,
        col_region,
        col_latency,
        col_stale,
        table_aria,
        empty_message,
    )
}

#[wasm_bindgen(js_name = renderGridReplicationPricingPanel)]
pub fn render_grid_replication_pricing_panel_wasm(
    replication_metrics_json: &str,
    pricing_metrics_json: &str,
    strict_gauge: u64,
    i18n_json: &str,
) -> String {
    render_grid_replication_pricing_panel_html(
        replication_metrics_json,
        pricing_metrics_json,
        strict_gauge,
        i18n_json,
    )
}

#[wasm_bindgen(js_name = renderGridVerificationPanel)]
pub fn render_grid_verification_panel_wasm(
    tasks_json: &str,
    pending_total: u64,
    col_job: &str,
    col_type: &str,
    col_pending: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    render_grid_verification_panel_html(
        tasks_json,
        pending_total,
        col_job,
        col_type,
        col_pending,
        table_aria,
        empty_message,
    )
}

#[wasm_bindgen(js_name = renderGridVerificationMetricsStrip)]
pub fn render_grid_verification_metrics_strip_wasm(
    verification_metrics_json: &str,
    pending_total: u64,
) -> String {
    render_grid_verification_metrics_strip_html(verification_metrics_json, pending_total)
}

#[wasm_bindgen(js_name = renderGridSettlementTrustMetricsStrip)]
pub fn render_grid_settlement_trust_metrics_strip_wasm(
    settlement_metrics_json: &str,
    trust_metrics_json: &str,
    trust_score_gauge: u64,
) -> String {
    render_grid_settlement_trust_metrics_strip_html(
        settlement_metrics_json,
        trust_metrics_json,
        trust_score_gauge,
    )
}

#[wasm_bindgen(js_name = renderGridPrefetchMetricsStrip)]
pub fn render_grid_prefetch_metrics_strip_wasm(
    prefetch_metrics_json: &str,
    pull_bytes_gauge: u64,
) -> String {
    render_grid_prefetch_metrics_strip_html(prefetch_metrics_json, pull_bytes_gauge)
}

#[wasm_bindgen(js_name = renderGridLocalityMetricsStrip)]
pub fn render_grid_locality_metrics_strip_wasm(
    locality_metrics_json: &str,
    hot_promote_gauge: u64,
) -> String {
    render_grid_locality_metrics_strip_html(locality_metrics_json, hot_promote_gauge)
}

#[wasm_bindgen(js_name = renderGridFeeSplitMetricsStrip)]
pub fn render_grid_fee_split_metrics_strip_wasm(
    fee_split_metrics_json: &str,
    applied_gauge: u64,
) -> String {
    render_grid_fee_split_metrics_strip_html(fee_split_metrics_json, applied_gauge)
}

#[wasm_bindgen(js_name = renderGridGovernanceMetricsStrip)]
pub fn render_grid_governance_metrics_strip_wasm(
    governance_metrics_json: &str,
    update_policy_json: &str,
    advisory_gauge: u64,
) -> String {
    render_grid_governance_metrics_strip_html(
        governance_metrics_json,
        update_policy_json,
        advisory_gauge,
    )
}

#[wasm_bindgen(js_name = renderNetworkProfilesPanel)]
pub fn render_network_profiles_panel_wasm(
    rows_json: &str,
    col_peer: &str,
    col_region: &str,
    col_latency: &str,
    col_bandwidth: &str,
    table_aria: &str,
    empty_message: &str,
) -> String {
    render_network_profiles_panel_html(
        rows_json,
        col_peer,
        col_region,
        col_latency,
        col_bandwidth,
        table_aria,
        empty_message,
    )
}

/// Parse Prometheus gauge from `/metrics` text (PH-S672).
#[wasm_bindgen(js_name = parsePrometheusGauge)]
pub fn parse_prometheus_gauge_wasm(metrics_text: &str, metric_name: &str) -> u64 {
    parse_prometheus_gauge(metrics_text, metric_name)
}

#[wasm_bindgen(js_name = renderTelegramSeatsPanel)]
pub fn render_telegram_seats_panel_wasm(
    snapshot_json: &str,
    col_policy: &str,
    col_limit: &str,
    col_active: &str,
    col_bound: &str,
    table_aria: &str,
) -> String {
    render_telegram_seats_panel_html(
        snapshot_json,
        col_policy,
        col_limit,
        col_active,
        col_bound,
        table_aria,
    )
}

#[wasm_bindgen(js_name = renderMonitoringDashboardsPanel)]
pub fn render_monitoring_dashboards_panel_wasm(
    dashboards_json: &str,
    col_name: &str,
    col_description: &str,
    col_metrics: &str,
    col_public: &str,
    col_created: &str,
    table_aria: &str,
    em_dash: &str,
    na_label: &str,
    public_label: &str,
    private_label: &str,
    metrics_n_template: &str,
    empty_message: &str,
) -> String {
    render_monitoring_dashboards_panel_html(
        dashboards_json,
        col_name,
        col_description,
        col_metrics,
        col_public,
        col_created,
        table_aria,
        em_dash,
        na_label,
        public_label,
        private_label,
        metrics_n_template,
        empty_message,
    )
}

#[wasm_bindgen(js_name = buildMonitoringAlertsUrl)]
pub fn build_monitoring_alerts_url_wasm(limit: u32, acknowledged: Option<bool>) -> String {
    build_monitoring_alerts_url(limit, acknowledged)
}

#[wasm_bindgen(js_name = buildMonitoringActiveAlertsUrl)]
pub fn build_monitoring_active_alerts_url_wasm(limit: u32) -> String {
    build_monitoring_active_alerts_url(limit)
}

#[wasm_bindgen(js_name = buildAlertRulesUrl)]
pub fn build_alert_rules_url_wasm() -> String {
    build_alert_rules_url()
}

#[wasm_bindgen(js_name = buildMonitoringDashboardsUrl)]
pub fn build_monitoring_dashboards_url_wasm() -> String {
    build_monitoring_dashboards_url()
}

#[wasm_bindgen(js_name = buildMonitoringAlertAcknowledgeUrl)]
pub fn build_monitoring_alert_acknowledge_url_wasm(alert_id: &str) -> String {
    build_monitoring_alert_acknowledge_url(alert_id)
}

#[wasm_bindgen(js_name = buildMonitoringMetricLatestUrl)]
pub fn build_monitoring_metric_latest_url_wasm(metric_name: &str, limit: u32) -> String {
    build_monitoring_metric_latest_url(metric_name, limit)
}

#[wasm_bindgen(js_name = buildAuditEventsUrl)]
pub fn build_audit_events_url_wasm(limit: u32) -> String {
    build_audit_events_url(limit)
}

#[wasm_bindgen(js_name = buildAdminOverviewUrl)]
pub fn build_admin_overview_url_wasm() -> String {
    build_admin_overview_url()
}

#[wasm_bindgen(js_name = formatUptime)]
pub fn format_uptime_wasm(seconds: u64) -> String {
    format_uptime(seconds)
}

#[wasm_bindgen(js_name = formatPercent)]
pub fn format_percent_wasm(value: f64) -> String {
    format_percent(value)
}

#[wasm_bindgen(js_name = formatMegabytes)]
pub fn format_megabytes_wasm(value: f64) -> String {
    format_megabytes(value)
}

#[wasm_bindgen(js_name = formatBytes)]
pub fn format_bytes_wasm(bytes: u64) -> String {
    format_bytes(bytes)
}

/// Security admin: unix timestamp with never-label fallback (PH-S628).
#[wasm_bindgen(js_name = formatUnixTimestamp)]
pub fn format_unix_timestamp_wasm(secs: i64, never_label: &str) -> String {
    format_unix_timestamp_display(if secs > 0 { Some(secs) } else { None }, never_label)
}

/// Security admin: rotation kind label (PH-S628).
#[wasm_bindgen(js_name = formatRotationKind)]
pub fn format_rotation_kind_wasm(kind: &str) -> String {
    format_rotation_kind(kind)
}

/// Topology admin: ISO timestamp display (PH-S636).
#[wasm_bindgen(js_name = formatTopologyTimestamp)]
pub fn format_topology_timestamp_wasm(iso: &str) -> String {
    format_topology_timestamp(empty_as_none(iso))
}

/// Topology admin: load fraction percent label (PH-S636).
#[wasm_bindgen(js_name = formatLoadFraction)]
pub fn format_load_fraction_wasm(x: f64) -> String {
    format_load_fraction(x)
}

/// Topology admin: latency ms label (PH-S636).
#[wasm_bindgen(js_name = formatLatencyMs)]
pub fn format_latency_ms_wasm(latency: f64) -> String {
    format_latency_ms(latency)
}

#[wasm_bindgen(js_name = alertSeverityBadgeClass)]
pub fn alert_severity_badge_class_wasm(severity: &str) -> String {
    alert_severity_badge_class(empty_as_none(severity))
}

#[wasm_bindgen(js_name = buildDashboardMetricsWindowUrl)]
pub fn build_dashboard_metrics_window_url_wasm(
    hours: u32,
    limit: u32,
    now_rfc3339: &str,
) -> String {
    build_dashboard_metrics_window_url(hours, limit, now_rfc3339)
}

#[wasm_bindgen(js_name = groupMetricsByName)]
pub fn group_metrics_by_name_wasm(metrics_json: &str) -> JsValue {
    let data: Vec<Value> = serde_json::from_str(metrics_json).unwrap_or_default();
    serde_wasm_bindgen::to_value(&group_metrics_by_name(&data)).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen(js_name = sanitizeChartId)]
pub fn sanitize_chart_id_wasm(name: &str) -> String {
    sanitize_chart_id(name)
}

#[wasm_bindgen(js_name = renderMetricsChartGridHtml)]
pub fn render_metrics_chart_grid_html_wasm(title: &str, parts_json: &str) -> String {
    let parts: Vec<String> = serde_json::from_str(parts_json).unwrap_or_default();
    render_metrics_chart_grid_html(title, &parts)
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

/// Admin payout batch panel HTML (PH-S564).
#[wasm_bindgen(js_name = renderPayoutBatchPanelHtml)]
pub fn render_payout_batch_panel_html_wasm(
    latest_json: &str,
    history_json: &str,
    i18n_json: &str,
) -> String {
    render_payout_batch_panel_html(latest_json, history_json, i18n_json)
}

/// Admin payout batch history strip HTML (PH-S771).
#[wasm_bindgen(js_name = renderPayoutBatchHistoryStripHtml)]
pub fn render_payout_batch_history_strip_html_wasm(history_json: &str, i18n_json: &str) -> String {
    render_payout_batch_history_strip_html(history_json, i18n_json)
}

/// Admin secret rotation panel HTML (PH-S810).
#[wasm_bindgen(js_name = renderSecretRotationPanelHtml)]
pub fn render_secret_rotation_panel_html_wasm(rows_json: &str, i18n_json: &str) -> String {
    render_secret_rotation_panel_html(rows_json, i18n_json)
}

/// Topology stats strip with formatted timestamp (PH-S811).
#[wasm_bindgen(js_name = renderTopologyStatsStripHtml)]
pub fn render_topology_stats_strip_html_wasm(summary_json: &str, i18n_json: &str) -> String {
    render_topology_stats_strip_html(summary_json, i18n_json)
}

/// Topology hub label helper (PH-S566).
#[wasm_bindgen(js_name = topologyHubLabel)]
pub fn topology_hub_label_wasm(node_id: &str, degree: u32, max_degree: u32) -> String {
    topology_hub_label(node_id, degree as usize, max_degree as usize)
}

/// Topology short node id helper (PH-S566).
#[wasm_bindgen(js_name = shortTopologyNodeId)]
pub fn short_topology_node_id_wasm(node_id: &str) -> String {
    short_topology_node_id(node_id)
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
    fn render_grid_replication_pricing_panel_wasm_ph_s700() {
        let html = render_grid_replication_pricing_panel_wasm(
            r#"{"metrics":{"strict_total":1,"enqueue_total":2}}"#,
            r#"{"metrics":{"fresh_served_total":3,"stale_served_total":0}}"#,
            0,
            r#"{}"#,
        );
        assert!(html.contains("admin-metrics-strip"));
        assert!(html.contains("<strong>1</strong>"));
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
