//! Admin libraries table HTML (PH-S821 wasm glue).

use serde_json::Value;

fn library_key(lib: &Value) -> String {
    lib.get("name")
        .or_else(|| lib.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}

fn library_installed(lib: &Value) -> bool {
    if lib.get("installed").and_then(|v| v.as_bool()) == Some(true) {
        return true;
    }
    lib.get("metadata")
        .and_then(|m| m.get("installed_at"))
        .is_some()
}

/// Libraries table HTML for admin panel (PH-S821).
#[allow(clippy::too_many_arguments)]
pub fn render_libs_panel_html(
    libs_json: &str,
    col_name: &str,
    col_version: &str,
    col_status: &str,
    col_actions: &str,
    table_aria: &str,
    installed_label: &str,
    not_installed_label: &str,
    uninstall_label: &str,
    update_label: &str,
    install_label: &str,
    empty_message: &str,
) -> String {
    use crate::format::escape_html;
    use crate::table::empty_state_html;

    let libs: Vec<Value> = serde_json::from_str(libs_json).unwrap_or_default();
    if libs.is_empty() {
        return empty_state_html(empty_message, None, "📚", None);
    }
    let rows: String = libs
        .iter()
        .map(|lib| {
            let key = library_key(lib);
            let version = lib
                .get("version")
                .or_else(|| lib.get("installed_version"))
                .and_then(|v| v.as_str())
                .unwrap_or("N/A");
            let installed = library_installed(lib);
            let status_badge = if installed {
                format!(
                    r#"<span class="status-badge active">{}</span>"#,
                    escape_html(installed_label)
                )
            } else {
                format!(
                    r#"<span class="status-badge inactive">{}</span>"#,
                    escape_html(not_installed_label)
                )
            };
            let actions = if installed {
                format!(
                    r#"<button type="button" class="btn" data-lib-name="{}" data-lib-action="uninstall">{}</button>
                    <button type="button" class="btn" data-lib-name="{}" data-lib-action="update">{}</button>"#,
                    escape_html(&key),
                    escape_html(uninstall_label),
                    escape_html(&key),
                    escape_html(update_label),
                )
            } else {
                format!(
                    r#"<button type="button" class="btn btn-primary" data-lib-name="{}" data-lib-action="install">{}</button>"#,
                    escape_html(&key),
                    escape_html(install_label),
                )
            };
            format!(
                r#"<tr>
                  <td><strong>{}</strong></td>
                  <td>{}</td>
                  <td>{}</td>
                  <td>{}</td>
                </tr>"#,
                escape_html(&key),
                escape_html(version),
                status_badge,
                actions,
            )
        })
        .collect();
    format!(
        r#"<div class="admin-table-container"><table class="admin-table" aria-label="{}">
            <thead>
              <tr>
                <th>{}</th>
                <th>{}</th>
                <th>{}</th>
                <th>{}</th>
              </tr>
            </thead>
            <tbody>{}</tbody>
          </table></div>"#,
        escape_html(table_aria),
        escape_html(col_name),
        escape_html(col_version),
        escape_html(col_status),
        escape_html(col_actions),
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_libs_panel_html_ph_s821() {
        let html = render_libs_panel_html(
            r#"[{"name":"lib-a","version":"1.0.0","installed":true}]"#,
            "Name",
            "Version",
            "Status",
            "Actions",
            "Libraries",
            "Installed",
            "Not Installed",
            "Uninstall",
            "Update",
            "Install",
            "No libraries",
        );
        assert!(html.contains("admin-table"));
        assert!(html.contains("lib-a"));
        assert!(html.contains("data-lib-action=\"uninstall\""));
    }

    #[test]
    fn render_libs_panel_empty_ph_s821() {
        let html = render_libs_panel_html(
            "[]", "N", "V", "S", "A", "L", "I", "NI", "U", "Up", "In", "Empty",
        );
        assert!(html.contains("admin-empty-state"));
    }
}
