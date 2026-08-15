//! Integration tests for VM Isolation module
//!
//! Tests:
//! - Network isolation configuration
//! - Filesystem isolation configuration
//! - Platform isolator creation
//! - Isolation support detection
//! - Isolation application (placeholder tests)

use poolai::vm::{
    FilesystemIsolationConfig, FilesystemIsolator, NetworkInterfaceMode, NetworkIsolationConfig,
    NetworkIsolator, PlatformFilesystemIsolator, PlatformNetworkIsolator,
};
#[cfg(target_os = "windows")]
use poolai::vm::{VmIsolation, VmManager, VmResources};

#[tokio::test]
async fn test_network_isolation_config_default() {
    let config = NetworkIsolationConfig::default();
    assert!(!config.enabled);
    assert!(config.allowed_interfaces.is_empty());
    assert!(config.allowed_ports.is_empty());
    assert!(config.allow_loopback);
}

#[tokio::test]
async fn test_network_isolation_config_custom() {
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["eth0".to_string(), "lo".to_string()],
        allowed_ports: vec![80, 443, 8080],
        allow_loopback: false,
        strict: false,
        ..Default::default()
    };

    assert!(config.enabled);
    assert_eq!(config.allowed_interfaces.len(), 2);
    assert_eq!(config.allowed_ports.len(), 3);
    assert!(!config.allow_loopback);
    assert_eq!(config.interface_mode, NetworkInterfaceMode::Veth);
}

#[tokio::test]
async fn test_network_isolation_macvlan_config() {
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["eth0".to_string()],
        interface_mode: NetworkInterfaceMode::Macvlan,
        macvlan_mode: Some("private".into()),
        macvlan_address: Some("10.0.0.50/24".into()),
        ..Default::default()
    };
    assert_eq!(config.interface_mode, NetworkInterfaceMode::Macvlan);
    assert_eq!(config.macvlan_mode.as_deref(), Some("private"));
}

#[cfg(target_os = "linux")]
#[test]
fn test_macvlan_link_name_helper() {
    use poolai::vm::linux::{macvlan_link_name, validate_macvlan_mode, veth_link_names};

    assert_eq!(macvlan_link_name(7, "eth0"), "macvlan-poolai-7-eth0");
    assert_eq!(validate_macvlan_mode(None).unwrap(), "bridge");
    assert!(validate_macvlan_mode(Some("invalid")).is_err());
    let (host, ns) = veth_link_names(7, "eth0");
    assert!(host.ends_with("-h"));
    assert!(ns.ends_with("-n"));
}

/// Round-trip apply/remove with loopback-only config (needs `vm-isolation-linux` + CAP_SYS_ADMIN).
#[cfg(all(target_os = "linux", feature = "vm-isolation-linux"))]
#[tokio::test]
async fn test_network_isolation_apply_remove_loopback_linux() {
    let isolator = PlatformNetworkIsolator::new();
    let process_id = 90_001u32;
    let config = NetworkIsolationConfig {
        enabled: true,
        allow_loopback: true,
        strict: false,
        ..Default::default()
    };

    let apply = isolator.apply_network_isolation(process_id, &config);
    if apply.is_err() {
        // Graceful skip on hosts without unshare privileges (e.g. CI without caps).
        return;
    }
    assert!(isolator.remove_network_isolation(process_id).is_ok());
    // Idempotent second remove
    assert!(isolator.remove_network_isolation(process_id).is_ok());
}

#[tokio::test]
async fn test_filesystem_isolation_config_default() {
    let config = FilesystemIsolationConfig::default();
    assert!(!config.enabled);
    assert!(config.root_dir.is_none());
    assert!(config.allowed_paths.is_empty());
    assert!(config.read_only_paths.is_empty());
    assert!(!config.use_chroot);
}

#[tokio::test]
async fn test_filesystem_isolation_config_custom() {
    use std::path::PathBuf;

    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: Some(PathBuf::from("/tmp/vm-root")),
        allowed_paths: vec![PathBuf::from("/tmp/allowed")],
        read_only_paths: vec![PathBuf::from("/tmp/readonly")],
        use_chroot: true,
        strict: false,
    };

    assert!(config.enabled);
    assert!(config.root_dir.is_some());
    assert_eq!(config.allowed_paths.len(), 1);
    assert_eq!(config.read_only_paths.len(), 1);
    assert!(config.use_chroot);
}

#[tokio::test]
async fn test_platform_network_isolator_creation() {
    let _isolator = PlatformNetworkIsolator::new();
}

#[tokio::test]
async fn test_platform_filesystem_isolator_creation() {
    let _isolator = PlatformFilesystemIsolator::new();
}

