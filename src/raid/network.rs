use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use reqwest;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::network::network::NetworkSystem;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstConfig {
    pub id: String,
    pub target_url: String,
    pub concurrent_requests: u32,
    pub request_timeout: u64,
    pub max_retries: u32,
    pub retry_delay: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub current_concurrent_requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstMetrics {
    pub config: BurstConfig,
    pub stats: BurstStats,
}

pub struct NetworkManager {
    bursts: Arc<Mutex<HashMap<String, BurstMetrics>>>,
    client: reqwest::Client,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            bursts: Arc::new(Mutex::new(HashMap::new())),
            client: reqwest::Client::new(),
        }
    }

    pub async fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn add_burst(&self, config: BurstConfig) -> Result<(), Error> {
        let mut bursts = self.bursts.lock().await;
        
        if bursts.contains_key(&config.id) {
            return Err(Error::NetworkError(format!("Burst with id {} already exists", config.id)));
        }

        bursts.insert(config.id.clone(), BurstMetrics {
            config: config.clone(),
            stats: BurstStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                last_request_time: None,
                last_error: None,
                current_concurrent_requests: 0,
            },
        });

        Ok(())
    }

    pub async fn execute_burst(&self, id: &str) -> Result<(), Error> {
        let mut bursts = self.bursts.lock().await;
        
        let burst = bursts.get_mut(id)
            .ok_or_else(|| Error::NetworkError(format!("Burst with id {} not found", id)))?;

        if !burst.config.active {
            return Err(Error::NetworkError("Burst is not active".to_string()));
        }

        // Execute concurrent requests
        let mut handles = Vec::new();
        for _ in 0..burst.config.concurrent_requests {
            let client = self.client.clone();
            let config = burst.config.clone();
            let handle = tokio::spawn(async move {
                Self::execute_request(&client, &config).await
            });
            handles.push(handle);
        }

        // Wait for all requests to complete
        for handle in handles {
            match handle.await {
                Ok(Ok(_)) => {
                    burst.stats.successful_requests += 1;
                }
                Ok(Err(e)) => {
                    burst.stats.failed_requests += 1;
                    burst.stats.last_error = Some(e.to_string());
                }
                Err(_) => {
                    burst.stats.failed_requests += 1;
                    burst.stats.last_error = Some("Task failed".to_string());
                }
            }
            burst.stats.total_requests += 1;
        }

        Ok(())
    }

    async fn execute_request(client: &reqwest::Client, config: &BurstConfig) -> Result<(), Error> {
        let start = std::time::Instant::now();
        
        for retry in 0..config.max_retries {
            match client.get(&config.target_url)
                .timeout(std::time::Duration::from_secs(config.request_timeout))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(());
                    }
                }
                Err(e) => {
                    if retry == config.max_retries - 1 {
                        return Err(Error::RequestError(e));
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(config.retry_delay)).await;
                }
            }
        }

        Err(Error::NetworkError("Max retries exceeded".to_string()))
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        // Cleanup and close all connections
        Ok(())
    }
} 