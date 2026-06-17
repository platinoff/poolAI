//! ML pipeline metric helpers — parity with `admin_charts.js` (PH-S43, PH-S155, PH-S275, PH-S284, PH-S287, PH-S294, PH-S297, PH-S314, PH-S317).

use crate::format::escape_html;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Metric keys collected for sparklines / summary (`POOLAI_ML_METRIC_KEYS` in JS).
pub const ML_METRIC_KEYS: &[&str] = &[
    "latency_ms",
    "memory_mb",
    "flops",
    "compression_ratio",
    "bytes_in",
    "bytes_out",
    "accuracy",
    "final_loss",
    "f1_proxy",
    "epochs_run",
    "size_mb_before",
    "size_mb_after",
    "max_abs_recon_error",
    "samples_evaluated",
    "pruned_count",
    "sparsity_ratio",
    "feature_dim",
    "sample_count",
];

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ChartPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ChartScale {
    pub min: f64,
    pub max: f64,
    pub range: f64,
    #[serde(rename = "chartWidth")]
    pub chart_width: f64,
    #[serde(rename = "chartHeight")]
    pub chart_height: f64,
    pub padding: f64,
    pub points: Vec<ChartPoint>,
    pub polyline: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MlStepRow {
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    #[serde(rename = "pipelineStatus")]
    pub pipeline_status: String,
    #[serde(rename = "stepId")]
    pub step_id: String,
    #[serde(rename = "stepStatus")]
    pub step_status: String,
    #[serde(rename = "stepKind")]
    pub step_kind: String,
    pub output: Value,
}

/// Mirrors `poolaiParseMlNumeric(val)`.
pub fn parse_ml_numeric(value: Option<&str>) -> Option<f64> {
    let raw = value.filter(|s| !s.is_empty())?;
    let n: f64 = raw.parse().ok()?;
    if n.is_finite() {
        Some(n)
    } else {
        None
    }
}

fn value_to_display_string(v: &Value) -> Option<String> {
    if v.is_null() {
        return None;
    }
    if let Some(s) = v.as_str() {
        if s.is_empty() {
            return None;
        }
        return Some(s.to_string());
    }
    Some(v.to_string())
}

/// Mirrors `poolaiFormatMlMetricSummary(output)` — up to 4 `key=value` pairs.
pub fn format_ml_metric_summary(output: &Value) -> String {
    let Some(obj) = output.as_object() else {
        return "—".to_string();
    };
    let mut parts = Vec::new();
    for key in ML_METRIC_KEYS {
        if let Some(v) = obj.get(*key) {
            if let Some(display) = value_to_display_string(v) {
                parts.push(format!("{key}={display}"));
            }
        }
        if parts.len() >= 4 {
            break;
        }
    }
    if parts.is_empty() {
        if let Some(status) = obj.get("status").and_then(value_to_display_string) {
            return format!("status={status}");
        }
        return "—".to_string();
    }
    parts.join(", ")
}

/// Mirrors `poolaiMetricPointValues(data)`.
pub fn metric_point_values(data: &[Value]) -> Vec<f64> {
    data.iter()
        .map(|d| {
            d.get("value")
                .and_then(|v| {
                    if let Some(n) = v.as_f64() {
                        Some(n)
                    } else if let Some(s) = v.as_str() {
                        parse_ml_numeric(Some(s))
                    } else {
                        None
                    }
                })
                .unwrap_or(0.0)
        })
        .collect()
}

/// Mirrors `poolaiChartScale(values, width, height, padding)`.
pub fn chart_scale(values: &[f64], width: f64, height: f64, padding: f64) -> ChartScale {
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min = if min.is_finite() { min } else { 0.0 };
    let max = if max.is_finite() { max } else { 0.0 };
    let range = if (max - min).abs() < f64::EPSILON {
        1.0
    } else {
        max - min
    };
    let chart_width = width - padding * 2.0;
    let chart_height = height - padding * 2.0;
    let denom = (values.len().saturating_sub(1).max(1)) as f64;
    let points: Vec<ChartPoint> = values
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let x = padding + (i as f64 / denom) * chart_width;
            let y = padding + chart_height - ((*v - min) / range) * chart_height;
            ChartPoint { x, y }
        })
        .collect();
    let polyline = points
        .iter()
        .map(|p| format!("{},{}", p.x, p.y))
        .collect::<Vec<_>>()
        .join(" ");
    ChartScale {
        min,
        max,
        range,
        chart_width,
        chart_height,
        padding,
        points,
        polyline,
    }
}

