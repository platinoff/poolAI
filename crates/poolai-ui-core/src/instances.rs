//! Admin instances table HTML (PH-S490 wasm glue).

use serde_json::Value;

/// Instances table HTML for admin panel (PH-S490).
#[allow(clippy::too_many_arguments)]
pub fn render_instances_panel_html(
    instances_json: &str,
    col_instance_id: &str,
    col_model_id: &str,
    col_status: &str,
    col_strategy: &str,
    col_nodes: &str,
    col_created: &str,
    col_actions: &str,
    table_aria: &str,
    view_label: &str,
    delete_label: &str,
    empty_message: &str,
) -> String {
    use crate::format::escape_html;
    use crate::table::empty_state_html;

    let instances: Vec<Value> = serde_json::from_str(instances_json).unwrap_or_default();
    if instances.is_empty() {
        return empty_state_html(empty_message, None, "🧠", None);
    }
    let rows: String = instances
        .iter()
        .map(|inst| {
            let iid = inst
                .get("instance_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let model_id = inst
                .get("model_id")
                .and_then(|v| v.as_str())
                .unwrap_or("—");
            let status = inst
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let strategy = inst
                .get("placement")
                .and_then(|p| p.get("strategy"))
                .and_then(|v| v.as_str())
                .unwrap_or("—");
            let nodes = inst
                .get("placement")
                .and_then(|p| p.get("node_ids"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|n| n.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            let created = inst
                .get("created_at")
                .and_then(|v| v.as_str())
                .unwrap_or("—");
            format!(
                r#"<tr>
                  <td>{}</td>
                  <td>{}</td>
                  <td><span class="badge">{}</span></td>
                  <td>{}</td>
                  <td>{}</td>
                  <td>{}</td>
                  <td>
                    <button type="button" class="btn btn-sm" data-instance-view="{}">{}</button>
                    <button type="button" class="btn btn-sm btn-danger" data-instance-delete="{}">{}</button>
                  </td>
                </tr>"#,
                escape_html(iid),
                escape_html(model_id),
                escape_html(status),
                escape_html(strategy),
                escape_html(&nodes),
                escape_html(created),
                escape_html(iid),
                escape_html(view_label),
                escape_html(iid),
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
                <th>{}</th>
                <th>{}</th>
                <th>{}</th>
              </tr>
            </thead>
            <tbody>{}</tbody>
          </table></div>"#,
        escape_html(table_aria),
        escape_html(col_instance_id),
        escape_html(col_model_id),
        escape_html(col_status),
        escape_html(col_strategy),
        escape_html(col_nodes),
        escape_html(col_created),
        escape_html(col_actions),
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_instances_panel_html_ph_s490() {
        let html = render_instances_panel_html(
            r#"[{"instance_id":"i1","model_id":"m1","status":"running","placement":{"strategy":"round_robin","node_ids":["n1"]},"created_at":"2026-06-18T00:00:00Z"}]"#,
            "Instance ID",
            "Model ID",
            "Status",
            "Strategy",
            "Nodes",
            "Created",
            "Actions",
            "Instances",
            "View",
            "Delete",
            "No instances",
        );
        assert!(html.contains("i1"));
        assert!(html.contains("round_robin"));
        assert!(html.contains("data-instance-view"));
    }

    #[test]
    fn render_instances_panel_empty_ph_s490() {
        let html = render_instances_panel_html(
            "[]",
            "Instance ID",
            "Model ID",
            "Status",
            "Strategy",
            "Nodes",
            "Created",
            "Actions",
            "Instances",
            "View",
            "Delete",
            "No instances found",
        );
        assert!(html.contains("No instances found"));
    }
}
