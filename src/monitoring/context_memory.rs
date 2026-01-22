//! Context Memory Monitoring Module
//!
//! Provides monitoring and tracking of context memory usage for AI models,
//! particularly for Cursor AI integration. Tracks context size, changes, and
//! memory usage patterns.
//!
//! # Features
//!
//! - **Context Size Tracking**: Monitor current, maximum, and average context sizes
//! - **Change Tracking**: Track file additions, modifications, and deletions
//! - **Memory Usage**: Monitor RAM, disk, and cache usage
//! - **Optimization**: Detect and suggest context optimizations
//!
//! # Example
//!
//! ```rust,no_run
//! use poolai::monitoring::context_memory::ContextMemoryMonitor;
//!
//! # async fn example() -> Result<(), poolai::core::error::AppError> {
//! let monitor = ContextMemoryMonitor::new();
//!
//! // Track a file addition
//! monitor.track_file_added("src/main.rs", 1024).await?;
//!
//! // Get current metrics
//! let metrics = monitor.get_metrics().await;
//! println!("Context size: {} bytes", metrics.current_size);
//! # Ok(())
//! # }
//! ```

use crate::core::error::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Represents a change in context
#[derive(Debug, Clone)]
pub struct ContextChange {
    pub timestamp: Instant,
    pub change_type: ChangeType,
    pub file_path: String,
    pub size_bytes: usize,
}

/// Type of context change
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    FileAdded,
    FileModified,
    FileDeleted,
    ContextCleared,
}

/// Memory usage metrics
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub ram_bytes: usize,
    pub disk_bytes: usize,
    pub cache_bytes: usize,
    pub timestamp: Instant,
}

/// Context memory metrics
#[derive(Debug, Clone)]
pub struct ContextMetrics {
    pub current_size: usize,
    pub max_size: usize,
    pub average_size: f64,
    pub file_count: usize,
    pub changes_count: usize,
    pub memory_usage: MemoryUsage,
    pub last_update: Instant,
}

/// Context Memory Monitor
///
/// Monitors and tracks context memory usage for AI models.
/// Provides metrics on context size, changes, and memory usage.
///
/// # Thread Safety
///
/// All methods are async and thread-safe, using `Arc<RwLock<>>` internally.
///
/// # Example
///
/// ```rust,no_run
/// use poolai::monitoring::context_memory::ContextMemoryMonitor;
///
/// # async fn example() -> Result<(), poolai::core::error::AppError> {
/// let monitor = ContextMemoryMonitor::new();
///
/// // Track changes
/// monitor.track_file_added("src/main.rs", 1024).await?;
/// monitor.track_file_modified("src/lib.rs", 2048).await?;
///
/// // Get metrics
/// let metrics = monitor.get_metrics().await;
/// println!("Context: {} bytes, Files: {}", metrics.current_size, metrics.file_count);
/// # Ok(())
/// # }
/// ```
pub struct ContextMemoryMonitor {
    files: Arc<RwLock<HashMap<String, usize>>>,
    changes: Arc<RwLock<Vec<ContextChange>>>,
    max_size: Arc<RwLock<usize>>,
    total_size_history: Arc<RwLock<Vec<(Instant, usize)>>>,
    memory_usage_history: Arc<RwLock<Vec<MemoryUsage>>>,
}

