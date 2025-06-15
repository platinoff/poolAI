pub mod error;
pub mod tls;
pub mod bridges;
pub mod lmrouter;
pub mod reward_system;
pub mod lib_manager;
pub mod vm;
pub mod tgbot;
pub mod workers;
pub mod admin;
pub mod tuning;
pub mod pool;
pub mod model;
pub mod file_mirror;
pub mod loadbalancer;
pub mod burstraid;
pub mod tokenizer;
pub mod smallworld;
pub mod soladdr;
pub mod state;
pub mod config;
pub mod platform;

pub use error::Error;
pub use tls::TLSConfig;
pub use bridges::Bridge;
pub use lmrouter::LMRouter;
pub use reward_system::RewardSystem;
pub use lib_manager::LibManager;
pub use vm::VMManager;
pub use tgbot::MiningBot;
pub use workers::WorkerManager;
pub use admin::AdminPanel;
pub use tuning::TuningSystem;
pub use pool::{PoolManager, PoolConfig, PoolStats, PoolWorker};
pub use model::MiningModel;
pub use file_mirror::FileMirrorTask;
pub use loadbalancer::LoadBalancer;
pub use burstraid::BurstRaid;
pub use tokenizer::Tokenizer;
pub use smallworld::SmallWorld;
pub use soladdr::SolanaAddress;
pub use state::AppState;
pub use config::Config;
pub use platform::{PlatformService, SystemInfo, create_service, create_system_info};

use std::sync::Arc;
use log::{info, error};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;
use std::str::FromStr;
use solana_sdk::{
    signature::{Keypair, Signature},
    system_instruction,
    transaction::Transaction,
};
use std::collections::HashMap;

mod admin_panel;
mod admin_ui;

use admin_panel::{AdminPanel, AdminConfig};
use admin_ui::AdminUI;

