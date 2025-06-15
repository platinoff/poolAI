use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use log::info;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use uuid::Uuid;
use chrono;
use thiserror::Error;
use url::Url;
use tokio::sync::Mutex as TokioMutex;
use chrono::{DateTime, Utc};
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

/// Ошибки, которые могут возникнуть при работе с мостами
#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Bridge configuration not found: {0}")]
    ConfigNotFound(String),
    #[error("Amount below minimum: {0}")]
    AmountTooLow(f64),
    #[error("Amount above maximum: {0}")]
    AmountTooHigh(f64),
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("Invalid status transition: {0} -> {1}")]
    InvalidStatusTransition(BridgeStatus, BridgeStatus),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Invalid network URL: {0}")]
    InvalidNetworkUrl(String),
}

/// Конфигурация моста между сетями
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Исходная сеть
    pub source_network: String,
    /// Целевая сеть
    pub target_network: String,
    /// Процент комиссии
    pub fee_percentage: f64,
    /// Минимальная сумма для перевода
    pub min_amount: f64,
    /// Максимальная сумма для перевода
    pub max_amount: f64,
    /// URL исходной сети
    pub source_network_url: String,
    /// URL целевой сети
    pub target_network_url: String,
    pub name: String,
    pub url: String,
    pub api_key: String,
    pub timeout: u64,
    pub retry_attempts: u32,
    pub active: bool,
}

impl BridgeConfig {
    pub fn validate_urls(&self) -> Result<(), BridgeError> {
        let source_url = Url::parse(&self.source_network_url)
            .map_err(|e| BridgeError::InvalidNetworkUrl(format!("Source network URL: {}", e)))?;
        
        let target_url = Url::parse(&self.target_network_url)
            .map_err(|e| BridgeError::InvalidNetworkUrl(format!("Target network URL: {}", e)))?;
        
        if source_url.scheme() != "https" || target_url.scheme() != "https" {
            return Err(BridgeError::InvalidNetworkUrl("URLs must use HTTPS".to_string()));
        }
        
        Ok(())
    }
}

/// Транзакция моста
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    /// Уникальный идентификатор транзакции
    pub id: String,
    /// Адрес отправителя
    pub source_address: Pubkey,
    /// Адрес получателя
    pub target_address: Pubkey,
    /// Сумма перевода
    pub amount: f64,
    /// Текущий статус транзакции
    pub status: BridgeStatus,
    /// Временная метка создания транзакции
    pub timestamp: i64,
}

/// Статус транзакции моста
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BridgeStatus {
    /// Транзакция ожидает обработки
    Pending,
    /// Транзакция в процессе обработки
    Processing,
    /// Транзакция успешно завершена
    Completed,
    /// Транзакция завершилась с ошибкой
    Failed(String),
}

/// Менеджер мостов, управляющий конфигурациями и транзакциями
pub struct BridgeManager {
    configs: Arc<RwLock<HashMap<String, BridgeConfig>>>,
    transactions: Arc<RwLock<HashMap<String, BridgeTransaction>>>,
    bridges: Arc<TokioMutex<Vec<BridgeMetrics>>>,
}