/// Mirrors `poolaiFlattenMlStepRows(pipelines)`.
pub fn flatten_ml_step_rows(pipelines: &[Value]) -> Vec<MlStepRow> {
    let mut rows = Vec::new();
    for pipeline in pipelines {
        let Some(obj) = pipeline.as_object() else {
            continue;
        };
        let pipeline_id = obj
            .get("id")
            .and_then(value_to_display_string)
            .unwrap_or_default();
        let pipeline_name = obj
            .get("name")
            .and_then(value_to_display_string)
            .unwrap_or_else(|| {
                if pipeline_id.is_empty() {
                    "pipeline".to_string()
                } else {
                    pipeline_id.clone()
                }
            });
        let pipeline_status = obj
            .get("status")
            .and_then(value_to_display_string)
            .unwrap_or_default();
        let step_results = obj.get("step_results").and_then(|v| v.as_object());
        let Some(step_results) = step_results else {
            continue;
        };
        for (step_id, step_val) in step_results {
            let step_obj = step_val.as_object();
            let step_status = step_obj
                .and_then(|s| s.get("status"))
                .and_then(value_to_display_string)
                .unwrap_or_default();
            let output = step_obj
                .and_then(|s| s.get("output"))
                .cloned()
                .unwrap_or(Value::Object(Default::default()));
            let output_obj = output.as_object();
            let step_kind = output_obj
                .and_then(|o| o.get("step_kind").and_then(value_to_display_string))
                .or_else(|| {
                    output_obj.and_then(|o| o.get("step_id").and_then(value_to_display_string))
                })
                .unwrap_or_else(|| step_id.clone());
            rows.push(MlStepRow {
                pipeline_id: pipeline_id.clone(),
                pipeline_name: pipeline_name.clone(),
                pipeline_status: pipeline_status.clone(),
                step_id: step_id.clone(),
                step_status,
                step_kind,
                output,
            });
        }
    }
    rows
}

/// Mirrors `poolaiCollectMlSparklineSeries(rows)` — label → numeric series.
pub fn collect_ml_sparkline_series(rows: &[MlStepRow]) -> BTreeMap<String, Vec<f64>> {
    let mut series: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for row in rows {
        let kind = if row.step_kind.is_empty() {
            "step".to_string()
        } else {
            row.step_kind.clone()
        };
        let Some(output) = row.output.as_object() else {
            continue;
        };
        for key in ML_METRIC_KEYS {
            let raw = output.get(*key).and_then(value_to_display_string);
            let Some(raw) = raw else {
                continue;
            };
            let Some(n) = parse_ml_numeric(Some(&raw)) else {
                continue;
            };
            let label = format!("{kind} · {key}");
            series.entry(label).or_default().push(n);
        }
    }
    series
}

