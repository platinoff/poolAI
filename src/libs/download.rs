//! Library Download Module
//!
//! Provides:
//! - HTTP client for downloading libraries
//! - Archive extraction (tar, zip)
//! - Checksum verification
//! - Download progress tracking

use crate::core::error::AppError;
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tracing::info;
use futures_util::StreamExt;

/// Download progress callback
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send + Sync>;

/// Download a library from URL
pub async fn download_library(
    url: &str,
    destination: &Path,
    expected_checksum: Option<&str>,
) -> Result<PathBuf, AppError> {
    info!("Downloading library from: {}", url);
    
    // Create HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout
        .build()
        .map_err(|e| AppError::ConfigError(format!("Failed to create HTTP client: {}", e)))?;
    
    // Download file
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| AppError::ConfigError(format!("Failed to download library: {}", e)))?;
    
    if !response.status().is_success() {
        return Err(AppError::ConfigError(format!(
            "Failed to download library: HTTP {}",
            response.status()
        )));
    }
    
    // Get content length for progress tracking
    let content_length = response.content_length().unwrap_or(0);
    info!("Downloading {} bytes", content_length);
    
    // Create destination file
    let mut file = tokio::fs::File::create(destination)
        .await
        .map_err(|e| AppError::ConfigError(format!("Failed to create destination file: {}", e)))?;
    
    // Download with progress tracking
    let mut stream = response.bytes_stream();
    let mut hasher = Sha256::new();
    let mut downloaded = 0u64;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| AppError::ConfigError(format!("Download error: {}", e)))?;
        
        // Write chunk to file
        file.write_all(&chunk).await
            .map_err(|e| AppError::ConfigError(format!("Failed to write file: {}", e)))?;
        
        // Update checksum
        hasher.update(&chunk);
        
        // Update progress
        downloaded += chunk.len() as u64;
        if content_length > 0 && downloaded % (content_length / 10).max(1) == 0 {
            let progress = (downloaded * 100) / content_length;
            info!("Download progress: {}%", progress);
        }
    }
    
    // Finalize file
    file.sync_all().await
        .map_err(|e| AppError::ConfigError(format!("Failed to sync file: {}", e)))?;
    
    // Verify checksum if provided
    let calculated_checksum = format!("{:x}", hasher.finalize());
    if let Some(expected) = expected_checksum {
        if calculated_checksum != expected {
            return Err(AppError::ConfigError(format!(
                "Checksum mismatch: expected {}, got {}",
                expected, calculated_checksum
            )));
        }
        info!("Checksum verified successfully");
    } else {
        info!("Downloaded checksum: {}", calculated_checksum);
    }
    
    info!("Library downloaded successfully to: {:?}", destination);
    Ok(destination.to_path_buf())
}

/// Extract archive (tar.gz, tar, zip)
pub async fn extract_archive(
    archive_path: &Path,
    destination: &Path,
) -> Result<PathBuf, AppError> {
    info!("Extracting archive: {:?} to {:?}", archive_path, destination);
    
    // Ensure destination directory exists
    tokio::fs::create_dir_all(destination).await
        .map_err(|e| AppError::ConfigError(format!("Failed to create destination directory: {}", e)))?;
    
    // Determine archive type from extension
    let extension = archive_path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    
    match extension {
        "gz" | "tgz" => {
            // Handle tar.gz
            extract_tar_gz(archive_path, destination).await?;
        }
        "tar" => {
            // Handle tar
            extract_tar(archive_path, destination).await?;
        }
        "zip" => {
            // Handle zip
            extract_zip(archive_path, destination).await?;
        }
        _ => {
            return Err(AppError::ConfigError(format!(
                "Unsupported archive format: {}",
                extension
            )));
        }
    }
    
    info!("Archive extracted successfully to: {:?}", destination);
    Ok(destination.to_path_buf())
}

/// Extract tar.gz archive
async fn extract_tar_gz(archive_path: &Path, destination: &Path) -> Result<(), AppError> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use std::io::BufReader;
    
    let file = File::open(archive_path)
        .map_err(|e| AppError::ConfigError(format!("Failed to open archive: {}", e)))?;
    
    let gz = GzDecoder::new(BufReader::new(file));
    let mut archive = tar::Archive::new(gz);
    
    archive.unpack(destination)
        .map_err(|e| AppError::ConfigError(format!("Failed to extract tar.gz: {}", e)))?;
    
    Ok(())
}

/// Extract tar archive
async fn extract_tar(archive_path: &Path, destination: &Path) -> Result<(), AppError> {
    use std::fs::File;
    
    // Run blocking I/O in spawn_blocking
    let archive_path = archive_path.to_path_buf();
    let destination = destination.to_path_buf();
    
    tokio::task::spawn_blocking(move || {
        let file = File::open(&archive_path)
            .map_err(|e| AppError::ConfigError(format!("Failed to open archive: {}", e)))?;
        
        let mut archive = tar::Archive::new(file);
        
        archive.unpack(&destination)
            .map_err(|e| AppError::ConfigError(format!("Failed to extract tar: {}", e)))?;
        
        Ok::<(), AppError>(())
    })
    .await
    .map_err(|e| AppError::ConfigError(format!("Task join error: {}", e)))??;
    
    Ok(())
}

/// Extract zip archive
async fn extract_zip(archive_path: &Path, destination: &Path) -> Result<(), AppError> {
    use std::fs::File;
    use std::io::Read;
    
    // Run blocking I/O in spawn_blocking
    let archive_path = archive_path.to_path_buf();
    let destination = destination.to_path_buf();
    
    tokio::task::spawn_blocking(move || {
        let file = File::open(&archive_path)
            .map_err(|e| AppError::ConfigError(format!("Failed to open archive: {}", e)))?;
        
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| AppError::ConfigError(format!("Failed to read zip archive: {}", e)))?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| AppError::ConfigError(format!("Failed to read zip entry: {}", e)))?;
            
            let outpath = destination.join(file.name());
            
            if file.name().ends_with('/') {
                // Directory
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| AppError::ConfigError(format!("Failed to create directory: {}", e)))?;
            } else {
                // File
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| AppError::ConfigError(format!("Failed to create parent directory: {}", e)))?;
                }
                
                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| AppError::ConfigError(format!("Failed to create file: {}", e)))?;
                
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| AppError::ConfigError(format!("Failed to read zip entry: {}", e)))?;
                
                std::io::Write::write_all(&mut outfile, &buffer)
                    .map_err(|e| AppError::ConfigError(format!("Failed to write file: {}", e)))?;
            }
        }
        
        Ok::<(), AppError>(())
    })
    .await
    .map_err(|e| AppError::ConfigError(format!("Task join error: {}", e)))??;
    
    Ok(())
}

/// Calculate file checksum
pub async fn calculate_checksum(file_path: &Path) -> Result<String, AppError> {
    let mut file = tokio::fs::File::open(file_path).await
        .map_err(|e| AppError::ConfigError(format!("Failed to open file: {}", e)))?;
    
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];
    
    loop {
        let n = file.read(&mut buffer).await
            .map_err(|e| AppError::ConfigError(format!("Failed to read file: {}", e)))?;
        
        if n == 0 {
            break;
        }
        
        hasher.update(&buffer[..n]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

