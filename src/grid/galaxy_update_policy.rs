//! Galaxy governance update policy env stub (PH-S549, §9.5 / §9.8).

use crate::grid::galaxy_governance_metrics::set_update_notify_pending;

/// Env: `notify` | `auto` | `never` (Galaxy §9.5 opt-in update policy).
pub const ENV_UPDATE_POLICY: &str = "POOLAI_UPDATE_POLICY";

/// Optional signed release manifest URL for notify tick audit.
pub const ENV_RELEASE_MANIFEST_URL: &str = "POOLAI_RELEASE_MANIFEST_URL";

/// Parsed update policy mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdatePolicyMode {
    Notify,
    Auto,
    Never,
}

/// Config from env (defaults to `notify`).
pub fn update_policy_from_env() -> UpdatePolicyMode {
    match std::env::var(ENV_UPDATE_POLICY)
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("auto") => UpdatePolicyMode::Auto,
        Some("never") | Some("off") => UpdatePolicyMode::Never,
        _ => UpdatePolicyMode::Notify,
    }
}

/// Optional manifest URL from env.
pub fn release_manifest_url_from_env() -> Option<String> {
    std::env::var(ENV_RELEASE_MANIFEST_URL)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Notify tick: bump pending gauge + audit log (PH-S549).
pub fn tick_update_notify_from_env() {
    if update_policy_from_env() != UpdatePolicyMode::Notify {
        return;
    }
    let pending = crate::grid::galaxy_governance_metrics::update_notify_pending().saturating_add(1);
    set_update_notify_pending(pending);
    if let Some(url) = release_manifest_url_from_env() {
        tracing::info!(
            target: "poolai_update_notify",
            pending,
            manifest_url = %url,
            "update notify tick (Galaxy §9.8)"
        );
    } else {
        tracing::info!(
            target: "poolai_update_notify",
            pending,
            "update notify tick (Galaxy §9.8)"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_policy_notify_tick_ph_s549() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::grid::galaxy_governance_metrics::reset_governance_metrics_for_test();
        std::env::set_var(ENV_UPDATE_POLICY, "notify");
        std::env::set_var(
            ENV_RELEASE_MANIFEST_URL,
            "https://example.com/manifest.json",
        );
        tick_update_notify_from_env();
        assert_eq!(
            crate::grid::galaxy_governance_metrics::update_notify_pending(),
            1
        );
        std::env::remove_var(ENV_UPDATE_POLICY);
        std::env::remove_var(ENV_RELEASE_MANIFEST_URL);
        crate::grid::galaxy_governance_metrics::reset_governance_metrics_for_test();
    }

    #[test]
    fn update_policy_never_skips_tick_ph_s549() {
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        crate::grid::galaxy_governance_metrics::reset_governance_metrics_for_test();
        std::env::set_var(ENV_UPDATE_POLICY, "never");
        tick_update_notify_from_env();
        assert_eq!(
            crate::grid::galaxy_governance_metrics::update_notify_pending(),
            0
        );
        std::env::remove_var(ENV_UPDATE_POLICY);
    }
}
