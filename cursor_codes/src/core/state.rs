use std::sync::Arc;
use parking_lot::{RwLock, Mutex};
use std::collections::HashMap;
use solana_sdk::pubkey::Pubkey;
use crate::model::MiningModel;
use crate::core::CursorCore;
use crate::core::burstraid::BurstRaidManager;
use crate::core::vobe_dancing::VobeDancer;
use crate::core::vibe::VibeManager;
use crate::core::reward_system::RewardSystem;
use tokio::sync::Mutex as TokioMutex;
use crate::pool::PoolManager;
use crate::core::lib_manager::LibraryManager;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::pool::{PoolConfig, PoolStats};
use crate::core::error::CursorError;
use crate::core::config::AppConfig;
use crate::monitoring::metrics::MetricsSystem;
use crate::monitoring::logger::LoggerSystem;
use crate::monitoring::alert::AlertSystem;
use crate::runtime::worker::WorkerManager;
use crate::runtime::scheduler::SchedulerSystem;
use crate::runtime::queue::QueueSystem;
use crate::runtime::cache::CacheSystem;
use crate::runtime::storage::StorageSystem;
use crate::network::network::NetworkSystem;
use crate::platform::model::ModelSystem;

use crate::{
    tls::TLSConfig,
    bridges::Bridge,
    lmrouter::LMRouter,
    lib_manager::LibManager,
    vm::VMManager,
    tgbot::MiningBot,
    workers::WorkerManager,
    admin::AdminPanel,
    tuning::TuningSystem,
    pool::PoolMigrationManager,
    loadbalancer::LoadBalancer,
    burstraid::BurstRaid,
    tokenizer::Tokenizer,
    smallworld::SmallWorld,
};

pub struct Worker {
    pub id: String,
    pub solana_address: Pubkey,
    pub mining_power: f64,
}

pub struct RaidNode {
    pub last_heartbeat: std::time::Instant,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeStatus {
    Active,
    Degraded,
    Failed,
}

pub struct AppState {
    pub workers: RwLock<HashMap<String, Worker>>,
    pub raid_status: Mutex<HashMap<Pubkey, RaidNode>>,
    pub model: Arc<Mutex<MiningModel>>,
    pub core: Arc<CursorCore>,
    pub raid_manager: Arc<BurstRaidManager>,
    pub vobe_dancer: Arc<RwLock<VobeDancer>>,
    pub vibe_manager: Arc<RwLock<VibeManager>>,
    pub reward_system: Arc<RwLock<RewardSystem>>,
    pub lib_manager: Arc<RwLock<LibraryManager>>,
    pub worker_manager: Arc<WorkerManager>,
    pub pool_manager: Arc<RwLock<PoolManager>>,
    pub burst_raid: Arc<RwLock<BurstRaidManager>>,
}

impl AppState {
    pub fn new(
        reward_system: RewardSystem,
        lib_manager: LibraryManager,
        worker_manager: WorkerManager,
        pool_manager: PoolManager,
        model: MiningModel,
        burst_raid: BurstRaidManager,
    ) -> Self {
        Self {
            workers: RwLock::new(HashMap::new()),
            raid_status: Mutex::new(HashMap::new()),
            model: Arc::new(Mutex::new(model)),
            core: Arc::new(CursorCore::new()),
            raid_manager: Arc::new(BurstRaidManager::new()),
            vobe_dancer: Arc::new(RwLock::new(VobeDancer::new())),
            vibe_manager: Arc::new(RwLock::new(VibeManager::new())),
            reward_system: Arc::new(RwLock::new(reward_system)),
            lib_manager: Arc::new(RwLock::new(lib_manager)),
            worker_manager: Arc::new(worker_manager),
            pool_manager: Arc::new(RwLock::new(pool_manager)),
            burst_raid: Arc::new(RwLock::new(burst_raid)),
        }
    }

    pub fn add_worker(&self, worker: Worker) {
        self.workers.write().insert(worker.id.clone(), worker);
    }

    pub fn update_raid_status(&self, node_id: Pubkey, status: NodeStatus) {
        let mut raid_status = self.raid_status.lock();
        if let Some(node) = raid_status.get_mut(&node_id) {
            node.status = status;
            node.last_heartbeat = std::time::Instant::now();
        }
    }
} 