//! Windows-specific isolation implementations
//!
//! AppContainer profile planning (always) + optional native hooks (`vm-isolation-windows`).

use crate::core::error::AppError;
use crate::vm::isolation::windows_plan::{
    plan_filesystem_isolation, plan_network_isolation, AppContainerState,
};
use crate::vm::isolation::{
    FilesystemIsolationConfig, FilesystemIsolator, NetworkIsolationConfig, NetworkIsolator,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;
#[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
use tracing::warn;

#[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
mod windows_native;

/// Windows network isolator using AppContainers and Windows Firewall plans.
pub struct WindowsNetworkIsolator {
    appcontainer_states: Arc<Mutex<HashMap<u32, AppContainerState>>>,
}

impl WindowsNetworkIsolator {
    pub fn new() -> Self {
        Self {
            appcontainer_states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_appcontainer_state(&self, process_id: u32) -> Option<AppContainerState> {
        self.appcontainer_states
            .lock()
            .unwrap()
            .get(&process_id)
            .cloned()
    }

    #[allow(dead_code)] // ops introspection helper
    pub fn list_appcontainer_states(&self) -> Vec<(u32, AppContainerState)> {
        self.appcontainer_states
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
}

impl NetworkIsolator for WindowsNetworkIsolator {
    fn apply_network_isolation(
        &self,
        process_id: u32,
        config: &NetworkIsolationConfig,
    ) -> Result<(), AppError> {
        if !config.enabled {
            return Ok(());
        }

        if !config.allow_loopback
            && config.allowed_interfaces.is_empty()
            && config.allowed_ports.is_empty()
        {
            return Err(AppError::ConfigError(
                "Network isolation configuration would block all network access. At least one of allow_loopback, allowed_interfaces, or allowed_ports must be enabled.".to_string(),
            ));
        }

        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        info!(
            "Applying network isolation to process {}: loopback={}, interfaces={:?}, ports={:?}",
            process_id, config.allow_loopback, config.allowed_interfaces, config.allowed_ports
        );

        #[cfg_attr(
            not(all(target_os = "windows", feature = "vm-isolation-windows")),
            allow(unused_mut)
        )]
        let mut state = plan_network_isolation(process_id, config);

        #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
        {
            if let Err(e) = windows_native::apply_network_profile(&mut state, config) {
                if config.strict {
                    return Err(e);
                }
                warn!(
                    "Windows native network isolation failed for process {} (graceful): {}",
                    process_id, e
                );
            }
        }

        #[cfg(not(all(target_os = "windows", feature = "vm-isolation-windows")))]
        {
            info!(
                "Network isolation plan stored for process {} (profile={}, rules={}); enable `vm-isolation-windows` on Windows for native AppContainer apply",
                process_id,
                state.profile_name,
                state.firewall_rules.len()
            );
        }

        self.appcontainer_states
            .lock()
            .unwrap()
            .insert(process_id, state);
        Ok(())
    }

    fn remove_network_isolation(&self, process_id: u32) -> Result<(), AppError> {
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        info!("Removing network isolation from process {}", process_id);

        let removed = self.appcontainer_states.lock().unwrap().remove(&process_id);
        if let Some(state) = removed {
            #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
            {
                if let Err(e) = windows_native::remove_network_profile(&state) {
                    warn!(
                        "Windows native network isolation cleanup for process {}: {}",
                        process_id, e
                    );
                }
            }
            info!(
                "Removed isolation plan for profile {} (rules={})",
                state.profile_name,
                state.firewall_rules.len()
            );
        }

        Ok(())
    }

    fn is_supported(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Windows filesystem isolator using AppContainer path plans.
pub struct WindowsFilesystemIsolator {
    appcontainer_states: Arc<Mutex<HashMap<u32, AppContainerState>>>,
}

impl WindowsFilesystemIsolator {
    pub fn new() -> Self {
        Self {
            appcontainer_states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_appcontainer_state(&self, process_id: u32) -> Option<AppContainerState> {
        self.appcontainer_states
            .lock()
            .unwrap()
            .get(&process_id)
            .cloned()
    }

    #[allow(dead_code)] // ops introspection helper
    pub fn list_appcontainer_states(&self) -> Vec<(u32, AppContainerState)> {
        self.appcontainer_states
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
}

impl FilesystemIsolator for WindowsFilesystemIsolator {
    fn apply_filesystem_isolation(
        &self,
        process_id: u32,
        config: &FilesystemIsolationConfig,
    ) -> Result<(), AppError> {
        if !config.enabled {
            return Ok(());
        }

        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        if config.use_chroot && config.root_dir.is_none() {
            return Err(AppError::ConfigError(
                "use_chroot requires root_dir to be specified".to_string(),
            ));
        }

        if let Some(ref root_dir) = config.root_dir {
            if root_dir.to_string_lossy().is_empty() {
                return Err(AppError::ConfigError(
                    "Root directory cannot be empty".to_string(),
                ));
            }
        }

        info!(
            "Applying filesystem isolation to process {}: root_dir={:?}, allowed_paths={}, read_only_paths={}",
            process_id,
            config.root_dir,
            config.allowed_paths.len(),
            config.read_only_paths.len()
        );

        #[cfg_attr(
            not(all(target_os = "windows", feature = "vm-isolation-windows")),
            allow(unused_mut)
        )]
        let mut state = plan_filesystem_isolation(process_id, config)?;

        #[cfg(all(target_os = "windows", feature = "vm-isolation-windows"))]
        {
            if let Err(e) = windows_native::apply_filesystem_profile(&mut state, config) {
                if config.strict {
                    return Err(e);
                }
                warn!(
                    "Windows native filesystem isolation failed for process {} (graceful): {}",
                    process_id, e
                );
            }
        }

        #[cfg(not(all(target_os = "windows", feature = "vm-isolation-windows")))]
        {
            info!(
                "Filesystem isolation plan stored for process {} (profile={})",
                process_id, state.profile_name
            );
        }

        self.appcontainer_states
            .lock()
            .unwrap()
            .insert(process_id, state);
        Ok(())
    }

    fn remove_filesystem_isolation(&self, process_id: u32) -> Result<(), AppError> {
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0".to_string(),
            ));
        }

        info!("Removing filesystem isolation from process {}", process_id);
        let _ = self.appcontainer_states.lock().unwrap().remove(&process_id);
        Ok(())
    }

    fn is_supported(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
