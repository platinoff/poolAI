use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use log::info;
use rand::Rng;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::runtime::worker::WorkerManager;
use cursor_codes::runtime::scheduler::SchedulerSystem;
use cursor_codes::runtime::queue::QueueSystem;
use cursor_codes::runtime::cache::CacheSystem;
use cursor_codes::runtime::storage::StorageSystem;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Node error: {0}")]
    NodeError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Message error: {0}")]
    MessageError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub id: String,
    pub connections: Vec<String>,
    pub memory_map: HashMap<String, f64>, // Map of memory regions to availability scores
    pub seeds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub id: String,
    pub position: (f64, f64),
    pub connections: Vec<String>,
    pub max_connections: usize,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub total_messages: u64,
    pub successful_messages: u64,
    pub failed_messages: u64,
    pub average_latency: f64,
    pub last_message_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub config: NodeConfig,
    pub stats: NodeStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub rewiring_probability: f64,
    pub max_distance: f64,
    pub message_timeout: u64,
    pub max_retries: u32,
}

pub struct SmallWorldManager {
    config: Arc<Mutex<NetworkConfig>>,
    nodes: Arc<Mutex<HashMap<String, NodeMetrics>>>,
    neurons: Arc<Mutex<HashMap<String, Neuron>>>,
    k: usize, // Number of nearest neighbors
    p: f64,  // Rewiring probability
}

impl SmallWorldManager {
    pub fn new(config: NetworkConfig, k: usize, p: f64) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            nodes: Arc::new(Mutex::new(HashMap::new())),
            neurons: Arc::new(Mutex::new(HashMap::new())),
            k,
            p,
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn add_node(&self, config: NodeConfig) -> Result<(), Error> {
        let mut nodes = self.nodes.lock().await;
        
        if nodes.contains_key(&config.id) {
            return Err(Error::NodeError(format!("Node '{}' already exists", config.id)));
        }

        let metrics = NodeMetrics {
            config,
            stats: NodeStats {
                total_messages: 0,
                successful_messages: 0,
                failed_messages: 0,
                average_latency: 0.0,
                last_message_time: None,
                last_error: None,
            },
        };

        nodes.insert(metrics.config.id.clone(), metrics);
        info!("Added new node: {}", metrics.config.id);
        Ok(())
    }

    pub async fn add_neuron(&self, id: String, seeds: Vec<String>) -> Result<(), Error> {
        let mut neurons = self.neurons.lock().await;
        let mut connections = Vec::new();
        
        // Connect to k nearest neighbors
        for i in 1..=self.k {
            let prev_id = format!("{}-{}", id, i);
            let next_id = format!("{}+{}", id, i);
            
            if neurons.contains_key(&prev_id) {
                connections.push(prev_id);
            }
            if neurons.contains_key(&next_id) {
                connections.push(next_id);
            }
        }
        
        // Random rewiring
        for i in 0..connections.len() {
            if rand::random::<f64>() < self.p {
                if let Some(random_neuron) = neurons.keys().choose(&mut rand::thread_rng()) {
                    connections[i] = random_neuron.clone();
                }
            }
        }
        
        let neuron = Neuron {
            id: id.clone(),
            connections,
            memory_map: HashMap::new(),
            seeds,
        };
        
        neurons.insert(id, neuron);
        Ok(())
    }

    pub async fn update_memory_map(&self, neuron_id: &str, region: String, availability: f64) -> Result<(), Error> {
        let mut neurons = self.neurons.lock().await;
        if let Some(neuron) = neurons.get_mut(neuron_id) {
            neuron.memory_map.insert(region, availability);
            Ok(())
        } else {
            Err(Error::NodeError(format!("Neuron '{}' not found", neuron_id)))
        }
    }

    pub async fn get_available_memory(&self, neuron_id: &str) -> Result<Vec<(String, f64)>, Error> {
        let neurons = self.neurons.lock().await;
        if let Some(neuron) = neurons.get(neuron_id) {
            let mut regions: Vec<_> = neuron.memory_map.iter()
                .map(|(region, &score)| (region.clone(), score))
                .collect();
            regions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            Ok(regions)
        } else {
            Err(Error::NodeError(format!("Neuron '{}' not found", neuron_id)))
        }
    }

    pub async fn get_optimal_path(&self, start_id: &str, target_region: &str) -> Result<Option<Vec<String>>, Error> {
        let neurons = self.neurons.lock().await;
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut paths = HashMap::new();
        
        queue.push_back(start_id.to_string());
        paths.insert(start_id.to_string(), vec![start_id.to_string()]);
        
        while let Some(current_id) = queue.pop_front() {
            if visited.contains(&current_id) {
                continue;
            }
            
            visited.insert(current_id.clone());
            
            if let Some(neuron) = neurons.get(&current_id) {
                if neuron.memory_map.contains_key(target_region) {
                    return Ok(paths.get(&current_id).cloned());
                }
                
                for connected_id in &neuron.connections {
                    if !visited.contains(connected_id) {
                        let mut new_path = paths[&current_id].clone();
                        new_path.push(connected_id.clone());
                        paths.insert(connected_id.clone(), new_path);
                        queue.push_back(connected_id.clone());
                    }
                }
            }
        }
        
        Ok(None)
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Cleanup and close all connections
        let mut nodes = self.nodes.lock().await;
        let mut neurons = self.neurons.lock().await;
        nodes.clear();
        neurons.clear();
        Ok(())
    }
} 