#[tokio::test]
async fn test_network_isolation_support_detection() {
    let isolator = PlatformNetworkIsolator::new();
    let supported = isolator.is_supported();

    // On Linux/Windows, should be true (even if not fully implemented)
    // On other platforms, should be false
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    assert!(
        supported,
        "Network isolation should be supported on Linux/Windows"
    );

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    assert!(
        !supported,
        "Network isolation should not be supported on unsupported platforms"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_support_detection() {
    let isolator = PlatformFilesystemIsolator::new();
    let supported = isolator.is_supported();

    // On Linux/Windows, should be true (even if not fully implemented)
    // On other platforms, should be false
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    assert!(
        supported,
        "Filesystem isolation should be supported on Linux/Windows"
    );

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    assert!(
        !supported,
        "Filesystem isolation should not be supported on unsupported platforms"
    );
}

#[tokio::test]
async fn test_network_isolation_apply_disabled() {
    let isolator = PlatformNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: false,
        ..Default::default()
    };

    // Should succeed even if isolation is disabled
    let result = isolator.apply_network_isolation(12345, &config);
    assert!(result.is_ok(), "Applying disabled isolation should succeed");
}

#[tokio::test]
async fn test_filesystem_isolation_apply_disabled() {
    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: false,
        ..Default::default()
    };

    // Should succeed even if isolation is disabled
    let result = isolator.apply_filesystem_isolation(12345, &config);
    assert!(result.is_ok(), "Applying disabled isolation should succeed");
}

#[tokio::test]
async fn test_network_isolation_remove() {
    let isolator = PlatformNetworkIsolator::new();

    // Removing isolation should always succeed (even if not applied)
    let result = isolator.remove_network_isolation(12345);
    assert!(result.is_ok(), "Removing network isolation should succeed");
}

#[tokio::test]
async fn test_filesystem_isolation_remove() {
    let isolator = PlatformFilesystemIsolator::new();

    // Removing isolation should always succeed (even if not applied)
    let result = isolator.remove_filesystem_isolation(12345);
    assert!(
        result.is_ok(),
        "Removing filesystem isolation should succeed"
    );
}

#[tokio::test]
async fn test_network_isolation_apply_enabled() {
    let isolator = PlatformNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["eth0".to_string()],
        allowed_ports: vec![80, 443],
        allow_loopback: true,
        strict: false,
        ..Default::default()
    };

    // Should succeed (even if not fully implemented, placeholders return Ok)
    let result = isolator.apply_network_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Applying network isolation should succeed (placeholder)"
    );
}

#[tokio::test]
async fn test_network_isolation_apply_invalid_config() {
    let isolator = PlatformNetworkIsolator::new();
    // Config that would block all network access
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec![],
        allowed_ports: vec![],
        allow_loopback: false,
        strict: false,
        ..Default::default()
    };

    // Should fail validation
    let result = isolator.apply_network_isolation(12345, &config);
    assert!(
        result.is_err(),
        "Network isolation with blocking config should fail validation"
    );
}

#[tokio::test]
async fn test_network_isolation_apply_invalid_process_id() {
    let isolator = PlatformNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["eth0".to_string()],
        allowed_ports: vec![80, 443],
        allow_loopback: true,
        strict: false,
        ..Default::default()
    };

    // Should fail with invalid process ID
    let result = isolator.apply_network_isolation(0, &config);
    assert!(
        result.is_err(),
        "Network isolation with process ID 0 should fail"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_apply_invalid_process_id() {
    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: Some(std::path::PathBuf::from("/tmp/vm-root")),
        allowed_paths: vec![],
        read_only_paths: vec![],
        use_chroot: true,
        strict: false,
    };

    // Should fail with invalid process ID
    let result = isolator.apply_filesystem_isolation(0, &config);
    assert!(
        result.is_err(),
        "Filesystem isolation with process ID 0 should fail"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_apply_chroot_without_root_dir() {
    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: None,
        allowed_paths: vec![],
        read_only_paths: vec![],
        use_chroot: true, // Requires root_dir
        strict: false,
    };

    // Should fail validation
    let result = isolator.apply_filesystem_isolation(12345, &config);
    assert!(
        result.is_err(),
        "Filesystem isolation with use_chroot but no root_dir should fail"
    );
}

#[tokio::test]
async fn test_network_isolation_graceful_degradation() {
    let isolator = PlatformNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["eth0".to_string()],
        allowed_ports: vec![80, 443],
        allow_loopback: true,
        strict: false, // Graceful degradation enabled
        ..Default::default()
    };

    // Should succeed even if system calls fail (graceful degradation)
    let result = isolator.apply_network_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Network isolation should succeed with graceful degradation"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_graceful_degradation() {
    use std::path::PathBuf;

    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: Some(PathBuf::from("/tmp/vm-root")),
        allowed_paths: vec![PathBuf::from("/tmp/allowed")],
        read_only_paths: vec![PathBuf::from("/tmp/readonly")],
        use_chroot: true,
        strict: false, // Graceful degradation enabled
    };

    // Should succeed even if system calls fail (graceful degradation)
    let result = isolator.apply_filesystem_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Filesystem isolation should succeed with graceful degradation"
    );
}

