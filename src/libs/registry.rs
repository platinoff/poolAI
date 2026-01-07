//! Library Registry - Registry of available libraries
//!
//! Provides:
//! - Library discovery
//! - Search and filtering
//! - Metadata management

use crate::core::error::AppError;
use crate::libs::LibraryInfo;
use std::cmp::Ordering;
use std::collections::HashMap;
use tracing::info;

/// Library Registry - Manages registry of available libraries
pub struct LibraryRegistry {
    libraries: HashMap<String, Vec<String>>, // name -> versions
    metadata: HashMap<String, LibraryInfo>,  // name:version -> LibraryInfo
    download_urls: HashMap<String, String>,  // name:version -> download URL
}

impl LibraryRegistry {
    /// Create new library registry
    pub fn new() -> Self {
        Self {
            libraries: HashMap::new(),
            metadata: HashMap::new(),
            download_urls: HashMap::new(),
        }
    }

    /// Initialize registry
    pub async fn initialize(&mut self) -> Result<(), AppError> {
        info!("Initializing Library Registry");

        // Future improvement: Load registry from remote source or local cache
        // 1. Check for local cache file (e.g., registry.json or registry.toml)
        //    - Load from config.base_path.join("registry.json")
        //    - Parse JSON/TOML into registry structure
        //    - Handle file not found gracefully (empty registry)
        // 2. If local cache doesn't exist or is stale, fetch from remote source
        //    - Define remote registry URL in config (e.g., registry_url field)
        //    - Use HTTP client (reqwest crate) to fetch registry data
        //    - Parse remote response (JSON/YAML)
        //    - Cache locally for offline use
        // 3. Implement cache refresh logic
        //    - Check cache age (last_updated timestamp)
        //    - Refresh if cache is older than TTL (e.g., 24 hours)
        //    - Background refresh to avoid blocking initialization
        // 4. Handle errors gracefully
        //    - Fallback to empty registry if remote fetch fails
        //    - Log warnings for network errors
        //    - Continue with cached data if available
        // Example:
        //    if let Ok(cache_file) = fs::read_to_string(cache_path).await {
        //        self.load_from_json(&cache_file)?;
        //    } else if let Some(remote_url) = &config.registry_url {
        //        let registry_data = fetch_registry(remote_url).await?;
        //        self.load_from_json(&registry_data)?;
        //        self.save_cache(cache_path).await?;
        //    }
        // For now, initialize with empty registry

        info!("Library Registry initialized");
        Ok(())
    }

    /// Register a library in the registry
    pub async fn register(
        &mut self,
        name: &str,
        version: &str,
        info: LibraryInfo,
    ) -> Result<(), AppError> {
        let versions = self
            .libraries
            .entry(name.to_string())
            .or_insert_with(Vec::new);

        if !versions.contains(&version.to_string()) {
            versions.push(version.to_string());
            // Keep versions sorted (basic semver: MAJOR.MINOR.PATCH)
            versions.sort_by(|a, b| semver_cmp(a, b));
        }

        let key = format!("{}:{}", name, version);
        self.metadata.insert(key, info);

        Ok(())
    }

    /// Get available versions for a library
    pub fn get_versions(&self, name: &str) -> Option<&Vec<String>> {
        self.libraries.get(name)
    }

    /// Get latest version for a library
    pub fn get_latest_version(&self, name: &str) -> Option<String> {
        if let Some(versions) = self.libraries.get(name) {
            // Versions are sorted, so last is latest
            versions.last().cloned()
        } else {
            None
        }
    }

    /// Get download URL for a library version
    pub fn get_download_url(&self, name: &str, version: &str) -> Option<String> {
        let key = format!("{}:{}", name, version);
        self.download_urls.get(&key).cloned()
    }

    /// Set download URL for a library version
    pub fn set_download_url(&mut self, name: &str, version: &str, url: &str) {
        let key = format!("{}:{}", name, version);
        self.download_urls.insert(key, url.to_string());
    }

    /// Search libraries by name
    pub fn search(&self, query: &str) -> Vec<String> {
        self.libraries
            .keys()
            .filter(|name| name.contains(query))
            .cloned()
            .collect()
    }

    /// Get library metadata
    pub fn get_metadata(&self, name: &str, version: &str) -> Option<&LibraryInfo> {
        let key = format!("{}:{}", name, version);
        self.metadata.get(&key)
    }
}

fn semver_cmp(a: &str, b: &str) -> Ordering {
    let a_parts: Vec<u32> = a.split('.').filter_map(|s| s.parse::<u32>().ok()).collect();
    let b_parts: Vec<u32> = b.split('.').filter_map(|s| s.parse::<u32>().ok()).collect();

    for i in 0..3 {
        let av = a_parts.get(i).copied().unwrap_or(0);
        let bv = b_parts.get(i).copied().unwrap_or(0);
        match av.cmp(&bv) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    a.cmp(b)
}

impl Default for LibraryRegistry {
    fn default() -> Self {
        Self::new()
    }
}
