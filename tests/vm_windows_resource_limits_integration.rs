//! Integration tests for Windows resource limits (Job Objects, post-spawn).

#[cfg(not(target_os = "windows"))]
use poolai::vm::PlatformResourceLimiter;
use poolai::vm::{validate_resource_limits, ResourceLimiter, ResourceLimits};
#[cfg(target_os = "windows")]
use poolai::vm::{
    PlatformResourceLimiter, VmIsolation, VmManager, VmResources, WindowsJobObjectLimiter,
};
use uuid::Uuid;

#[cfg(target_os = "windows")]
const WIN_PING: &str = r"C:\Windows\System32\ping.exe";

#[cfg(target_os = "windows")]
fn spawn_short_lived_process() -> tokio::process::Child {
    tokio::process::Command::new(WIN_PING)
        .args(["127.0.0.1", "-n", "3"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn Windows ping.exe")
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_windows_job_object_limiter_creation() {
    let limiter = WindowsJobObjectLimiter::new().expect("limiter init");
    let state = limiter.get_job_object_state(Uuid::new_v4()).await;
    assert!(state.is_none());
}

#[tokio::test]
async fn test_apply_limits_pre_spawn_validates() {
    let limiter: Box<dyn ResourceLimiter> = Box::new(PlatformResourceLimiter::new());
    let limits = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 2048,
        gpu_device: None,
    };
    #[cfg(target_os = "windows")]
    let mut command = tokio::process::Command::new(WIN_PING);
    #[cfg(not(target_os = "windows"))]
    let mut command = tokio::process::Command::new("echo");
    let result = limiter.apply_limits(&mut command, &limits).await;
    assert!(result.is_ok());
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_apply_limits_post_spawn_records_job_state() {
    let limiter = PlatformResourceLimiter::new();
    let instance_id = Uuid::new_v4();
    let limits = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 256,
        gpu_device: None,
    };

    let mut child = spawn_short_lived_process();
    let pid = child.id().expect("pid");

    limiter
        .apply_limits_post_spawn(instance_id, pid, &limits)
        .await
        .expect("post-spawn limits");

    let state = limiter
        .windows_limiter()
        .get_job_object_state(instance_id)
        .await
        .expect("job state");
    assert_eq!(state.cpu_cores, Some(2));
    assert_eq!(state.memory_mb, Some(256));
    assert!(state.process_assigned);

    let _ = child.kill().await;
}

#[tokio::test]
async fn test_resource_limits_validation_windows() {
    let too_low = ResourceLimits {
        cpu_cores: 2,
        memory_mb: 32,
        gpu_device: None,
    };
    assert!(validate_resource_limits(&too_low).is_err());

    let ok = ResourceLimits {
        cpu_cores: 0,
        memory_mb: 128,
        gpu_device: None,
    };
    assert!(validate_resource_limits(&ok).is_ok());
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_vm_manager_post_spawn_resource_limits() {
    let manager = VmManager::new();
    assert!(manager.is_resource_limits_supported());

    let mut resources = VmResources::default();
    resources.cpu_cores = 1;
    resources.memory_mb = 128;
    resources.gpu_required = false;
    let instance = manager
        .create_instance(
            "limits-test".to_string(),
            resources,
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();

    let mut child = spawn_short_lived_process();
    let pid = child.id().expect("pid");

    manager
        .apply_instance_resource_limits_post_spawn(instance.id, pid)
        .await
        .expect("post-spawn via VmManager");

    let _ = child.kill().await;
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_vm_manager_resource_usage_without_process() {
    let manager = VmManager::new();
    let instance = manager
        .create_instance(
            "no-process".to_string(),
            VmResources::default(),
            VmIsolation::ProcessSandbox,
        )
        .await
        .unwrap();
    let result = manager.get_instance_resource_usage(instance.id).await;
    assert!(result.is_err());
}
