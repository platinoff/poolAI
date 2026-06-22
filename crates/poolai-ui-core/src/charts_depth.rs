//! Admin charts wasm depth classification (PH-S924, band 27).

use serde_json::Value;

/// Admin charts wasm renderer depth (sparkline / line chart band 27).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartsDepth {
    None,
    SparklineHtml,
    LineChartHtml,
    SparklineLineChartHtml,
}

/// Classify charts wasm depth from optional feature stub (PH-S924).
pub fn charts_depth_stub(features: Option<&Value>) -> ChartsDepth {
    let Some(f) = features else {
        return ChartsDepth::None;
    };
    let spark = f
        .get("sparkline_wasm")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let line = f
        .get("line_chart_wasm")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    match (spark, line) {
        (true, true) => ChartsDepth::SparklineLineChartHtml,
        (true, false) => ChartsDepth::SparklineHtml,
        (false, true) => ChartsDepth::LineChartHtml,
        (false, false) => ChartsDepth::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn charts_depth_stub_ph_s924() {
        assert_eq!(charts_depth_stub(None), ChartsDepth::None);
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
    }
}
