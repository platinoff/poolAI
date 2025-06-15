use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::Duration;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cache_type: String,
    pub max_size: u64,
    pub max_items: u32,
    pub ttl: Duration,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_items: u64,
    pub total_hits: u64,
    pub total_misses: u64,
    pub current_items: u32,
    pub current_size: u64,
    pub evicted_items: u64,
    pub last_operation_time: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub config: CacheConfig,
    pub stats: CacheStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub key: String,
    pub cache_id: String,
    pub value: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hits: u32,
}

pub struct CacheSystem {
    caches: Arc<Mutex<HashMap<String, CacheMetrics>>>,
    items: Arc<Mutex<HashMap<String, CacheItem>>>,
}

impl CacheSystem {
    pub fn new() -> Self {
        Self {
            caches: Arc::new(Mutex::new(HashMap::new())),
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_cache(&self, config: CacheConfig) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        
        if caches.contains_key(&config.id) {
            return Err(format!("Cache '{}' already exists", config.id));
        }

        let metrics = CacheMetrics {
            config,
            stats: CacheStats {
                total_items: 0,
                total_hits: 0,
                total_misses: 0,
                current_items: 0,
                current_size: 0,
                evicted_items: 0,
                last_operation_time: None,
                last_error: None,
            },
        };

        caches.insert(metrics.config.id.clone(), metrics);
        info!("Added new cache: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_cache(&self, id: &str) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        let mut items = self.items.lock().await;
        
        if !caches.contains_key(id) {
            return Err(format!("Cache '{}' not found", id));
        }

        // Remove associated items
        items.retain(|_, i| i.cache_id != id);
        
        caches.remove(id);
        info!("Removed cache: {}", id);
        Ok(())
    }

    pub async fn set_item(
        &self,
        cache_id: &str,
        key: &str,
        value: &str,
        size: u64,
    ) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        let mut items = self.items.lock().await;
        
        let cache = caches
            .get_mut(cache_id)
            .ok_or_else(|| format!("Cache '{}' not found", cache_id))?;

        if !cache.config.active {
            return Err("Cache is not active".to_string());
        }

        if cache.stats.current_items >= cache.config.max_items {
            return Err("Cache has reached maximum items".to_string());
        }

        if cache.stats.current_size + size > cache.config.max_size {
            return Err("Cache has reached maximum size".to_string());
        }

        let now = Utc::now();
        let expires_at = now + chrono::Duration::from_std(cache.config.ttl).unwrap();

        let item = CacheItem {
            key: key.to_string(),
            cache_id: cache_id.to_string(),
            value: value.to_string(),
            size,
            created_at: now,
            expires_at,
            hits: 0,
        };

        items.insert(key.to_string(), item.clone());
        cache.stats.current_items += 1;
        cache.stats.current_size += size;
        cache.stats.total_items += 1;

        info!(
            "Set item: {} in cache: {} (size: {}, expires: {})",
            key, cache_id, size, expires_at
        );
        Ok(())
    }

    pub async fn get_item(&self, cache_id: &str, key: &str) -> Result<Option<String>, String> {
        let mut caches = self.caches.lock().await;
        let mut items = self.items.lock().await;
        
        let cache = caches
            .get_mut(cache_id)
            .ok_or_else(|| format!("Cache '{}' not found", cache_id))?;

        if !cache.config.active {
            return Err("Cache is not active".to_string());
        }

        let now = Utc::now();
        let item = items.get_mut(key);

        match item {
            Some(item) if item.expires_at > now => {
                item.hits += 1;
                cache.stats.total_hits += 1;
                cache.stats.last_operation_time = Some(now);
                info!("Cache hit: {} in cache: {}", key, cache_id);
                Ok(Some(item.value.clone()))
            }
            Some(_) => {
                // Item expired
                items.remove(key);
                cache.stats.current_items -= 1;
                cache.stats.current_size -= item.size;
                cache.stats.total_misses += 1;
                cache.stats.last_operation_time = Some(now);
                info!("Cache miss (expired): {} in cache: {}", key, cache_id);
                Ok(None)
            }
            None => {
                cache.stats.total_misses += 1;
                cache.stats.last_operation_time = Some(now);
                info!("Cache miss: {} in cache: {}", key, cache_id);
                Ok(None)
            }
        }
    }

    pub async fn remove_item(&self, cache_id: &str, key: &str) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        let mut items = self.items.lock().await;
        
        let cache = caches
            .get_mut(cache_id)
            .ok_or_else(|| format!("Cache '{}' not found", cache_id))?;

        if !cache.config.active {
            return Err("Cache is not active".to_string());
        }

        if let Some(item) = items.remove(key) {
            cache.stats.current_items -= 1;
            cache.stats.current_size -= item.size;
            info!("Removed item: {} from cache: {}", key, cache_id);
        }

        Ok(())
    }

    pub async fn clear_cache(&self, cache_id: &str) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        let mut items = self.items.lock().await;
        
        let cache = caches
            .get_mut(cache_id)
            .ok_or_else(|| format!("Cache '{}' not found", cache_id))?;

        if !cache.config.active {
            return Err("Cache is not active".to_string());
        }

        // Remove all items for this cache
        items.retain(|_, i| i.cache_id != cache_id);
        
        cache.stats.current_items = 0;
        cache.stats.current_size = 0;
        info!("Cleared cache: {}", cache_id);
        Ok(())
    }

    pub async fn get_cache(&self, id: &str) -> Result<CacheMetrics, String> {
        let caches = self.caches.lock().await;
        
        caches
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Cache '{}' not found", id))
    }

    pub async fn get_all_caches(&self) -> Vec<CacheMetrics> {
        let caches = self.caches.lock().await;
        caches.values().cloned().collect()
    }

    pub async fn get_active_caches(&self) -> Vec<CacheMetrics> {
        let caches = self.caches.lock().await;
        caches
            .values()
            .filter(|c| c.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_items(&self, cache_id: &str) -> Vec<CacheItem> {
        let items = self.items.lock().await;
        items
            .values()
            .filter(|i| i.cache_id == cache_id)
            .cloned()
            .collect()
    }

    pub async fn set_cache_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        
        let cache = caches
            .get_mut(id)
            .ok_or_else(|| format!("Cache '{}' not found", id))?;

        cache.config.active = active;
        info!(
            "Cache '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_cache_config(&self, id: &str, new_config: CacheConfig) -> Result<(), String> {
        let mut caches = self.caches.lock().await;
        
        let cache = caches
            .get_mut(id)
            .ok_or_else(|| format!("Cache '{}' not found", id))?;

        cache.config = new_config;
        info!("Updated cache configuration: {}", id);
        Ok(())
    }
} 