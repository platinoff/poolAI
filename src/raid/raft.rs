//! Raft Consensus Integration for Distributed RAID
//!
//! This module provides Raft consensus for ensuring consistency across
//! distributed RAID nodes. It integrates with async-raft library.

#[cfg(feature = "raft")]
use crate::core::error::AppError;
#[cfg(feature = "raft")]
use crate::raid::RaidManager;
#[cfg(feature = "raft")]
use async_raft::{
    config::Config,
    Raft,
    storage::{CurrentSnapshotData, HardState, InitialState, RaftStorage},
    raft::{Entry, MembershipConfig, ClientWriteRequest},
    AppData, AppDataResponse, NodeId,
};
#[cfg(feature = "raft")]
use anyhow::Result;
#[cfg(feature = "raft")]
use async_trait::async_trait;
#[cfg(feature = "raft")]
#[cfg(feature = "raft")]
use std::sync::Arc;
#[cfg(feature = "raft")]
#[cfg(feature = "raft")]
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::RwLock,
};
#[cfg(feature = "raft")]
use tracing::{info, warn};

/// Raft configuration for Distributed RAID
#[cfg(feature = "raft")]
pub struct RaftConfig {
    /// Node ID in the cluster
    pub node_id: NodeId,
    /// Cluster membership (list of node IDs)
    pub cluster_members: Vec<NodeId>,
    /// Election timeout in milliseconds
    pub election_timeout: u64,
    /// Heartbeat interval in milliseconds
    pub heartbeat_interval: u64,
}

#[cfg(feature = "raft")]
impl Default for RaftConfig {
    fn default() -> Self {
        Self {
            node_id: 1,
            cluster_members: vec![1],
            election_timeout: 1000,  // 1 second
            heartbeat_interval: 100, // 100ms
        }
    }
}

/// Raft storage for Distributed RAID
///
/// This will implement RaftStorage trait for async-raft, providing
/// persistent storage for Raft logs and state.
///
/// Note: async-raft 0.6.1 API verification needed before full implementation.
/// The trait methods will be implemented after confirming the exact API.
#[cfg(feature = "raft")]
#[derive(Clone)]
pub struct RaidRaftStorage {
    /// Reference to the RAID manager
    raid_manager: Arc<RwLock<RaidManager>>,
    /// Node ID
    node_id: NodeId,
    /// Storage path for Raft data
    storage_path: std::path::PathBuf,
}

#[cfg(feature = "raft")]
impl RaidRaftStorage {
    /// Create a new Raft storage
    pub fn new(
        node_id: NodeId,
        raid_manager: Arc<RwLock<RaidManager>>,
        storage_path: std::path::PathBuf,
    ) -> Self {
        Self {
            node_id,
            raid_manager,
            storage_path,
        }
    }

    /// Get the path for Raft log storage
    pub fn log_path(&self) -> std::path::PathBuf {
        self.storage_path.join("raft_log.json")
    }

    /// Get the path for Raft state storage
    pub fn state_path(&self) -> std::path::PathBuf {
        self.storage_path.join("raft_state.json")
    }

    /// Get the path for snapshot storage
    pub fn snapshot_path(&self, snapshot_id: &str) -> std::path::PathBuf {
        self.storage_path.join(format!("snapshot_{}.snap", snapshot_id))
    }

    /// Load hard state from disk
    async fn load_hard_state(&self) -> Result<HardState> {
        let state_path = self.state_path();
        if !state_path.exists() {
            return Ok(HardState {
                current_term: 0,
                voted_for: None,
            });
        }

        let mut file = File::open(&state_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let state: HardState = serde_json::from_str(&contents)
            .map_err(|e| anyhow::anyhow!("Failed to parse hard state: {}", e))?;
        Ok(state)
    }

    /// Save hard state to disk
    async fn save_hard_state_internal(&self, hs: &HardState) -> Result<()> {
        let state_path = self.state_path();
        let contents = serde_json::to_string_pretty(hs)
            .map_err(|e| anyhow::anyhow!("Failed to serialize hard state: {}", e))?;
        
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&state_path)
            .await?;
        file.write_all(contents.as_bytes()).await?;
        file.sync_all().await?;
        Ok(())
    }