#[tokio::test]
async fn test_network_isolation_loopback_setup() {
    let isolator = PlatformNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec![],
        allowed_ports: vec![],
        allow_loopback: true, // Enable loopback
        strict: false,
        ..Default::default()
    };

    // Should succeed with loopback enabled
    // Note: On Windows or without root, this may only validate config
    let result = isolator.apply_network_isolation(12345, &config);
    // Should succeed (either actually set up loopback or gracefully degrade)
    assert!(
        result.is_ok(),
        "Network isolation with loopback should succeed"
    );
}

#[tokio::test]
async fn test_network_isolation_no_loopback() {
    let isolator = PlatformNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["eth0".to_string()], // At least one interface allowed
        allowed_ports: vec![80, 443],
        allow_loopback: false, // Disable loopback
        strict: false,
        ..Default::default()
    };

    // Should succeed even without loopback (other interfaces allowed)
    let result = isolator.apply_network_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Network isolation without loopback should succeed if other interfaces allowed"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_bind_mounts() {
    use std::path::PathBuf;

    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: None, // No chroot, just bind mounts
        allowed_paths: vec![PathBuf::from("/tmp/test-allowed")],
        read_only_paths: vec![PathBuf::from("/tmp/test-readonly")],
        use_chroot: false,
        strict: false,
    };

    // Should succeed (either actually set up bind mounts or gracefully degrade)
    // Note: On Windows or without root, this may only validate config
    let result = isolator.apply_filesystem_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Filesystem isolation with bind mounts should succeed"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_read_only_mounts() {
    use std::path::PathBuf;

    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: None,
        allowed_paths: vec![],
        read_only_paths: vec![
            PathBuf::from("/tmp/test-readonly-1"),
            PathBuf::from("/tmp/test-readonly-2"),
        ],
        use_chroot: false,
        strict: false,
    };

    // Should succeed with read-only mounts
    let result = isolator.apply_filesystem_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Filesystem isolation with read-only mounts should succeed"
    );
}

#[tokio::test]
async fn test_filesystem_isolation_apply_enabled() {
    use std::path::PathBuf;

    let isolator = PlatformFilesystemIsolator::new();
    let config = FilesystemIsolationConfig {
        enabled: true,
        root_dir: Some(PathBuf::from("/tmp/vm-root")),
        allowed_paths: vec![PathBuf::from("/tmp/allowed")],
        read_only_paths: vec![PathBuf::from("/tmp/readonly")],
        use_chroot: true,
        strict: false,
    };

    // Should succeed (even if not fully implemented, placeholders return Ok)
    let result = isolator.apply_filesystem_isolation(12345, &config);
    assert!(
        result.is_ok(),
        "Applying filesystem isolation should succeed (placeholder)"
    );
}

#[test]
fn test_windows_plan_firewall_rules_from_config() {
    use poolai::vm::windows_plan::{plan_firewall_rules, plan_network_isolation};

    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_interfaces: vec!["Ethernet".to_string()],
        allowed_ports: vec![8080],
        allow_loopback: true,
        strict: false,
        ..Default::default()
    };
    let rules = plan_firewall_rules(1001, &config);
    assert!(rules.len() >= 3);
    let state = plan_network_isolation(1001, &config);
    assert_eq!(state.profile_name, "PoolAI-VM-1001");
    assert_eq!(state.firewall_rules.len(), rules.len());
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_windows_network_isolator_tracks_plan() {
    use poolai::vm::WindowsNetworkIsolator;

    let isolator = WindowsNetworkIsolator::new();
    let config = NetworkIsolationConfig {
        enabled: true,
        allowed_ports: vec![8080],
        allow_loopback: true,
        ..Default::default()
    };
    isolator.apply_network_isolation(4242, &config).unwrap();
    let state = isolator
        .get_appcontainer_state(4242)
        .expect("state tracked");
    assert_eq!(state.profile_name, "PoolAI-VM-4242");
    assert!(!state.firewall_rules.is_empty());
    isolator.remove_network_isolation(4242).unwrap();
    assert!(isolator.get_appcontainer_state(4242).is_none());
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_vm_manager_apply_isolation_hardware_vm_tracks_plans() {
    let manager = VmManager::new();

    let instance = manager
        .create_instance(
            "hw-vm".to_string(),
            VmResources::default(),
            VmIsolation::HardwareVm,
        )
        .await
        .expect("instance created");

    let process_id = 4243u32;
    manager
        .apply_isolation(process_id, instance.id)
        .await
        .expect("apply_isolation should succeed for HardwareVm");

    let net_state = manager
        .get_network_isolation_plan_state(process_id)
        .expect("network plan stored");
    assert_eq!(net_state.profile_name, "PoolAI-VM-4243");
    assert!(!net_state.firewall_rules.is_empty());

    let fs_state = manager
        .get_filesystem_isolation_plan_state(process_id)
        .expect("filesystem plan stored");
    assert_eq!(fs_state.profile_name, "PoolAI-VM-4243");
}
