//! Operator / dev-stand helpers (PH-S1011…S1018 band 37).

pub mod last_run;
pub mod power;

/// Light compile profile — fewer features for faster local iteration (PH-S1011).
pub const LIGHT_FEATURES: &str = "enterprise,test-utils";

/// Full dev stand features (default `run-poolai build`).
pub const FULL_FEATURES: &str = "enterprise,ml,cloud,test-utils";