    /// Load log entries from disk
    async fn load_log_entries(&self) -> Result<Vec<Entry<RaidRaftOperation>>> {
        let log_path = self.log_path();
        if !log_path.exists() {
            return Ok(Vec::new());
        }

        let mut file = File::open(&log_path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let entries: Vec<Entry<RaidRaftOperation>> = serde_json::from_str(&contents)
            .map_err(|e| anyhow::anyhow!("Failed to parse log entries: {}", e))?;
        Ok(entries)
    }

    /// Save log entries to disk
    async fn save_log_entries(&self, entries: &[Entry<RaidRaftOperation>]) -> Result<()> {
        let log_path = self.log_path();
        let contents = serde_json::to_string_pretty(entries)
            .map_err(|e| anyhow::anyhow!("Failed to serialize log entries: {}", e))?;
        
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)
            .await?;
        file.write_all(contents.as_bytes()).await?;
        file.sync_all().await?;
        Ok(())
    }
}

/// Implement RaftStorage trait for RaidRaftStorage
#[cfg(feature = "raft")]
#[async_trait]
impl RaftStorage<RaidRaftOperation, RaidRaftResponse> for RaidRaftStorage {
    type Snapshot = File;
    type ShutdownError = AppError;

    async fn get_membership_config(&self) -> Result<MembershipConfig> {
        // For now, return initial membership config
        // TODO: Implement reverse search through log to find latest membership config
        Ok(MembershipConfig::new_initial(self.node_id))
    }

    async fn get_initial_state(&self) -> Result<InitialState> {
        let hard_state = self.load_hard_state().await?;
        let entries = self.load_log_entries().await?;

        let last_log_index = entries.len() as u64;
        let last_log_term = entries
            .last()
            .map(|e| e.term)
            .unwrap_or(0);

        // TODO: Track last_applied_log separately
        let last_applied_log = last_log_index;

        let membership = self.get_membership_config().await?;

        Ok(InitialState {
            last_log_index,
            last_log_term,
            last_applied_log,
            hard_state,
            membership,
        })
    }

    async fn save_hard_state(&self, hs: &HardState) -> Result<()> {
        self.save_hard_state_internal(hs).await
    }

    async fn get_log_entries(&self, start: u64, stop: u64) -> Result<Vec<Entry<RaidRaftOperation>>> {
        let entries = self.load_log_entries().await?;
        let start_idx = start as usize;
        let stop_idx = (stop as usize).min(entries.len());
        
        if start_idx >= entries.len() {
            return Ok(Vec::new());
        }

        Ok(entries[start_idx..stop_idx].to_vec())
    }

    async fn delete_logs_from(&self, start: u64, stop: Option<u64>) -> Result<()> {
        let mut entries = self.load_log_entries().await?;
        let start_idx = start as usize;
        
        if start_idx >= entries.len() {
            return Ok(());
        }

        let stop_idx = stop.map(|s| s as usize).unwrap_or(entries.len());
        
        if stop_idx <= start_idx {
            return Ok(());
        }

        // Remove entries from start to stop
        entries.drain(start_idx..stop_idx.min(entries.len()));
        self.save_log_entries(&entries).await
    }

    async fn append_entry_to_log(&self, entry: &Entry<RaidRaftOperation>) -> Result<()> {
        let mut entries = self.load_log_entries().await?;
        entries.push(entry.clone());
        self.save_log_entries(&entries).await
    }

    async fn replicate_to_log(&self, entries: &[Entry<RaidRaftOperation>]) -> Result<()> {
        let mut all_entries = self.load_log_entries().await?;
        all_entries.extend_from_slice(entries);
        self.save_log_entries(&all_entries).await
    }

    async fn apply_entry_to_state_machine(
        &self,
        _index: &u64,
        data: &RaidRaftOperation,
    ) -> Result<RaidRaftResponse> {
        let state_machine = RaidRaftStateMachine::new(self.raid_manager.clone());
        state_machine
            .apply_operation(data)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to apply operation: {}", e))
    }

    async fn replicate_to_state_machine(
        &self,
        entries: &[(&u64, &RaidRaftOperation)],
    ) -> Result<()> {
        let state_machine = RaidRaftStateMachine::new(self.raid_manager.clone());
        for (_index, data) in entries {
            state_machine
                .apply_operation(data)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to apply operation: {}", e))?;
        }
        Ok(())
    }

    async fn do_log_compaction(&self) -> Result<CurrentSnapshotData<Self::Snapshot>> {
        // TODO: Implement proper snapshot creation
        // For now, return a placeholder
        Err(anyhow::anyhow!("Snapshot creation not yet implemented"))
    }

    async fn create_snapshot(&self) -> Result<(String, Box<Self::Snapshot>)> {
        // TODO: Implement snapshot creation
        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let snapshot_path = self.snapshot_path(&snapshot_id);
        let file = File::create(&snapshot_path).await?;
        Ok((snapshot_id, Box::new(file)))
    }

