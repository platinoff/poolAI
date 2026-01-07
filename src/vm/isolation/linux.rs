//! Linux-specific isolation implementations
//!
//! Uses Linux namespaces, cgroups, and other Linux-specific features
//! for network and filesystem isolation.

use crate::core::error::AppError;
use crate::vm::isolation::{
    FilesystemIsolationConfig, FilesystemIsolator, NetworkIsolationConfig, NetworkIsolator,
};
use std::path::PathBuf;
use tracing::{info, warn};

#[cfg(feature = "vm-isolation-linux")]
use nix::mount::{mount, MsFlags};
#[cfg(feature = "vm-isolation-linux")]
use nix::sched::{unshare, CloneFlags};
#[cfg(feature = "vm-isolation-linux")]
use nix::unistd::chroot;
#[cfg(feature = "vm-isolation-linux")]
use std::fs;
#[cfg(feature = "vm-isolation-linux")]
use std::process::Command;

/// Linux network isolator using network namespaces
pub struct LinuxNetworkIsolator;

impl LinuxNetworkIsolator {
    pub fn new() -> Self {
        Self
    }

    /// Set up loopback interface in the current network namespace
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_loopback_interface() -> Result<(), AppError> {
        // Use `ip` command to bring up loopback interface
        // This is simpler than using raw socket calls
        let output = Command::new("ip")
            .args(&["link", "set", "lo", "up"])
            .output()
            .map_err(|e| AppError::ConfigError(format!("Failed to execute ip command: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to set up loopback interface: {}",
                stderr
            )));
        }

        Ok(())
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_loopback_interface() -> Result<(), AppError> {
        // No-op when feature is not enabled
        Ok(())
    }
}

impl NetworkIsolator for LinuxNetworkIsolator {
    fn apply_network_isolation(
        &self,
        process_id: u32,
        config: &NetworkIsolationConfig,
    ) -> Result<(), AppError> {
        if !config.enabled {
            return Ok(());
        }

        // Validate configuration
        if !config.allow_loopback
            && config.allowed_interfaces.is_empty()
            && config.allowed_ports.is_empty()
        {
            return Err(AppError::ConfigError(
                "Network isolation configuration would block all network access. At least one of allow_loopback, allowed_interfaces, or allowed_ports must be enabled.".to_string(),
            ));
        }

        // Validate process exists
        // Note: In a real implementation, we would check if the process exists
        // For now, we just validate the process_id is non-zero
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        // Log configuration details
        info!(
            "Applying network isolation to process {}: loopback={}, interfaces={:?}, ports={:?}",
            process_id, config.allow_loopback, config.allowed_interfaces, config.allowed_ports
        );

        #[cfg(feature = "vm-isolation-linux")]
        {
            // Attempt to create network namespace
            // Note: This requires root privileges or CAP_NET_ADMIN
            match unshare(CloneFlags::CLONE_NEWNET) {
                Ok(_) => {
                    info!(
                        "Successfully created network namespace for process {}",
                        process_id
                    );

                    // Set up loopback interface if allowed
                    if config.allow_loopback {
                        match Self::setup_loopback_interface() {
                            Ok(_) => {
                                info!(
                                    "Successfully set up loopback interface for process {}",
                                    process_id
                                );
                            }
                            Err(e) => {
                                let error_msg = format!(
                                    "Failed to set up loopback interface for process {}: {}",
                                    process_id, e
                                );
                                if config.strict {
                                    return Err(AppError::ConfigError(error_msg));
                                } else {
                                    warn!("{}. Continuing without loopback.", error_msg);
                                }
                            }
                        }
                    }

                    // TODO: Additional configuration:
                    // - Configure allowed interfaces (requires veth pairs or macvlan)
                    // - Set up firewall rules for allowed ports (iptables/nftables)
                    // - Move process to namespace (requires setns or process creation in namespace)
                }
                Err(e) => {
                    let error_msg = format!(
                        "Failed to create network namespace for process {}: {}",
                        process_id, e
                    );
                    if config.strict {
                        return Err(AppError::ConfigError(format!(
                            "{}. Isolation is required (strict mode enabled).",
                            error_msg
                        )));
                    } else {
                        warn!(
                            "{}. Isolation may not be fully applied (graceful degradation).",
                            error_msg
                        );
                        // Continue with validation-only mode
                    }
                }
            }
        }

        #[cfg(not(feature = "vm-isolation-linux"))]
        {
            warn!(
                "Network isolation configuration validated for process {}, but full implementation requires 'vm-isolation-linux' feature and system calls (unshare, setns)",
                process_id
            );
        }

        Ok(())
    }

