//! Galaxy Grid fee split (PH-S58): **primary 0.1%** (dev) + **secondary 1–5%** (admin),
//! remainder to worker/operator pool. Integer math on gross payment in **atomic units**
//! (e.g. SOL lamports). See `docs/concept/POOLAI_GALAXY_GRID.md`.

use thiserror::Error;

/// Basis-points denominator (100% = 10_000 bps).
pub const BPS_DENOMINATOR: u32 = 10_000;

/// Fixed primary dev fee: **0.1%** = 10 bps.
pub const PRIMARY_DEV_FEE_BPS: u16 = 10;

/// Minimum secondary admin fee: **1%** = 100 bps.
pub const SECONDARY_ADMIN_FEE_MIN_BPS: u16 = 100;

/// Maximum secondary admin fee: **5%** = 500 bps.
pub const SECONDARY_ADMIN_FEE_MAX_BPS: u16 = 500;

/// UX copy: lower secondary improves competitiveness (Galaxy Grid concept).
pub const SECONDARY_FEE_UX_HINT: &str =
    "Lower secondary fee (1–5%) improves market competitiveness; higher fee reduces worker payout.";

/// Split result: all amounts in the same atomic unit as gross input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GalaxyFeeSplit {
    /// Primary dev wallet (0.1% of gross, floor).
    pub primary_dev_lamports: u64,
    /// Admin srvN secondary fee (1–5% of gross, floor; caller-validated bps).
    pub secondary_admin_lamports: u64,
    /// Remainder after fees: Telegram edge worker share, or local operator pool when no Telegram edge.
    pub worker_or_operator_pool_lamports: u64,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GalaxyFeeSplitError {
    #[error("secondary admin fee must be {min}..={max} bps, got {got}")]
    SecondaryFeeBpsOutOfRange { min: u16, max: u16, got: u16 },

    #[error("fee amounts overflow gross payment")]
    FeeExceedsGross,
}

/// `gross * bps / BPS_DENOMINATOR` rounded **down** (floor). Result ≤ `gross` for `bps ≤ 10_000`.
#[inline]
pub fn mul_bps_floor_u64(gross: u64, bps: u16) -> u64 {
    let numer = gross as u128 * u128::from(bps);
    let out = numer / u128::from(BPS_DENOMINATOR);
    // For bps ≤ 10_000, out ≤ gross; fits u64 when gross is u64.
    out as u64
}

/// Split `gross_lamports` into primary (0.1%), secondary admin (`secondary_admin_bps` in 100..=500),
/// and remainder to worker/operator pool.
pub fn split_gross_payment(
    gross_lamports: u64,
    secondary_admin_bps: u16,
) -> Result<GalaxyFeeSplit, GalaxyFeeSplitError> {
    if !(SECONDARY_ADMIN_FEE_MIN_BPS..=SECONDARY_ADMIN_FEE_MAX_BPS).contains(&secondary_admin_bps) {
        return Err(GalaxyFeeSplitError::SecondaryFeeBpsOutOfRange {
            min: SECONDARY_ADMIN_FEE_MIN_BPS,
            max: SECONDARY_ADMIN_FEE_MAX_BPS,
            got: secondary_admin_bps,
        });
    }

    let primary_dev_lamports = mul_bps_floor_u64(gross_lamports, PRIMARY_DEV_FEE_BPS);
    let secondary_admin_lamports = mul_bps_floor_u64(gross_lamports, secondary_admin_bps);

    let worker_or_operator_pool_lamports = gross_lamports
        .checked_sub(primary_dev_lamports)
        .and_then(|x| x.checked_sub(secondary_admin_lamports))
        .ok_or(GalaxyFeeSplitError::FeeExceedsGross)?;

    Ok(GalaxyFeeSplit {
        primary_dev_lamports,
        secondary_admin_lamports,
        worker_or_operator_pool_lamports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_sol_secondary_one_percent() {
        let gross = 1_000_000_000u64;
        let s = split_gross_payment(gross, 100).unwrap();
        assert_eq!(s.primary_dev_lamports, 1_000_000);
        assert_eq!(s.secondary_admin_lamports, 10_000_000);
        assert_eq!(s.worker_or_operator_pool_lamports, 989_000_000);
        assert_eq!(
            s.primary_dev_lamports
                + s.secondary_admin_lamports
                + s.worker_or_operator_pool_lamports,
            gross
        );
    }

    #[test]
    fn one_sol_secondary_five_percent() {
        let gross = 1_000_000_000u64;
        let s = split_gross_payment(gross, 500).unwrap();
        assert_eq!(s.primary_dev_lamports, 1_000_000);
        assert_eq!(s.secondary_admin_lamports, 50_000_000);
        assert_eq!(s.worker_or_operator_pool_lamports, 949_000_000);
    }

    #[test]
    fn secondary_min_boundary() {
        let s = split_gross_payment(10_000, SECONDARY_ADMIN_FEE_MIN_BPS).unwrap();
        assert_eq!(s.primary_dev_lamports, 10);
        assert_eq!(s.secondary_admin_lamports, 100);
        assert_eq!(s.worker_or_operator_pool_lamports, 9890);
    }

    #[test]
    fn secondary_max_boundary() {
        let s = split_gross_payment(10_000, SECONDARY_ADMIN_FEE_MAX_BPS).unwrap();
        assert_eq!(s.primary_dev_lamports, 10);
        assert_eq!(s.secondary_admin_lamports, 500);
        assert_eq!(s.worker_or_operator_pool_lamports, 9490);
    }

    #[test]
    fn rejects_secondary_below_min() {
        assert_eq!(
            split_gross_payment(1_000_000, 99),
            Err(GalaxyFeeSplitError::SecondaryFeeBpsOutOfRange {
                min: SECONDARY_ADMIN_FEE_MIN_BPS,
                max: SECONDARY_ADMIN_FEE_MAX_BPS,
                got: 99,
            })
        );
    }

    #[test]
    fn rejects_secondary_above_max() {
        assert_eq!(
            split_gross_payment(1_000_000, 501),
            Err(GalaxyFeeSplitError::SecondaryFeeBpsOutOfRange {
                min: SECONDARY_ADMIN_FEE_MIN_BPS,
                max: SECONDARY_ADMIN_FEE_MAX_BPS,
                got: 501,
            })
        );
    }

    #[test]
    fn zero_gross() {
        let s = split_gross_payment(0, 100).unwrap();
        assert_eq!(s.primary_dev_lamports, 0);
        assert_eq!(s.secondary_admin_lamports, 0);
        assert_eq!(s.worker_or_operator_pool_lamports, 0);
    }

    #[test]
    fn mul_bps_floor_small_gross() {
        assert_eq!(mul_bps_floor_u64(99, PRIMARY_DEV_FEE_BPS), 0);
        assert_eq!(mul_bps_floor_u64(10_000, PRIMARY_DEV_FEE_BPS), 10);
    }
}
