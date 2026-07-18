//! Grid metrics parity hardening band depth (PH-S1069…S1078, band 43).

use serde_json::Value;

/// Band-43 grid metrics parity hardening depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridMetricsParityDepth {
    None,
    VerificationExtended,
    ReplicationExtended,
    PricingExtended,
    PrefetchExtended,
    SettlementTrustExtended,
    StandSmokeV3,
    ParityContracts,
    DocsCanon,
    FullGridMetricsParity,
}

/// FM §5.24 band-43 marker rows.
pub const FM_BAND43_ROWS: &[&str] = &[
    "5.24",
    "Grid metrics parity",
    "PH-S1069…S1078",
    "validate_band6_metrics_parity_v3",
];

/// Grid metrics parity adoption markers for band 43.
pub const GRID_METRICS_PARITY_BAND43_ROWS: &[&str] = &[
    "PH-S1069",
    "VERIFICATION_EXTENDED_PARITY",
    "PH-S1070",
    "REPLICATION_EXTENDED_PARITY",
    "PH-S1073",
    "grid_metrics_json_prometheus_parity_band6_v3",
    "PH-S1078",
];

/// All grid `*-metrics` API paths covered by band 43 hardening.
pub const GRID_METRICS_API_PATHS: &[&str] = &[
    "/api/v1/grid/verification-metrics",
    "/api/v1/grid/replay-metrics",
    "/api/v1/grid/settlement-metrics",
    "/api/v1/grid/trust-metrics",
    "/api/v1/grid/replication-metrics",
    "/api/v1/grid/pricing-metrics",
    "/api/v1/grid/prefetch-metrics",
    "/api/v1/grid/locality-metrics",
    "/api/v1/grid/fee-split-metrics",
    "/api/v1/grid/governance-metrics",
    "/api/v1/grid/payout-batch-metrics",
];

pub fn grid_metrics_parity_depth_stub(features: Option<&Value>) -> GridMetricsParityDepth {
    let Some(f) = features else {
        return GridMetricsParityDepth::None;
    };
    let verification = f
        .get("verification_extended")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let replication = f
        .get("replication_extended")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let pricing = f
        .get("pricing_extended")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let prefetch = f
        .get("prefetch_extended")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let settlement_trust = f
        .get("settlement_trust_extended")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let smoke_v3 = f
        .get("stand_smoke_v3")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let contracts = f
        .get("parity_contracts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("docs_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let flags = [
        verification,
        replication,
        pricing,
        prefetch,
        settlement_trust,
        smoke_v3,
        contracts,
        docs,
    ];
    let count = flags.iter().filter(|&&b| b).count();
    match count {
        0 => GridMetricsParityDepth::None,
        8 => GridMetricsParityDepth::FullGridMetricsParity,
        _ if verification && !replication => GridMetricsParityDepth::VerificationExtended,
        _ if replication => GridMetricsParityDepth::ReplicationExtended,
        _ if pricing => GridMetricsParityDepth::PricingExtended,
        _ if prefetch => GridMetricsParityDepth::PrefetchExtended,
        _ if settlement_trust => GridMetricsParityDepth::SettlementTrustExtended,
        _ if smoke_v3 => GridMetricsParityDepth::StandSmokeV3,
        _ if contracts => GridMetricsParityDepth::ParityContracts,
        _ if docs => GridMetricsParityDepth::DocsCanon,
        _ => GridMetricsParityDepth::FullGridMetricsParity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn grid_metrics_parity_depth_stub_ph_s1077() {
        assert_eq!(
            grid_metrics_parity_depth_stub(None),
            GridMetricsParityDepth::None
        );
        assert_eq!(
            grid_metrics_parity_depth_stub(Some(&json!({"verification_extended": true}))),
            GridMetricsParityDepth::VerificationExtended
        );
        assert_eq!(
            grid_metrics_parity_depth_stub(Some(&json!({
                "verification_extended": true,
                "replication_extended": true,
                "pricing_extended": true,
                "prefetch_extended": true,
                "settlement_trust_extended": true,
                "stand_smoke_v3": true,
                "parity_contracts": true,
                "docs_canon": true
            }))),
            GridMetricsParityDepth::FullGridMetricsParity
        );
    }
}