#[derive(Error, Debug)]
pub enum CursorError {
    #[error("Bridge error: {0}")]
    BridgeError(String),
    #[error("Model error: {0}")]
    ModelError(String),
    #[error("Token error: {0}")]
    TokenError(String),
    #[error("Solana error: {0}")]
    SolanaError(String),
    #[error("RPC error: {0}")]
    RpcError(String),
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub struct CursorCore {
    bridge_manager: Arc<bridges::BridgeManager>,
    lm_router: Arc<lmrouter::LMRouter>,
    load_balancer: Arc<loadbalancer::LoadBalancer>,
    solana_manager: Arc<soladdr::SolanaAddressManager>,
    token_manager: Arc<tgtoken::TokenManager>,
    rpc_client: Arc<RpcClient>,
    keypair: Keypair,
    recent_blockhash: Signature,
}

impl CursorCore {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            bridge_manager: Arc::new(bridges::BridgeManager::new()),
            lm_router: Arc::new(lmrouter::LMRouter::new()),
            load_balancer: Arc::new(loadbalancer::LoadBalancer::new(3, 1000, 60)),
            solana_manager: Arc::new(soladdr::SolanaAddressManager::new()),
            token_manager: Arc::new(tgtoken::TokenManager::new()),
            rpc_client: Arc::new(RpcClient::new(rpc_url.to_string())),
            keypair: Keypair::new(),
            recent_blockhash: Signature::default(),
        }
    }

    pub async fn initialize_bridge(
        &self,
        source_network: &str,
        target_network: &str,
        fee_percentage: f64,
        min_amount: f64,
        max_amount: f64,
    ) -> Result<String, CursorError> {
        let bridge_config = bridges::BridgeConfig {
            source_network: source_network.to_string(),
            target_network: target_network.to_string(),
            fee_percentage,
            min_amount,
            max_amount,
        };

        let bridge_id = uuid::Uuid::new_v4().to_string();
        self.bridge_manager.add_bridge(bridge_id.clone(), bridge_config);
        info!("Initialized bridge between {} and {}", source_network, target_network);
        Ok(bridge_id)
    }

    pub async fn register_language_model(
        &self,
        model_id: String,
        config: lmrouter::ModelConfig,
    ) -> Result<(), CursorError> {
        self.lm_router.register_model(model_id.clone(), config.clone());
        self.load_balancer.register_model(model_id, config)
            .await
            .map_err(|e| CursorError::ModelError(e.to_string()))?;
        Ok(())
    }

    pub async fn create_solana_wallet(&self, label: String) -> Result<Pubkey, CursorError> {
        self.solana_manager.generate_new_address(label)
            .map_err(|e| CursorError::SolanaError(e.to_string()))
    }

    pub async fn register_token(
        &self,
        label: String,
        mint_address: &str,
        decimals: u8,
        name: String,
        symbol: String,
    ) -> Result<(), CursorError> {
        self.token_manager.register_token(label, mint_address, decimals, name, symbol)
            .map_err(|e| CursorError::TokenError(e.to_string()))
    }

    pub async fn get_model_response(
        &self,
        prompt: &str,
        requirements: &lmrouter::ModelRequirements,
    ) -> Result<String, CursorError> {
        let (model_id, _) = self.load_balancer.get_available_model(requirements)
            .await
            .map_err(|e| CursorError::ModelError(e.to_string()))?;

        // Здесь будет реализация вызова модели
        let response = format!("Response from model {}: {}", model_id, prompt);
        
        self.load_balancer.update_model_stats(&model_id, true, 0.1)
            .await
            .map_err(|e| CursorError::ModelError(e.to_string()))?;
            
        Ok(response)
    }

    pub async fn transfer_tokens(
        &self,
        from_label: &str,
        to_address: &str,
        amount: u64,
        token_label: &str,
    ) -> Result<String, CursorError> {
        let to_pubkey = Pubkey::from_str(to_address)
            .map_err(|e| CursorError::SolanaError(format!("Invalid destination address: {}", e)))?;

        let token_info = self.token_manager.get_token_info(token_label)
            .ok_or_else(|| CursorError::TokenError("Token not found".to_string()))?;

        let from_pubkey = self.solana_manager.get_address(from_label)
            .ok_or_else(|| CursorError::SolanaError("Source address not found".to_string()))?;

        let transfer_instruction = self.token_manager.create_transfer_instruction(
            &from_pubkey,
            &to_pubkey,
            &from_pubkey,
            amount,
        );

        let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
            &[transfer_instruction],
            Some(&from_pubkey),
        );

        self.solana_manager.sign_transaction(from_label, &mut transaction)
            .map_err(|e| CursorError::SolanaError(e.to_string()))?;

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)
            .await
            .map_err(|e| CursorError::RpcError(format!("Transaction failed: {}", e)))?;

        info!("Token transfer completed: {}", signature);
        Ok(signature.to_string())
    }

    pub async fn transfer_sol(
        &self,
        from: &Keypair,
        to: &Pubkey,
        amount: f64,
    ) -> Result<Signature, CursorError> {
        let lamports = (amount * 1_000_000_000.0) as u64;
        let recent_blockhash = self.rpc_client.get_latest_blockhash()
            .map_err(|e| CursorError::RpcError(e.to_string()))?;
        let transaction = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(&from.pubkey(), to, lamports)],
            Some(&from.pubkey()),
            &[from],
            recent_blockhash,
        );
        self.rpc_client.send_and_confirm_transaction(&transaction)
            .map_err(|e| CursorError::TransactionError(e.to_string()))
    }

    pub async fn start_admin_panel(&self, address: &str, admin_token: String) -> std::io::Result<()> {
        let config = AdminConfig {
            admin_token,
            allowed_ips: vec![],
            rate_limit: 100,
        };

        let panel = AdminPanel::new(self.pool_manager.clone(), config);
        let ui = AdminUI::new(format!("http://{}", address));

        tokio::spawn(async move {
            if let Err(e) = panel.start_server(address).await {
                error!("Admin panel server error: {}", e);
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_core_initialization() {
        let core = CursorCore::new("https://api.mainnet-beta.solana.com");
        assert!(core.initialize_bridge("ethereum", "solana", 0.1, 0.01, 1000.0).await.is_ok());
    }

    #[tokio::test]
    async fn test_wallet_creation() {
        let core = CursorCore::new("https://api.mainnet-beta.solana.com");
        assert!(core.create_solana_wallet("test_wallet".to_string()).await.is_ok());
    }

    #[tokio::test]
    async fn test_token_registration() {
        let core = CursorCore::new("https://api.mainnet-beta.solana.com");
        assert!(core.register_token(
            "test_token".to_string(),
            "11111111111111111111111111111111",
            9,
            "Test Token".to_string(),
            "TEST".to_string(),
        ).await.is_ok());
    }
} 