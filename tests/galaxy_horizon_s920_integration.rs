//! PH-S929: Galaxy horizon close band (PH-S920…S928) — admin charts sparkline/line wasm-only.

use poolai_ui_core::charts_depth::{charts_depth_stub, ChartsDepth};
use poolai_ui_core::grid_replication_pricing::{admin_wasm_slim_depth_stub, AdminWasmSlimDepth};
use poolai_ui_core::ml::{render_line_chart_html, render_sparkline_html};
use serde_json::json;

#[test]
fn horizon_s920_band_charts_wasm_only_ph_s929() {
    assert_eq!(
        charts_depth_stub(Some(&json!({"sparkline_wasm": true}))),
        ChartsDepth::SparklineHtml
    );
    assert_eq!(
        charts_depth_stub(Some(&json!({"line_chart_wasm": true}))),
        ChartsDepth::LineChartHtml
    );
    assert_eq!(
        charts_depth_stub(Some(&json!({
            "sparkline_wasm": true,
            "line_chart_wasm": true
        }))),
        ChartsDepth::SparklineLineChartHtml
    );
    assert_eq!(
        admin_wasm_slim_depth_stub(Some(&json!({"charts_glue": true}))),
        AdminWasmSlimDepth::ChartsGlue
    );

    let spark = render_sparkline_html("cpu", &[1.0, 2.0, 3.0], 200.0, 40.0, "Avg: ");
    assert!(spark.contains("metric-sparkline-card"));
    assert!(spark.contains("<polyline"));

    let line = render_line_chart_html(
        "latency_ms",
        &[10.0, 20.0, 15.0],
        600.0,
        200.0,
        40.0,
        "3 points",
        "Min:",
        "Max:",
        "Avg:",
    );
    assert!(line.contains("metric-chart-container"));
    assert!(line.contains("latency_ms"));

    let build_script = include_str!("../bin/build-ui-wasm.sh");
    assert!(build_script.contains("poolai-ui-wasm"));
}
