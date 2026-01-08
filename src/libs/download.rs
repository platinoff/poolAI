//! Library Download Module
//!
//! Provides:
//! - HTTP client for downloading libraries
//! - Archive extraction (tar, zip)
//! - Checksum verification
//! - Download progress tracking

use crate::core::error::AppError;
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;

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
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to create HTTP client. Context: Cannot initialize HTTP client for library download. \
            Suggestion: Check network configuration and ensure reqwest crate is properly configured. \
            URL: '{}', Error: {}",
            url, e
        )))?;

    // Download file
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to download library. Context: HTTP request failed during library download. \
            Suggestion: Check network connectivity, verify URL is accessible, and ensure firewall allows outbound connections. \
            URL: '{}', Error: {}",
            url, e
        )))?;

    if !response.status().is_success() {
        return Err(AppError::ConfigError(format!(
            "Failed to download library: HTTP {}. Context: Server returned non-success status code. \
            Suggestion: Verify the download URL is correct and the server is accessible. \
            URL: '{}', Status: {}",
            response.status(), url, response.status()
        )));
    }

    // Get content length for progress tracking
    let content_length = response.content_length().unwrap_or(0);
    info!("Downloading {} bytes", content_length);

    // Create destination file
    let mut file = tokio::fs::File::create(destination)
        .await
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to create destination file. Context: Cannot create file for downloaded library. \
            Suggestion: Check filesystem permissions and ensure parent directory exists. \
            Path: '{}', Error: {}",
            destination.display(), e
        )))?;

    // Download with progress tracking
    let mut stream = response.bytes_stream();
    let mut hasher = Sha256::new();
    let mut downloaded = 0u64;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| AppError::ConfigError(format!(
            "Download error. Context: Failed to read chunk from HTTP response stream. \
            Suggestion: Check network stability and retry the download. \
            URL: '{}', Downloaded: {} bytes, Error: {}",
            url, downloaded, e
        )))?;

        // Write chunk to file
        file.write_all(&chunk)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to write file. Context: Cannot write downloaded chunk to destination file. \
                Suggestion: Check disk space and filesystem permissions. \
                Path: '{}', Downloaded: {} bytes, Error: {}",
                destination.display(), downloaded, e
            )))?;

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
    file.sync_all()
        .await
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to sync file. Context: Cannot flush file data to disk after download. \
            Suggestion: Check disk space and filesystem integrity. \
            Path: '{}', Error: {}",
            destination.display(), e
        )))?;

    // Verify checksum if provided
    let calculated_checksum = format!("{:x}", hasher.finalize());
    if let Some(expected) = expected_checksum {
        if calculated_checksum != expected {
            return Err(AppError::ConfigError(format!(
                "Checksum mismatch. Context: Downloaded file integrity verification failed. \
                Suggestion: File may be corrupted or tampered with. Try downloading again or verify the source. \
                Expected: '{}', Calculated: '{}', Path: '{}'",
                expected, calculated_checksum, destination.display()
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
pub async fn extract_archive(archive_path: &Path, destination: &Path) -> Result<PathBuf, AppError> {
    info!(
        "Extracting archive: {:?} to {:?}",
        archive_path, destination
    );

    // Ensure destination directory exists
    tokio::fs::create_dir_all(destination).await.map_err(|e| {
        AppError::ConfigError(format!(
            "Failed to create destination directory. Context: Cannot create directory for archive extraction. \
            Suggestion: Check filesystem permissions and ensure parent directory exists. \
            Path: '{}', Error: {}",
            destination.display(), e
        ))
    })?;

    // Determine archive type from extension
    let extension = archive_path
        .extension()
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
                "Unsupported archive format. Context: Archive file extension is not recognized. \
                Suggestion: Supported formats are: .tar.gz, .tgz, .tar, .zip. \
                Archive: '{}', Extension: '{}'",
                archive_path.display(), extension
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
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to open archive. Context: Cannot open tar.gz archive file for extraction. \
            Suggestion: Check file permissions and verify archive file is not corrupted. \
            Archive: '{}', Error: {}",
            archive_path.display(), e
        )))?;

    let gz = GzDecoder::new(BufReader::new(file));
    let mut archive = tar::Archive::new(gz);

    archive
        .unpack(destination)
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to extract tar.gz. Context: Cannot extract contents from tar.gz archive. \
            Suggestion: Verify archive integrity and ensure destination directory is writable. \
            Archive: '{}', Destination: '{}', Error: {}",
            archive_path.display(), destination.display(), e
        )))?;

    Ok(())
}

