//! Cache Manager for Stage 4.1 Runtime
//!
//! Provides in-memory LRU (Least Recently Used) caching with TTL (Time To Live)
//! support for performance optimization.
//!
//! # Features
//!
//! - **LRU Eviction**: Automatically evicts least recently used items when capacity is reached
//! - **TTL Support**: Configurable time-to-live for cache entries
//! - **Size Management**: Configurable cache size limits (number of entries)
//! - **Usage Monitoring**: Track cache usage percentage and statistics
//! - **Lifecycle Control**: Initialize, start, and shutdown operations
//!
//! # Example
//!
//! ```no_run
//! use poolai::runtime::cache::CacheManager;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut cache = CacheManager::new(1000); // 1000 entries max
//! cache.initialize().await?;
//! cache.start().await?;
//!
//! // Set an entry with TTL (default: 1 hour)
//! cache.put("key1", "value1", None).await;
//!
//! // Get an entry
//! if let Some(value) = cache.get("key1").await {
//!     println!("Found: {}", value);
//! }
//!
//! let usage = cache.get_usage_percentage().await;
//! println!("Cache usage: {:.1}%", usage * 100.0);
//!
//! cache.shutdown().await?;
//! # Ok(())
//! # }
//! ```

use chrono::{DateTime, Duration, Utc};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Default TTL (Time To Live) for cache entries: 1 hour
const DEFAULT_TTL_SECONDS: i64 = 3600;

/// Cache entry with TTL metadata
#[derive(Debug, Clone)]
struct CacheEntry<V> {
    value: V,
    expires_at: DateTime<Utc>,
}

/// Cache Manager with LRU eviction and TTL support
pub struct CacheManager {
    size_limit: usize,
    cache: Arc<RwLock<LruCache<String, CacheEntry<String>>>>,
    initialized: Arc<RwLock<bool>>,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
}

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of entries evicted due to LRU
    pub evictions: u64,
    /// Number of entries expired due to TTL
    pub expirations: u64,
}

impl CacheManager {
    /// Create a new cache manager with the specified size limit (number of entries)
    ///
    /// # Arguments
    ///
    /// * `size_limit` - Maximum number of entries in the cache
    pub fn new(size_limit: usize) -> Self {
        let cache_capacity = NonZeroUsize::new(size_limit.max(1)).unwrap();
        Self {
            size_limit,
            cache: Arc::new(RwLock::new(LruCache::new(cache_capacity))),
            initialized: Arc::new(RwLock::new(false)),
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Initialize the cache manager
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut initialized = self.initialized.write().await;
        if *initialized {
            return Ok(());
        }

        info!(
            "Initializing Cache Manager with LRU eviction (capacity: {})",
            self.size_limit
        );

        // Clear any existing entries
        let mut cache = self.cache.write().await;
        cache.clear();
        drop(cache);

        *initialized = true;
        debug!("Cache Manager initialized");
        Ok(())
    }

    /// Start the cache manager
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cache Manager started");
        Ok(())
    }

    /// Get a value from the cache by key
    ///
    /// Returns `None` if the key doesn't exist or if the entry has expired.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    pub async fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;

