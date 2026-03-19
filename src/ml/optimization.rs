//! Model Optimization (Stage 4.4, ML.1)
//!
//! Profiling, hyperparameter tuning, quantization, pruning.
//! See `docs/development/FUTURE_DEVELOPMENT_ROADMAP.md`.

use serde::{Deserialize, Serialize};

/// Quantization level for model compression.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum QuantizationLevel {
    #[default]
    None,
    Int8,
    Int4,
}

/// Optimization profile (tuning + quantization + pruning).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OptimizationProfile {
    pub quantization: QuantizationLevel,
    pub pruning_ratio: f32,
}

impl OptimizationProfile {
    pub fn default_fast() -> Self {
        Self {
            quantization: QuantizationLevel::Int8,
            pruning_ratio: 0.1,
        }
    }

    pub fn default_balanced() -> Self {
        Self {
            quantization: QuantizationLevel::None,
            pruning_ratio: 0.0,
        }
    }
}

/// Model performance profile (ML.1 profiling).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModelProfile {
    /// Inference latency in milliseconds (placeholder).
    pub latency_ms: f64,
    /// Peak memory in MB (placeholder).
    pub memory_mb: f64,
    /// FLOPs estimate (placeholder).
    pub flops: u64,
}

/// Stub: profile model and return placeholder metrics.
pub fn profile_model() -> ModelProfile {
    ModelProfile {
        latency_ms: 12.5,
        memory_mb: 256.0,
        flops: 1_000_000_000,
    }
}

/// Hyperparameter tuning config (ML.1 tuning).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TuningConfig {
    pub learning_rate_min: f64,
    pub learning_rate_max: f64,
    pub batch_size_candidates: Vec<u32>,
}

impl TuningConfig {
    pub fn default_config() -> Self {
        Self {
            learning_rate_min: 1e-5,
            learning_rate_max: 1e-2,
            batch_size_candidates: vec![8, 16, 32, 64],
        }
    }
}

/// Result of a tuning suggestion (stub).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TuningResult {
    pub learning_rate: f64,
    pub batch_size: u32,
    pub suggested_epochs: u32,
}

/// Stub: suggest next hyperparameters.
pub fn suggest_hyperparams(_config: &TuningConfig) -> TuningResult {
    TuningResult {
        learning_rate: 1e-3,
        batch_size: 32,
        suggested_epochs: 10,
    }
}

/// Result of applying quantization (stub).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct QuantizationResult {
    pub level: QuantizationLevel,
    pub size_mb_before: f64,
    pub size_mb_after: f64,
    pub compression_ratio: f64,
}

/// Stub: apply quantization from profile; returns placeholder result.
pub fn apply_quantization(profile: &OptimizationProfile) -> QuantizationResult {
    let (size_before, size_after) = match profile.quantization {
        QuantizationLevel::None => (128.0, 128.0),
        QuantizationLevel::Int8 => (128.0, 32.0),
        QuantizationLevel::Int4 => (128.0, 16.0),
    };
    let ratio = if size_after > 0.0 {
        size_before / size_after
    } else {
        1.0
    };
    QuantizationResult {
        level: profile.quantization,
        size_mb_before: size_before,
        size_mb_after: size_after,
        compression_ratio: ratio,
    }
}

/// Pruning strategy type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PruningStrategy {
    MagnitudeBased,
    Structured,
    Unstructured,
}

impl Default for PruningStrategy {
    fn default() -> Self {
        Self::MagnitudeBased
    }
}

/// Pruning configuration
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PruningConfig {
    pub strategy: PruningStrategy,
    pub ratio: f32,
    pub iterative: bool,
    pub iterations: u32,
}

impl PruningConfig {
    /// Create default pruning configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::ml::optimization::PruningConfig;
    ///
    /// let config = PruningConfig::default_config();
    /// assert_eq!(config.ratio, 0.1);
    /// ```
    pub fn default_config() -> Self {
        Self {
            strategy: PruningStrategy::MagnitudeBased,
            ratio: 0.1,
            iterative: false,
            iterations: 1,
        }
    }
}

/// Result of applying pruning
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PruningResult {
    pub strategy: PruningStrategy,
    pub weights_before: usize,
    pub weights_after: usize,
    pub pruned_count: usize,
    pub compression_ratio: f64,
    pub accuracy_drop: f64,
}

