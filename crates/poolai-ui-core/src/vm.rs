//! Admin VM instances table HTML (PH-S499 wasm glue).

use serde_json::Value;

fn vm_status_badge_class(status: &str) -> &'static str {
    let low = status.to_ascii_lowercase();
    if low.starts_with("failed") {
        "error"
    } else if low == "stopped" {
        "inactive"
    } else {
        "active"
    }
}

fn vm_status_label(status: &Value) -> String {
    match status {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// VM instances table HTML for admin panel (PH-S499).
pub fn render_vm_panel_html(
    instances_json: &str,
    col_name: &str,
    col_status: &str,
    col_resources: &str,
    col_actions: &str,
    table_aria: &str,
    res_cpu_label: &str,
    res_mem_label: &str,
    start_label: &str,
    stop_label: &str,
    delete_label: &str,
    empty_message: &str,
) -> String {
    use crate::format::escape_html;
    use crate::table::empty_state_html;

    let instances: Vec<Value> = serde_json::from_str(instances_json).unwrap_or_default();
    if instances.is_empty() {
        return empty_state_html(empty_message, None, "🖥", None);
    }
    let rows: String = instances
        .iter()
        .map(|inst| {
            let id = inst.get("id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let name = inst.get("name").and_then(|v| v.as_str()).unwrap_or(id);
            let status_val = inst.get("status").cloned().unwrap_or(Value::Null);
            let status_text = vm_status_label(&status_val);
            let badge_cls = vm_status_badge_class(&status_text);
            let cpu = inst
                .get("resources")
                .and_then(|r| r.get("cpu_cores"))
                .and_then(|v| v.as_u64())
                .map(|n| n.to_string())
                .unwrap_or_else(|| "—".into());
            let mem = inst
                .get("resources")
                .and_then(|r| r.get("memory_mb"))
                .and_then(|v| v.as_u64())
                .map(|n| n.to_string())
                .unwrap_or_else(|| "—".into());
            format!(
                r#"<tr>
                  <td>{}</td>
                  <td><span class="status-badge {}">{}</span></td>
                  <td>{} {} , {} {}MB</td>
                  <td>
                    <button type="button" class="btn" data-vm-id="{}" data-vm-action="start">{}</button>
                    <button type="button" class="btn" data-vm-id="{}" data-vm-action="stop">{}</button>
                    <button type="button" class="btn btn-danger" data-vm-id="{}" data-vm-action="delete">{}</button>
                  </td>
                </tr>"#,
                escape_html(name),
                badge_cls,
                escape_html(&status_text),
                escape_html(res_cpu_label),
                escape_html(&cpu),
                escape_html(res_mem_label),
                escape_html(&mem),
                escape_html(id),
                escape_html(start_label),
                escape_html(id),
                escape_html(stop_label),
                escape_html(id),
                escape_html(delete_label),
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
        escape_html(col_status),
        escape_html(col_resources),
        escape_html(col_actions),
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_vm_panel_html_ph_s499() {
        let html = render_vm_panel_html(
            r#"[{"id":"vm-1","name":"test-vm","status":"Running","resources":{"cpu_cores":2,"memory_mb":2048}}]"#,
            "Name",
            "Status",
            "Resources",
            "Actions",
            "VM Instances",
            "CPU:",
            "Memory:",
            "Start",
            "Stop",
            "Delete",
            "No VMs",
        );
        assert!(html.contains("test-vm"));
        assert!(html.contains("Running"));
        assert!(html.contains("data-vm-action=\"start\""));
    }

    #[test]
    fn render_vm_panel_empty_ph_s499() {
        let html = render_vm_panel_html(
            "[]",
            "Name",
            "Status",
            "Resources",
            "Actions",
            "VM Instances",
            "CPU:",
            "Memory:",
            "Start",
            "Stop",
            "Delete",
            "No VM instances found",
        );
        assert!(html.contains("No VM instances found"));
    }
}
