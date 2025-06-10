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
    pub tls_config: Arc<TLSConfig>,
    pub bridge: Arc<Bridge>,
    pub router: Arc<LMRouter>,
    pub lib_manager: Arc<RwLock<LibraryManager>>,
    pub vm_manager: Arc<TokioMutex<VMManager>>,
    pub worker_manager: Arc<WorkerManager>,
    pub tuning_system: Arc<TuningSystem>,
    pub pool_manager: Arc<RwLock<PoolManager>>,
    pub load_balancer: Arc<LoadBalancer>,
    pub burst_raid: Arc<RwLock<BurstRaidManager>>,
    pub tokenizer: Arc<Tokenizer>,
    pub small_world: Arc<SmallWorld>,
    pub admin_panel: Arc<AdminPanel>,
    pub bot: Arc<MiningBot>,
}

impl AppState {
    pub fn new(
        tls_config: TLSConfig,
        bridge: Bridge,
        router: LMRouter,
        reward_system: RewardSystem,
        lib_manager: LibManager,
        vm_manager: VMManager,
        worker_manager: WorkerManager,
        tuning_system: TuningSystem,
        pool_manager: PoolMigrationManager,
        model: MiningModel,
        load_balancer: LoadBalancer,
        burst_raid: BurstRaid,
        tokenizer: Tokenizer,
        small_world: SmallWorld,
        admin_panel: AdminPanel,
        bot: MiningBot,
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
            tls_config: Arc::new(tls_config),
            bridge: Arc::new(bridge),
            router: Arc::new(router),
            lib_manager: Arc::new(RwLock::new(LibraryManager::new())),
            vm_manager: Arc::new(TokioMutex::new(vm_manager)),
            worker_manager: Arc::new(worker_manager),
            tuning_system: Arc::new(tuning_system),
            pool_manager: Arc::new(RwLock::new(PoolManager::new())),
            load_balancer: Arc::new(load_balancer),
            burst_raid: Arc::new(RwLock::new(BurstRaidManager::new())),
            tokenizer: Arc::new(tokenizer),
            small_world: Arc::new(small_world),
            admin_panel: Arc::new(admin_panel),
            bot: Arc::new(bot),
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