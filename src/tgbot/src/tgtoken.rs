use std::sync::Arc;
use parking_lot::RwLock;
use solana_sdk::{
    pubkey::Pubkey,
    instruction::Instruction,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use thiserror::Error;
use log::info;
use std::str::FromStr;
use std::collections::HashMap;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use std::sync::Mutex;
use hex;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Invalid token address: {0}")]
    InvalidTokenAddress(String),
    #[error("Token account not found: {0}")]
    TokenAccountNotFound(String),
    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),
    #[error("Token operation failed: {0}")]
    OperationFailed(String),
    #[error("Invalid token configuration: {0}")]
    InvalidConfig(String),
    #[error("Secure random generation failed")]
    RandomError,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub mint_address: Pubkey,
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub secure_id: String,
}

impl TokenInfo {
    pub fn validate(&self) -> Result<(), TokenError> {
        if self.name.is_empty() || self.symbol.is_empty() {
            return Err(TokenError::InvalidConfig("Token name and symbol cannot be empty".to_string()));
        }
        if self.decimals > 9 {
            return Err(TokenError::InvalidConfig("Token decimals cannot exceed 9".to_string()));
        }
        Ok(())
    }
}

pub struct TokenManager {
    tokens: Arc<RwLock<HashMap<String, TokenInfo>>>,
    rng: Mutex<SystemRandom>,
}

impl TokenManager {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(RwLock::new(HashMap::new())),
            rng: Mutex::new(SystemRandom::new()),
        }
    }

    pub fn register_token(
        &self,
        label: String,
        mint_address: &str,
        decimals: u8,
        name: String,
        symbol: String,
    ) -> Result<(), TokenError> {
        let mint_pubkey = Pubkey::from_str(mint_address)
            .map_err(|e| TokenError::InvalidTokenAddress(e.to_string()))?;

        let mut rng = self.rng.lock().map_err(|_| TokenError::RandomError)?;
        let mut secure_id = [0u8; 32];
        rng.fill(&mut secure_id).map_err(|_| TokenError::RandomError)?;
        let secure_id = hex::encode(secure_id);

        let token_info = TokenInfo {
            mint_address: mint_pubkey,
            decimals,
            name,
            symbol,
            secure_id,
        };

        token_info.validate()?;

        self.tokens.write()
            .map_err(|_| TokenError::OperationFailed("Failed to write to tokens map".to_string()))?
            .insert(label, token_info.clone());

        info!("Registered token: {} ({})", token_info.name, token_info.symbol);
        Ok(())
    }

    pub fn create_mint_instruction(
        &self,
        mint_pubkey: &Pubkey,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        decimals: u8,
    ) -> Instruction {
        token_instruction::initialize_mint(
            &TOKEN_PROGRAM_ID,
            mint_pubkey,
            mint_authority,
            freeze_authority,
            decimals,
        ).unwrap()
    }

    pub fn create_token_account_instruction(
        &self,
        token_account: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Instruction {
        token_instruction::create_account(
            &TOKEN_PROGRAM_ID,
            token_account,
            mint,
            owner,
        ).unwrap()
    }

    pub fn create_transfer_instruction(
        &self,
        source: &Pubkey,
        destination: &Pubkey,
        authority: &Pubkey,
        amount: u64,
    ) -> Instruction {
        token_instruction::transfer(
            &TOKEN_PROGRAM_ID,
            source,
            destination,
            authority,
            &[],
            amount,
        ).unwrap()
    }

    pub fn create_mint_to_instruction(
        &self,
        mint: &Pubkey,
        destination: &Pubkey,
        mint_authority: &Pubkey,
        amount: u64,
    ) -> Instruction {
        token_instruction::mint_to(
            &TOKEN_PROGRAM_ID,
            mint,
            destination,
            mint_authority,
            &[],
            amount,
        ).unwrap()
    }

    pub fn get_token_info(&self, label: &str) -> Option<TokenInfo> {
        self.tokens.read()
            .ok()
            .and_then(|tokens| tokens.get(label).cloned())
    }

    pub fn get_token_balance(
        &self,
        token_account: &Pubkey,
        client: &solana_client::rpc_client::RpcClient,
    ) -> Result<u64, TokenError> {
        let account_data = client.get_account_data(token_account)
            .map_err(|e| TokenError::TokenAccountNotFound(e.to_string()))?;

        let token_account = TokenAccount::unpack(&account_data)
            .map_err(|e| TokenError::OperationFailed(e.to_string()))?;

        Ok(token_account.amount)
    }

    pub fn format_amount(&self, amount: u64, decimals: u8) -> String {
        let divisor = 10u64.pow(decimals as u32);
        let whole = amount / divisor;
        let fractional = amount % divisor;
        format!("{}.{:0width$}", whole, fractional, width = decimals as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_registration() {
        let manager = TokenManager::new();
        let result = manager.register_token(
            "test_token".to_string(),
            "11111111111111111111111111111111",
            9,
            "Test Token".to_string(),
            "TEST".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_amount_formatting() {
        let manager = TokenManager::new();
        assert_eq!(manager.format_amount(1234567890, 9), "1.234567890");
        assert_eq!(manager.format_amount(1000000000, 9), "1.000000000");
        assert_eq!(manager.format_amount(100000000, 9), "0.100000000");
    }
} 