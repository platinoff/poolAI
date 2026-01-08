//! Unit tests for LoadBalancer operations

#[cfg(feature = "cloud")]
use poolai::cloud::loadbalancing::{Backend, LoadBalancer};
#[cfg(feature = "cloud")]
use poolai::AppError;

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_loadbalancer_creation() {
    let loadbalancer = LoadBalancer::new();
    // Just verify it can be created
    let _ = loadbalancer;
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_loadbalancer_initialization() -> Result<(), AppError> {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await?;
    loadbalancer.shutdown().await?;
    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_add_backend_empty_id() {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await.unwrap();

    let backend = Backend {
        id: String::new(),
        address: "10.0.1.10".to_string(),
        port: 8080,
        weight: 100,
    };

    let result = loadbalancer.add_backend(backend).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Backend ID cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_add_backend_empty_address() {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await.unwrap();

    let backend = Backend {
        id: "backend-1".to_string(),
        address: String::new(),
        port: 8080,
        weight: 100,
    };

    let result = loadbalancer.add_backend(backend).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Backend address cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_add_backend_zero_port() {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await.unwrap();

    let backend = Backend {
        id: "backend-1".to_string(),
        address: "10.0.1.10".to_string(),
        port: 0,
        weight: 100,
    };

    let result = loadbalancer.add_backend(backend).await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Backend port must be greater than 0"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_add_backend_duplicate_id() {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await.unwrap();

    let backend1 = Backend {
        id: "backend-1".to_string(),
        address: "10.0.1.10".to_string(),
        port: 8080,
        weight: 100,
    };

    let backend2 = Backend {
        id: "backend-1".to_string(), // Same ID
        address: "10.0.1.11".to_string(),
        port: 8080,
        weight: 100,
    };

    loadbalancer.add_backend(backend1).await.unwrap();
    let result = loadbalancer.add_backend(backend2).await;

    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("already exists"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_add_backend_success() -> Result<(), AppError> {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await?;

    let backend = Backend {
        id: "backend-1".to_string(),
        address: "10.0.1.10".to_string(),
        port: 8080,
        weight: 100,
    };

    loadbalancer.add_backend(backend).await?;

    let backends = loadbalancer.list_backends().await;
    assert_eq!(backends.len(), 1);
    assert_eq!(backends[0].id, "backend-1");

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_remove_backend_empty_id() {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await.unwrap();

    let result = loadbalancer.remove_backend("").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("Backend ID cannot be empty"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_remove_backend_not_found() {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await.unwrap();

    let result = loadbalancer.remove_backend("non-existent").await;
    assert!(result.is_err());
    if let Err(AppError::ValidationError(msg)) = result {
        assert!(msg.contains("does not exist"));
    }
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_remove_backend_success() -> Result<(), AppError> {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await?;

    let backend = Backend {
        id: "backend-1".to_string(),
        address: "10.0.1.10".to_string(),
        port: 8080,
        weight: 100,
    };

    loadbalancer.add_backend(backend).await?;
    loadbalancer.remove_backend("backend-1").await?;

    let backends = loadbalancer.list_backends().await;
    assert_eq!(backends.len(), 0);

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_list_backends() -> Result<(), AppError> {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await?;

    let backend1 = Backend {
        id: "backend-1".to_string(),
        address: "10.0.1.10".to_string(),
        port: 8080,
        weight: 100,
    };

    let backend2 = Backend {
        id: "backend-2".to_string(),
        address: "10.0.1.11".to_string(),
        port: 8080,
        weight: 100,
    };

    loadbalancer.add_backend(backend1).await?;
    loadbalancer.add_backend(backend2).await?;

    let backends = loadbalancer.list_backends().await;
    assert_eq!(backends.len(), 2);

    Ok(())
}

#[cfg(feature = "cloud")]
#[tokio::test]
async fn test_get_health_status() -> Result<(), AppError> {
    let loadbalancer = LoadBalancer::new();
    loadbalancer.initialize().await?;

    let backend = Backend {
        id: "backend-1".to_string(),
        address: "10.0.1.10".to_string(),
        port: 8080,
        weight: 100,
    };

    loadbalancer.add_backend(backend).await?;

    let health = loadbalancer.get_health_status().await?;
    assert_eq!(health.total_backends, 1);
    assert_eq!(health.healthy_backends, 1);
    assert_eq!(health.unhealthy_backends, 0);

    Ok(())
}
