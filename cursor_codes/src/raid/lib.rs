pub mod mount;
pub mod worker;
pub mod storage;
pub mod network;
pub mod config;
pub mod smallworld;
pub mod worker_interface;
pub mod vm;
pub mod admin;

pub use mount::MountManager;
pub use worker::WorkerManager;
pub use storage::StorageManager;
pub use network::NetworkManager;
pub use config::RaidConfig;
pub use smallworld::{SmallWorldManager, Neuron, NodeConfig, NetworkConfig};
pub use worker_interface::{WorkerInterfaceManager, HardwareInfo, DeviceType, WorkerMetrics};
pub use vm::{VmManager, VmConfig, VmStatus, Device, DeviceType as VmDeviceType, VmStats};
pub use admin::{AdminPanel, AdminConfig};

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;
use cursor_codes::runtime::scheduler::SchedulerSystem;
use cursor_codes::runtime::queue::QueueSystem;
use cursor_codes::runtime::cache::CacheSystem;
use cursor_codes::runtime::storage::StorageSystem;

pub struct RaidSystem {
    mount_manager: Arc<RwLock<MountManager>>,
    worker_manager: Arc<RwLock<WorkerManager>>,
    storage_manager: Arc<RwLock<StorageManager>>,
    network_manager: Arc<RwLock<NetworkManager>>,
    smallworld_manager: Arc<RwLock<SmallWorldManager>>,
    worker_interface_manager: Arc<RwLock<WorkerInterfaceManager>>,
    vm_manager: Arc<RwLock<VmManager>>,
    admin_panel: Arc<RwLock<AdminPanel>>,
    config: RaidConfig,
}

impl RaidSystem {
    pub fn new(config: RaidConfig, bot_token: String) -> Self {
        let network_config = NetworkConfig {
            rewiring_probability: 0.1,
            max_distance: 100.0,
            message_timeout: 30,
            max_retries: 3,
        };

        let vm_manager = Arc::new(RwLock::new(VmManager::new()));
        let worker_interface = Arc::new(RwLock::new(WorkerInterfaceManager::new(bot_token.clone())));

        let admin_config = AdminConfig {
            admin_token: "admin_token".to_string(), // TODO: Load from config
            allowed_ips: vec![],
            rate_limit: 100,
            session_timeout_minutes: 30,
        };

        Self {
            mount_manager: Arc::new(RwLock::new(MountManager::new())),
            worker_manager: Arc::new(RwLock::new(WorkerManager::new())),
            storage_manager: Arc::new(RwLock::new(StorageManager::new())),
            network_manager: Arc::new(RwLock::new(NetworkManager::new())),
            smallworld_manager: Arc::new(RwLock::new(SmallWorldManager::new(network_config, 4, 0.1))),
            worker_interface_manager: worker_interface.clone(),
            vm_manager: vm_manager.clone(),
            admin_panel: Arc::new(RwLock::new(AdminPanel::new(
                vm_manager,
                worker_interface,
                admin_config,
            ))),
            config,
        }
    }

    pub async fn start(&self) -> Result<(), Error> {
        // Инициализация системы
        self.mount_manager.write().await.init()?;
        self.worker_manager.write().await.init()?;
        self.storage_manager.write().await.init()?;
        self.network_manager.write().await.init()?;
        self.smallworld_manager.write().await.init()?;
        self.worker_interface_manager.write().await.init()?;
        self.vm_manager.write().await.init()?;

        // Запуск админ-панели
        let admin_panel = self.admin_panel.read().await;
        admin_panel.start_server("127.0.0.1:8080").await?;

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Error> {
        // Остановка системы
        self.mount_manager.write().await.shutdown()?;
        self.worker_manager.write().await.shutdown()?;
        self.storage_manager.write().await.shutdown()?;
        self.network_manager.write().await.shutdown()?;
        self.smallworld_manager.write().await.shutdown()?;
        self.worker_interface_manager.write().await.shutdown()?;
        self.vm_manager.write().await.shutdown()?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mount error: {0}")]
    Mount(#[from] mount::Error),
    #[error("Worker error: {0}")]
    Worker(#[from] worker::Error),
    #[error("Storage error: {0}")]
    Storage(#[from] storage::Error),
    #[error("Network error: {0}")]
    Network(#[from] network::Error),
    #[error("Config error: {0}")]
    Config(#[from] config::Error),
    #[error("SmallWorld error: {0}")]
    SmallWorld(#[from] smallworld::Error),
    #[error("WorkerInterface error: {0}")]
    WorkerInterface(#[from] worker_interface::Error),
    #[error("VM error: {0}")]
    Vm(#[from] vm::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
} 