    fn remove_network_isolation(&self, process_id: u32) -> Result<(), AppError> {
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        info!("Removing network isolation from process {}", process_id);

        #[cfg(feature = "vm-isolation-linux")]
        {
            // TODO: Full cleanup implementation:
            // 1. Move process back to original network namespace
            // 2. Clean up network namespace if it was created by us
            // 3. Remove firewall rules
            // Note: This is complex because we need to track which namespace was created
            info!(
                "Network isolation removal for process {} (cleanup requires namespace tracking)",
                process_id
            );
        }

        #[cfg(not(feature = "vm-isolation-linux"))]
        {
            warn!(
                "Network isolation removal requested for process {}, but full implementation requires 'vm-isolation-linux' feature",
                process_id
            );
        }

        Ok(())
    }

    fn is_supported(&self) -> bool {
        // Network namespaces are supported on Linux
        true
    }
}

/// Linux filesystem isolator using chroot and bind mounts
pub struct LinuxFilesystemIsolator;

impl LinuxFilesystemIsolator {
    pub fn new() -> Self {
        Self
    }

    /// Set up a bind mount for filesystem isolation
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_bind_mount(
        source: &PathBuf,
        root_dir: Option<&PathBuf>,
        read_only: bool,
    ) -> Result<(), AppError> {
        // Validate source path exists
        if !source.exists() {
            return Err(AppError::ConfigError(format!(
                "Source path does not exist: {:?}",
                source
            )));
        }

        // If root_dir is provided and use_chroot is enabled, we need to create
        // the mount point inside the chroot directory
        // For now, we'll just set up the bind mount in the current namespace
        let target = if let Some(root_dir) = root_dir {
            // Create target path inside root_dir
            let relative_path = source.strip_prefix("/").unwrap_or(source);
            let target_path = root_dir.join(relative_path);
            if let Some(parent) = target_path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    return Err(AppError::ConfigError(format!(
                        "Failed to create target directory {:?}: {}",
                        parent, e
                    )));
                }
            }
            target_path
        } else {
            source.clone()
        };

        // Set up bind mount flags
        let mut flags = MsFlags::MS_BIND | MsFlags::MS_REC;
        if read_only {
            flags |= MsFlags::MS_RDONLY;
        }

        // Create bind mount
        mount(
            Some(source.as_os_str()),
            target.as_os_str(),
            None::<&str>,
            flags,
            None::<&str>,
        )
        .map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to create bind mount from {:?} to {:?}: {}",
                source, target, e
            ))
        })?;

        Ok(())
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_bind_mount(
        _source: &PathBuf,
        _root_dir: Option<&PathBuf>,
        _read_only: bool,
    ) -> Result<(), AppError> {
        // No-op when feature is not enabled
        Ok(())
    }
}

