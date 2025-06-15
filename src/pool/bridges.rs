use actix_web::{web, HttpResponse, Responder, get, post, delete};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use log::{info, warn, error};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tokio::sync::Mutex;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::network::network::NetworkSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub name: String,
    pub source_network: String,
    pub target_network: String,
    pub fee_percentage: f64,
    pub min_amount: f64,
    pub max_amount: f64,
    pub source_network_url: String,
    pub target_network_url: String,
    pub api_key: String,
    pub timeout: u64,
    pub retry_attempts: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    pub id: String,
    pub bridge_id: String,
    pub source_address: String,
    pub target_address: String,
    pub amount: f64,
    pub fee: f64,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub struct BridgeManager {
    bridges: Arc<RwLock<HashMap<String, BridgeConfig>>>,
    transactions: Arc<RwLock<HashMap<String, BridgeTransaction>>>,
}

impl BridgeManager {
    pub fn new() -> Self {
        Self {
            bridges: Arc::new(RwLock::new(HashMap::new())),
            transactions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_bridge(&self, config: BridgeConfig) -> Result<(), String> {
        let mut bridges = self.bridges.write();
        if bridges.contains_key(&config.name) {
            return Err("Bridge with this name already exists".to_string());
        }
        bridges.insert(config.name.clone(), config);
        Ok(())
    }

    pub async fn remove_bridge(&self, bridge_id: &str) -> Result<(), String> {
        let mut bridges = self.bridges.write();
        if !bridges.contains_key(bridge_id) {
            return Err("Bridge not found".to_string());
        }
        bridges.remove(bridge_id);
        Ok(())
    }

    pub async fn get_all_bridges(&self) -> Vec<BridgeConfig> {
        let bridges = self.bridges.read();
        bridges.values().cloned().collect()
    }

    pub async fn get_bridge(&self, bridge_id: &str) -> Option<BridgeConfig> {
        let bridges = self.bridges.read();
        bridges.get(bridge_id).cloned()
    }

    pub async fn create_transaction(
        &self,
        bridge_id: &str,
        source_address: String,
        target_address: String,
        amount: f64,
    ) -> Result<BridgeTransaction, String> {
        let bridges = self.bridges.read();
        let bridge = bridges.get(bridge_id)
            .ok_or_else(|| "Bridge not found".to_string())?;

        if !bridge.active {
            return Err("Bridge is inactive".to_string());
        }

        if amount < bridge.min_amount || amount > bridge.max_amount {
            return Err(format!(
                "Amount must be between {} and {}",
                bridge.min_amount, bridge.max_amount
            ));
        }

        let fee = amount * (bridge.fee_percentage / 100.0);
        let transaction = BridgeTransaction {
            id: Uuid::new_v4().to_string(),
            bridge_id: bridge_id.to_string(),
            source_address,
            target_address,
            amount,
            fee,
            status: TransactionStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            error: None,
        };

        let mut transactions = self.transactions.write();
        transactions.insert(transaction.id.clone(), transaction.clone());
        Ok(transaction)
    }

    pub async fn get_transaction(&self, transaction_id: &str) -> Option<BridgeTransaction> {
        let transactions = self.transactions.read();
        transactions.get(transaction_id).cloned()
    }

    pub async fn get_transactions_by_status(&self, status: TransactionStatus) -> Vec<BridgeTransaction> {
        let transactions = self.transactions.read();
        transactions.values()
            .filter(|t| t.status == status)
            .cloned()
            .collect()
    }

    pub async fn update_transaction_status(
        &self,
        transaction_id: &str,
        status: TransactionStatus,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut transactions = self.transactions.write();
        let transaction = transactions.get_mut(transaction_id)
            .ok_or_else(|| "Transaction not found".to_string())?;

        transaction.status = status;
        transaction.error = error;
        transaction.updated_at = Utc::now();
        Ok(())
    }

    pub async fn process_transactions(&self) {
        let transactions = self.transactions.read();
        let pending_transactions: Vec<_> = transactions.values()
            .filter(|t| t.status == TransactionStatus::Pending)
            .cloned()
            .collect();

        for transaction in pending_transactions {
            let bridge = self.get_bridge(&transaction.bridge_id).await;
            if let Some(bridge) = bridge {
                if !bridge.active {
                    self.update_transaction_status(
                        &transaction.id,
                        TransactionStatus::Failed,
                        Some("Bridge is inactive".to_string()),
                    ).await.ok();
                    continue;
                }

                // Here you would implement the actual bridge transaction logic
                // For example, calling the source and target network APIs
                // This is just a placeholder
                match self.execute_bridge_transaction(&transaction, &bridge).await {
                    Ok(_) => {
                        self.update_transaction_status(
                            &transaction.id,
                            TransactionStatus::Completed,
                            None,
                        ).await.ok();
                    }
                    Err(e) => {
                        self.update_transaction_status(
                            &transaction.id,
                            TransactionStatus::Failed,
                            Some(e),
                        ).await.ok();
                    }
                }
            }
        }
    }

    async fn execute_bridge_transaction(
        &self,
        transaction: &BridgeTransaction,
        bridge: &BridgeConfig,
    ) -> Result<(), String> {
        // This is where you would implement the actual bridge transaction logic
        // For example:
        // 1. Call source network API to lock funds
        // 2. Call target network API to mint/burn tokens
        // 3. Handle any errors and retries
        // 4. Update transaction status

        // For now, we'll just simulate a successful transaction
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_bridge() {
        let manager = BridgeManager::new();
        let config = BridgeConfig {
            name: "test_bridge".to_string(),
            source_network: "ethereum".to_string(),
            target_network: "polygon".to_string(),
            fee_percentage: 0.1,
            min_amount: 0.01,
            max_amount: 1000.0,
            source_network_url: "https://eth-mainnet".to_string(),
            target_network_url: "https://polygon-mainnet".to_string(),
            api_key: "test_key".to_string(),
            timeout: 30000,
            retry_attempts: 3,
            active: true,
        };

        assert!(manager.add_bridge(config.clone()).await.is_ok());
        assert!(manager.add_bridge(config).await.is_err());
    }

    #[tokio::test]
    async fn test_create_transaction() {
        let manager = BridgeManager::new();
        let config = BridgeConfig {
            name: "test_bridge".to_string(),
            source_network: "ethereum".to_string(),
            target_network: "polygon".to_string(),
            fee_percentage: 0.1,
            min_amount: 0.01,
            max_amount: 1000.0,
            source_network_url: "https://eth-mainnet".to_string(),
            target_network_url: "https://polygon-mainnet".to_string(),
            api_key: "test_key".to_string(),
            timeout: 30000,
            retry_attempts: 3,
            active: true,
        };

        manager.add_bridge(config).await.unwrap();

        let result = manager.create_transaction(
            "test_bridge",
            "0x123".to_string(),
            "0x456".to_string(),
            0.5,
        ).await;

        assert!(result.is_ok());
        let transaction = result.unwrap();
        assert_eq!(transaction.status, TransactionStatus::Pending);
        assert_eq!(transaction.fee, 0.5 * 0.001);
    }
} 