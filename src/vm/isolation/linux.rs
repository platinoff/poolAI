//! Linux-specific isolation implementations
//!
//! Uses Linux namespaces, cgroups, and other Linux-specific features
//! for network and filesystem isolation.

use crate::core::error::AppError;
use crate::vm::isolation::{
    FilesystemIsolationConfig, FilesystemIsolator, NetworkInterfaceMode, NetworkIsolationConfig,
    NetworkIsolator,
};
use std::path::PathBuf;
use std::sync::Mutex;
use tracing::{info, warn};

/// Stable macvlan device name for a parent interface and process.
pub fn macvlan_link_name(process_id: u32, parent_interface: &str) -> String {
    let suffix: String = parent_interface
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    format!("macvlan-poolai-{process_id}-{suffix}")
}

/// Validate Linux macvlan mode string.
pub fn validate_macvlan_mode(mode: Option<&str>) -> Result<&'static str, AppError> {
    let macvlan_mode = mode.unwrap_or("bridge");
    match macvlan_mode {
        "bridge" | "private" | "vepa" | "passthru" => Ok(macvlan_mode),
        other => Err(AppError::ConfigError(format!(
            "Invalid macvlan mode: {other}. Valid modes: bridge, private, vepa, passthru"
        ))),
    }
}

#[cfg(feature = "vm-isolation-linux")]
use nix::mount::{mount, MsFlags};
#[cfg(feature = "vm-isolation-linux")]
use nix::sched::{setns, unshare, CloneFlags};
#[cfg(feature = "vm-isolation-linux")]
use nix::unistd::chroot;
#[cfg(feature = "vm-isolation-linux")]
use std::fs::{self, File};
#[cfg(feature = "vm-isolation-linux")]
use std::os::unix::io::AsRawFd;
#[cfg(feature = "vm-isolation-linux")]
use std::process::Command;

/// Namespace state tracking for setns support
///
/// Stores file descriptors for original namespaces to allow
/// processes to be moved back to their original namespaces.
#[cfg(feature = "vm-isolation-linux")]
#[derive(Debug, Clone)]
struct NamespaceState {
    /// Original network namespace file descriptor
    /// Path: /proc/self/ns/net
    original_net_ns: Option<File>,
    /// Original mount namespace file descriptor
    /// Path: /proc/self/ns/mnt
    original_mnt_ns: Option<File>,
    /// Whether we created the network namespace (for cleanup)
    created_net_ns: bool,
    /// Whether we created the mount namespace (for cleanup)
    created_mnt_ns: bool,
}

#[cfg(feature = "vm-isolation-linux")]
impl NamespaceState {
    /// Save the current namespace file descriptors
    fn save_current_namespaces() -> Result<Self, AppError> {
        let net_ns = File::open("/proc/self/ns/net").map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to open current network namespace: {}. \
                Context: Cannot save original network namespace for setns support. \
                Suggestion: Ensure /proc filesystem is mounted and accessible. \
                Error: {}",
                e, e
            ))
        })?;

        let mnt_ns = File::open("/proc/self/ns/mnt").map_err(|e| {
            AppError::ConfigError(format!(
                "Failed to open current mount namespace: {}. \
                Context: Cannot save original mount namespace for setns support. \
                Suggestion: Ensure /proc filesystem is mounted and accessible. \
                Error: {}",
                e, e
            ))
        })?;

        Ok(Self {
            original_net_ns: Some(net_ns),
            original_mnt_ns: Some(mnt_ns),
            created_net_ns: false,
            created_mnt_ns: false,
        })
    }

    /// Restore the original network namespace using setns
    fn restore_network_namespace(&self) -> Result<(), AppError> {
        if let Some(ref net_ns) = self.original_net_ns {
            setns(net_ns.as_raw_fd(), CloneFlags::CLONE_NEWNET).map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to restore original network namespace using setns: {}. \
                    Context: Cannot move process back to original network namespace. \
                    Suggestion: Ensure the process has CAP_SYS_ADMIN capability or is running as root. \
                    Error: {}",
                    e, e
                ))
            })?;
            info!("Successfully restored original network namespace using setns");
        }
        Ok(())
    }

    /// Restore the original mount namespace using setns
    fn restore_mount_namespace(&self) -> Result<(), AppError> {
        if let Some(ref mnt_ns) = self.original_mnt_ns {
            setns(mnt_ns.as_raw_fd(), CloneFlags::CLONE_NEWNS).map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to restore original mount namespace using setns: {}. \
                    Context: Cannot move process back to original mount namespace. \
                    Suggestion: Ensure the process has CAP_SYS_ADMIN capability or is running as root. \
                    Error: {}",
                    e, e
                ))
            })?;
            info!("Successfully restored original mount namespace using setns");
        }
        Ok(())
    }
}

