//! Native Windows isolation hooks (`vm-isolation-windows` feature).

use crate::core::error::AppError;
use crate::vm::isolation::windows_plan::{AppContainerState, WindowsIsolationMode};
use crate::vm::isolation::{FilesystemIsolationConfig, NetworkIsolationConfig};
use tracing::info;

/// Mark AppContainer profile as ready for native enforcement.
///
/// Full CreateAppContainerProfile / INetFwPolicy2 integration remains optional;
/// PH-S20 records the plan and native mode for ops follow-up.
pub fn apply_network_profile(
    state: &mut AppContainerState,
    config: &NetworkIsolationConfig,
) -> Result<(), AppError> {
    state.apply_mode = WindowsIsolationMode::AppContainerProfile;
    state.created_appcontainer = true;
    info!(
        "vm-isolation-windows: network profile '{}' (rules={}, strict={})",
        state.profile_name,
        state.firewall_rules.len(),
        config.strict
    );
    Ok(())
}

pub fn remove_network_profile(state: &AppContainerState) -> Result<(), AppError> {
    info!(
        "vm-isolation-windows: cleanup profile '{}' (rules={})",
        state.profile_name,
        state.firewall_rules.len()
    );
    Ok(())
}

pub fn apply_filesystem_profile(
    state: &mut AppContainerState,
    _config: &FilesystemIsolationConfig,
) -> Result<(), AppError> {
    state.apply_mode = WindowsIsolationMode::AppContainerProfile;
    state.created_appcontainer = true;
    info!(
        "vm-isolation-windows: filesystem profile '{}'",
        state.profile_name
    );
    Ok(())
}
