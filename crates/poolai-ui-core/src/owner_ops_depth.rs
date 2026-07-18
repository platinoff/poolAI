//! Owner ops UX band depth (PH-S1011…S1018, band 37).

use serde_json::Value;

/// Band-37 owner ops depth flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerOpsDepth {
    None,
    LightLaunch,
    QuickPreset,
    VisionLaunch,
    LastRunPersist,
    AdminPowerUi,
    PowerWire,
    VisionPowerUi,
    FullOwnerOps,
}

/// RUN_LOCAL / README markers for band 37.
pub const OWNER_OPS_BAND37_ROWS: &[&str] = &[
    "PH-S1011",
    "PH-S1012",
    "quick preset",
    "--light",
    "PH-S1013",
    "open-docs-vision",
    "PH-S1014",
    "last_run.json",
    "PH-S1015",
    "PH-S1016",
    "/api/v1/ops/power",
    "PH-S1017",
    "PH-S1018",
];

/// FM §5.17 owner queue rows.
pub const FM_BAND37_ROWS: &[&str] = &[
    "5.17",
    "owner ops UX v2",
    "PH-S1011…S1018",
    "light launch",
    "power controls",
];

pub fn owner_ops_depth_stub(features: Option<&Value>) -> OwnerOpsDepth {
    let Some(f) = features else {
        return OwnerOpsDepth::None;
    };
    let light = f
        .get("light_launch")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let quick = f
        .get("quick_preset")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vision = f
        .get("vision_launch")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let last_run = f
        .get("last_run_persist")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let admin_power = f
        .get("admin_power_ui")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let wire = f
        .get("power_wire")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vision_power = f
        .get("vision_power_ui")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if light && quick && vision && last_run && admin_power && wire && vision_power {
        OwnerOpsDepth::FullOwnerOps
    } else if vision_power {
        OwnerOpsDepth::VisionPowerUi
    } else if wire {
        OwnerOpsDepth::PowerWire
    } else if admin_power {
        OwnerOpsDepth::AdminPowerUi
    } else if last_run {
        OwnerOpsDepth::LastRunPersist
    } else if vision {
        OwnerOpsDepth::VisionLaunch
    } else if quick {
        OwnerOpsDepth::QuickPreset
    } else if light {
        OwnerOpsDepth::LightLaunch
    } else {
        OwnerOpsDepth::None
    }
}

/// Admin power panel markup + fetch glue (PH-S1015).
pub fn admin_power_panel_script() -> String {
    r#"(function () {
  function poolaiSaveAdminLaunchPrefs() {
    try {
      var prefs = {
        port: window.location.port || "8080",
        path: window.location.pathname,
        saved_at: Date.now()
      };
      localStorage.setItem("poolai.admin.lastLaunch", JSON.stringify(prefs));
    } catch (e) { /* ignore */ }
  }
  window.poolaiSaveAdminLaunchPrefs = poolaiSaveAdminLaunchPrefs;
  window.poolaiAdminPowerAction = function (action) {
    poolaiSaveAdminLaunchPrefs();
    return fetch("/api/v1/ops/power", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ action: action })
    }).then(function (r) { return r.json(); });
  };
  window.poolaiOpenAdminPowerModal = function () {
    var modal = document.getElementById("poolaiAdminPowerModal");
    if (!modal) return;
    modal.classList.add("show");
    modal.setAttribute("aria-hidden", "false");
    modal.setAttribute("aria-modal", "true");
  };
  window.poolaiCloseAdminPowerModal = function () {
    var modal = document.getElementById("poolaiAdminPowerModal");
    if (!modal) return;
    modal.classList.remove("show");
    modal.setAttribute("aria-hidden", "true");
    modal.setAttribute("aria-modal", "false");
  };
  window.poolaiConfirmAdminPower = function (action) {
    poolaiAdminPowerAction(action).then(function (body) {
      if (typeof poolaiAdminAnnounce === "function") {
        poolaiAdminAnnounce("Power " + action + ": " + (body.note || "accepted"));
      }
      poolaiCloseAdminPowerModal();
    }).catch(function () {
      if (typeof poolaiAdminAnnounce === "function") {
        poolaiAdminAnnounce("Power action failed");
      }
    });
  };
})();"#
        .to_string()
}

/// Admin power modal HTML fragment (PH-S1015).
pub fn admin_power_modal_html() -> String {
    r#"<div id="poolaiAdminPowerModal" class="modal" role="dialog" aria-labelledby="poolaiAdminPowerTitle" aria-modal="false" aria-hidden="true">
  <motion.div class="modal-content">
    <motion.div class="modal-header">
      <h3 id="poolaiAdminPowerTitle">PoolAI power</h3>
      <button type="button" class="modal-close" aria-label="Close" onclick="poolaiCloseAdminPowerModal()">&times;</button>
    </motion.div>
    <p>Оберіть дію для локального стенду (dev guard — без reboot хоста).</p>
    <div class="modal-footer">
      <button type="button" class="btn btn-danger" onclick="poolaiConfirmAdminPower('shutdown')">Виключити</button>
      <button type="button" class="btn btn-secondary" onclick="poolaiConfirmAdminPower('reboot')">Перезавантажити</button>
      <button type="button" class="btn" onclick="poolaiCloseAdminPowerModal()">Скасувати</button>
    </div>
  </motion.div>
</motion.div>"#
        .replace("<motion.", "<")
        .replace("</motion.", "</")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn owner_ops_depth_stub_ph_s1018() {
        assert_eq!(owner_ops_depth_stub(None), OwnerOpsDepth::None);
        assert_eq!(
            owner_ops_depth_stub(Some(&json!({"light_launch": true}))),
            OwnerOpsDepth::LightLaunch
        );
        assert_eq!(
            owner_ops_depth_stub(Some(&json!({
                "light_launch": true,
                "quick_preset": true,
                "vision_launch": true,
                "last_run_persist": true,
                "admin_power_ui": true,
                "power_wire": true,
                "vision_power_ui": true
            }))),
            OwnerOpsDepth::FullOwnerOps
        );
    }

    #[test]
    fn admin_power_panel_script_ph_s1015() {
        let script = admin_power_panel_script();
        assert!(script.contains("poolaiAdminPowerAction"));
        assert!(script.contains("/api/v1/ops/power"));
    }
}
