//! Windows isolation planning (AppContainer profile + firewall rules).
//!
//! Pure Rust — no Windows API calls. Used by `windows.rs` and unit tests on all targets.

use crate::core::error::AppError;
use crate::vm::isolation::{FilesystemIsolationConfig, NetworkIsolationConfig};
use std::path::Path;

/// How isolation was applied on the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum WindowsIsolationMode {
    /// Profile and rules computed; host APIs not invoked (default without `vm-isolation-windows`).
    #[default]
    PlanOnly,
    /// AppContainer profile creation attempted (`vm-isolation-windows` on Windows).
    AppContainerProfile,
}

/// Planned Windows Firewall rule (INetFwRule / netsh naming).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FirewallRulePlan {
    pub name: String,
    pub direction: String,
    pub protocol: String,
    pub local_ports: Vec<u16>,
    pub action: String,
}

/// Tracked AppContainer + firewall state for one VM process.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppContainerState {
    pub profile_name: String,
    pub appcontainer_sid: Option<String>,
    pub firewall_rules: Vec<FirewallRulePlan>,
    pub wfp_filters: Vec<u64>,
    pub created_appcontainer: bool,
    pub apply_mode: WindowsIsolationMode,
}

impl AppContainerState {
    pub fn new(profile_name: impl Into<String>) -> Self {
        Self {
            profile_name: profile_name.into(),
            appcontainer_sid: None,
            firewall_rules: Vec::new(),
            wfp_filters: Vec::new(),
            created_appcontainer: false,
            apply_mode: WindowsIsolationMode::PlanOnly,
        }
    }
}

/// Stable AppContainer profile name for a process id.
pub fn profile_name_for_process(process_id: u32) -> String {
    format!("PoolAI-VM-{process_id}")
}

/// Build firewall rule plans from network isolation config.
pub fn plan_firewall_rules(
    process_id: u32,
    config: &NetworkIsolationConfig,
) -> Vec<FirewallRulePlan> {
    let prefix = format!("PoolAI-VM-{process_id}");
    let mut rules = Vec::new();

    if config.allow_loopback {
        rules.push(FirewallRulePlan {
            name: format!("{prefix}-loopback-out"),
            direction: "out".to_string(),
            protocol: "any".to_string(),
            local_ports: vec![],
            action: "allow".to_string(),
        });
    }

    for port in &config.allowed_ports {
        rules.push(FirewallRulePlan {
            name: format!("{prefix}-tcp-{port}-out"),
            direction: "out".to_string(),
            protocol: "TCP".to_string(),
            local_ports: vec![*port],
            action: "allow".to_string(),
        });
    }

    for iface in &config.allowed_interfaces {
        let safe = iface.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
        rules.push(FirewallRulePlan {
            name: format!("{prefix}-iface-{safe}-out"),
            direction: "out".to_string(),
            protocol: "any".to_string(),
            local_ports: vec![],
            action: "allow".to_string(),
        });
    }

    rules
}

/// Plan network isolation (AppContainer profile + firewall rules).
pub fn plan_network_isolation(
    process_id: u32,
    config: &NetworkIsolationConfig,
) -> AppContainerState {
    let profile_name = profile_name_for_process(process_id);
    let mut state = AppContainerState::new(profile_name.clone());
    state.firewall_rules = plan_firewall_rules(process_id, config);
    state.appcontainer_sid = Some(format!("S-1-15-2-{process_id:08X}-plan"));
    state
}

/// Plan filesystem isolation (profile + path capabilities summary).
pub fn plan_filesystem_isolation(
    process_id: u32,
    config: &FilesystemIsolationConfig,
) -> Result<AppContainerState, AppError> {
    let profile_name = profile_name_for_process(process_id);
    let mut state = AppContainerState::new(profile_name);
    state.appcontainer_sid = Some(format!("S-1-15-2-{process_id:08X}-fs-plan"));

    if let Some(ref root) = config.root_dir {
        validate_windows_path(root)?;
        state.firewall_rules.push(FirewallRulePlan {
            name: format!("PoolAI-VM-{process_id}-fs-root"),
            direction: "out".to_string(),
            protocol: "any".to_string(),
            local_ports: vec![],
            action: "allow".to_string(),
        });
    }

    for path in &config.allowed_paths {
        validate_windows_path(path)?;
    }
    for path in &config.read_only_paths {
        validate_windows_path(path)?;
    }

    Ok(state)
}

fn validate_windows_path(path: &Path) -> Result<(), AppError> {
    let s = path.to_string_lossy();
    if s.is_empty() {
        return Err(AppError::ConfigError(
            "Filesystem path cannot be empty".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn profile_name_is_stable() {
        assert_eq!(profile_name_for_process(42), "PoolAI-VM-42");
    }

    #[test]
    fn plan_firewall_includes_ports_and_loopback() {
        let config = NetworkIsolationConfig {
            enabled: true,
            allowed_ports: vec![8080, 443],
            allow_loopback: true,
            ..Default::default()
        };
        let rules = plan_firewall_rules(7, &config);
        assert!(rules.iter().any(|r| r.name.contains("loopback")));
        assert!(rules.iter().any(|r| r.name.contains("tcp-8080")));
        assert!(rules.iter().any(|r| r.name.contains("tcp-443")));
    }

    #[test]
    fn plan_network_populates_state() {
        let config = NetworkIsolationConfig {
            enabled: true,
            allowed_ports: vec![80],
            allow_loopback: true,
            ..Default::default()
        };
        let state = plan_network_isolation(99, &config);
        assert_eq!(state.profile_name, "PoolAI-VM-99");
        assert!(!state.firewall_rules.is_empty());
        assert!(state.appcontainer_sid.is_some());
    }

    #[test]
    fn plan_filesystem_rejects_empty_path() {
        let config = FilesystemIsolationConfig {
            enabled: true,
            root_dir: Some(PathBuf::from("")),
            ..Default::default()
        };
        assert!(plan_filesystem_isolation(1, &config).is_err());
    }
}
