use solana_sdk::{
    pubkey::Pubkey,
    instruction::AccountMeta,
    signature::Signature,
    transaction::Transaction,
    signer::keypair::Keypair,
    system_instruction,
};
use std::str::FromStr;
use thiserror::Error;
use log::info;
use std::sync::Arc;
use parking_lot::RwLock;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex as TokioMutex;
use chrono;

#[derive(Error, Debug)]
pub enum SolanaAddressError {
    #[error("Invalid address format: {0}")]
    InvalidFormat(String),
    #[error("Address validation failed: {0}")]
    ValidationError(String),
    #[error("Keypair generation failed")]
    KeypairError,
    #[error("Secure random generation failed")]
    RandomError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressConfig {
    pub id: String,
    pub pubkey: String,
    pub label: String,
    pub is_wallet: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressStats {
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub last_transaction_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_error: Option<String>,
    pub balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressMetrics {
    pub config: AddressConfig,
    pub stats: AddressStats,
}

pub struct SolanaAddress {
    addresses: Arc<TokioMutex<std::collections::HashMap<String, AddressMetrics>>>,
    keypairs: Arc<TokioMutex<std::collections::HashMap<String, Keypair>>>,
}

impl SolanaAddress {
    pub fn new() -> Self {
        Self {
            addresses: Arc::new(TokioMutex::new(std::collections::HashMap::new())),
            keypairs: Arc::new(TokioMutex::new(std::collections::HashMap::new())),
        }
    }

    pub async fn add_address(&self, config: AddressConfig) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        
        if addresses.contains_key(&config.id) {
            return Err(format!("Address '{}' already exists", config.id));
        }

        // Validate public key
        if let Err(e) = Pubkey::from_str(&config.pubkey) {
            return Err(format!("Invalid public key: {}", e));
        }

        let metrics = AddressMetrics {
            config,
            stats: AddressStats {
                total_transactions: 0,
                successful_transactions: 0,
                failed_transactions: 0,
                last_transaction_time: None,
                last_error: None,
                balance: 0,
            },
        };

        addresses.insert(metrics.config.id.clone(), metrics);
        info!("Added new address: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_address(&self, id: &str) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        let mut keypairs = self.keypairs.lock().await;
        
        if !addresses.contains_key(id) {
            return Err(format!("Address '{}' not found", id));
        }

        addresses.remove(id);
        keypairs.remove(id);
        info!("Removed address: {}", id);
        Ok(())
    }

    pub async fn generate_wallet(&self, id: &str, label: &str) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        let mut keypairs = self.keypairs.lock().await;
        
        if addresses.contains_key(id) {
            return Err(format!("Address '{}' already exists", id));
        }

        let keypair = Keypair::new();
        let pubkey = keypair.pubkey().to_string();

        let config = AddressConfig {
            id: id.to_string(),
            pubkey,
            label: label.to_string(),
            is_wallet: true,
            active: true,
        };

        let metrics = AddressMetrics {
            config,
            stats: AddressStats {
                total_transactions: 0,
                successful_transactions: 0,
                failed_transactions: 0,
                last_transaction_time: None,
                last_error: None,
                balance: 0,
            },
        };

        addresses.insert(id.to_string(), metrics);
        keypairs.insert(id.to_string(), keypair);
        info!("Generated new wallet: {}", id);
        Ok(())
    }

    pub async fn sign_transaction(
        &self,
        id: &str,
        transaction: &mut Transaction,
    ) -> Result<(), String> {
        let keypairs = self.keypairs.lock().await;
        
        let keypair = keypairs
            .get(id)
            .ok_or_else(|| format!("Wallet '{}' not found", id))?;

        transaction.sign(&[keypair], transaction.message.recent_blockhash);
        info!("Signed transaction for wallet: {}", id);
        Ok(())
    }

    pub async fn update_balance(&self, id: &str, balance: u64) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        
        let address = addresses
            .get_mut(id)
            .ok_or_else(|| format!("Address '{}' not found", id))?;

        address.stats.balance = balance;
        info!("Updated balance for address {}: {}", id, balance);
        Ok(())
    }

    pub async fn record_transaction(
        &self,
        id: &str,
        success: bool,
        error: Option<String>,
    ) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        
        let address = addresses
            .get_mut(id)
            .ok_or_else(|| format!("Address '{}' not found", id))?;

        address.stats.total_transactions += 1;
        if success {
            address.stats.successful_transactions += 1;
            address.stats.last_error = None;
        } else {
            address.stats.failed_transactions += 1;
            address.stats.last_error = error;
        }
        
        address.stats.last_transaction_time = Some(chrono::Utc::now());
        info!(
            "Recorded {} transaction for address: {}",
            if success { "successful" } else { "failed" },
            id
        );
        Ok(())
    }

    pub async fn get_address(&self, id: &str) -> Result<AddressMetrics, String> {
        let addresses = self.addresses.lock().await;
        
        addresses
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Address '{}' not found", id))
    }

    pub async fn get_all_addresses(&self) -> Vec<AddressMetrics> {
        let addresses = self.addresses.lock().await;
        addresses.values().cloned().collect()
    }

    pub async fn get_active_addresses(&self) -> Vec<AddressMetrics> {
        let addresses = self.addresses.lock().await;
        addresses
            .values()
            .filter(|a| a.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_wallets(&self) -> Vec<AddressMetrics> {
        let addresses = self.addresses.lock().await;
        addresses
            .values()
            .filter(|a| a.config.is_wallet)
            .cloned()
            .collect()
    }

    pub async fn set_address_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        
        let address = addresses
            .get_mut(id)
            .ok_or_else(|| format!("Address '{}' not found", id))?;

        address.config.active = active;
        info!(
            "Address '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_address_config(&self, id: &str, new_config: AddressConfig) -> Result<(), String> {
        let mut addresses = self.addresses.lock().await;
        
        let address = addresses
            .get_mut(id)
            .ok_or_else(|| format!("Address '{}' not found", id))?;

        // Validate public key
        if let Err(e) = Pubkey::from_str(&new_config.pubkey) {
            return Err(format!("Invalid public key: {}", e));
        }

        address.config = new_config;
        info!("Updated address configuration: {}", id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_generation() {
        let manager = SolanaAddress::new();
        let pubkey = manager.generate_wallet("test", "test").unwrap();
        assert_eq!(manager.get_address("test").unwrap().config.pubkey, pubkey);
    }

    #[test]
    fn test_address_validation() {
        let valid_address = "11111111111111111111111111111111";
        let invalid_address = "invalid";
        
        assert!(SolanaAddress::validate_address(valid_address).is_ok());
        assert!(SolanaAddress::validate_address(invalid_address).is_err());
    }
} 