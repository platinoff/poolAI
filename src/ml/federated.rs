//! Federated Learning (Stage 4.4, ML.3)
//!
//! Planned: distributed training, gradient aggregation, privacy-preserving learning.
//! See `docs/development/FUTURE_DEVELOPMENT_ROADMAP.md`.

use serde::{Deserialize, Serialize};

/// Aggregation mode for federated rounds.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AggregationMode {
    #[default]
    FedAvg,
    FedProx,
}

/// Federated round config stub.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FederatedConfig {
    pub aggregation: AggregationMode,
    pub min_clients_per_round: u32,
}

impl FederatedConfig {
    pub fn default_config() -> Self {
        Self {
            aggregation: AggregationMode::FedAvg,
            min_clients_per_round: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn federated_config_default() {
        let c = FederatedConfig::default_config();
        assert_eq!(c.aggregation, AggregationMode::FedAvg);
        assert_eq!(c.min_clients_per_round, 2);
    }
}