/// Apply pruning to model weights
///
/// # Arguments
///
/// * `weights` - Model weights to prune
/// * `config` - Pruning configuration
///
/// # Example
///
/// ```rust
/// use poolai::ml::optimization::{apply_pruning, PruningConfig, PruningStrategy};
///
/// let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05];
/// let mut config = PruningConfig::default_config();
/// config.ratio = 0.2;
///
/// let result = apply_pruning(&weights, &config);
/// assert!(result.pruned_count > 0);
/// ```
pub fn apply_pruning(weights: &[f64], config: &PruningConfig) -> PruningResult {
    if weights.is_empty() {
        return PruningResult {
            strategy: config.strategy,
            weights_before: 0,
            weights_after: 0,
            pruned_count: 0,
            compression_ratio: 1.0,
            accuracy_drop: 0.0,
        };
    }

    let weights_before = weights.len();
    let target_count = (weights_before as f32 * (1.0 - config.ratio)) as usize;
    let pruned_count = weights_before - target_count;

    // Apply pruning based on strategy
    let pruned_weights = match config.strategy {
        PruningStrategy::MagnitudeBased => magnitude_based_pruning(weights, target_count),
        PruningStrategy::Structured => structured_pruning(weights, target_count),
        PruningStrategy::Unstructured => unstructured_pruning(weights, target_count),
    };

    // Logical "after" size is the number of retained (non-zero) weights.
    let weights_after = pruned_weights.iter().filter(|&&w| w != 0.0).count();
    let compression_ratio = if weights_after > 0 {
        weights_before as f64 / weights_after as f64
    } else {
        1.0
    };

    // Estimate accuracy drop (simplified)
    let accuracy_drop = config.ratio as f64 * 0.1; // Assume 10% accuracy drop per 10% pruning

    PruningResult {
        strategy: config.strategy,
        weights_before,
        weights_after,
        pruned_count,
        compression_ratio,
        accuracy_drop,
    }
}

/// Magnitude-based pruning: remove weights with smallest absolute values
fn magnitude_based_pruning(weights: &[f64], target_count: usize) -> Vec<f64> {
    if target_count >= weights.len() {
        return weights.to_vec();
    }

    // Create vector of (index, absolute_value)
    let mut indexed: Vec<(usize, f64)> = weights
        .iter()
        .enumerate()
        .map(|(i, &w)| (i, w.abs()))
        .collect();

    // Sort by absolute value (ascending)
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    // Keep top N weights (largest absolute values)
    let keep_indices: std::collections::HashSet<usize> = indexed
        .iter()
        .rev()
        .take(target_count)
        .map(|(i, _)| *i)
        .collect();

    // Return pruned weights (set pruned to 0)
    weights
        .iter()
        .enumerate()
        .map(|(i, &w)| if keep_indices.contains(&i) { w } else { 0.0 })
        .collect()
}

/// Structured pruning: remove entire channels/filters
fn structured_pruning(weights: &[f64], target_count: usize) -> Vec<f64> {
    if target_count >= weights.len() {
        return weights.to_vec();
    }

    // For structured pruning, we group weights into "channels"
    // In a real implementation, this would work with actual layer structures
    let channel_size = (weights.len() as f32 / 8.0).ceil() as usize; // Assume 8 channels
    let num_channels = (weights.len() + channel_size - 1) / channel_size;

    // Calculate channel magnitudes
    let mut channel_magnitudes: Vec<(usize, f64)> = (0..num_channels)
        .map(|ch| {
            let start = ch * channel_size;
            let end = (start + channel_size).min(weights.len());
            let magnitude: f64 = weights[start..end].iter().map(|w| w.abs()).sum();
            (ch, magnitude)
        })
        .collect();

    // Sort by magnitude (ascending)
    channel_magnitudes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    // Calculate how many channels to keep
    let target_channels = (target_count + channel_size - 1) / channel_size;
    let keep_channels: std::collections::HashSet<usize> = channel_magnitudes
        .iter()
        .rev()
        .take(target_channels)
        .map(|(ch, _)| *ch)
        .collect();

    // Prune channels not in keep set
    weights
        .chunks(channel_size)
        .enumerate()
        .flat_map(|(ch, chunk)| {
            if keep_channels.contains(&ch) {
                chunk.to_vec()
            } else {
                vec![0.0; chunk.len()]
            }
        })
        .collect()
}

/// Unstructured pruning: remove individual weights (fine-grained)
fn unstructured_pruning(weights: &[f64], target_count: usize) -> Vec<f64> {
    // Unstructured pruning is similar to magnitude-based but more fine-grained
    // For simplicity, we use magnitude-based approach
    magnitude_based_pruning(weights, target_count)
}

