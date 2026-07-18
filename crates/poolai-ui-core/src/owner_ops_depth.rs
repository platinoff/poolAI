//! Owner ops UX band depth (PH-S1011…S1018, band 37) + UI polish (PH-S1019…S1026, band 38).

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

fn poolai_power_t_helper() -> &'static str {
    r#"function poolaiPowerT(key, fallback) {
  return (typeof poolaiT === "function") ? poolaiT(key, fallback) : fallback;
}"#
}

fn poolai_power_announce_helper() -> &'static str {
    r#"function poolaiPowerAnnounce(message, priority) {
  if (typeof adminAnnounceLive === "function") {
    adminAnnounceLive(message, priority);
    return;
  }
  var live = document.getElementById("aria_live_region") || document.getElementById("vision-power-status");
  if (live && message) live.textContent = message;
}"#
}

fn poolai_save_launch_prefs_helper() -> &'static str {
    r#"function poolaiSaveLaunchPrefs(storageKey) {
  try {
    var prefs = {
      port: window.location.port || "8080",
      path: window.location.pathname,
      saved_at: Date.now()
    };
    localStorage.setItem(storageKey || "poolai.ui.lastLaunch", JSON.stringify(prefs));
  } catch (e) { /* ignore */ }
}"#
}

/// Admin power panel markup + fetch glue (PH-S1015, PH-S1020/S1026).
pub fn admin_power_panel_script() -> String {
    format!(
        r#"(function () {{
  {poolai_power_t_helper}
  {poolai_power_announce_helper}
  {poolai_save_launch_prefs_helper}
  window.poolaiSaveAdminLaunchPrefs = function () {{
    poolaiSaveLaunchPrefs("poolai.admin.lastLaunch");
  }};
  window.poolaiAdminPowerAction = function (action) {{
    poolaiSaveAdminLaunchPrefs();
    try {{
      localStorage.setItem("poolai.admin.lastPower", JSON.stringify({{ action: action, saved_at: Date.now() }}));
    }} catch (e) {{ /* ignore */ }}
    return fetch("/api/v1/ops/power", {{
      method: "POST",
      headers: {{ "Content-Type": "application/json" }},
      body: JSON.stringify({{ action: action }})
    }}).then(function (r) {{ return r.json(); }});
  }};
  window.poolaiOpenAdminPowerModal = function () {{
    if (typeof showModal === "function") {{
      showModal("poolaiAdminPowerModal");
    }}
  }};
  window.poolaiCloseAdminPowerModal = function () {{
    if (typeof hideModal === "function") {{
      hideModal("poolaiAdminPowerModal");
    }}
  }};
  window.poolaiConfirmAdminPower = function (action) {{
    poolaiAdminPowerAction(action).then(function (body) {{
      var label = poolaiPowerT("admin.power.accepted", "accepted");
      poolaiPowerAnnounce(
        poolaiPowerT("admin.power.result", "Power {{action}}: {{note}}")
          .replace("{{action}}", action)
          .replace("{{note}}", body.note || label),
        "polite"
      );
      poolaiCloseAdminPowerModal();
    }}).catch(function () {{
      poolaiPowerAnnounce(poolaiPowerT("admin.power.failed", "Power action failed"), "assertive");
    }});
  }};
}})();"#,
        poolai_power_t_helper = poolai_power_t_helper(),
        poolai_power_announce_helper = poolai_power_announce_helper(),
        poolai_save_launch_prefs_helper = poolai_save_launch_prefs_helper(),
    )
}

