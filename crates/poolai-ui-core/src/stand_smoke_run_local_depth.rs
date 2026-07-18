//! Stand smoke + RUN_LOCAL ops band depth (PH-S1089…S1098, band 45).

use serde_json::Value;

/// Band-45 stand smoke + RUN_LOCAL ops depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandSmokeRunLocalDepth {
    None,
    HealthShape,
    MonitoringAlerts,
    MonitoringDashboards,
    VmInstances,
    RunLocalSmoke,
    VerifyDevStandHook,
    QuickStandSmoke,
    DocsCanon,
    FullRunLocalBand45,
}

/// Required `/api/v1/health` keys for `run-poolai quick` wait gate (PH-S1089).
pub const RUN_LOCAL_HEALTH_KEYS: &[&str] = &["status", "version", "checks"];

/// `--run-local-smoke` case names (PH-S1093).
pub const RUN_LOCAL_SMOKE_CASES: &[&str] = &[
    "health",
    "monitoring_alerts",
    "monitoring_dashboards",
    "vm_instances",
    "ops_power_openapi",
    "jobs_store_backend",
];

/// FM §5.26 band-45 marker rows.
pub const FM_BAND45_ROWS: &[&str] = &[
    "5.26",
    "stand smoke + RUN_LOCAL",
    "PH-S1089…S1098",
    "stand_smoke_run_local_depth",
];

/// Stand smoke RUN_LOCAL adoption markers for band 45.
pub const STAND_SMOKE_RUN_LOCAL_BAND45_ROWS: &[&str] = &[
    "PH-S1089",
    "RUN_LOCAL_HEALTH_KEYS",
    "PH-S1090",
    "monitoring_alerts",
    "PH-S1093",
    "--run-local-smoke",
    "PH-S1094",
    "VERIFY_STAND_SMOKE",
    "PH-S1095",
    "stand_smoke_run_local_depth",
    "PH-S1098",
];

pub fn stand_smoke_run_local_depth_stub(features: Option<&Value>) -> StandSmokeRunLocalDepth {
    let Some(f) = features else {
        return StandSmokeRunLocalDepth::None;
    };
    let health = f
        .get("health_shape")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let alerts = f
        .get("monitoring_alerts")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let dashboards = f
        .get("monitoring_dashboards")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vm = f
        .get("vm_instances")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let run_local = f
        .get("run_local_smoke")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let verify = f
        .get("verify_dev_stand_hook")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let quick = f
        .get("quick_stand_smoke")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let docs = f
        .get("docs_canon")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if run_local && verify && quick && docs && health && alerts && dashboards && vm {
        return StandSmokeRunLocalDepth::FullRunLocalBand45;
    }
    if run_local && verify && quick {
        return StandSmokeRunLocalDepth::FullRunLocalBand45;
    }
    if quick {
        return StandSmokeRunLocalDepth::QuickStandSmoke;
    }
    if verify {
        return StandSmokeRunLocalDepth::VerifyDevStandHook;
    }
    if run_local {
        return StandSmokeRunLocalDepth::RunLocalSmoke;
    }
    if vm {
        return StandSmokeRunLocalDepth::VmInstances;
    }
    if dashboards {
        return StandSmokeRunLocalDepth::MonitoringDashboards;
    }
    if alerts {
        return StandSmokeRunLocalDepth::MonitoringAlerts;
    }
    if health {
        return StandSmokeRunLocalDepth::HealthShape;
    }
    if docs {
        return StandSmokeRunLocalDepth::DocsCanon;
    }
    StandSmokeRunLocalDepth::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stand_smoke_run_local_depth_stub_ph_s1095() {
        assert_eq!(
            stand_smoke_run_local_depth_stub(None),
            StandSmokeRunLocalDepth::None
        );
        assert_eq!(
            stand_smoke_run_local_depth_stub(Some(&json!({"health_shape": true}))),
            StandSmokeRunLocalDepth::HealthShape
        );
        assert!(!RUN_LOCAL_SMOKE_CASES.is_empty());
        assert!(FM_BAND45_ROWS.contains(&"PH-S1089…S1098"));
    }
}
