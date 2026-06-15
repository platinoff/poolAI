//! Serve built admin WASM artifacts from `src/ui/wasm/` (PH-S151).
//!
//! Artifacts are produced by [`bin/build-ui-wasm.sh`](../../bin/build-ui-wasm.sh); 404 when missing is OK (JS fallback).

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::{Path, PathBuf};

use crate::core::state::ApiContext;

/// Shared ES module bootstrap for `poolai-ui-wasm` (PH-S152/S153/S155/S193).
pub const POOLAI_UI_WASM_MODULE: &str = r#"
import init, {
  formatUsdMicro, formatUnixSecs, leaseStateLabel, compatStatusLabel, protocolVersionLabel,
  poolaiUiWasmVersion,
  escapeHtml, escapeRegex, formatIsoDatetime, formatLocaleTimeHms,
  apiErrorMessageFromBody, apiErrorDetailFromBody, formatFetchError,
  emptyStateHtml, renderTableHtml, formFieldHtml, buildTableCsv, buildTableJson,
  compareSortValues, rowMatchesQuery, highlightQueryHtml,
  parseMlNumeric, formatMlMetricSummary, metricPointValues, chartScale,
  flattenMlStepRows, collectMlSparklineSeries, normalizeTheme,
  trapTabAction, modalFocusableSelector, adminDynamicModalHtml,
} from '/ui/wasm/poolai_ui_wasm.js';
window.poolaiUiWasm = {
  ready: false, failed: false,
  formatUsdMicro, formatUnixSecs, leaseStateLabel, compatStatusLabel, protocolVersionLabel,
  escapeHtml, escapeRegex, formatIsoDatetime, formatLocaleTimeHms,
  apiErrorMessageFromBody, apiErrorDetailFromBody, formatFetchError,
  emptyStateHtml, renderTableHtml, formFieldHtml, buildTableCsv, buildTableJson,
  compareSortValues, rowMatchesQuery, highlightQueryHtml,
  parseMlNumeric, formatMlMetricSummary, metricPointValues, chartScale,
  flattenMlStepRows, collectMlSparklineSeries, normalizeTheme,
  trapTabAction, modalFocusableSelector, adminDynamicModalHtml,
};
try {
  await init();
  window.poolaiUiWasm.ready = true;
  document.documentElement.dataset.poolaiUiWasm = poolaiUiWasmVersion();
} catch (err) {
  window.poolaiUiWasm.failed = true;
  console.warn('poolai-ui-wasm init failed', err);
}
window.dispatchEvent(new Event('poolai-ui-wasm-ready'));
"#;

pub fn ui_wasm_routes() -> Router<ApiContext> {
    Router::new()
        .route("/wasm/poolai_ui_wasm.js", get(serve_poolai_ui_wasm_js))
        .route("/wasm/poolai_ui_wasm_bg.wasm", get(serve_poolai_ui_wasm_bg))
}

fn ui_wasm_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ui/wasm")
}

async fn serve_poolai_ui_wasm_js() -> Response {
    serve_ui_wasm_file("poolai_ui_wasm.js", "application/javascript; charset=utf-8").await
}

async fn serve_poolai_ui_wasm_bg() -> Response {
    serve_ui_wasm_file("poolai_ui_wasm_bg.wasm", "application/wasm").await
}

async fn serve_ui_wasm_file(name: &str, content_type: &str) -> Response {
    match read_ui_wasm_file(name).await {
        Ok(bytes) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            bytes,
        )
            .into_response(),
        Err(status) => status.into_response(),
    }
}

async fn read_ui_wasm_file(name: &str) -> Result<Vec<u8>, StatusCode> {
    let path = ui_wasm_dir().join(name);
    if !is_safe_wasm_filename(name) {
        return Err(StatusCode::NOT_FOUND);
    }
    tokio::fs::read(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
}

fn is_safe_wasm_filename(name: &str) -> bool {
    name == "poolai_ui_wasm.js" || name == "poolai_ui_wasm_bg.wasm"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_dir_under_manifest() {
        let dir = ui_wasm_dir();
        assert!(dir.ends_with("src/ui/wasm"));
        assert!(dir.starts_with(Path::new(env!("CARGO_MANIFEST_DIR"))));
    }

    #[test]
    fn safe_wasm_filenames() {
        assert!(is_safe_wasm_filename("poolai_ui_wasm.js"));
        assert!(is_safe_wasm_filename("poolai_ui_wasm_bg.wasm"));
        assert!(!is_safe_wasm_filename("../Cargo.toml"));
    }
}
