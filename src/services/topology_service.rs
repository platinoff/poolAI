//! Topology queries for the HTTP API (snapshot, per-node resources).

use crate::core::state::ApiContext;
use crate::pool::topology::{NodeResources, Topology, TopologyManager};
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

/// `AppState::topology_manager` is not attached (startup incomplete or test default).
#[derive(Debug, Clone, Copy)]
pub struct TopologyNotReady;

pub struct TopologyService;

impl TopologyService {
    fn manager(ctx: &ApiContext) -> Result<Arc<TokioRwLock<TopologyManager>>, TopologyNotReady> {
        ctx.topology_manager.get().cloned().ok_or(TopologyNotReady)
    }

    pub async fn get_snapshot(ctx: &ApiContext) -> Result<Topology, TopologyNotReady> {
        let m = Self::manager(ctx)?;
        let mgr = m.read().await;
        Ok(mgr.get_topology_snapshot().await)
    }

    pub async fn get_node_resources(
        ctx: &ApiContext,
        node_id: &str,
    ) -> Result<Option<NodeResources>, TopologyNotReady> {
        let m = Self::manager(ctx)?;
        let mgr = m.read().await;
        Ok(mgr.get_node_resources(node_id).await)
    }
}