/// Admin power modal HTML fragment (PH-S1015, PH-S1020 i18n).
pub fn admin_power_modal_html() -> String {
    r#"<motion.div id="poolaiAdminPowerModal" class="modal" role="dialog" aria-labelledby="poolaiAdminPowerTitle" aria-modal="false" aria-hidden="true">
  <div class="modal-content">
    <motion.div class="modal-header">
      <h3 id="poolaiAdminPowerTitle" data-i18n="admin.power.title">PoolAI power</h3>
      <button type="button" class="modal-close" data-i18n-aria="admin.power.close" aria-label="Close" onclick="poolaiCloseAdminPowerModal()">&times;</button>
    </motion.div>
    <p data-i18n="admin.power.body">Choose an action for the local stand (dev guard — no host reboot).</p>
    <div class="modal-footer">
      <button type="button" class="btn btn-danger" data-i18n="admin.power.shutdown" onclick="poolaiConfirmAdminPower('shutdown')">Shutdown</button>
      <button type="button" class="btn btn-secondary" data-i18n="admin.power.reboot" onclick="poolaiConfirmAdminPower('reboot')">Reboot</button>
      <button type="button" class="btn" data-i18n="admin.power.cancel" onclick="poolaiCloseAdminPowerModal()">Cancel</button>
    </div>
  </div>
</motion.div>"#
        .replace("<motion.", "<")
        .replace("</motion.", "</")
}

/// Home `/ui` shell power shortcut glue (PH-S1021).
pub fn home_power_shell_script() -> String {
    format!(
        r#"(function () {{
  {poolai_power_t_helper}
  {poolai_power_announce_helper}
  {poolai_save_launch_prefs_helper}
  function poolaiHomePowerAction(action) {{
    poolaiSaveLaunchPrefs("poolai.home.lastLaunch");
    try {{
      localStorage.setItem("poolai.home.lastPower", JSON.stringify({{ action: action, saved_at: Date.now() }}));
    }} catch (e) {{ /* ignore */ }}
    return fetch("/api/v1/ops/power", {{
      method: "POST",
      headers: {{ "Content-Type": "application/json" }},
      body: JSON.stringify({{ action: action }})
    }}).then(function (r) {{ return r.json(); }});
  }}
  function bindHomePowerButtons() {{
    var shutdown = document.getElementById("home-power-shutdown");
    var reboot = document.getElementById("home-power-reboot");
    if (shutdown) {{
      shutdown.addEventListener("click", function () {{
        poolaiHomePowerAction("shutdown").then(function (body) {{
          var label = poolaiPowerT("home.power.accepted", "accepted");
          poolaiPowerAnnounce(
            poolaiPowerT("home.power.result", "Power shutdown: {{note}}").replace("{{note}}", body.note || label),
            "polite"
          );
        }}).catch(function () {{
          poolaiPowerAnnounce(poolaiPowerT("home.power.failed", "Power action failed"), "assertive");
        }});
      }});
    }}
    if (reboot) {{
      reboot.addEventListener("click", function () {{
        poolaiHomePowerAction("reboot").then(function (body) {{
          var label = poolaiPowerT("home.power.accepted", "accepted");
          poolaiPowerAnnounce(
            poolaiPowerT("home.power.result", "Power reboot: {{note}}").replace("{{note}}", body.note || label),
            "polite"
          );
        }}).catch(function () {{
          poolaiPowerAnnounce(poolaiPowerT("home.power.failed", "Power action failed"), "assertive");
        }});
      }});
    }}
  }}
  if (document.readyState === "loading") {{
    document.addEventListener("DOMContentLoaded", bindHomePowerButtons);
  }} else {{
    bindHomePowerButtons();
  }}
}})();"#,
        poolai_power_t_helper = poolai_power_t_helper(),
        poolai_power_announce_helper = poolai_power_announce_helper(),
        poolai_save_launch_prefs_helper = poolai_save_launch_prefs_helper(),
    )
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
        assert!(script.contains("showModal(\"poolaiAdminPowerModal\")"));
        assert!(script.contains("/api/v1/ops/power"));
        assert!(script.contains("adminAnnounceLive"));
        assert!(script.contains("admin.power.result"));
    }

    #[test]
    fn home_power_shell_script_ph_s1021() {
        let script = home_power_shell_script();
        assert!(script.contains("home-power-shutdown"));
        assert!(script.contains("/api/v1/ops/power"));
        assert!(script.contains("poolai.home.lastPower"));
    }
}
