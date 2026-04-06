//! Cloud integration operations (Kubernetes, providers, autoscaler, load balancer).

use crate::cloud::autoscaling::ScalingMetrics;
use crate::cloud::loadbalancing::LoadBalancerHealth;
use crate::cloud::CloudManager;
use crate::core::error::AppError;
use crate::core::state::ApiContext;
use std::sync::Arc;

pub const CLOUD_MANAGER_UNAVAILABLE_MESSAGE: &str =
    "Cloud manager not attached. Suggestion: build with feature `cloud` and complete startup.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloudCapabilities {
    pub kubernetes: bool,
    pub aws: bool,
    pub azure: bool,
    pub gcp: bool,
    pub autoscaling: bool,
    pub load_balancing: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudServiceError {
    ManagerUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudSubsystemError {
    ManagerUnavailable,
}

#[derive(Debug)]
pub enum CloudIntegrationError {
    ManagerUnavailable,
    SubsystemDisabled,
    Operation(AppError),
}

fn require_cloud_manager(ctx: &ApiContext) -> Result<Arc<CloudManager>, CloudServiceError> {
    ctx.cloud_manager
        .get()
        .cloned()
        .ok_or(CloudServiceError::ManagerUnavailable)
}

pub struct CloudService;

impl CloudService {
    pub fn capabilities(ctx: &ApiContext) -> Result<CloudCapabilities, CloudServiceError> {
        let m = require_cloud_manager(ctx)?;
        Ok(CloudCapabilities {
            kubernetes: m.kubernetes().is_some(),
            aws: m.aws().is_some(),
            azure: m.azure().is_some(),
            gcp: m.gcp().is_some(),
            autoscaling: m.autoscaler().is_some(),
            load_balancing: m.loadbalancer().is_some(),
        })
    }

    pub async fn kubernetes_cluster_available(
        ctx: &ApiContext,
    ) -> Result<bool, CloudSubsystemError> {
        let m = require_cloud_manager(ctx).map_err(|_| CloudSubsystemError::ManagerUnavailable)?;
        let Some(k8s) = m.kubernetes() else {
            return Ok(false);
        };
        Ok(k8s.is_cluster_available().await)
    }

    pub async fn autoscaler_metrics(
        ctx: &ApiContext,
        resource_id: &str,
    ) -> Result<ScalingMetrics, CloudIntegrationError> {
        let m =
            require_cloud_manager(ctx).map_err(|_| CloudIntegrationError::ManagerUnavailable)?;
        let Some(autoscaler) = m.autoscaler() else {
            return Err(CloudIntegrationError::SubsystemDisabled);
        };
        autoscaler
            .get_metrics(resource_id)
            .await
            .map_err(CloudIntegrationError::Operation)
    }

    pub async fn load_balancer_health(
        ctx: &ApiContext,
    ) -> Result<LoadBalancerHealth, CloudIntegrationError> {
        let m =
            require_cloud_manager(ctx).map_err(|_| CloudIntegrationError::ManagerUnavailable)?;
        let Some(lb) = m.loadbalancer() else {
            return Err(CloudIntegrationError::SubsystemDisabled);
        };
        lb.get_health_status()
            .await
            .map_err(CloudIntegrationError::Operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud::CloudConfig;
    use crate::core::state::AppState;
    use std::sync::Arc;

    async fn test_ctx_with_cloud() -> ApiContext {
        let ctx = Arc::new(AppState::new());
        let cm = Arc::new(CloudManager::new(CloudConfig::default()));
        cm.initialize().await.expect("init ok");
        ctx.attach_cloud_manager(cm).expect("attach once");
        ctx
    }

    #[tokio::test]
    async fn capabilities_match_default_config() {
        let ctx = test_ctx_with_cloud().await;
        let caps = CloudService::capabilities(&ctx).expect("manager present");
        assert!(!caps.kubernetes);
        assert!(!caps.aws);
        assert!(!caps.azure);
        assert!(!caps.gcp);
        assert!(!caps.autoscaling);
        assert!(!caps.load_balancing);
    }

    #[tokio::test]
    async fn kubernetes_unavailable_when_disabled() {
        let ctx = test_ctx_with_cloud().await;
        assert!(!CloudService::kubernetes_cluster_available(&ctx)
            .await
            .expect("query"));
    }

    #[tokio::test]
    async fn autoscaler_errors_when_disabled() {
        let ctx = test_ctx_with_cloud().await;
        assert!(matches!(
            CloudService::autoscaler_metrics(&ctx, "x").await,
            Err(CloudIntegrationError::SubsystemDisabled)
        ));
    }

    #[tokio::test]
    async fn load_balancer_errors_when_disabled() {
        let ctx = test_ctx_with_cloud().await;
        assert!(matches!(
            CloudService::load_balancer_health(&ctx).await,
            Err(CloudIntegrationError::SubsystemDisabled)
        ));
    }
}