impl FilesystemIsolator for LinuxFilesystemIsolator {
    fn apply_filesystem_isolation(
        &self,
        process_id: u32,
        config: &FilesystemIsolationConfig,
    ) -> Result<(), AppError> {
        if !config.enabled {
            return Ok(());
        }

        // Validate process ID
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        // Validate root directory if provided
        if let Some(ref root_dir) = config.root_dir {
            if !root_dir.is_absolute() {
                return Err(AppError::ConfigError(format!(
                    "Root directory must be an absolute path: {:?}",
                    root_dir
                )));
            }
        }

        // Validate that if use_chroot is true, root_dir must be provided
        if config.use_chroot && config.root_dir.is_none() {
            return Err(AppError::ConfigError(
                "use_chroot requires root_dir to be specified".to_string(),
            ));
        }

        // Log configuration details
        info!(
            "Applying filesystem isolation to process {}: root_dir={:?}, allowed_paths={}, read_only_paths={}, use_chroot={}",
            process_id,
            config.root_dir,
            config.allowed_paths.len(),
            config.read_only_paths.len(),
            config.use_chroot
        );

        #[cfg(feature = "vm-isolation-linux")]
        {
            // Create mount namespace for isolation
            let mount_ns_result = unshare(CloneFlags::CLONE_NEWNS);
            match mount_ns_result {
                Ok(_) => {
                    info!(
                        "Successfully created mount namespace for process {}",
                        process_id
                    );

                    // Set up bind mounts for allowed paths
                    for allowed_path in &config.allowed_paths {
                        if let Err(e) =
                            Self::setup_bind_mount(allowed_path, config.root_dir.as_ref(), false)
                        {
                            let error_msg = format!(
                                "Failed to set up bind mount for {:?}: {}",
                                allowed_path, e
                            );
                            if config.strict {
                                return Err(AppError::ConfigError(error_msg));
                            } else {
                                warn!("{}. Continuing without this mount.", error_msg);
                            }
                        } else {
                            info!("Successfully set up bind mount for: {:?}", allowed_path);
                        }
                    }

                    // Set up read-only mounts
                    for read_only_path in &config.read_only_paths {
                        if let Err(e) =
                            Self::setup_bind_mount(read_only_path, config.root_dir.as_ref(), true)
                        {
                            let error_msg = format!(
                                "Failed to set up read-only mount for {:?}: {}",
                                read_only_path, e
                            );
                            if config.strict {
                                return Err(AppError::ConfigError(error_msg));
                            } else {
                                warn!("{}. Continuing without this mount.", error_msg);
                            }
                        } else {
                            info!(
                                "Successfully set up read-only mount for: {:?}",
                                read_only_path
                            );
                        }
                    }

                    // Apply chroot if requested
                    if config.use_chroot {
                        if let Some(ref root_dir) = config.root_dir {
                            // Ensure root directory exists
                            if !root_dir.exists() {
                                match fs::create_dir_all(root_dir) {
                                    Ok(_) => {
                                        info!("Created root directory: {:?}", root_dir);
                                    }
                                    Err(e) => {
                                        let error_msg = format!(
                                            "Failed to create root directory {:?}: {}",
                                            root_dir, e
                                        );
                                        if config.strict {
                                            return Err(AppError::ConfigError(error_msg));
                                        } else {
                                            warn!("{}. Continuing without chroot.", error_msg);
                                            return Ok(()); // Skip chroot but continue
                                        }
                                    }
                                }
                            }

                            // Apply chroot
                            match chroot(root_dir) {
                                Ok(_) => {
                                    info!(
                                        "Successfully applied chroot to {:?} for process {}",
                                        root_dir, process_id
                                    );
                                }
                                Err(e) => {
                                    let error_msg = format!(
                                        "Failed to apply chroot to {:?} for process {}: {}",
                                        root_dir, process_id, e
                                    );
                                    if config.strict {
                                        return Err(AppError::ConfigError(error_msg));
                                    } else {
                                        warn!(
                                            "{}. Isolation may not be fully applied (graceful degradation).",
                                            error_msg
                                        );
                                        // Continue without chroot
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!(
                        "Failed to create mount namespace for process {}: {}",
                        process_id, e
                    );
                    if config.strict {
                        return Err(AppError::ConfigError(format!(
                            "{}. Isolation is required (strict mode enabled).",
                            error_msg
                        )));
                    } else {
                        warn!(
                            "{}. Isolation may not be fully applied (graceful degradation).",
                            error_msg
                        );
                        // Continue with validation-only mode
                    }
                }
            }
        }

        #[cfg(not(feature = "vm-isolation-linux"))]
        {
            warn!(
                "Filesystem isolation configuration validated for process {}, but full implementation requires 'vm-isolation-linux' feature and system calls (chroot, mount, unshare)",
                process_id
            );
        }

        Ok(())
    }

    fn remove_filesystem_isolation(&self, process_id: u32) -> Result<(), AppError> {
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        info!("Removing filesystem isolation from process {}", process_id);

        #[cfg(feature = "vm-isolation-linux")]
        {
            // TODO: Full cleanup implementation:
            // 1. Unmount bind mounts
            // 2. Move process back to original mount namespace
            // 3. Clean up temporary directories if created
            // Note: This is complex because we need to track what was mounted
            info!(
                "Filesystem isolation removal for process {} (cleanup requires mount tracking)",
                process_id
            );
        }

        #[cfg(not(feature = "vm-isolation-linux"))]
        {
            warn!(
                "Filesystem isolation removal requested for process {}, but full implementation requires 'vm-isolation-linux' feature",
                process_id
            );
        }

        Ok(())
    }

    fn is_supported(&self) -> bool {
        // chroot and mount namespaces are supported on Linux
        true
    }
}
