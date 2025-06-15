use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use log::info;
use tokio::sync::Mutex;
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub id: String,
    pub connections: Vec<String>,
    pub memory_map: HashMap<String, f64>, // Map of memory regions to availability scores
    pub seeds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmallWorld {
    neurons: Arc<RwLock<HashMap<String, Neuron>>>,
    k: usize, // Number of nearest neighbors
    p: f64,  // Rewiring probability
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

impl SmallWorld {
    pub fn new(k: usize, p: f64) -> Self {
        Self {
            neurons: Arc::new(RwLock::new(HashMap::new())),
            k,
            p,
        }
    }

    pub fn add_neuron(&self, id: String, seeds: Vec<String>) {
        let mut neurons = self.neurons.write();
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
    }

    pub fn update_memory_map(&self, neuron_id: &str, region: String, availability: f64) {
        let mut neurons = self.neurons.write();
        if let Some(neuron) = neurons.get_mut(neuron_id) {
            neuron.memory_map.insert(region, availability);
        }
    }

    pub fn get_available_memory(&self, neuron_id: &str) -> Vec<(String, f64)> {
        let neurons = self.neurons.read();
        if let Some(neuron) = neurons.get(neuron_id) {
            let mut regions: Vec<_> = neuron.memory_map.iter()
                .map(|(region, &score)| (region.clone(), score))
                .collect();
            regions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            regions
        } else {
            Vec::new()
        }
    }

    pub fn propagate_memory_info(&self, start_id: &str, depth: usize) {
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();
        to_visit.push((start_id.to_string(), 0));
        
        while let Some((current_id, current_depth)) = to_visit.pop() {
            if current_depth > depth || visited.contains(&current_id) {
                continue;
            }
            
            visited.insert(current_id.clone());
            
            let neurons = self.neurons.read();
            if let Some(neuron) = neurons.get(&current_id) {
                // Propagate memory information to connected neurons
                for connected_id in &neuron.connections {
                    if !visited.contains(connected_id) {
                        to_visit.push((connected_id.clone(), current_depth + 1));
                    }
                }
            }
        }
    }

    pub fn visualize(&self) -> String {
        let neurons = self.neurons.read();
        serde_json::to_string_pretty(&*neurons).unwrap()
    }

    pub fn get_optimal_path(&self, start_id: &str, target_region: &str) -> Option<Vec<String>> {
        let neurons = self.neurons.read();
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
                    return paths.get(&current_id).cloned();
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
        
        None
    }

    pub fn generate_network(&self) -> Vec<Vec<usize>> {
        let mut network = vec![vec![]; self.k];
        for i in 1..=self.k {
            // ... existing code ...
        }
        for i in 0..self.k {
            for j in (i + 1)..self.k {
                if rand::random::<f64>() < self.p {
                    // ... existing code ...
                }
            }
        }
        network
    }
}

pub struct SmallWorld {
    config: Arc<Mutex<NetworkConfig>>,
    nodes: Arc<Mutex<HashMap<String, NodeMetrics>>>,
}

impl SmallWorld {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, config: NodeConfig) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        if nodes.contains_key(&config.id) {
            return Err(format!("Node '{}' already exists", config.id));
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

    pub async fn remove_node(&self, id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        if !nodes.contains_key(id) {
            return Err(format!("Node '{}' not found", id));
        }

        // Remove connections to this node from other nodes
        for node in nodes.values_mut() {
            node.config.connections.retain(|conn_id| conn_id != id);
        }

        nodes.remove(id);
        info!("Removed node: {}", id);
        Ok(())
    }

    pub async fn connect_nodes(&self, node1_id: &str, node2_id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node1 = nodes
            .get_mut(node1_id)
            .ok_or_else(|| format!("Node '{}' not found", node1_id))?;
        
        let node2 = nodes
            .get_mut(node2_id)
            .ok_or_else(|| format!("Node '{}' not found", node2_id))?;

        if !node1.config.active || !node2.config.active {
            return Err("One or both nodes are not active".to_string());
        }

        if node1.config.connections.len() >= node1.config.max_connections {
            return Err(format!("Node '{}' has reached maximum connections", node1_id));
        }

        if node2.config.connections.len() >= node2.config.max_connections {
            return Err(format!("Node '{}' has reached maximum connections", node2_id));
        }

        if !node1.config.connections.contains(&node2_id.to_string()) {
            node1.config.connections.push(node2_id.to_string());
        }

        if !node2.config.connections.contains(&node1_id.to_string()) {
            node2.config.connections.push(node1_id.to_string());
        }

        info!("Connected nodes: {} <-> {}", node1_id, node2_id);
        Ok(())
    }

    pub async fn disconnect_nodes(&self, node1_id: &str, node2_id: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node1 = nodes
            .get_mut(node1_id)
            .ok_or_else(|| format!("Node '{}' not found", node1_id))?;
        
        let node2 = nodes
            .get_mut(node2_id)
            .ok_or_else(|| format!("Node '{}' not found", node2_id))?;

        node1.config.connections.retain(|id| id != node2_id);
        node2.config.connections.retain(|id| id != node1_id);

        info!("Disconnected nodes: {} <-> {}", node1_id, node2_id);
        Ok(())
    }

    pub async fn send_message(&self, from_id: &str, to_id: &str, message: &str) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        let config = self.config.lock().await;
        
        let from_node = nodes
            .get_mut(from_id)
            .ok_or_else(|| format!("Node '{}' not found", from_id))?;
        
        let to_node = nodes
            .get_mut(to_id)
            .ok_or_else(|| format!("Node '{}' not found", to_id))?;

        if !from_node.config.active || !to_node.config.active {
            return Err("One or both nodes are not active".to_string());
        }

        if !from_node.config.connections.contains(&to_id.to_string()) {
            return Err(format!("Nodes '{}' and '{}' are not connected", from_id, to_id));
        }

        let start_time = chrono::Utc::now();
        let mut retries = 0;

        while retries < config.max_retries {
            match self.deliver_message(from_node, to_node, message).await {
                Ok(latency) => {
                    from_node.stats.successful_messages += 1;
                    to_node.stats.successful_messages += 1;
                    
                    from_node.stats.average_latency = 
                        (from_node.stats.average_latency * (from_node.stats.total_messages - 1) as f64 + latency) 
                        / from_node.stats.total_messages as f64;
                    
                    to_node.stats.average_latency = 
                        (to_node.stats.average_latency * (to_node.stats.total_messages - 1) as f64 + latency) 
                        / to_node.stats.total_messages as f64;
                    
                    from_node.stats.last_message_time = Some(start_time);
                    to_node.stats.last_message_time = Some(start_time);
                    
                    from_node.stats.last_error = None;
                    to_node.stats.last_error = None;
                    
                    return Ok(());
                }
                Err(e) => {
                    retries += 1;
                    if retries < config.max_retries {
                        tokio::time::sleep(std::time::Duration::from_millis(config.message_timeout)).await;
                        continue;
                    }
                    
                    from_node.stats.failed_messages += 1;
                    to_node.stats.failed_messages += 1;
                    
                    from_node.stats.last_error = Some(e.clone());
                    to_node.stats.last_error = Some(e);
                    
                    return Err("Message delivery failed after maximum retries".to_string());
                }
            }
        }

        Err("Message delivery failed".to_string())
    }

    async fn deliver_message(
        &self,
        from_node: &mut NodeMetrics,
        to_node: &mut NodeMetrics,
        message: &str,
    ) -> Result<f64, String> {
        // Simulate message delivery
        let distance = self.calculate_distance(&from_node.config.position, &to_node.config.position);
        let latency = distance * 10.0; // Simulate network latency based on distance
        
        tokio::time::sleep(std::time::Duration::from_millis(latency as u64)).await;
        
        from_node.stats.total_messages += 1;
        to_node.stats.total_messages += 1;
        
        Ok(latency)
    }

    fn calculate_distance(&self, pos1: &(f64, f64), pos2: &(f64, f64)) -> f64 {
        let dx = pos1.0 - pos2.0;
        let dy = pos1.1 - pos2.1;
        (dx * dx + dy * dy).sqrt()
    }

    pub async fn rewire_network(&self) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        let config = self.config.lock().await;
        let mut rng = rand::thread_rng();

        for node in nodes.values_mut() {
            if !node.config.active {
                continue;
            }

            for i in 0..node.config.connections.len() {
                if rng.gen::<f64>() < config.rewiring_probability {
                    // Find a new random connection
                    let available_nodes: Vec<_> = nodes
                        .values()
                        .filter(|n| {
                            n.config.active
                                && n.config.id != node.config.id
                                && !node.config.connections.contains(&n.config.id)
                                && n.config.connections.len() < n.config.max_connections
                        })
                        .map(|n| n.config.id.clone())
                        .collect();

                    if !available_nodes.is_empty() {
                        let new_connection = &available_nodes[rng.gen_range(0..available_nodes.len())];
                        let old_connection = node.config.connections[i].clone();
                        
                        // Disconnect old connection
                        if let Some(old_node) = nodes.get_mut(&old_connection) {
                            old_node.config.connections.retain(|id| id != &node.config.id);
                        }
                        
                        // Connect to new node
                        node.config.connections[i] = new_connection.clone();
                        if let Some(new_node) = nodes.get_mut(new_connection) {
                            new_node.config.connections.push(node.config.id.clone());
                        }
                        
                        info!(
                            "Rewired connection: {} {} -> {}",
                            node.config.id, old_connection, new_connection
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn get_node(&self, id: &str) -> Result<NodeMetrics, String> {
        let nodes = self.nodes.lock().await;
        
        nodes
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Node '{}' not found", id))
    }

    pub async fn get_all_nodes(&self) -> Vec<NodeMetrics> {
        let nodes = self.nodes.lock().await;
        nodes.values().cloned().collect()
    }

    pub async fn get_active_nodes(&self) -> Vec<NodeMetrics> {
        let nodes = self.nodes.lock().await;
        nodes
            .values()
            .filter(|n| n.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_node_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        
        let node = nodes
            .get_mut(id)
            .ok_or_else(|| format!("Node '{}' not found", id))?;

        node.config.active = active;
        info!(
            "Node '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_network_config(&self, new_config: NetworkConfig) -> Result<(), String> {
        let mut config = self.config.lock().await;
        *config = new_config;
        info!("Updated network configuration");
        Ok(())
    }

    pub async fn get_network_config(&self) -> NetworkConfig {
        self.config.lock().await.clone()
    }
} 