    async fn finalize_snapshot_installation(
        &self,
        _index: u64,
        _term: u64,
        delete_through: Option<u64>,
        _id: String,
        _snapshot: Box<Self::Snapshot>,
    ) -> Result<()> {
        // TODO: Implement snapshot installation finalization
        if let Some(delete_through) = delete_through {
            self.delete_logs_from(0, Some(delete_through + 1)).await?;
        }
        Ok(())
    }

    async fn get_current_snapshot(&self) -> Result<Option<CurrentSnapshotData<Self::Snapshot>>> {
        // TODO: Implement snapshot retrieval
        Ok(None)
    }
}

/// Raft state machine for RAID operations
///
/// This implements the state machine that processes Raft log entries
/// and applies them to the local RAID storage.
#[cfg(feature = "raft")]
pub struct RaidRaftStateMachine {
    /// Reference to the RAID manager
    raid_manager: Arc<RwLock<RaidManager>>,
}

#[cfg(feature = "raft")]
impl RaidRaftStateMachine {
    /// Create a new Raft state machine
    pub fn new(raid_manager: Arc<RwLock<RaidManager>>) -> Self {
        Self { raid_manager }
    }

    /// Apply a Raft operation to the state machine
    pub async fn apply_operation(
        &self,
        operation: &RaidRaftOperation,
    ) -> Result<RaidRaftResponse, AppError> {
        match operation {
            RaidRaftOperation::PutArtifact {
                artifact_id,
                data,
                metadata: _metadata,
            } => {
                let manager = self.raid_manager.read().await;
                let artifact_ref = manager.put_artifact(artifact_id, data).await?;
                Ok(RaidRaftResponse::Success {
                    message: format!("Artifact {} stored", artifact_ref.id),
                })
            }
            RaidRaftOperation::DeleteArtifact { artifact_id } => {
                let _manager = self.raid_manager.read().await;
                // TODO: Convert artifact_id string to Uuid and implement delete
                // For now, this is a placeholder
                Ok(RaidRaftResponse::Success {
                    message: format!("Artifact {} deleted", artifact_id),
                })
            }
            RaidRaftOperation::SyncArtifacts { artifacts } => {
                // TODO: Implement sync logic
                Ok(RaidRaftResponse::Success {
                    message: format!("Synced {} artifacts", artifacts.len()),
                })
            }
        }
    }
}

/// Raft operations that can be applied to the state machine
#[cfg(feature = "raft")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RaidRaftOperation {
    /// Put artifact operation
    PutArtifact {
        artifact_id: String,
        data: Vec<u8>,
        metadata: crate::raid::manifest::ArtifactManifest,
    },
    /// Delete artifact operation
    DeleteArtifact { artifact_id: String },
    /// Sync artifacts operation
    SyncArtifacts { artifacts: Vec<String> },
}

/// Raft operation response
#[cfg(feature = "raft")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RaidRaftResponse {
    /// Success response
    Success { message: String },
    /// Error response
    Error { error: String },
}

// Placeholder implementation - will be completed in next steps
#[cfg(feature = "raft")]
impl AppData for RaidRaftOperation {}

#[cfg(feature = "raft")]
impl AppDataResponse for RaidRaftResponse {}

/// Raft node wrapper for Distributed RAID
///
/// This wraps the async-raft Raft instance and provides
/// integration with the RAID module.
#[cfg(feature = "raft")]
pub struct RaidRaftNode {
    /// Raft instance (initialized in initialize())
    /// Using Arc<RwLock<>> for interior mutability since Raft needs to be mutable
    raft_instance: Arc<RwLock<Option<Raft<RaidRaftOperation, RaidRaftResponse, crate::raid::raft_transport::HttpRaftTransport, RaidRaftStorage>>>>,
    /// Configuration
    config: RaftConfig,
    /// RAID manager reference
    raid_manager: Arc<RwLock<RaidManager>>,
    /// Raft storage
    storage: RaidRaftStorage,
    /// Raft state machine
    state_machine: RaidRaftStateMachine,
    /// Raft network transport
    transport: crate::raid::raft_transport::HttpRaftTransport,
}

#[cfg(feature = "raft")]
impl RaidRaftNode {
    /// Create a new Raft node
    pub fn new(
        config: RaftConfig,
        raid_manager: Arc<RwLock<RaidManager>>,
        storage_path: std::path::PathBuf,
    ) -> Result<Self, AppError> {
        let storage = RaidRaftStorage::new(config.node_id, raid_manager.clone(), storage_path);
        let state_machine = RaidRaftStateMachine::new(raid_manager.clone());
        let transport = crate::raid::raft_transport::HttpRaftTransport::new();

        Ok(Self {
            raft_instance: Arc::new(RwLock::new(None)),
            config,
            raid_manager,
            storage,
            state_machine,
            transport,
        })
    }