/// Extract tar archive
async fn extract_tar(archive_path: &Path, destination: &Path) -> Result<(), AppError> {
    use std::fs::File;

    // Run blocking I/O in spawn_blocking
    let archive_path = archive_path.to_path_buf();
    let destination = destination.to_path_buf();

    let archive_path_str = archive_path.display().to_string();
    let destination_str = destination.display().to_string();
    tokio::task::spawn_blocking(move || {
        let file = File::open(&archive_path)
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to open archive. Context: Cannot open tar archive file for extraction. \
                Suggestion: Check file permissions and verify archive file is not corrupted. \
                Archive: '{}', Error: {}",
                archive_path_str, e
            )))?;

        let mut archive = tar::Archive::new(file);

        archive
            .unpack(&destination)
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to extract tar. Context: Cannot extract contents from tar archive. \
                Suggestion: Verify archive integrity and ensure destination directory is writable. \
                Archive: '{}', Destination: '{}', Error: {}",
                archive_path_str, destination_str, e
            )))?;

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
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to open archive. Context: Cannot open zip archive file for extraction. \
                Suggestion: Check file permissions and verify archive file is not corrupted. \
                Archive: '{}', Error: {}",
                archive_path.display(), e
            )))?;

        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to read zip archive. Context: Cannot parse zip archive structure. \
                Suggestion: Verify archive integrity and ensure file is a valid ZIP archive. \
                Archive: '{}', Error: {}",
                archive_path.display(), e
            )))?;

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| AppError::ConfigError(format!(
                    "Failed to read zip entry. Context: Cannot read entry from zip archive. \
                    Suggestion: Archive may be corrupted or incomplete. Try re-downloading. \
                    Archive: '{}', Entry index: {}, Error: {}",
                    archive_path.display(), i, e
                )))?;

            let outpath = destination.join(file.name());

            if file.name().ends_with('/') {
                // Directory
                std::fs::create_dir_all(&outpath).map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to create directory. Context: Cannot create directory from zip archive. \
                        Suggestion: Check filesystem permissions and ensure parent directory exists. \
                        Path: '{}', Error: {}",
                        outpath.display(), e
                    ))
                })?;
            } else {
                // File
                if let Some(parent) = outpath.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        AppError::ConfigError(format!(
                            "Failed to create parent directory. Context: Cannot create parent directory for extracted file. \
                            Suggestion: Check filesystem permissions. \
                            Path: '{}', Error: {}",
                            parent.display(), e
                        ))
                    })?;
                }

                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| AppError::ConfigError(format!(
                        "Failed to create file. Context: Cannot create extracted file from zip archive. \
                        Suggestion: Check filesystem permissions and disk space. \
                        Path: '{}', Error: {}",
                        outpath.display(), e
                    )))?;

                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).map_err(|e| {
                    AppError::ConfigError(format!(
                        "Failed to read zip entry. Context: Cannot read file contents from zip archive. \
                        Suggestion: Archive may be corrupted. Try re-downloading. \
                        Entry: '{}', Error: {}",
                        file.name(), e
                    ))
                })?;

                std::io::Write::write_all(&mut outfile, &buffer)
                    .map_err(|e| AppError::ConfigError(format!(
                        "Failed to write file. Context: Cannot write extracted file contents to disk. \
                        Suggestion: Check disk space and filesystem permissions. \
                        Path: '{}', Error: {}",
                        outpath.display(), e
                    )))?;
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
    let mut file = tokio::fs::File::open(file_path)
        .await
        .map_err(|e| AppError::ConfigError(format!(
            "Failed to open file. Context: Cannot open file for checksum calculation. \
            Suggestion: Check file permissions and ensure file exists. \
            Path: '{}', Error: {}",
            file_path.display(), e
        )))?;

    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];

    loop {
        let n = file
            .read(&mut buffer)
            .await
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to read file. Context: Cannot read file data for checksum calculation. \
                Suggestion: Check file permissions and verify file is not locked by another process. \
                Path: '{}', Error: {}",
                file_path.display(), e
            )))?;

        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Create a tar.gz archive from a directory
pub async fn create_artifact_archive(
    source_dir: &Path,
    archive_path: &Path,
) -> Result<PathBuf, AppError> {
    info!(
        "Creating artifact archive from {:?} to {:?}",
        source_dir, archive_path
    );

    // Ensure parent directory exists
    if let Some(parent) = archive_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to create archive parent directory. Context: Cannot create parent directory for artifact archive. \
                Suggestion: Check filesystem permissions and ensure parent directory path is valid. \
                Path: '{}', Error: {}",
                parent.display(), e
            ))
        })?;
    }

    let source_dir = source_dir.to_path_buf();
    let archive_path = archive_path.to_path_buf();

    let archive_path_clone = archive_path.clone();

    // Run blocking I/O in spawn_blocking
    tokio::task::spawn_blocking(move || {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::fs::File;

        let file = File::create(&archive_path)
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to create archive file. Context: Cannot create tar.gz archive file. \
                Suggestion: Check filesystem permissions and disk space. \
                Path: '{}', Error: {}",
                archive_path.display(), e
            )))?;

        let gz = GzEncoder::new(file, Compression::default());
        let mut tar = tar::Builder::new(gz);

        tar.append_dir_all(".", &source_dir).map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to add directory to archive. Context: Cannot add source directory contents to tar archive. \
                Suggestion: Check source directory permissions and verify directory is accessible. \
                Source: '{}', Archive: '{}', Error: {}",
                source_dir.display(), archive_path.display(), e
            ))
        })?;

        tar.finish()
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to finish archive. Context: Cannot finalize tar.gz archive creation. \
                Suggestion: Check disk space and filesystem integrity. \
                Archive: '{}', Error: {}",
                archive_path.display(), e
            )))?;

        Ok::<PathBuf, AppError>(archive_path)
    })
    .await
    .map_err(|e| AppError::ConfigError(format!("Task join error: {}", e)))??;

    info!(
        "Artifact archive created successfully: {:?}",
        archive_path_clone
    );
    Ok(archive_path_clone)
}