/// Mirrors `poolaiRenderSparkline(label, values, opts)` — compact sparkline card HTML (PH-S275).
pub fn render_sparkline_html(
    label: &str,
    values: &[f64],
    width: f64,
    height: f64,
    avg_label: &str,
) -> String {
    if values.is_empty() {
        return String::new();
    }
    let scale = chart_scale(values, width, height, 4.0);
    let avg = values.iter().sum::<f64>() / values.len() as f64;
    let w = width.round() as u32;
    let h = height.round() as u32;
    format!(
        concat!(
            r#"<div class="metric-sparkline-card">"#,
            r#"<div class="metric-sparkline-label">{label}</div>"#,
            r#"<svg width="{w}" height="{h}" class="metric-sparkline-svg" role="img" aria-label="{label}">"#,
            r#"<polyline points="{poly}" fill="none" stroke="var(--primary, #67e480)" stroke-width="1.5" />"#,
            r#"</svg>"#,
            r#"<div class="metric-sparkline-avg"><span class="metric-sparkline-avg-label">{avg_lbl}</span>"#,
            r#"<strong>{avg:.1}</strong></div></div>"#
        ),
        label = escape_html(label),
        w = w,
        h = h,
        poly = scale.polyline,
        avg_lbl = escape_html(avg_label),
        avg = avg
    )
}

