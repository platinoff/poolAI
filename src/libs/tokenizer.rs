use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;
use parking_lot::RwLock;
use log::info;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

pub const POOL_COMMISSION_ADDRESS: &str = "GcdgNtdE8NEk3z9sQ5jXv2tqguZjSYqPqNAtjsjPNJx8";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerConfig {
    pub id: String,
    pub model_name: String,
    pub vocab_size: usize,
    pub max_length: usize,
    pub special_tokens: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerStats {
    pub total_tokens: u64,
    pub total_texts: u64,
    pub average_tokens_per_text: f64,
    pub max_tokens_in_text: usize,
    pub last_processed_text: Option<String>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerMetrics {
    pub config: TokenizerConfig,
    pub stats: TokenizerStats,
}

#[derive(Debug, Clone)]
pub struct RewardCalculation {
    base_reward: f64,
    commission_rate: f64,
    market_price: f64,
}

impl RewardCalculation {
    pub fn new(base_reward: f64, commission_rate: f64) -> Self {
        Self {
            base_reward,
            commission_rate,
            market_price: 0.0,
        }
    }

    pub fn calculate_reward(&self, power: f64, duration: f64) -> f64 {
        let raw_reward = self.base_reward * power * duration;
        let commission = raw_reward * self.commission_rate;
        raw_reward - commission
    }

    pub fn update_market_price(&mut self, new_price: f64) {
        self.market_price = new_price;
        info!("Updated market price to: {}", new_price);
    }

    pub fn get_optimal_price(&self) -> f64 {
        // Calculate optimal price based on market price and pool commission
        self.market_price * (1.0 - self.commission_rate)
    }

    pub fn distribute_rewards(
        &self,
        worker_address: &Pubkey,
        amount: f64,
        commission_address: &Pubkey,
    ) -> Result<(), String> {
        let commission = amount * self.commission_rate;
        let worker_reward = amount - commission;

        // TODO: Implement actual Solana transaction for reward distribution
        info!(
            "Distributing rewards:\nWorker {}: {}\nCommission {}: {}",
            worker_address, worker_reward, commission_address, commission
        );

        Ok(())
    }

    pub fn calculate_pool_share(&self, total_power: f64, worker_power: f64) -> f64 {
        if total_power == 0.0 {
            return 0.0;
        }
        worker_power / total_power
    }

    pub fn adjust_difficulty(&self, current_difficulty: f64, target_time: f64, actual_time: f64) -> f64 {
        let adjustment_factor = target_time / actual_time;
        current_difficulty * adjustment_factor
    }
}

pub struct Tokenizer {
    calculations: Arc<RwLock<HashMap<String, RewardCalculation>>>,
    tokenizers: Arc<Mutex<HashMap<String, TokenizerMetrics>>>,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            calculations: Arc::new(RwLock::new(HashMap::new())),
            tokenizers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_calculation(&self, id: String, calculation: RewardCalculation) {
        self.calculations.write().insert(id, calculation);
    }

    pub fn get_calculation(&self, id: &str) -> Option<RewardCalculation> {
        self.calculations.read().get(id).cloned()
    }

    pub fn update_market_prices(&self, prices: HashMap<String, f64>) {
        let mut calculations = self.calculations.write();
        for (id, price) in prices {
            if let Some(calc) = calculations.get_mut(&id) {
                calc.update_market_price(price);
            }
        }
    }

    pub fn calculate_total_rewards(&self, worker_powers: HashMap<String, f64>, duration: f64) -> HashMap<String, f64> {
        let mut rewards = HashMap::new();
        let calculations = self.calculations.read();
        
        for (worker_id, power) in worker_powers {
            if let Some(calc) = calculations.get(&worker_id) {
                let reward = calc.calculate_reward(power, duration);
                rewards.insert(worker_id, reward);
            }
        }
        
        rewards
    }

    pub async fn add_tokenizer(&self, config: TokenizerConfig) -> Result<(), String> {
        let mut tokenizers = self.tokenizers.lock().await;
        
        if tokenizers.contains_key(&config.id) {
            return Err(format!("Tokenizer '{}' already exists", config.id));
        }

        let metrics = TokenizerMetrics {
            config,
            stats: TokenizerStats {
                total_tokens: 0,
                total_texts: 0,
                average_tokens_per_text: 0.0,
                max_tokens_in_text: 0,
                last_processed_text: None,
                last_error: None,
            },
        };

        tokenizers.insert(metrics.config.id.clone(), metrics);
        info!("Added new tokenizer: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_tokenizer(&self, id: &str) -> Result<(), String> {
        let mut tokenizers = self.tokenizers.lock().await;
        
        if !tokenizers.contains_key(id) {
            return Err(format!("Tokenizer '{}' not found", id));
        }

        tokenizers.remove(id);
        info!("Removed tokenizer: {}", id);
        Ok(())
    }

    pub async fn tokenize(&self, id: &str, text: &str) -> Result<Vec<u32>, String> {
        let mut tokenizers = self.tokenizers.lock().await;
        
        let tokenizer = tokenizers
            .get_mut(id)
            .ok_or_else(|| format!("Tokenizer '{}' not found", id))?;

        if !tokenizer.config.active {
            return Err("Tokenizer is not active".to_string());
        }

        // Simulate tokenization
        let tokens = self.process_text(text, &tokenizer.config).await?;
        
        // Update statistics
        tokenizer.stats.total_tokens += tokens.len() as u64;
        tokenizer.stats.total_texts += 1;
        tokenizer.stats.average_tokens_per_text = 
            (tokenizer.stats.average_tokens_per_text * (tokenizer.stats.total_texts - 1) as f64 + tokens.len() as f64) 
            / tokenizer.stats.total_texts as f64;
        
        if tokens.len() > tokenizer.stats.max_tokens_in_text {
            tokenizer.stats.max_tokens_in_text = tokens.len();
        }
        
        tokenizer.stats.last_processed_text = Some(text.to_string());
        tokenizer.stats.last_error = None;

        Ok(tokens)
    }

    async fn process_text(&self, text: &str, config: &TokenizerConfig) -> Result<Vec<u32>, String> {
        // Simulate tokenization process
        let mut tokens = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for word in words {
            if tokens.len() >= config.max_length {
                return Err("Text exceeds maximum length".to_string());
            }

            // Simple hash-based tokenization
            let token = word.as_bytes().iter().fold(0u32, |acc, &x| acc.wrapping_add(x as u32));
            tokens.push(token % config.vocab_size as u32);
        }

        Ok(tokens)
    }

    pub async fn detokenize(&self, id: &str, tokens: &[u32]) -> Result<String, String> {
        let tokenizers = self.tokenizers.lock().await;
        
        let tokenizer = tokenizers
            .get(id)
            .ok_or_else(|| format!("Tokenizer '{}' not found", id))?;

        if !tokenizer.config.active {
            return Err("Tokenizer is not active".to_string());
        }

        // Simulate detokenization
        let text = self.reconstruct_text(tokens, &tokenizer.config).await?;
        Ok(text)
    }

    async fn reconstruct_text(&self, tokens: &[u32], config: &TokenizerConfig) -> Result<String, String> {
        // Simulate text reconstruction
        let mut text = String::new();
        
        for &token in tokens {
            if token >= config.vocab_size as u32 {
                return Err("Invalid token".to_string());
            }
            
            // Simple token-to-character mapping
            let char = (token % 26 + b'a' as u32) as u8 as char;
            text.push(char);
        }

        Ok(text)
    }

    pub async fn get_tokenizer(&self, id: &str) -> Result<TokenizerMetrics, String> {
        let tokenizers = self.tokenizers.lock().await;
        
        tokenizers
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Tokenizer '{}' not found", id))
    }

    pub async fn get_all_tokenizers(&self) -> Vec<TokenizerMetrics> {
        let tokenizers = self.tokenizers.lock().await;
        tokenizers.values().cloned().collect()
    }

    pub async fn get_active_tokenizers(&self) -> Vec<TokenizerMetrics> {
        let tokenizers = self.tokenizers.lock().await;
        tokenizers
            .values()
            .filter(|t| t.config.active)
            .cloned()
            .collect()
    }

    pub async fn set_tokenizer_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut tokenizers = self.tokenizers.lock().await;
        
        let tokenizer = tokenizers
            .get_mut(id)
            .ok_or_else(|| format!("Tokenizer '{}' not found", id))?;

        tokenizer.config.active = active;
        info!(
            "Tokenizer '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_tokenizer_config(&self, id: &str, new_config: TokenizerConfig) -> Result<(), String> {
        let mut tokenizers = self.tokenizers.lock().await;
        
        let tokenizer = tokenizers
            .get_mut(id)
            .ok_or_else(|| format!("Tokenizer '{}' not found", id))?;

        tokenizer.config = new_config;
        info!("Updated tokenizer configuration: {}", id);
        Ok(())
    }
} 