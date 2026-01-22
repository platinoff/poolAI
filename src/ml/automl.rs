//! AutoML Integration (Stage 4.4, ML.2)
//!
//! Planned: model selection, feature engineering, pipeline generation.
//! See `docs/development/FUTURE_DEVELOPMENT_ROADMAP.md`.

use serde::{Deserialize, Serialize};

/// AutoML config stub.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AutomlConfig {
    pub auto_feature_engineering: bool,
    pub max_trials: u32,
}

impl AutomlConfig {
    pub fn default_config() -> Self {
        Self {
            auto_feature_engineering: true,
            max_trials: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automl_config_default() {
        let c = AutomlConfig::default_config();
        assert!(c.auto_feature_engineering);
        assert_eq!(c.max_trials, 100);
    }
}