/// Apply iterative pruning
///
/// # Arguments
///
/// * `weights` - Model weights to prune
/// * `config` - Pruning configuration with iterative flag
///
/// # Example
///
/// ```rust
/// use poolai::ml::optimization::{apply_iterative_pruning, PruningConfig};
///
/// let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05];
/// let mut config = PruningConfig::default_config();
/// config.iterative = true;
/// config.iterations = 3;
/// config.ratio = 0.3; // Total 30%, 10% per iteration
///
/// let result = apply_iterative_pruning(&weights, &config);
/// assert!(result.pruned_count > 0);
/// ```
pub fn apply_iterative_pruning(weights: &[f64], config: &PruningConfig) -> PruningResult {
    if !config.iterative || config.iterations == 0 {
        return apply_pruning(weights, config);
    }

    let mut current_weights = weights.to_vec();
    let ratio_per_iteration = config.ratio / config.iterations as f32;
    let mut total_pruned = 0;
    let mut total_accuracy_drop = 0.0;

    for _ in 0..config.iterations {
        let iter_config = PruningConfig {
            strategy: config.strategy,
            ratio: ratio_per_iteration,
            iterative: false,
            iterations: 1,
        };

        let result = apply_pruning(&current_weights, &iter_config);
        total_pruned += result.pruned_count;
        total_accuracy_drop += result.accuracy_drop;

        // Update weights (in real implementation, would retrain here)
        current_weights = magnitude_based_pruning(&current_weights, result.weights_after);
    }

    let retained_after = current_weights.iter().filter(|&&w| w != 0.0).count();

    PruningResult {
        strategy: config.strategy,
        weights_before: weights.len(),
        weights_after: retained_after,
        pruned_count: total_pruned,
        compression_ratio: if retained_after > 0 {
            weights.len() as f64 / retained_after as f64
        } else {
            1.0
        },
        accuracy_drop: total_accuracy_drop,
    }
}