    /// Initialize the Raft node
    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing Raft node {}", self.config.node_id);

        // Create storage directory if it doesn't exist
        let storage_path = self.storage.storage_path.clone();
        // Create the directory itself, not just parent
        tokio::fs::create_dir_all(&storage_path)
            .await
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to create Raft storage directory: {}",
                    e
                ))
            })?;

        // Build Raft configuration
        let raft_config = Config::build("poolai-raid-cluster".to_string())
            .election_timeout_min(self.config.election_timeout)
            .election_timeout_max(self.config.election_timeout * 2)
            .heartbeat_interval(self.config.heartbeat_interval)
            .validate()
            .map_err(|e| {
                AppError::ConfigError(format!("Failed to build Raft config: {}", e))
            })?;

        // Initialize Raft instance
        // Note: Raft::new takes Arc<Config>, Arc<Network>, Arc<Storage>
        // State machine is part of Storage implementation
        let config_arc = Arc::new(raft_config);
        let transport_arc = Arc::new(self.transport.clone());
        let storage_arc = Arc::new(self.storage.clone());
        let raft = Raft::new(
            self.config.node_id,
            config_arc,
            transport_arc,
            storage_arc,
        );

        // Store raft instance
        *self.raft_instance.write().await = Some(raft);
        
        info!("Raft node {} initialized successfully", self.config.node_id);
        
        // For single-node clusters, initialize the cluster and wait for leader election
        // In multi-node clusters, only the first node should initialize the cluster
        if self.config.cluster_members.len() == 1 {
            info!("Single-node cluster detected, initializing cluster and waiting for leader election...");
            use std::collections::HashSet;
            let members: HashSet<NodeId> = self.config.cluster_members.iter().cloned().collect();
            self.raft_instance.read().await.as_ref().unwrap().initialize(members).await.map_err(|e| {
                AppError::ConfigError(format!("Failed to initialize single-node Raft cluster: {}", e))
            })?;
            self.wait_for_leader(5000).await?;
            info!("Node {} is now leader.", self.config.node_id);
        }
        
        Ok(())
    }

    /// Initialize a multi-node Raft cluster
    /// 
    /// This should be called on the first node to bootstrap the cluster.
    /// Other nodes should just call `initialize()` without calling this method.
    pub async fn initialize_cluster(&self) -> Result<(), AppError> {
        if self.config.cluster_members.len() == 1 {
            // Single-node cluster is handled in initialize()
            return Ok(());
        }

        info!("Initializing multi-node Raft cluster with {} nodes", self.config.cluster_members.len());
        
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            // Initialize the cluster with all members
            // async-raft expects HashSet<NodeId>
            use std::collections::HashSet;
            let members: HashSet<NodeId> = self.config.cluster_members.iter().cloned().collect();
            raft.initialize(members).await.map_err(|e| {
                AppError::ConfigError(format!("Failed to initialize multi-node Raft cluster: {}", e))
            })?;
            info!("Multi-node cluster initialized successfully");
        } else {
            return Err(AppError::ConfigError("Raft instance not initialized".to_string()));
        }
        
        Ok(())
    }

    /// Check if this node is the leader
    pub async fn is_leader(&self) -> bool {
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            // Use metrics to check if node is leader
            let metrics_receiver = raft.metrics();
            let metrics = metrics_receiver.borrow();
            // State enum is private, but we can check current_leader
            metrics.current_leader == Some(self.config.node_id)
        } else {
            false
        }
    }

    /// Get current Raft term
    pub async fn current_term(&self) -> u64 {
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            let metrics_receiver = raft.metrics();
            let metrics = metrics_receiver.borrow();
            metrics.current_term
        } else {
            0
        }
    }

    /// Get current Raft role (Leader, Follower, Candidate)
    pub async fn current_role(&self) -> String {
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            let metrics_receiver = raft.metrics();
            let metrics = metrics_receiver.borrow();
            // State enum is private, use current_leader to determine role
            if metrics.current_leader == Some(self.config.node_id) {
                "Leader".to_string()
            } else if metrics.current_leader.is_some() {
                "Follower".to_string()
            } else {
                "Candidate".to_string()
            }
        } else {
            "Follower".to_string()
        }
    }

    /// Apply a Raft operation (for leader nodes)
    pub async fn apply_operation(
        &self,
        operation: RaidRaftOperation,
    ) -> Result<RaidRaftResponse, AppError> {
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            // Use Raft's client_write for consensus-based operations
            let request = ClientWriteRequest::new(operation);
            let response = raft
                .client_write(request)
                .await
                .map_err(|e| AppError::ConfigError(format!("Raft client_write failed: {}", e)))?;
            Ok(response.data)
        } else {
            // Fallback to direct state machine application if Raft not initialized
            warn!("Raft instance not initialized, applying operation directly to state machine");
            self.state_machine.apply_operation(&operation).await
        }
    }

    /// Get reference to transport for adding nodes
    pub fn transport(&self) -> &crate::raid::raft_transport::HttpRaftTransport {
        &self.transport
    }

    /// Wait for this node to become leader (with timeout)
    /// 
    /// This is useful for single-node clusters or testing scenarios
    /// where we need to ensure the node is leader before operations.
    pub async fn wait_for_leader(&self, timeout_ms: u64) -> Result<bool, AppError> {
        use tokio::time::{sleep, Duration, timeout};
        
        let timeout_duration = Duration::from_millis(timeout_ms);
        let check_interval = Duration::from_millis(100);
        
        let result = timeout(timeout_duration, async {
            loop {
                if self.is_leader().await {
                    return true;
                }
                sleep(check_interval).await;
            }
        }).await;
        
        match result {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(_) => Ok(false), // Timeout
        }
    }

    /// Trigger election manually (for testing or single-node clusters)
    /// 
    /// Note: In a multi-node cluster, elections happen automatically.
    /// This method is mainly useful for single-node clusters or testing.
    pub async fn trigger_election(&self) -> Result<(), AppError> {
        let instance_guard = self.raft_instance.read().await;
        if instance_guard.is_some() {
            // async-raft doesn't have a direct trigger_election method
            // Elections happen automatically based on timeout
            // For single-node clusters, the node should become leader automatically
            info!("Election will be triggered automatically by Raft");
            Ok(())
        } else {
            Err(AppError::ConfigError("Raft instance not initialized".to_string()))
        }
    }

    /// Get Raft metrics for monitoring
    /// 
    /// Returns current term, leader, and other Raft state information
    pub async fn get_metrics(&self) -> Result<String, AppError> {
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            let metrics_receiver = raft.metrics();
            let metrics = metrics_receiver.borrow();
            Ok(format!(
                "term: {}, leader: {:?}, last_log_index: {}",
                metrics.current_term,
                metrics.current_leader,
                metrics.last_log_index
            ))
        } else {
            Err(AppError::ConfigError("Raft instance not initialized".to_string()))
        }
    }

    /// Get current leader ID (if any)
    /// 
    /// Returns Some(node_id) if there is a leader, None otherwise
    pub async fn get_current_leader(&self) -> Option<NodeId> {
        let instance_guard = self.raft_instance.read().await;
        if let Some(ref raft) = *instance_guard {
            let metrics_receiver = raft.metrics();
            let metrics = metrics_receiver.borrow();
            metrics.current_leader
        } else {
            None
        }
    }

    /// Wait for any leader to be elected in the cluster (with timeout)
    /// 
    /// This is useful for multi-node clusters where we need to wait
    /// for leader election to complete before operations.
    pub async fn wait_for_any_leader(&self, timeout_ms: u64) -> Result<Option<NodeId>, AppError> {
        use tokio::time::{sleep, Duration, timeout};
        
        let timeout_duration = Duration::from_millis(timeout_ms);
        let check_interval = Duration::from_millis(100);
        
        let result = timeout(timeout_duration, async {
            loop {
                if let Some(leader) = self.get_current_leader().await {
                    return Some(leader);
                }
                sleep(check_interval).await;
            }
        }).await;
        
        match result {
            Ok(Some(leader)) => Ok(Some(leader)),
            Ok(None) => Ok(None),
            Err(_) => Ok(None), // Timeout
        }
    }
}

/// Placeholder for non-raft builds
#[cfg(not(feature = "raft"))]
pub struct RaidRaftNode;

#[cfg(not(feature = "raft"))]
impl RaidRaftNode {
    pub fn new(_config: ()) -> Result<Self, crate::core::error::AppError> {
        Err(crate::core::error::AppError::FeatureNotEnabled(
            "raft".to_string(),
        ))
    }

    pub async fn initialize(&self) -> Result<(), crate::core::error::AppError> {
        Err(crate::core::error::AppError::FeatureNotEnabled(
            "raft".to_string(),
        ))
    }

    pub async fn is_leader(&self) -> bool {
        false
    }

    pub async fn current_term(&self) -> u64 {
        0
    }

    pub async fn current_role(&self) -> String {
        "Disabled".to_string()
    }
}