impl Default for ContextMemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMemoryMonitor {
    /// Create a new context memory monitor
    ///
    /// # Example
    ///
    /// ```rust
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// let monitor = ContextMemoryMonitor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            files: Arc::new(RwLock::new(HashMap::new())),
            changes: Arc::new(RwLock::new(Vec::new())),
            max_size: Arc::new(RwLock::new(0)),
            total_size_history: Arc::new(RwLock::new(Vec::new())),
            memory_usage_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Track a file addition to context
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file
    /// * `size_bytes` - Size of the file in bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// monitor.track_file_added("src/main.rs", 1024).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_file_added(
        &self,
        file_path: &str,
        size_bytes: usize,
    ) -> Result<(), AppError> {
        let mut files = self.files.write().await;
        files.insert(file_path.to_string(), size_bytes);

        let mut changes = self.changes.write().await;
        changes.push(ContextChange {
            timestamp: Instant::now(),
            change_type: ChangeType::FileAdded,
            file_path: file_path.to_string(),
            size_bytes,
        });

        // Update max size
        let current_size: usize = files.values().sum();
        let mut max_size = self.max_size.write().await;
        if current_size > *max_size {
            *max_size = current_size;
        }

        // Record size history
        let mut history = self.total_size_history.write().await;
        history.push((Instant::now(), current_size));
        if history.len() > 1000 {
            history.drain(0..100);
        }

        // Track memory usage
        self.update_memory_usage().await?;

        Ok(())
    }

    /// Track a file modification in context
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file
    /// * `new_size_bytes` - New size of the file in bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// monitor.track_file_modified("src/lib.rs", 2048).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_file_modified(
        &self,
        file_path: &str,
        new_size_bytes: usize,
    ) -> Result<(), AppError> {
        let mut files = self.files.write().await;
        files.insert(file_path.to_string(), new_size_bytes);

        let mut changes = self.changes.write().await;
        changes.push(ContextChange {
            timestamp: Instant::now(),
            change_type: ChangeType::FileModified,
            file_path: file_path.to_string(),
            size_bytes: new_size_bytes,
        });

        // Update max size
        let current_size: usize = files.values().sum();
        let mut max_size = self.max_size.write().await;
        if current_size > *max_size {
            *max_size = current_size;
        }

        // Record size history
        let mut history = self.total_size_history.write().await;
        history.push((Instant::now(), current_size));
        if history.len() > 1000 {
            history.drain(0..100);
        }

        // Track memory usage
        self.update_memory_usage().await?;

        Ok(())
    }

    /// Track a file deletion from context
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// monitor.track_file_deleted("src/old.rs").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_file_deleted(&self, file_path: &str) -> Result<(), AppError> {
        let mut files = self.files.write().await;
        let size_bytes = files.remove(file_path).unwrap_or(0);

        let mut changes = self.changes.write().await;
        changes.push(ContextChange {
            timestamp: Instant::now(),
            change_type: ChangeType::FileDeleted,
            file_path: file_path.to_string(),
            size_bytes,
        });

        // Record size history
        let current_size: usize = files.values().sum();
        let mut history = self.total_size_history.write().await;
        history.push((Instant::now(), current_size));
        if history.len() > 1000 {
            history.drain(0..100);
        }

        // Track memory usage
        self.update_memory_usage().await?;

        Ok(())
    }

    /// Track context cleared
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// monitor.track_context_cleared().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn track_context_cleared(&self) -> Result<(), AppError> {
        let mut files = self.files.write().await;
        let total_size: usize = files.values().sum();
        files.clear();

        let mut changes = self.changes.write().await;
        changes.push(ContextChange {
            timestamp: Instant::now(),
            change_type: ChangeType::ContextCleared,
            file_path: String::new(),
            size_bytes: total_size,
        });

        // Record size history
        let mut history = self.total_size_history.write().await;
        history.push((Instant::now(), 0));
        if history.len() > 1000 {
            history.drain(0..100);
        }

        // Track memory usage
        self.update_memory_usage().await?;

        Ok(())
    }

    /// Get current context metrics
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// let metrics = monitor.get_metrics().await;
    /// println!("Size: {} bytes, Files: {}", metrics.current_size, metrics.file_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_metrics(&self) -> ContextMetrics {
        let files = self.files.read().await;
        let changes = self.changes.read().await;
        let max_size = self.max_size.read().await;
        let history = self.total_size_history.read().await;
        let memory_history = self.memory_usage_history.read().await;

        let current_size: usize = files.values().sum();
        let file_count = files.len();
        let changes_count = changes.len();

        // Calculate average size
        let average_size = if history.is_empty() {
            current_size as f64
        } else {
            let sum: usize = history.iter().map(|(_, size)| size).sum();
            sum as f64 / history.len() as f64
        };

        // Get latest memory usage
        let memory_usage = memory_history
            .last()
            .cloned()
            .unwrap_or_else(|| MemoryUsage {
                ram_bytes: 0,
                disk_bytes: 0,
                cache_bytes: 0,
                timestamp: Instant::now(),
            });

        ContextMetrics {
            current_size,
            max_size: *max_size,
            average_size,
            file_count,
            changes_count,
            memory_usage,
            last_update: Instant::now(),
        }
    }

    /// Get recent changes
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of changes to return
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// let changes = monitor.get_recent_changes(10).await;
    /// for change in changes {
    ///     println!("{:?}: {}", change.change_type, change.file_path);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_recent_changes(&self, limit: usize) -> Vec<ContextChange> {
        let changes = self.changes.read().await;
        changes.iter().rev().take(limit).cloned().collect()
    }

    /// Get changes within a time window
    ///
    /// # Arguments
    ///
    /// * `duration` - Time window to look back
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// let changes = monitor.get_changes_in_window(Duration::from_secs(60)).await;
    /// println!("Changes in last minute: {}", changes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_changes_in_window(&self, duration: Duration) -> Vec<ContextChange> {
        let changes = self.changes.read().await;
        let cutoff = Instant::now() - duration;

        changes
            .iter()
            .filter(|change| change.timestamp >= cutoff)
            .cloned()
            .collect()
    }

    /// Update memory usage metrics
    ///
    /// This is a simplified implementation. In a real system, you would
    /// query the OS for actual memory usage.
    async fn update_memory_usage(&self) -> Result<(), AppError> {
        let files = self.files.read().await;
        let current_size: usize = files.values().sum();

        // Estimate memory usage (simplified)
        // In production, use system APIs to get actual memory usage
        let ram_bytes = current_size; // Assume all context is in RAM
        let disk_bytes = current_size / 2; // Assume some is cached on disk
        let cache_bytes = current_size / 4; // Assume some is in cache

        let mut memory_history = self.memory_usage_history.write().await;
        memory_history.push(MemoryUsage {
            ram_bytes,
            disk_bytes,
            cache_bytes,
            timestamp: Instant::now(),
        });

        // Limit history size
        if memory_history.len() > 1000 {
            memory_history.drain(0..100);
        }

        Ok(())
    }

    /// Get memory usage history
    ///
    /// # Arguments
    ///
    /// * `duration` - Time window to look back
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// let history = monitor.get_memory_usage_history(Duration::from_secs(300)).await;
    /// for usage in history {
    ///     println!("RAM: {} bytes, Disk: {} bytes", usage.ram_bytes, usage.disk_bytes);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_memory_usage_history(&self, duration: Duration) -> Vec<MemoryUsage> {
        let memory_history = self.memory_usage_history.read().await;
        let cutoff = Instant::now() - duration;

        memory_history
            .iter()
            .filter(|usage| usage.timestamp >= cutoff)
            .cloned()
            .collect()
    }

    /// Suggest context optimizations
    ///
    /// Analyzes current context and suggests optimizations to reduce memory usage.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use poolai::monitoring::context_memory::ContextMemoryMonitor;
    ///
    /// # async fn example() -> Result<(), poolai::core::error::AppError> {
    /// let monitor = ContextMemoryMonitor::new();
    /// let suggestions = monitor.suggest_optimizations().await;
    /// for suggestion in suggestions {
    ///     println!("Suggestion: {}", suggestion);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn suggest_optimizations(&self) -> Vec<String> {
        let metrics = self.get_metrics().await;
        let mut suggestions = Vec::new();

        // Suggest based on context size
        if metrics.current_size > 10_000_000 {
            // 10MB
            suggestions.push(
                "Context size is large (>10MB). Consider removing unused files or splitting context into smaller chunks.".to_string(),
            );
        }

        // Suggest based on file count
        if metrics.file_count > 100 {
            suggestions.push(
                "Many files in context (>100). Consider grouping related files or using summaries."
                    .to_string(),
            );
        }

        // Suggest based on memory usage
        if metrics.memory_usage.ram_bytes > 50_000_000 {
            // 50MB
            suggestions.push(
                "High RAM usage (>50MB). Consider using disk caching for less frequently accessed files.".to_string(),
            );
        }

        // Suggest based on changes frequency
        let recent_changes = self.get_changes_in_window(Duration::from_secs(60)).await;
        if recent_changes.len() > 50 {
            suggestions.push(
                "High change frequency (>50 changes/min). Consider batching changes or reducing update frequency.".to_string(),
            );
        }

        suggestions
    }
}
