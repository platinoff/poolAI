//! Galaxy fee split depth classification stub (PH-S783, §1.2).

use crate::grid::galaxy_fee_split_metrics::FeeSplitMetricsSnapshot;

/// Fee split wire depth (Galaxy §1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeeSplitDepth {
    None,
    Configured,
    Applied,
    FullDepth,
}

/// Classify fee split depth from optional metrics snapshot (PH-S783).
pub fn galaxy_fee_split_depth_stub(snapshot: Option<&FeeSplitMetricsSnapshot>) -> FeeSplitDepth {
    let Some(s) = snapshot else {
        return FeeSplitDepth::None;
    };
    if s.fee_split_applied_total > 0
        && s.primary_dev_fee_bps == crate::grid::galaxy_fee_split::PRIMARY_DEV_FEE_BPS
        && s.secondary_admin_fee_min_bps
            == crate::grid::galaxy_fee_split::SECONDARY_ADMIN_FEE_MIN_BPS
        && s.secondary_admin_fee_max_bps
            == crate::grid::galaxy_fee_split::SECONDARY_ADMIN_FEE_MAX_BPS
    {
        FeeSplitDepth::FullDepth
    } else if s.fee_split_applied_total > 0 {
        FeeSplitDepth::Applied
    } else if s.primary_dev_fee_bps > 0 {
        FeeSplitDepth::Configured
    } else {
        FeeSplitDepth::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::galaxy_fee_split::{
        PRIMARY_DEV_FEE_BPS, SECONDARY_ADMIN_FEE_MAX_BPS, SECONDARY_ADMIN_FEE_MIN_BPS,
    };
    use crate::grid::galaxy_fee_split_metrics::{
        fee_split_metrics_snapshot, record_fee_split_applied, reset_fee_split_metrics_for_test,
    };

    #[test]
    fn galaxy_fee_split_depth_stub_ph_s783() {
        let _lock = crate::grid::galaxy_fee_split_metrics::fee_split_metrics_test_lock();
        reset_fee_split_metrics_for_test();

        assert_eq!(galaxy_fee_split_depth_stub(None), FeeSplitDepth::None);

        let configured = fee_split_metrics_snapshot();
        assert_eq!(
            galaxy_fee_split_depth_stub(Some(&configured)),
            FeeSplitDepth::Configured
        );

        record_fee_split_applied();
        let applied = fee_split_metrics_snapshot();
        assert_eq!(
            galaxy_fee_split_depth_stub(Some(&applied)),
            FeeSplitDepth::FullDepth
        );
        assert_eq!(applied.primary_dev_fee_bps, PRIMARY_DEV_FEE_BPS);
        assert_eq!(
            applied.secondary_admin_fee_min_bps,
            SECONDARY_ADMIN_FEE_MIN_BPS
        );
        assert_eq!(
            applied.secondary_admin_fee_max_bps,
            SECONDARY_ADMIN_FEE_MAX_BPS
        );

        reset_fee_split_metrics_for_test();
    }
}