/// Mirrors JS `poolaiSanitizeChartId` (PH-S297).
pub fn sanitize_chart_id(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Mirrors `poolaiRenderLineChart` empty state.
pub fn render_line_chart_empty_html(no_data_label: &str) -> String {
    format!(r#"<div class="muted">{}</div>"#, escape_html(no_data_label))
}

/// Mirrors `poolaiRenderLineChart(metricName, data, opts)` (PH-S284).
pub fn render_line_chart_html(
    metric_name: &str,
    values: &[f64],
    width: f64,
    height: f64,
    padding: f64,
    points_label: &str,
    stat_min_lbl: &str,
    stat_max_lbl: &str,
    stat_avg_lbl: &str,
) -> String {
    if values.is_empty() {
        return render_line_chart_empty_html("No data available");
    }
    let scale = chart_scale(values, width, height, padding);
    let avg = values.iter().sum::<f64>() / values.len() as f64;
    let grad_id = format!("grad-{}", sanitize_chart_id(metric_name));
    let w = width.round() as u32;
    let h = height.round() as u32;
    let circles: String = scale
        .points
        .iter()
        .map(|p| {
            format!(
                r#"<circle cx="{:.1}" cy="{:.1}" r="3" fill="var(--primary, #67e480)" />"#,
                p.x, p.y
            )
        })
        .collect();
    format!(
        concat!(
            r#"<div class="metric-chart-container">"#,
            r#"<h4>{title}</h4>"#,
            r#"<svg width="{w}" height="{h}" class="metric-chart-svg" role="img" aria-label="{title}">"#,
            r#"<defs><linearGradient id="{grad}" x1="0%" y1="0%" x2="0%" y2="100%">"#,
            r#"<stop offset="0%" style="stop-color:var(--primary, #67e480);stop-opacity:0.3" />"#,
            r#"<stop offset="100%" style="stop-color:var(--primary, #67e480);stop-opacity:0.05" />"#,
            r#"</linearGradient></defs>"#,
            r#"<rect x="{pad:.0}" y="{pad:.0}" width="{cw:.0}" height="{ch:.0}" fill="url(#{grad})" />"#,
            r#"<polyline points="{poly}" fill="none" stroke="var(--primary, #67e480)" stroke-width="2" />"#,
            r#"{circles}"#,
            r#"<text x="{pad:.0}" y="{ymax:.0}" fill="var(--text, #f8f8f2)" font-size="12">{maxv:.1}</text>"#,
            r#"<text x="{pad:.0}" y="{ymin:.0}" fill="var(--text, #f8f8f2)" font-size="12">{minv:.1}</text>"#,
            r#"<text x="{xend:.0}" y="{ymin:.0}" fill="var(--text-muted, #a8b0bf)" font-size="10" text-anchor="end">{pts}</text>"#,
            r#"</svg>"#,
            r#"<div class="metric-stats">"#,
            r#"<span>{stat_min} <strong>{minv2:.2}</strong></span>"#,
            r#"<span>{stat_max} <strong>{maxv2:.2}</strong></span>"#,
            r#"<span>{stat_avg} <strong>{avg:.2}</strong></span>"#,
            r#"</div></div>"#
        ),
        title = escape_html(metric_name),
        w = w,
        h = h,
        grad = grad_id,
        pad = scale.padding,
        cw = scale.chart_width,
        ch = scale.chart_height,
        poly = scale.polyline,
        circles = circles,
        ymax = scale.padding - 10.0,
        ymin = height - scale.padding + 20.0,
        maxv = scale.max,
        minv = scale.min,
        xend = width - scale.padding,
        pts = escape_html(points_label),
        stat_min = escape_html(stat_min_lbl),
        stat_max = escape_html(stat_max_lbl),
        stat_avg = escape_html(stat_avg_lbl),
        minv2 = scale.min,
        maxv2 = scale.max,
        avg = avg
    )
}

/// Mirrors `poolaiGroupMetricsByName(metrics)` (PH-S287).
pub fn group_metrics_by_name(data: &[Value]) -> BTreeMap<String, Vec<Value>> {
    let mut by: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for m in data {
        let key = m
            .get("metric")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        by.entry(key).or_default().push(m.clone());
    }
    by
}

fn encode_uri_component(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for b in raw.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Mirrors `poolaiFetchMetricHistory` URL (PH-S314).
pub fn build_metric_history_url(
    metric_name: &str,
    start_time: &str,
    end_time: &str,
    limit: u32,
) -> String {
    format!(
        "/api/enterprise/monitoring/metrics?metric={}&start_time={}&end_time={}&limit={limit}",
        encode_uri_component(metric_name),
        encode_uri_component(start_time),
        encode_uri_component(end_time),
    )
}

/// Mirrors `poolaiFetchMetricsWindow` URL (PH-S317).
pub fn build_metrics_window_url(start_time: &str, end_time: &str, limit: u32) -> String {
    format!(
        "/api/enterprise/monitoring/metrics?start_time={}&end_time={}&limit={limit}",
        encode_uri_component(start_time),
        encode_uri_component(end_time),
    )
}

/// Mirrors `poolaiRenderMetricsChartGrid` card wrapper (PH-S294).
pub fn render_metrics_chart_grid_html(title: &str, parts: &[String]) -> String {
    if parts.is_empty() {
        return String::new();
    }
    format!(
        r#"<div class="admin-card"><h3>{}</h3><div class="metrics-charts-grid">{}</div></div>"#,
        escape_html(title),
        parts.join("")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_ml_numeric_cases() {
        assert_eq!(parse_ml_numeric(None), None);
        assert_eq!(parse_ml_numeric(Some("")), None);
        assert_eq!(parse_ml_numeric(Some("0.95")), Some(0.95));
        assert_eq!(parse_ml_numeric(Some("nan")), None);
    }

    #[test]
    fn format_ml_metric_summary_limits_four() {
        let output = json!({
            "latency_ms": 12,
            "memory_mb": 64,
            "flops": 1000,
            "accuracy": 0.9,
            "final_loss": 0.1
        });
        let s = format_ml_metric_summary(&output);
        assert_eq!(s.matches(',').count(), 3);
        assert!(s.contains("latency_ms=12"));
        assert!(!s.contains("final_loss"));
    }

    #[test]
    fn format_ml_metric_summary_status_fallback() {
        let output = json!({ "status": "ok" });
        assert_eq!(format_ml_metric_summary(&output), "status=ok");
        assert_eq!(format_ml_metric_summary(&json!(null)), "—");
    }

    #[test]
    fn metric_point_values_extracts_values() {
        let data = vec![json!({"value": 1.5}), json!({"value": null}), json!({})];
        assert_eq!(metric_point_values(&data), vec![1.5, 0.0, 0.0]);
    }

    #[test]
    fn chart_scale_polyline_matches_js_shape() {
        let scale = chart_scale(&[1.0, 3.0, 2.0], 100.0, 50.0, 4.0);
        assert_eq!(scale.points.len(), 3);
        assert!(scale.polyline.contains(','));
        assert!(scale.max >= scale.min);
    }

    #[test]
    fn flatten_ml_step_rows_shape() {
        let pipelines = vec![json!({
            "id": "p1",
            "name": "demo",
            "status": "running",
            "step_results": {
                "train": {
                    "status": "completed",
                    "output": { "step_kind": "train", "accuracy": "0.91" }
                }
            }
        })];
        let rows = flatten_ml_step_rows(&pipelines);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].pipeline_id, "p1");
        assert_eq!(rows[0].step_kind, "train");
    }

    #[test]
    fn collect_ml_sparkline_series_labels() {
        let rows = flatten_ml_step_rows(&[json!({
            "id": "p1",
            "step_results": {
                "s1": {
                    "status": "completed",
                    "output": { "step_kind": "train", "accuracy": "0.9", "latency_ms": "12" }
                }
            }
        })]);
        let series = collect_ml_sparkline_series(&rows);
        assert!(series.contains_key("train · accuracy"));
        assert!(series.contains_key("train · latency_ms"));
    }

    #[test]
    fn render_sparkline_html_ph_s275() {
        let html = render_sparkline_html("cpu", &[1.0, 3.0, 2.0], 200.0, 40.0, "Avg: ");
        assert!(html.contains("metric-sparkline-card"));
        assert!(html.contains("metric-sparkline-svg"));
        assert!(html.contains("<polyline"));
        assert!(html.contains("Avg:"));
    }

    #[test]
    fn render_line_chart_html_ph_s284() {
        let html = render_line_chart_html(
            "cpu",
            &[1.0, 3.0, 2.0],
            600.0,
            200.0,
            40.0,
            "3 points",
            "Min:",
            "Max:",
            "Avg:",
        );
        assert!(html.contains("metric-chart-container"));
        assert!(html.contains("metric-chart-svg"));
        assert!(html.contains("<polyline"));
    }

    #[test]
    fn group_metrics_by_name_ph_s287() {
        let grouped = group_metrics_by_name(&[
            json!({"metric": "a", "value": 1}),
            json!({"metric": "b", "value": 2}),
            json!({"metric": "a", "value": 3}),
        ]);
        assert_eq!(grouped.get("a").map(|v| v.len()), Some(2));
        assert_eq!(grouped.get("b").map(|v| v.len()), Some(1));
    }

    #[test]
    fn render_line_chart_empty_html_ph_s304() {
        let html = render_line_chart_empty_html("No data");
        assert!(html.contains("muted"));
        assert!(html.contains("No data"));
    }

    #[test]
    fn build_metric_history_url_ph_s314() {
        let url = build_metric_history_url(
            "cpu.usage",
            "2026-01-01T00:00:00.000Z",
            "2026-01-02T00:00:00.000Z",
            200,
        );
        assert!(url.contains("cpu%2Eusage"));
        assert!(url.contains("start_time="));
        assert!(url.contains("limit=200"));
    }

    #[test]
    fn build_metrics_window_url_ph_s317() {
        let url = build_metrics_window_url("2026-01-01T00:00:00.000Z", "2026-01-02T00:00:00.000Z", 60);
        assert!(url.starts_with("/api/enterprise/monitoring/metrics?"));
        assert!(!url.contains("metric="));
        assert!(url.contains("limit=60"));
    }

    #[test]
    fn sanitize_chart_id_ph_s297() {
        assert_eq!(sanitize_chart_id("cpu.usage"), "cpu_usage");
        assert_eq!(sanitize_chart_id("a-b_c"), "a-b_c");
    }

    #[test]
    fn render_metrics_chart_grid_html_ph_s294() {
        let html = render_metrics_chart_grid_html(
            "Metrics",
            &["<div class=\"metric-chart-container\">x</div>".into()],
        );
        assert!(html.contains("admin-card"));
        assert!(html.contains("metrics-charts-grid"));
        assert!(html.contains("metric-chart-container"));
        assert!(render_metrics_chart_grid_html("Empty", &[]).is_empty());
    }
}
