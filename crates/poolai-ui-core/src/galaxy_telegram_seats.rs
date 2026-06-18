//! Telegram seats coordinator snapshot table (PH-S517).

use serde_json::Value;

/// Read-only Telegram seats panel from `GET /api/v1/grid/telegram-seats` JSON.
pub fn render_telegram_seats_panel_html(
    snapshot_json: &str,
    col_policy: &str,
    col_limit: &str,
    col_active: &str,
    col_bound: &str,
    table_aria: &str,
) -> String {
    use crate::format::escape_html;

    let snap: Value = serde_json::from_str(snapshot_json).unwrap_or(Value::Null);
    let policy = snap
        .get("seat_policy")
        .and_then(|v| v.as_str())
        .unwrap_or("—");
    let limit = snap
        .get("seat_limit")
        .map(|v| {
            if v.is_null() {
                "∞".to_string()
            } else {
                v.to_string()
            }
        })
        .unwrap_or_else(|| "—".into());
    let active = snap
        .get("active_telegram_edge_workers")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "0".into());
    let bound = snap
        .get("bound_wallets_count")
        .map(|v| v.to_string())
        .unwrap_or_else(|| "0".into());
    format!(
        r#"<div class="admin-table-container"><table class="admin-table" aria-label="{}">
            <thead><tr>
              <th>{}</th><th>{}</th><th>{}</th><th>{}</th>
            </tr></thead>
            <tbody><tr>
              <td><span class="status-badge info">{}</span></td>
              <td>{}</td>
              <td>{}</td>
              <td>{}</td>
            </tr></tbody>
          </table></div>"#,
        escape_html(table_aria),
        escape_html(col_policy),
        escape_html(col_limit),
        escape_html(col_active),
        escape_html(col_bound),
        escape_html(policy),
        escape_html(&limit),
        escape_html(&active),
        escape_html(&bound),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_telegram_seats_panel_ph_s517() {
        let json = r#"{"seat_policy":"flat","seat_limit":5,"active_telegram_edge_workers":2,"bound_wallets_count":3}"#;
        let html = render_telegram_seats_panel_html(
            json,
            "Policy",
            "Limit",
            "Active",
            "Wallets",
            "Telegram seats",
        );
        assert!(html.contains("flat"));
        assert!(html.contains("5"));
        assert!(html.contains("2"));
    }
}
