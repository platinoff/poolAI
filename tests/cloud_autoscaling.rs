//! Unit tests for AutoScaler validation and operations

#[cfg(feature = "cloud")]
use poolai::cloud::autoscaling::AutoScaler;
#[cfg(feature = "cloud")]
use poolai::AppError;

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_autoscaler_creation() {
    let autoscaler = AutoScaler::new();
    // Just verify it can be created
    let _ = autoscaler;
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_autoscaler_initialization() -> Result<(), AppError> {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await?;
    autoscaler.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_up_empty_resource_id() {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await.unwrap();

    let result = autoscaler.scale_up("", 5).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Resource ID cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_up_zero_replicas() {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await.unwrap();

    let result = autoscaler.scale_up("resource-1", 0).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Target replicas must be greater than 0"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_up_not_greater_than_current() {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await.unwrap();

    // Current replicas is 1 (from get_metrics default)
    // Trying to scale to 1 should fail
    let result = autoscaler.scale_up("resource-1", 1).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("must be greater than current replicas"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_down_empty_resource_id() {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await.unwrap();

    let result = autoscaler.scale_down("", 1).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Resource ID cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_down_zero_replicas() {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await.unwrap();

    let result = autoscaler.scale_down("resource-1", 0).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Target replicas must be at least 1"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_scale_down_not_less_than_current() {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await.unwrap();

    // Current replicas is 1 (from get_metrics default)
    // Trying to scale to 1 or more should fail
    let result = autoscaler.scale_down("resource-1", 1).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("must be less than current replicas"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_get_metrics() -> Result<(), AppError> {
    let autoscaler = AutoScaler::new();
    autoscaler.initialize().await?;

    let metrics = autoscaler.get_metrics("resource-1").await?;
    assert_eq!(metrics.current_replicas, 1);
    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.memory_usage, 0.0);
    assert_eq!(metrics.request_rate, 0.0);

    Ok(())
}
