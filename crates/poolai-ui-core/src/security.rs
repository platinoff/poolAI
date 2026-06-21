//! Admin security secret rotation panel HTML (PH-S810).

use crate::format::{escape_html, format_rotation_kind, format_unix_timestamp_display};
use serde_json::Value;

fn t(i18n: &Value, key: &str, fallback: &str) -> String {
    i18n.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string()
}

fn rotation_kind_label(i18n: &Value, kind: &str) -> String {
    let key = match kind {
        "jwt" => Some("admin.sec.rot.kind.jwt"),
        "tls_certificate" => Some("admin.sec.rot.kind.tls"),
        "telegram_webhook" => Some("admin.sec.rot.kind.telegram"),
        _ => None,
    };
    match key {
        Some(k) => t(i18n, k, &format_rotation_kind(kind)),
        None => format_rotation_kind(kind),
    }
}

/// Secret rotation admin table panel (PH-S810 wasm slim).
pub fn render_secret_rotation_panel_html(rows_json: &str, i18n_json: &str) -> String {
    let rows: Value = serde_json::from_str(rows_json).unwrap_or(Value::Null);
    let i18n: Value = serde_json::from_str(i18n_json).unwrap_or(Value::Null);
    let never = t(&i18n, "admin.sec.rot.never", "Never");

    let rows_html: String = rows
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .map(|r| {
                    let kind = r.get("kind").and_then(|v| v.as_str()).unwrap_or("");
                    let configured = r.get("configured").and_then(|v| v.as_bool()).unwrap_or(false);
                    let hook_count = r.get("hook_count").and_then(|v| v.as_u64()).unwrap_or(0);
                    let last_unix = r
                        .get("last_rotated_unix")
                        .and_then(|v| v.as_u64())
                        .map(|n| n as i64);
                    let rotation_count = r.get("rotation_count").and_then(|v| v.as_u64()).unwrap_or(0);
                    let grace_active = r
                        .get("grace_active")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let kind_escaped = escape_html(kind);
                    let kind_json = serde_json::to_string(kind).unwrap_or_else(|_| "\"\"".to_string());
                    let status_badge = if configured {
                        format!(
                            r#"<span class="status-badge active">{}</span>"#,
                            escape_html(&t(
                                &i18n,
                                "admin.sec.rot.configured",
                                "Configured"
                            ))
                        )
                    } else {
                        format!(
                            r#"<span class="status-badge inactive">{}</span>"#,
                            escape_html(&t(
                                &i18n,
                                "admin.sec.rot.notConfigured",
                                "Not configured"
                            ))
                        )
                    };
                    let action_btn = if kind == "jwt" {
                        format!(
                            r#"<button type="button" class="btn btn-primary" onclick='rotateSecret({kind_json})'>{}</button>"#,
                            escape_html(&t(
                                &i18n,
                                "admin.sec.rot.reloadJwt",
                                "Reload JWT from env"
                            ))
                        )
                    } else if configured && hook_count > 0 {
                        format!(
                            r#"<button type="button" class="btn" onclick='rotateSecret({kind_json})'>{}</button>"#,
                            escape_html(&t(&i18n, "admin.sec.rot.run", "Run rotation"))
                        )
                    } else {
                        format!(
                            r#"<span class="muted">{}</span>"#,
                            escape_html(&t(&i18n, "admin.na", "N/A"))
                        )
                    };
                    let grace_label = if grace_active {
                        t(&i18n, "admin.mon.enabled", "Enabled")
                    } else {
                        t(&i18n, "admin.mon.disabled", "Disabled")
                    };
                    format!(
                        r#"<tr>
<td><strong>{kind_label}</strong><br><code>{kind_code}</code></td>
<td>{status}</td>
<td>{hook_count}</td>
<td>{last_rotated}</td>
<td>{rotation_count}</td>
<td>{grace}</td>
<td>{action}</td>
</tr>"#,
                        kind_label = escape_html(&rotation_kind_label(&i18n, kind)),
                        kind_code = kind_escaped,
                        status = status_badge,
                        hook_count = escape_html(&hook_count.to_string()),
                        last_rotated = escape_html(&format_unix_timestamp_display(
                            last_unix,
                            &never
                        )),
                        rotation_count = escape_html(&rotation_count.to_string()),
                        grace = escape_html(&grace_label),
                        action = action_btn,
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default();

    format!(
        r#"<div class="admin-header">
<h3>{heading}</h3>
<button type="button" class="btn" onclick="loadSecretRotation()">{refresh}</button>
</div>
<table class="admin-table" id="secret-rotation-table">
<thead><tr>
<th>{col_kind}</th>
<th>{col_status}</th>
<th>{col_hooks}</th>
<th>{col_last}</th>
<th>{col_count}</th>
<th>{col_grace}</th>
<th>{col_actions}</th>
</tr></thead>
<tbody>{rows_html}</tbody>
</table>
<p class="muted">{hint}</p>"#,
        heading = escape_html(&t(&i18n, "admin.sec.rot.heading", "Secret rotation")),
        refresh = escape_html(&t(&i18n, "admin.topo.refresh", "Refresh")),
        col_kind = escape_html(&t(&i18n, "admin.sec.rot.col.kind", "Secret")),
        col_status = escape_html(&t(&i18n, "admin.mon.col.statusCol", "Status")),
        col_hooks = escape_html(&t(&i18n, "admin.sec.rot.col.hooks", "Hooks")),
        col_last = escape_html(&t(&i18n, "admin.sec.rot.col.last", "Last rotated")),
        col_count = escape_html(&t(&i18n, "admin.sec.rot.col.count", "Count")),
        col_grace = escape_html(&t(&i18n, "admin.sec.rot.col.grace", "JWT grace")),
        col_actions = escape_html(&t(&i18n, "admin.mon.col.actions", "Actions")),
        hint = escape_html(&t(
            &i18n,
            "admin.sec.rot.hint",
            "Rotation runs registered hooks only; env vars must be set on the coordinator host."
        )),
        rows_html = rows_html,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn render_secret_rotation_panel_ph_s810() {
        let rows = json!([{
            "kind": "jwt",
            "configured": true,
            "hook_count": 1,
            "last_rotated_unix": 1_712_998_800u64,
            "rotation_count": 2,
            "grace_active": false
        }]);
        let html = render_secret_rotation_panel_html(
            &rows.to_string(),
            r#"{"admin.sec.rot.heading":"Secret rotation"}"#,
        );
        assert!(html.contains("secret-rotation-table"));
        assert!(html.contains("rotateSecret"));
        assert!(html.contains("<code>jwt</code>"));
        assert!(html.contains("Reload JWT from env"));
    }
}