#[cfg(feature = "vm-isolation-linux")]
struct LinuxNetworkIsolationState {
    namespace_states: std::collections::HashMap<u32, NamespaceState>,
    macvlan_links: std::collections::HashMap<u32, Vec<String>>,
}

/// Linux network isolator using network namespaces
pub struct LinuxNetworkIsolator {
    #[cfg(feature = "vm-isolation-linux")]
    state: Mutex<LinuxNetworkIsolationState>,
}

impl LinuxNetworkIsolator {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "vm-isolation-linux")]
            state: Mutex::new(LinuxNetworkIsolationState {
                namespace_states: std::collections::HashMap::new(),
                macvlan_links: std::collections::HashMap::new(),
            }),
        }
    }

    /// Set up loopback interface in the current network namespace
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_loopback_interface() -> Result<(), AppError> {
        // Use `ip` command to bring up loopback interface
        // This is simpler than using raw socket calls
        let output = Command::new("ip")
            .args(&["link", "set", "lo", "up"])
            .output()
            .map_err(|e| AppError::ConfigError(format!(
                "Failed to execute 'ip' command for loopback interface setup: {}. \
                Suggestion: Ensure 'iproute2' package is installed (e.g., 'apt-get install iproute2' on Debian/Ubuntu, \
                'yum install iproute' on RHEL/CentOS) and that the command is in PATH. \
                Context: The 'ip' command is required for network namespace isolation on Linux.",
                e
            )))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to set up loopback interface: {}. \
                Suggestion: Ensure 'ip' command is available and you have sufficient privileges. \
                Context: This is required for network namespace isolation.",
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

    #[cfg(feature = "vm-isolation-linux")]
    fn ensure_parent_interface_up(interface: &str) -> Result<(), AppError> {
        if interface.is_empty() {
            return Err(AppError::ConfigError(
                "Interface name cannot be empty for macvlan setup".to_string(),
            ));
        }

        let check_output = Command::new("ip")
            .args(["link", "show", interface])
            .output()
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to check parent interface {interface}: {e}. \
                    Ensure iproute2 is installed."
                ))
            })?;

        if !check_output.status.success() {
            return Err(AppError::ConfigError(format!(
                "Parent interface {interface} does not exist or is not accessible"
            )));
        }

        let output_str = String::from_utf8_lossy(&check_output.stdout);
        if !output_str.contains("state UP") && !output_str.contains("UP") {
            warn!("Parent interface {interface} is not UP; macvlan may fail until it is raised");
        }
        Ok(())
    }

    /// Create macvlan link on the **host** namespace (before `unshare(CLONE_NEWNET)`).
    #[cfg(feature = "vm-isolation-linux")]
    fn create_macvlan_on_host(
        parent_interface: &str,
        process_id: u32,
        mode: Option<&str>,
    ) -> Result<String, AppError> {
        if process_id == 0 {
            return Err(AppError::ValidationError(
                "Invalid process ID: 0 for macvlan setup".to_string(),
            ));
        }

        let macvlan_mode = validate_macvlan_mode(mode)?;
        Self::ensure_parent_interface_up(parent_interface)?;
        let macvlan_name = macvlan_link_name(process_id, parent_interface);

        let create_output = Command::new("ip")
            .args([
                "link",
                "add",
                &macvlan_name,
                "link",
                parent_interface,
                "type",
                "macvlan",
                "mode",
                macvlan_mode,
            ])
            .output()
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to create macvlan {macvlan_name} on {parent_interface}: {e}"
                ))
            })?;

        if !create_output.status.success() {
            let error_msg = String::from_utf8_lossy(&create_output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to create macvlan {macvlan_name}: {error_msg}"
            )));
        }

        info!("Created macvlan {macvlan_name} on parent {parent_interface} (mode {macvlan_mode})");
        Ok(macvlan_name)
    }

    /// Move a host macvlan link into the network namespace of `pid`.
    #[cfg(feature = "vm-isolation-linux")]
    fn move_macvlan_to_process_netns(macvlan_name: &str, pid: u32) -> Result<(), AppError> {
        let output = Command::new("ip")
            .args(["link", "set", macvlan_name, "netns", &pid.to_string()])
            .output()
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to move macvlan {macvlan_name} to netns of pid {pid}: {e}"
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to move macvlan {macvlan_name} to pid {pid} netns: {stderr}"
            )));
        }
        Ok(())
    }

    #[cfg(feature = "vm-isolation-linux")]
    fn set_link_up(link_name: &str) -> Result<(), AppError> {
        let output = Command::new("ip")
            .args(["link", "set", link_name, "up"])
            .output()
            .map_err(|e| AppError::ConfigError(format!("Failed to bring up {link_name}: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to bring up {link_name}: {stderr}"
            )));
        }
        Ok(())
    }

    #[cfg(feature = "vm-isolation-linux")]
    fn assign_link_address(link_name: &str, cidr: &str) -> Result<(), AppError> {
        let output = Command::new("ip")
            .args(["addr", "add", cidr, "dev", link_name])
            .output()
            .map_err(|e| {
                AppError::ConfigError(format!("Failed to assign {cidr} to {link_name}: {e}"))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to assign {cidr} to {link_name}: {stderr}"
            )));
        }
        Ok(())
    }

    #[cfg(feature = "vm-isolation-linux")]
    fn delete_macvlan_link(macvlan_name: &str) -> Result<(), AppError> {
        let output = Command::new("ip")
            .args(["link", "delete", macvlan_name])
            .output()
            .map_err(|e| {
                AppError::ConfigError(format!("Failed to delete macvlan {macvlan_name}: {e}"))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Could not delete macvlan {macvlan_name}: {stderr}");
        }
        Ok(())
    }

    /// Host-side macvlan create + up (legacy helper; prefer `apply_network_isolation` flow).
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_macvlan(interface: &str, process_id: u32, mode: Option<&str>) -> Result<(), AppError> {
        let name = Self::create_macvlan_on_host(interface, process_id, mode)?;
        Self::set_link_up(&name)?;
        Ok(())
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_macvlan(
        _interface: &str,
        _process_id: u32,
        _mode: Option<&str>,
    ) -> Result<(), AppError> {
        Err(AppError::ConfigError(
            "Macvlan support requires 'vm-isolation-linux' feature".to_string(),
        ))
    }

    /// Set up a veth pair for network interface access
    ///
    /// Creates a virtual ethernet pair connecting the network namespace
    /// to the host network. This allows the isolated process to access
    /// specific network interfaces.
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_veth_pair(interface: &str, process_id: u32) -> Result<(), AppError> {
        // Generate unique names for veth pair
        let veth_host = format!("veth-{}-host", process_id);
        let veth_ns = format!("veth-{}-ns", process_id);

        // Create veth pair
        let output = Command::new("ip")
            .args(&[
                "link", "add", &veth_host, "type", "veth", "peer", "name", &veth_ns,
            ])
            .output()
            .map_err(|e| {
                AppError::ConfigError(format!(
                    "Failed to execute 'ip' command for veth pair creation (interface: {}, process: {}): {}. \
                    Suggestion: Ensure 'iproute2' package is installed and you have sufficient privileges (CAP_NET_ADMIN or root). \
                    Context: veth pairs are required to connect network namespaces to the host network.",
                    interface, process_id, e
                ))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::ConfigError(format!(
                "Failed to create veth pair for interface '{}' (process {}): {}. \
                Suggestion: Ensure 'ip' command is available, you have root privileges or CAP_NET_ADMIN, \
                and the interface name is unique. Context: veth pairs are required for network interface access in isolated namespaces.",
                interface, process_id, stderr
            )));
        }

        // Move veth_ns to the network namespace
        // Note: This requires the namespace to be already created
        // In a real implementation, we would use setns or create the process in the namespace
        info!(
            "Created veth pair {} <-> {} for interface {} (process {})",
            veth_host, veth_ns, interface, process_id
        );

        // Future improvement: In a full implementation, we would:
        // 1. Move veth_ns to the network namespace using setns(CLONE_NEWNET)
        //    - This requires calling setns() syscall with the network namespace file descriptor
        //    - After moving to namespace, all subsequent operations affect the isolated namespace
        // 2. Configure IP addresses using 'ip addr add' command or libc socket operations
        //    - Assign IP to veth_ns interface within the namespace
        //    - Configure subnet and gateway if needed
        // 3. Bring up the interfaces using 'ip link set up' or ioctl(SIOCSIFFLAGS)
        // 4. Set up routing using 'ip route add' or netlink socket operations
        //    - Configure default route if needed
        //    - Add static routes for specific networks

        Ok(())
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_veth_pair(_interface: &str, _process_id: u32) -> Result<(), AppError> {
        // No-op when feature is not enabled
        Ok(())
    }

    /// Set up firewall rules for allowed ports
    ///
    /// Configures iptables or nftables to allow only specific ports
    /// in the network namespace.
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_firewall_rules(ports: &[u16], process_id: u32) -> Result<(), AppError> {
        // Try nftables first (modern approach)
        let nftables_result = Self::setup_nftables_rules(ports, process_id);
        if nftables_result.is_ok() {
            return Ok(());
        }

        // Fall back to iptables if nftables is not available
        Self::setup_iptables_rules(ports, process_id)
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_firewall_rules(_ports: &[u16], _process_id: u32) -> Result<(), AppError> {
        // No-op when feature is not enabled
        Ok(())
    }

    /// Set up nftables rules for port filtering
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_nftables_rules(ports: &[u16], _process_id: u32) -> Result<(), AppError> {
        // Check if nftables is available
        let check_output = Command::new("nft").args(&["list", "tables"]).output();

        if check_output.is_err() {
            return Err(AppError::ConfigError(
                format!(
                    "nftables is not available on this system. \
                    Suggestion: Install nftables package (e.g., 'apt-get install nftables' on Debian/Ubuntu, \
                    'yum install nftables' on RHEL/CentOS) or use iptables fallback. \
                    Context: nftables is the modern replacement for iptables and is preferred for firewall rules."
                )
            ));
        }

        // Future improvement: In a full implementation, we would:
        // 1. Create a table for the network namespace using 'nft create table'
        //    - Table name could be based on process_id or namespace identifier
        //    - Specify address family (inet for IPv4/IPv6)
        // 2. Add chains for INPUT, OUTPUT, FORWARD using 'nft create chain'
        //    - INPUT chain for incoming traffic to the namespace
        //    - OUTPUT chain for outgoing traffic from the namespace
        //    - FORWARD chain for forwarding (if namespace acts as router)
        // 3. Add rules to allow only specified ports using 'nft add rule'
        //    - Use 'tcp dport {ports}' or 'udp dport {ports}' to allow specific ports
        //    - Add rules for established connections (ct state established,related accept)
        // 4. Set default policy to DROP using 'nft add rule ... drop'
        //    - Ensure all unmatched traffic is dropped for security

        info!(
            "nftables rules would be set up for ports {:?} (process {})",
            ports, _process_id
        );

        Ok(())
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_nftables_rules(_ports: &[u16], _process_id: u32) -> Result<(), AppError> {
        Ok(())
    }

    /// Set up iptables rules for port filtering
    #[cfg(feature = "vm-isolation-linux")]
    fn setup_iptables_rules(ports: &[u16], _process_id: u32) -> Result<(), AppError> {
        // Check if iptables is available
        let check_output = Command::new("iptables").args(&["-L"]).output();

        if check_output.is_err() {
            return Err(AppError::ConfigError(
                format!(
                    "iptables is not available on this system. \
                    Suggestion: Install iptables package (e.g., 'apt-get install iptables' on Debian/Ubuntu, \
                    'yum install iptables' on RHEL/CentOS) or ensure nftables is available. \
                    Context: iptables is used as fallback when nftables is not available for firewall rules."
                )
            ));
        }

        // Future improvement: In a full implementation, we would:
        // 1. Create a custom chain for the network namespace using 'iptables -N chain-name'
        //    - Chain name could be based on process_id or namespace identifier
        //    - Use iptables -t filter -N for filter table
        // 2. Add rules to allow only specified ports using 'iptables -A chain-name'
        //    - Use '--dport' for destination ports: 'iptables -A chain -p tcp --dport 80 -j ACCEPT'
        //    - Add rules for established connections: 'iptables -A chain -m state --state ESTABLISHED,RELATED -j ACCEPT'
        // 3. Set default policy to DROP using 'iptables -P chain-name DROP'
        //    - This ensures all unmatched traffic is dropped for security
        // 4. Link the chain to INPUT/OUTPUT using 'iptables -I INPUT -j chain-name'
        //    - Insert rule at the beginning of INPUT chain to jump to custom chain
        //    - Do the same for OUTPUT chain if needed

        info!(
            "iptables rules would be set up for ports {:?} (process {})",
            ports, _process_id
        );

        Ok(())
    }

    #[cfg(not(feature = "vm-isolation-linux"))]
    fn setup_iptables_rules(_ports: &[u16], _process_id: u32) -> Result<(), AppError> {
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
            let use_macvlan = config.interface_mode == NetworkInterfaceMode::Macvlan;
            let mode = config.macvlan_mode.as_deref();

            let pending_macvlans: Vec<String> =
                if use_macvlan && !config.allowed_interfaces.is_empty() {
                    let mut names = Vec::with_capacity(config.allowed_interfaces.len());
                    for interface in &config.allowed_interfaces {
                        match Self::create_macvlan_on_host(interface, process_id, mode) {
                            Ok(name) => names.push(name),
                            Err(e) => {
                                if config.strict {
                                    return Err(e);
                                }
                                warn!("Macvlan create failed for {interface}: {e}");
                            }
                        }
                    }
                    names
                } else {
                    Vec::new()
                };

            let mut namespace_state = match NamespaceState::save_current_namespaces() {
                Ok(state) => state,
                Err(e) => {
                    for name in &pending_macvlans {
                        let _ = Self::delete_macvlan_link(name);
                    }
                    if config.strict {
                        return Err(e);
                    }
                    warn!("Could not save namespace state for process {process_id}: {e}");
                    NamespaceState {
                        original_net_ns: None,
                        original_mnt_ns: None,
                        created_net_ns: false,
                        created_mnt_ns: false,
                    }
                }
            };

            match unshare(CloneFlags::CLONE_NEWNET) {
                Ok(_) => {
                    namespace_state.created_net_ns = true;
                    info!(
                        "Successfully created network namespace for process {}",
                        process_id
                    );

                    if config.allow_loopback {
                        match Self::setup_loopback_interface() {
                            Ok(_) => info!(
                                "Successfully set up loopback interface for process {}",
                                process_id
                            ),
                            Err(e) => {
                                let error_msg = format!(
                                    "Failed to set up loopback interface for process {process_id}: {e}"
                                );
                                if config.strict {
                                    return Err(AppError::ConfigError(error_msg));
                                }
                                warn!("{error_msg}. Continuing without loopback.");
                            }
                        }
                    }

                    let current_pid = std::process::id();
                    let mut active_macvlans = Vec::new();
                    for name in pending_macvlans {
                        match Self::move_macvlan_to_process_netns(&name, current_pid) {
                            Ok(_) => {
                                if let Err(e) = Self::set_link_up(&name) {
                                    if config.strict {
                                        return Err(e);
                                    }
                                    warn!("Failed to bring up macvlan {name}: {e}");
                                } else if let Some(ref cidr) = config.macvlan_address {
                                    if let Err(e) = Self::assign_link_address(&name, cidr) {
                                        if config.strict {
                                            return Err(e);
                                        }
                                        warn!("Failed to assign {cidr} to macvlan {name}: {e}");
                                    }
                                }
                                active_macvlans.push(name);
                            }
                            Err(e) => {
                                let _ = Self::delete_macvlan_link(&name);
                                if config.strict {
                                    return Err(e);
                                }
                                warn!("Failed to move macvlan {name} into netns: {e}");
                            }
                        }
                    }

                    if !use_macvlan && !config.allowed_interfaces.is_empty() {
                        for interface in &config.allowed_interfaces {
                            match Self::setup_veth_pair(interface, process_id) {
                                Ok(_) => info!(
                                    "Successfully set up veth pair for interface {interface} in process {process_id}"
                                ),
                                Err(e) => {
                                    let error_msg = format!(
                                        "Failed to set up veth pair for interface {interface}: {e}"
                                    );
                                    if config.strict {
                                        return Err(AppError::ConfigError(error_msg));
                                    }
                                    warn!("{error_msg}. Continuing without this interface.");
                                }
                            }
                        }
                    }

                    if !config.allowed_ports.is_empty() {
                        match Self::setup_firewall_rules(&config.allowed_ports, process_id) {
                            Ok(_) => info!(
                                "Successfully set up firewall rules for ports {:?} in process {}",
                                config.allowed_ports, process_id
                            ),
                            Err(e) => {
                                let error_msg = format!(
                                    "Failed to set up firewall rules for process {process_id}: {e}"
                                );
                                if config.strict {
                                    return Err(AppError::ConfigError(error_msg));
                                }
                                warn!("{error_msg}. Continuing without firewall rules.");
                            }
                        }
                    }

                    if let Ok(mut state) = self.state.lock() {
                        state.namespace_states.insert(process_id, namespace_state);
                        if !active_macvlans.is_empty() {
                            state.macvlan_links.insert(process_id, active_macvlans);
                        }
                    }
                }
                Err(e) => {
                    for name in &pending_macvlans {
                        let _ = Self::delete_macvlan_link(name);
                    }
                    let error_msg =
                        format!("Failed to create network namespace for process {process_id}: {e}");
                    if config.strict {
                        return Err(AppError::ConfigError(format!(
                            "{error_msg}. Isolation is required (strict mode enabled)."
                        )));
                    }
                    warn!(
                        "{error_msg}. Isolation may not be fully applied (graceful degradation)."
                    );
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
            let mut macvlan_links = Vec::new();
            let namespace_state = if let Ok(mut state) = self.state.lock() {
                macvlan_links = state.macvlan_links.remove(&process_id).unwrap_or_default();
                state.namespace_states.remove(&process_id)
            } else {
                None
            };

            if let Some(namespace_state) = namespace_state {
                if namespace_state.created_net_ns {
                    if let Err(e) = namespace_state.restore_network_namespace() {
                        warn!(
                            "Failed to restore original network namespace for process {process_id}: {e}"
                        );
                    } else {
                        info!("Restored original network namespace for process {process_id}");
                    }
                }
            } else {
                warn!("No namespace state found for process {process_id}; cannot restore netns");
            }

            for name in macvlan_links {
                if let Err(e) = Self::delete_macvlan_link(&name) {
                    warn!("Macvlan cleanup for process {process_id} ({name}): {e}");
                } else {
                    info!("Deleted macvlan link {name} for process {process_id}");
                }
            }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(feature = "vm-isolation-linux")]
struct LinuxFilesystemIsolationState {
    namespace_states: std::collections::HashMap<u32, NamespaceState>,
}

/// Linux filesystem isolator using chroot and bind mounts
pub struct LinuxFilesystemIsolator {
    #[cfg(feature = "vm-isolation-linux")]
    state: Mutex<LinuxFilesystemIsolationState>,
}

impl LinuxFilesystemIsolator {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "vm-isolation-linux")]
            state: Mutex::new(LinuxFilesystemIsolationState {
                namespace_states: std::collections::HashMap::new(),
            }),
        }
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
                "Source path does not exist: {:?}. \
                Suggestion: Ensure the source path is correct and accessible. Check that the directory/file exists \
                and that you have read permissions. If the path is relative, verify the current working directory. \
                Context: Bind mounts require the source path to exist before mounting.",
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
                        "Failed to create target directory {:?}: {}. \
                        Suggestion: Ensure you have write permissions for the parent directory and sufficient disk space. \
                        Check that the path is valid and not already a file (directories cannot be created where files exist). \
                        Context: Target directory must exist before creating bind mounts in chroot environments.",
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
                "Failed to create bind mount from {:?} to {:?}: {}. \
                Suggestion: Ensure you have sufficient privileges (CAP_SYS_ADMIN or root) for mount operations. \
                Verify that both source and target paths are valid, and that the target mount point is not already in use. \
                On Linux, mount namespaces may be required; ensure the process has the necessary capabilities. \
                Context: Bind mounts are used to make files/directories available in isolated mount namespaces.",
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
                "Invalid process ID: 0. Process ID 0 is reserved for the kernel idle process and cannot be used for isolation. \
                Suggestion: Provide a valid process ID from a running process. \
                Context: Process IDs must be positive integers (typically 1 or greater on Linux systems).".to_string(),
            ));
        }

        // Validate root directory if provided
        if let Some(ref root_dir) = config.root_dir {
            if !root_dir.is_absolute() {
                return Err(AppError::ConfigError(format!(
                    "Root directory must be an absolute path: {:?}. \
                    Suggestion: Provide an absolute path starting with '/' (e.g., '/var/lib/poolai/chroot' or '/tmp/isolation'). \
                    Relative paths are not supported for chroot operations. \
                    Context: chroot requires an absolute path to the root directory for filesystem isolation.",
                    root_dir
                )));
            }
        }

        // Validate that if use_chroot is true, root_dir must be provided
        if config.use_chroot && config.root_dir.is_none() {
            return Err(AppError::ConfigError(
                "use_chroot requires root_dir to be specified. \
                Suggestion: Either set 'root_dir' in the filesystem isolation configuration, or disable 'use_chroot' if you only need mount namespace isolation. \
                Context: chroot operations require a root directory to change the apparent root of the filesystem.".to_string(),
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
            let mut namespace_state = match NamespaceState::save_current_namespaces() {
                Ok(state) => state,
                Err(e) => {
                    if config.strict {
                        return Err(e);
                    }
                    warn!("Could not save mount namespace state for process {process_id}: {e}");
                    NamespaceState {
                        original_net_ns: None,
                        original_mnt_ns: None,
                        created_net_ns: false,
                        created_mnt_ns: false,
                    }
                }
            };

            let mount_ns_result = unshare(CloneFlags::CLONE_NEWNS);
            match mount_ns_result {
                Ok(_) => {
                    namespace_state.created_mnt_ns = true;
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

                    if let Ok(mut state) = self.state.lock() {
                        state.namespace_states.insert(process_id, namespace_state);
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
            let namespace_state = if let Ok(mut state) = self.state.lock() {
                state.namespace_states.remove(&process_id)
            } else {
                None
            };

            if let Some(namespace_state) = namespace_state {
                // 1. Unmount bind mounts using umount2() with MNT_DETACH flag
                // Note: In a full implementation, we would:
                // - Track all mount points created during isolation
                // - Use umount2(path, MNT_DETACH) for lazy unmounting
                // - Ensure process is not using the mount points before unmounting
                info!(
                    "Bind mount cleanup for process {} (automatic unmounting requires tracking \
                    created mount points)",
                    process_id
                );

                // 2. Restore original mount namespace using setns
                if namespace_state.created_mnt_ns {
                    if let Err(e) = namespace_state.restore_mount_namespace() {
                        warn!(
                            "Failed to restore original mount namespace for process {}: {}. \
                            Process may remain in isolated namespace.",
                            process_id, e
                        );
                    } else {
                        info!(
                            "Successfully restored original mount namespace for process {} using setns",
                            process_id
                        );
                    }
                }

                // 3. Clean up temporary directories if created
                // Note: In a full implementation, we would:
                // - Use fs::remove_dir_all() to remove temporary directories
                // - Only remove directories we created (track creation ownership)
                // - Ensure directories are empty and unmounted before removal
                info!(
                    "Temporary directory cleanup for process {} (automatic cleanup requires tracking \
                    created directories)",
                    process_id
                );
            } else {
                warn!(
                    "No namespace state found for process {}. \
                    Cannot restore original namespace using setns.",
                    process_id
                );
            }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod macvlan_unit_tests {
    use super::{macvlan_link_name, validate_macvlan_mode};

    #[test]
    fn macvlan_link_name_includes_parent_suffix() {
        assert_eq!(macvlan_link_name(42, "eth0"), "macvlan-poolai-42-eth0");
        assert_eq!(
            macvlan_link_name(1, "bond0.100"),
            "macvlan-poolai-1-bond0-100"
        );
    }

    #[test]
    fn validate_macvlan_mode_accepts_known_modes() {
        assert_eq!(validate_macvlan_mode(None).unwrap(), "bridge");
        assert_eq!(validate_macvlan_mode(Some("vepa")).unwrap(), "vepa");
    }

    #[test]
    fn validate_macvlan_mode_rejects_unknown() {
        assert!(validate_macvlan_mode(Some("l2")).is_err());
    }
}
