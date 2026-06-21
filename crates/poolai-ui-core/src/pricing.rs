//! Grid pricing formatters — parity with `src/ui/admin/grid_pricing.rs` embedded JS.

use crate::format::escape_html;
use serde_json::Value;
use std::str::FromStr;

/// UX copy for secondary admin fee range (parity with `galaxy_fee_split::SECONDARY_FEE_UX_HINT`).
pub const SECONDARY_FEE_UX_HINT: &str =
    "Lower secondary fee (1–5%) improves market competitiveness; higher fee reduces worker payout.";

/// Galaxy pricing unit keys (wire + admin `<select>` options).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PricingUnitKey {
    InferenceInputToken,
    InferenceOutputToken,
    InferenceBlendedToken,
    GpuSecond,
    JobFlat,
}

impl PricingUnitKey {
    pub const ALL: [Self; 5] = [
        Self::InferenceInputToken,
        Self::InferenceOutputToken,
        Self::InferenceBlendedToken,
        Self::GpuSecond,
        Self::JobFlat,
    ];

    pub const DEFAULT: Self = Self::InferenceBlendedToken;

    pub fn as_str(self) -> &'static str {
        match self {
            Self::InferenceInputToken => "inference_input_token",
            Self::InferenceOutputToken => "inference_output_token",
            Self::InferenceBlendedToken => "inference_blended_token",
            Self::GpuSecond => "gpu_second",
            Self::JobFlat => "job_flat",
        }
    }
}

impl FromStr for PricingUnitKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "inference_input_token" => Ok(Self::InferenceInputToken),
            "inference_output_token" => Ok(Self::InferenceOutputToken),
            "inference_blended_token" => Ok(Self::InferenceBlendedToken),
            "gpu_second" => Ok(Self::GpuSecond),
            "job_flat" => Ok(Self::JobFlat),
            _ => Err(()),
        }
    }
}

/// Admin grid-pricing `<select>` option strings (subset shown in UI).
pub const GRID_PRICING_UNIT_OPTIONS: &[&str] = &[
    "inference_blended_token",
    "inference_input_token",
    "inference_output_token",
];

/// Resolve unit key from form value; default matches JS `|| 'inference_blended_token'`.
pub fn resolve_unit_key(raw: Option<&str>) -> PricingUnitKey {
    raw.and_then(|s| PricingUnitKey::from_str(s).ok())
        .unwrap_or(PricingUnitKey::DEFAULT)
}

/// Mirrors `formatUsdMicro(usdMicro)`.
pub fn format_usd_micro(usd_micro: Option<f64>) -> String {
    let Some(n) = usd_micro.filter(|v| v.is_finite()) else {
        return "—".to_string();
    };
    format!("{:.6} USD", n / 1_000_000.0)
}

/// Mirrors `formatUnixSecs(secs)`.
pub fn format_unix_secs(secs: Option<f64>) -> String {
    let Some(n) = secs.filter(|v| v.is_finite() && *v > 0.0) else {
        return "—".to_string();
    };
    let secs_i64 = n.trunc() as i64;
    match chrono::DateTime::from_timestamp(secs_i64, 0) {
        Some(dt) => dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        None => secs_i64.to_string(),
    }
}

fn t(i18n: &Value, key: &str, fallback: &str) -> String {
    i18n.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string()
}