        // Check if entry exists and is not expired
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > Utc::now() {
                // Entry is valid - move to front (LRU update)
                let value = entry.value.clone();
                stats.hits += 1;
                return Some(value);
            } else {
                // Entry expired - remove it
                cache.pop(key);
                stats.expirations += 1;
                stats.misses += 1;
                return None;
            }
        }

        stats.misses += 1;
        None
    }

    /// Put a value into the cache with an optional TTL
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    /// * `value` - The value to cache
    /// * `ttl_seconds` - Optional TTL in seconds (defaults to 1 hour if None)
    pub async fn put(
        &self,
        key: impl Into<String>,
        value: impl Into<String>,
        ttl_seconds: Option<i64>,
    ) -> Option<String> {
        let key = key.into();
        let value = value.into();
        let ttl = ttl_seconds.unwrap_or(DEFAULT_TTL_SECONDS);
        let expires_at = Utc::now() + Duration::seconds(ttl);

        let mut cache = self.cache.write().await;

        // Check if we need to evict (LRU handles this automatically)
        let evicted = if cache.len() >= cache.cap().get() {
            // Check if an entry will be evicted
            if !cache.contains(&key) {
                // New entry will cause eviction
                true
            } else {
                false
            }
        } else {
            false
        };

        let old_value = cache
            .put(
                key.clone(),
                CacheEntry {
                    value: value.clone(),
                    expires_at,
                },
            )
            .map(|e| e.value);

        if evicted && old_value.is_none() {
            // An entry was evicted due to LRU
            let mut stats = self.stats.write().await;
            stats.evictions += 1;
        }

        old_value
    }

    /// Remove a value from the cache by key
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to remove
    pub async fn remove(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.write().await;
        cache.pop(key).map(|e| e.value)
    }

    /// Clear all entries from the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        let mut stats = self.stats.write().await;
        *stats = CacheStats::default();
        debug!("Cache cleared");
    }

    /// Get the current number of entries in the cache
    pub async fn len(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    /// Returns true if the cache has no entries.
    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    /// Get the cache capacity (maximum number of entries)
    pub fn capacity(&self) -> usize {
        self.size_limit
    }

    /// Get cache usage percentage (0.0 to 1.0)
    pub async fn get_usage_percentage(&self) -> f32 {
        let len = self.len().await;
        if self.size_limit == 0 {
            0.0
        } else {
            (len as f32 / self.size_limit as f32).min(1.0)
        }
    }

    /// Get cache usage percentage synchronously (0.0 to 1.0)
    ///
    /// This is a blocking version for use in non-async contexts.
    /// Prefer `get_usage_percentage()` in async contexts.
    pub fn get_usage_percentage_sync(&self) -> f32 {
        // Use a blocking call for non-async contexts
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async { self.get_usage_percentage().await })
        })
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Evict expired entries from the cache
    ///
    /// This method scans the cache and removes all expired entries.
    /// Call this periodically (e.g., in a background task) to clean up expired entries.
    pub async fn evict_expired(&self) -> usize {
        let mut cache = self.cache.write().await;
        let now = Utc::now();
        let mut expired_count = 0;

        // Collect expired keys
        let expired_keys: Vec<String> = cache
            .iter()
            .filter_map(|(key, entry)| {
                if entry.expires_at <= now {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();

        // Remove expired entries
        for key in expired_keys {
            if cache.pop(&key).is_some() {
                expired_count += 1;
            }
        }

        if expired_count > 0 {
            let mut stats = self.stats.write().await;
            stats.expirations += expired_count as u64;
            debug!("Evicted {} expired cache entries", expired_count);
        }

        expired_count
    }

    /// Shutdown the cache manager and clear all entries
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut initialized = self.initialized.write().await;
        if !*initialized {
            return Ok(());
        }

        info!("Shutting down Cache Manager");

        self.clear().await;

        *initialized = false;
        debug!("Cache Manager shut down");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_initialization() {
        let mut cache = CacheManager::new(100);
        assert!(cache.initialize().await.is_ok());
        assert_eq!(cache.capacity(), 100);
        assert!(cache.shutdown().await.is_ok());
    }

    #[tokio::test]
    async fn test_cache_put_get() {
        let mut cache = CacheManager::new(10);
        cache.initialize().await.unwrap();

        cache.put("key1", "value1", None).await;
        let value = cache.get("key1").await;
        assert_eq!(value, Some("value1".to_string()));

        cache.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cache_lru_eviction() {
        let mut cache = CacheManager::new(2);
        cache.initialize().await.unwrap();

        // Fill cache to capacity
        cache.put("key1", "value1", None).await;
        cache.put("key2", "value2", None).await;

        // Access key1 to make it recently used
        cache.get("key1").await;

        // Add key3, which should evict key2 (least recently used)
        cache.put("key3", "value3", None).await;

        assert!(cache.get("key1").await.is_some()); // Still in cache
        assert!(cache.get("key2").await.is_none()); // Evicted
        assert!(cache.get("key3").await.is_some()); // New entry

        cache.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cache_ttl_expiration() {
        let mut cache = CacheManager::new(10);
        cache.initialize().await.unwrap();

        // Add entry with very short TTL (1 second)
        cache.put("key1", "value1", Some(1)).await;
        assert!(cache.get("key1").await.is_some());

        // Wait for expiration
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Entry should be expired
        assert!(cache.get("key1").await.is_none());

        cache.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cache_evict_expired() {
        let mut cache = CacheManager::new(10);
        cache.initialize().await.unwrap();

        // Add entries with short TTL
        cache.put("key1", "value1", Some(1)).await;
        cache.put("key2", "value2", None).await; // 1 hour TTL

        // Wait for key1 to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Evict expired entries
        let evicted = cache.evict_expired().await;
        assert_eq!(evicted, 1);

        assert!(cache.get("key1").await.is_none()); // Expired and evicted
        assert!(cache.get("key2").await.is_some()); // Still valid

        cache.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cache_usage_percentage() {
        let mut cache = CacheManager::new(10);
        cache.initialize().await.unwrap();

        assert_eq!(cache.get_usage_percentage().await, 0.0);

        cache.put("key1", "value1", None).await;
        assert_eq!(cache.get_usage_percentage().await, 0.1);

        // Fill to capacity
        for i in 2..=10 {
            cache
                .put(format!("key{}", i), format!("value{}", i), None)
                .await;
        }
        assert_eq!(cache.get_usage_percentage().await, 1.0);

        cache.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let mut cache = CacheManager::new(10);
        cache.initialize().await.unwrap();

        cache.put("key1", "value1", None).await;
        cache.get("key1").await; // Hit
        cache.get("key2").await; // Miss

        let stats = cache.get_stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);

        cache.shutdown().await.unwrap();
    }
}
