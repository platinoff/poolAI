//! Grid pricing formatters — parity with `src/ui/admin/grid_pricing.rs` embedded JS.

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
}