/// L1 cache TTL metadata strip for grid-pricing admin (PH-S902).
pub fn render_grid_pricing_freshness_strip_html(
    pricing_response_json: &str,
    i18n_json: &str,
) -> String {
    let data: Value = serde_json::from_str(pricing_response_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);
    let freshness = data
        .get("freshness")
        .and_then(|v| v.as_str())
        .unwrap_or("—");
    let source = data.get("source").and_then(|v| v.as_str()).unwrap_or("—");
    let l1 = data.get("l1_cache").cloned().unwrap_or(Value::Null);
    let cache_age = l1
        .get("cache_age_secs")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string())
        .unwrap_or_else(|| "—".to_string());
    let ttl = l1
        .get("cache_ttl_secs")
        .and_then(|v| v.as_u64())
        .map(|n| n.to_string())
        .unwrap_or_else(|| "—".to_string());
    let fresh_until = l1
        .get("cache_fresh_until_secs")
        .and_then(|v| v.as_u64())
        .map(|n| format_unix_secs(Some(n as f64)))
        .unwrap_or_else(|| "—".to_string());
    let stale_until = l1
        .get("cache_stale_until_secs")
        .and_then(|v| v.as_u64())
        .map(|n| format_unix_secs(Some(n as f64)))
        .unwrap_or_else(|| "—".to_string());

    format!(
        r#"<div class="admin-card admin-metrics-strip grid-pricing-freshness-strip">
<span>{fresh_lbl}: <strong>{freshness}</strong></span>
<span>{source_lbl}: <strong>{source}</strong></span>
<span>{age_lbl}: <strong>{cache_age}</strong></span>
<span>{ttl_lbl}: <strong>{ttl}</strong></span>
<span>{fresh_until_lbl}: <strong>{fresh_until}</strong></span>
<span>{stale_until_lbl}: <strong>{stale_until}</strong></span>
</div>"#,
        fresh_lbl = escape_html(&t(&i18n, "admin.gridPricing.col.freshness", "Freshness")),
        freshness = escape_html(freshness),
        source_lbl = escape_html(&t(&i18n, "admin.gridPricing.col.source", "Source")),
        source = escape_html(source),
        age_lbl = escape_html(&t(&i18n, "admin.gridPricing.col.cacheAge", "Cache age (s)")),
        cache_age = escape_html(&cache_age),
        ttl_lbl = escape_html(&t(&i18n, "admin.gridPricing.col.cacheTtl", "Cache TTL (s)")),
        ttl = escape_html(&ttl),
        fresh_until_lbl = escape_html(&t(&i18n, "admin.gridPricing.col.freshUntil", "Fresh until")),
        fresh_until = escape_html(&fresh_until),
        stale_until_lbl = escape_html(&t(&i18n, "admin.gridPricing.col.staleUntil", "Stale until")),
        stale_until = escape_html(&stale_until),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_usd_micro_values() {
        assert_eq!(format_usd_micro(None), "—");
        assert_eq!(format_usd_micro(Some(f64::NAN)), "—");
        assert_eq!(format_usd_micro(Some(450_000.0)), "0.450000 USD");
        assert_eq!(format_usd_micro(Some(1_000_000.0)), "1.000000 USD");
    }

    #[test]
    fn format_unix_secs_values() {
        assert_eq!(format_unix_secs(None), "—");
        assert_eq!(format_unix_secs(Some(0.0)), "—");
        assert_eq!(format_unix_secs(Some(-1.0)), "—");
        assert_eq!(
            format_unix_secs(Some(1_718_280_000.0)),
            "2024-06-13T12:00:00Z"
        );
    }

    #[test]
    fn unit_key_default_and_parse() {
        assert_eq!(
            resolve_unit_key(None),
            PricingUnitKey::InferenceBlendedToken
        );
        assert_eq!(
            resolve_unit_key(Some("gpu_second")),
            PricingUnitKey::GpuSecond
        );
        assert_eq!(
            resolve_unit_key(Some("unknown")),
            PricingUnitKey::InferenceBlendedToken
        );
    }

    #[test]
    fn render_grid_pricing_freshness_strip_ph_s902() {
        let html = render_grid_pricing_freshness_strip_html(
            r#"{"freshness":"fresh","source":"cache","l1_cache":{"cache_age_secs":12,"cache_ttl_secs":300,"cache_fresh_until_secs":1718280300,"cache_stale_until_secs":1718283600}}"#,
            r#"{"admin.gridPricing.col.freshness":"Freshness"}"#,
        );
        assert!(html.contains("grid-pricing-freshness-strip"));
        assert!(html.contains("<strong>fresh</strong>"));
        assert!(html.contains("<strong>cache</strong>"));
        assert!(html.contains("<strong>12</strong>"));
        assert!(html.contains("<strong>300</strong>"));
    }
}
