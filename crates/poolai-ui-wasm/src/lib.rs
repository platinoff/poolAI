//! wasm32 exports for admin grid-pricing panel helpers (PH-S147).
//!
//! Wraps [`poolai_ui_core`] formatters so the same logic runs in the browser via WASM.

use chrono::{DateTime, Utc};
use poolai_ui_core::lease::lease_state;
use poolai_ui_core::pricing::{format_unix_secs, format_usd_micro};
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
///
/// `now_rfc3339` should come from `new Date().toISOString()` in the browser.
#[wasm_bindgen(js_name = leaseStateLabel)]
pub fn lease_state_label_wasm(expires_at: &str, now_rfc3339: &str) -> String {
    let now = parse_rfc3339_utc(now_rfc3339).unwrap_or_else(fallback_now_utc);
    let state = lease_state(empty_as_none(expires_at), now);
    state.as_str().to_string()
}

/// POC version string for smoke checks in browser devtools.
#[wasm_bindgen(js_name = poolaiUiWasmVersion)]
pub fn poolai_ui_wasm_version() -> String {
    "poolai-ui-wasm/0.1.0-ph-s147".to_string()
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
    }

    #[test]
    fn lease_state_label_active() {
        let now = Utc.with_ymd_and_hms(2026, 6, 13, 12, 0, 0).unwrap();
        let label = lease_state_label_wasm("2026-06-13T13:00:00Z", &now.to_rfc3339());
        assert_eq!(label, LeaseDisplayState::Active.as_str());
    }
}