impl BridgeManager {
    /// Создает новый экземпляр менеджера мостов
    pub fn new() -> Self {
        Self {
            configs: Arc::new(RwLock::new(HashMap::new())),
            transactions: Arc::new(RwLock::new(HashMap::new())),
            bridges: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Добавляет новую конфигурацию моста
    pub fn add_bridge(&self, id: String, config: BridgeConfig) -> Result<(), BridgeError> {
        if self.configs.read().contains_key(&id) {
            return Err(BridgeError::InternalError(format!("Bridge with id {} already exists", id)));
        }
        
        config.validate_urls()?;
        
        self.configs.write().insert(id.clone(), config.clone());
        info!("Added new bridge configuration: {:?}", config);
        Ok(())
    }

    /// Инициирует перевод через мост
    pub fn initiate_transfer(
        &self,
        source_address: Pubkey,
        target_address: Pubkey,
        amount: f64,
        bridge_id: &str,
    ) -> Result<String, BridgeError> {
        let configs = self.configs.read();
        if let Some(config) = configs.get(bridge_id) {
            if amount < config.min_amount {
                return Err(BridgeError::AmountTooLow(config.min_amount));
            }
            if amount > config.max_amount {
                return Err(BridgeError::AmountTooHigh(config.max_amount));
            }

            let transaction = BridgeTransaction {
                id: Uuid::new_v4().to_string(),
                source_address,
                target_address,
                amount,
                status: BridgeStatus::Pending,
                timestamp: chrono::Utc::now().timestamp(),
            };

            self.transactions.write().insert(transaction.id.clone(), transaction.clone());
            info!("Initiated bridge transfer: {:?}", transaction);
            Ok(transaction.id)
        } else {
            Err(BridgeError::ConfigNotFound(bridge_id.to_string()))
        }
    }

    /// Обновляет статус транзакции
    pub fn update_transaction_status(
        &self,
        transaction_id: &str,
        new_status: BridgeStatus,
    ) -> Result<(), BridgeError> {
        let mut transactions = self.transactions.write();
        if let Some(transaction) = transactions.get_mut(transaction_id) {
            // Проверяем допустимость перехода статуса
            if !is_valid_status_transition(&transaction.status, &new_status) {
                return Err(BridgeError::InvalidStatusTransition(
                    transaction.status.clone(),
                    new_status,
                ));
            }

            transaction.status = new_status.clone();
            info!("Updated transaction status: {:?}", transaction);
            Ok(())
        } else {
            Err(BridgeError::TransactionNotFound(transaction_id.to_string()))
        }
    }

    /// Получает информацию о транзакции
    pub fn get_transaction(&self, transaction_id: &str) -> Result<BridgeTransaction, BridgeError> {
        self.transactions
            .read()
            .get(transaction_id)
            .cloned()
            .ok_or_else(|| BridgeError::TransactionNotFound(transaction_id.to_string()))
    }

    /// Получает конфигурацию моста
    pub fn get_bridge_config(&self, bridge_id: &str) -> Result<BridgeConfig, BridgeError> {
        self.configs
            .read()
            .get(bridge_id)
            .cloned()
            .ok_or_else(|| BridgeError::ConfigNotFound(bridge_id.to_string()))
    }

    /// Получает все транзакции в определенном статусе
    pub fn get_transactions_by_status(&self, status: BridgeStatus) -> Vec<BridgeTransaction> {
        self.transactions
            .read()
            .values()
            .filter(|t| t.status == status)
            .cloned()
            .collect()
    }

    /// Получает все транзакции для определенного адреса
    pub fn get_transactions_by_address(&self, address: &Pubkey) -> Vec<BridgeTransaction> {
        self.transactions
            .read()
            .values()
            .filter(|t| t.source_address == *address || t.target_address == *address)
            .cloned()
            .collect()
    }

    pub async fn add_bridge(&self, config: BridgeConfig) -> Result<(), String> {
        let mut bridges = self.bridges.lock().await;
        
        // Check if bridge already exists
        if bridges.iter().any(|b| b.config.name == config.name) {
            return Err(format!("Bridge '{}' already exists", config.name));
        }

        let metrics = BridgeMetrics {
            config,
            stats: BridgeStats {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time: 0.0,
                last_request_time: None,
                last_error: None,
            },
        };

        bridges.push(metrics);
        info!("Added new bridge: {}", metrics.config.name);
        Ok(())
    }

    pub async fn remove_bridge(&self, name: &str) -> Result<(), String> {
        let mut bridges = self.bridges.lock().await;
        
        let initial_len = bridges.len();
        bridges.retain(|b| b.config.name != name);
        
        if bridges.len() == initial_len {
            return Err(format!("Bridge '{}' not found", name));
        }

        info!("Removed bridge: {}", name);
        Ok(())
    }

    pub async fn get_bridge(&self, name: &str) -> Result<BridgeMetrics, String> {
        let bridges = self.bridges.lock().await;
        
        bridges
            .iter()
            .find(|b| b.config.name == name)
            .cloned()
            .ok_or_else(|| format!("Bridge '{}' not found", name))
    }

    pub async fn update_bridge_stats(
        &self,
        name: &str,
        success: bool,
        response_time: f64,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut bridges = self.bridges.lock().await;
        
        let bridge = bridges
            .iter_mut()
            .find(|b| b.config.name == name)
            .ok_or_else(|| format!("Bridge '{}' not found", name))?;

        bridge.stats.total_requests += 1;
        if success {
            bridge.stats.successful_requests += 1;
        } else {
            bridge.stats.failed_requests += 1;
            bridge.stats.last_error = error;
        }

        // Update average response time
        let total_time = bridge.stats.average_response_time * (bridge.stats.total_requests - 1) as f64;
        bridge.stats.average_response_time = (total_time + response_time) / bridge.stats.total_requests as f64;
        
        bridge.stats.last_request_time = Some(Utc::now());

        Ok(())
    }

    pub async fn get_all_bridges(&self) -> Vec<BridgeMetrics> {
        let bridges = self.bridges.lock().await;
        bridges.clone()
    }

    pub async fn get_active_bridges(&self) -> Vec<BridgeMetrics> {
        let bridges = self.bridges.lock().await;
        bridges
            .iter()
            .filter(|b| b.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_bridge_active(&self, name: &str, active: bool) -> Result<(), String> {
        let mut bridges = self.bridges.lock().await;
        
        let bridge = bridges
            .iter_mut()
            .find(|b| b.config.name == name)
            .ok_or_else(|| format!("Bridge '{}' not found", name))?;

        bridge.config.active = active;
        info!(
            "Bridge '{}' {}",
            name,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }
}

/// Проверяет допустимость перехода между статусами
fn is_valid_status_transition(current: &BridgeStatus, new: &BridgeStatus) -> bool {
    match (current, new) {
        (BridgeStatus::Pending, BridgeStatus::Processing) => true,
        (BridgeStatus::Processing, BridgeStatus::Completed) => true,
        (BridgeStatus::Processing, BridgeStatus::Failed(_)) => true,
        (BridgeStatus::Pending, BridgeStatus::Failed(_)) => true,
        _ => false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeMetrics {
    pub config: BridgeConfig,
    pub stats: BridgeStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_config() {
        let manager = BridgeManager::new();
        let config = BridgeConfig {
            source_network: "solana".to_string(),
            target_network: "ethereum".to_string(),
            fee_percentage: 0.1,
            min_amount: 1.0,
            max_amount: 1000.0,
            source_network_url: "https://solana.com".to_string(),
            target_network_url: "https://ethereum.com".to_string(),
            name: "test_bridge".to_string(),
            url: "https://test.com".to_string(),
            api_key: "test_api_key".to_string(),
            timeout: 1000,
            retry_attempts: 3,
            active: true,
        };

        assert!(manager.add_bridge("test_bridge".to_string(), config.clone()).is_ok());
        assert!(manager.get_bridge_config("test_bridge").is_ok());
    }

    #[test]
    fn test_transaction_flow() {
        let manager = BridgeManager::new();
        let config = BridgeConfig {
            source_network: "solana".to_string(),
            target_network: "ethereum".to_string(),
            fee_percentage: 0.1,
            min_amount: 1.0,
            max_amount: 1000.0,
            source_network_url: "https://solana.com".to_string(),
            target_network_url: "https://ethereum.com".to_string(),
            name: "test_bridge".to_string(),
            url: "https://test.com".to_string(),
            api_key: "test_api_key".to_string(),
            timeout: 1000,
            retry_attempts: 3,
            active: true,
        };

        manager.add_bridge("test_bridge".to_string(), config).unwrap();
        
        let source = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        
        let tx_id = manager.initiate_transfer(source, target, 100.0, "test_bridge").unwrap();
        
        assert!(manager.update_transaction_status(&tx_id, BridgeStatus::Processing).is_ok());
        assert!(manager.update_transaction_status(&tx_id, BridgeStatus::Completed).is_ok());
    }

    #[test]
    fn test_invalid_status_transition() {
        let manager = BridgeManager::new();
        let config = BridgeConfig {
            source_network: "solana".to_string(),
            target_network: "ethereum".to_string(),
            fee_percentage: 0.1,
            min_amount: 1.0,
            max_amount: 1000.0,
            source_network_url: "https://solana.com".to_string(),
            target_network_url: "https://ethereum.com".to_string(),
            name: "test_bridge".to_string(),
            url: "https://test.com".to_string(),
            api_key: "test_api_key".to_string(),
            timeout: 1000,
            retry_attempts: 3,
            active: true,
        };

        manager.add_bridge("test_bridge".to_string(), config).unwrap();
        
        let source = Pubkey::new_unique();
        let target = Pubkey::new_unique();
        
        let tx_id = manager.initiate_transfer(source, target, 100.0, "test_bridge").unwrap();
        
        // Нельзя перейти из Completed обратно в Processing
        assert!(manager.update_transaction_status(&tx_id, BridgeStatus::Processing).is_ok());
        assert!(manager.update_transaction_status(&tx_id, BridgeStatus::Completed).is_ok());
        assert!(manager.update_transaction_status(&tx_id, BridgeStatus::Processing).is_err());
    }
} 