/// Evaluate pruning impact on model
///
/// # Arguments
///
/// * `weights_before` - Original weights
/// * `weights_after` - Pruned weights
///
/// # Example
///
/// ```rust
/// use poolai::ml::optimization::evaluate_pruning;
///
/// let before = vec![1.0, 2.0, 3.0, 4.0];
/// let after = vec![1.0, 0.0, 3.0, 0.0];
///
/// let impact = evaluate_pruning(&before, &after);
/// assert_eq!(impact.pruned_count, 2);
/// ```
pub fn evaluate_pruning(weights_before: &[f64], weights_after: &[f64]) -> PruningResult {
    if weights_before.len() != weights_after.len() {
        return PruningResult {
            strategy: PruningStrategy::MagnitudeBased,
            weights_before: weights_before.len(),
            weights_after: weights_after.len(),
            pruned_count: 0,
            compression_ratio: 1.0,
            accuracy_drop: 0.0,
        };
    }

    let pruned_count = weights_before
        .iter()
        .zip(weights_after.iter())
        .filter(|(b, a)| (b.abs() > 1e-10) && (a.abs() < 1e-10))
        .count();

    let compression_ratio = if weights_after.len() > 0 {
        weights_before.len() as f64 / weights_after.len() as f64
    } else {
        1.0
    };

    // Estimate accuracy drop based on pruning ratio
    let pruning_ratio = pruned_count as f64 / weights_before.len() as f64;
    let accuracy_drop = pruning_ratio * 0.1;

    PruningResult {
        strategy: PruningStrategy::MagnitudeBased,
        weights_before: weights_before.len(),
        weights_after: weights_after.len(),
        pruned_count,
        compression_ratio,
        accuracy_drop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimization_profile_defaults() {
        let fast = OptimizationProfile::default_fast();
        assert_eq!(fast.quantization, QuantizationLevel::Int8);
        assert!((fast.pruning_ratio - 0.1).abs() < 1e-6);

        let bal = OptimizationProfile::default_balanced();
        assert_eq!(bal.quantization, QuantizationLevel::None);
        assert_eq!(bal.pruning_ratio, 0.0);
    }

    #[test]
    fn profile_model_stub() {
        let p = profile_model();
        assert!(p.latency_ms > 0.0);
        assert!(p.memory_mb > 0.0);
        assert!(p.flops > 0);
    }

    #[test]
    fn suggest_hyperparams_stub() {
        let cfg = TuningConfig::default_config();
        let r = suggest_hyperparams(&cfg);
        assert!(r.learning_rate > 0.0);
        assert!(r.batch_size > 0);
        assert!(r.suggested_epochs > 0);
    }

    #[test]
    fn apply_quantization_stub() {
        let fast = OptimizationProfile::default_fast();
        let q = apply_quantization(&fast);
        assert_eq!(q.level, QuantizationLevel::Int8);
        assert!(q.size_mb_before > 0.0);
        assert!(q.size_mb_after > 0.0);
        assert!(q.compression_ratio >= 1.0);

        let bal = OptimizationProfile::default_balanced();
        let q2 = apply_quantization(&bal);
        assert_eq!(q2.level, QuantizationLevel::None);
        assert!((q2.compression_ratio - 1.0).abs() < 1e-6);
    }

    #[test]
    fn pruning_config_default() {
        let config = PruningConfig::default_config();
        assert_eq!(config.strategy, PruningStrategy::MagnitudeBased);
        assert!((config.ratio - 0.1).abs() < 1e-6);
        assert!(!config.iterative);
        assert_eq!(config.iterations, 1);
    }

    #[test]
    fn apply_pruning_magnitude_based() {
        let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2];
        let mut config = PruningConfig::default_config();
        config.strategy = PruningStrategy::MagnitudeBased;
        config.ratio = 0.25; // Prune 25%

        let result = apply_pruning(&weights, &config);
        assert!(result.pruned_count > 0);
        assert!(result.weights_after < result.weights_before);
        assert!(result.compression_ratio > 1.0);
    }

    #[test]
    fn apply_pruning_structured() {
        let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2];
        let mut config = PruningConfig::default_config();
        config.strategy = PruningStrategy::Structured;
        config.ratio = 0.25;

        let result = apply_pruning(&weights, &config);
        assert!(result.pruned_count > 0);
        assert_eq!(result.strategy, PruningStrategy::Structured);
    }

    #[test]
    fn apply_pruning_unstructured() {
        let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05];
        let mut config = PruningConfig::default_config();
        config.strategy = PruningStrategy::Unstructured;
        config.ratio = 0.2;

        let result = apply_pruning(&weights, &config);
        assert!(result.pruned_count > 0);
        assert_eq!(result.strategy, PruningStrategy::Unstructured);
    }

    #[test]
    fn apply_pruning_empty() {
        let weights = vec![];
        let config = PruningConfig::default_config();

        let result = apply_pruning(&weights, &config);
        assert_eq!(result.weights_before, 0);
        assert_eq!(result.weights_after, 0);
        assert_eq!(result.pruned_count, 0);
    }

    #[test]
    fn apply_pruning_zero_ratio() {
        let weights = vec![1.0, 2.0, 3.0];
        let mut config = PruningConfig::default_config();
        config.ratio = 0.0;

        let result = apply_pruning(&weights, &config);
        assert_eq!(result.pruned_count, 0);
        assert_eq!(result.weights_before, result.weights_after);
    }

    #[test]
    fn apply_iterative_pruning_works() {
        let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2];
        let mut config = PruningConfig::default_config();
        config.iterative = true;
        config.iterations = 3;
        config.ratio = 0.3; // Total 30%, 10% per iteration

        let result = apply_iterative_pruning(&weights, &config);
        assert!(result.pruned_count > 0);
        assert!(result.weights_after < result.weights_before);
    }

    #[test]
    fn apply_iterative_pruning_not_iterative() {
        let weights = vec![1.0, 2.0, 3.0];
        let mut config = PruningConfig::default_config();
        config.iterative = false;

        let result = apply_iterative_pruning(&weights, &config);
        assert_eq!(result.strategy, PruningStrategy::MagnitudeBased);
    }

    #[test]
    fn evaluate_pruning_basic() {
        let before = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let after = vec![1.0, 0.0, 3.0, 0.0, 5.0];

        let result = evaluate_pruning(&before, &after);
        assert_eq!(result.pruned_count, 2);
        assert_eq!(result.weights_before, 5);
        assert_eq!(result.weights_after, 5);
    }

    #[test]
    fn evaluate_pruning_different_lengths() {
        let before = vec![1.0, 2.0, 3.0];
        let after = vec![1.0, 2.0];

        let result = evaluate_pruning(&before, &after);
        assert_eq!(result.pruned_count, 0);
        assert_ne!(result.weights_before, result.weights_after);
    }
}
