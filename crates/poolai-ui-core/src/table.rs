//! Admin table HTML builders and export helpers — parity with `admin_common.js` (PH-S42, PH-S153).

use crate::format::escape_html;
use serde::Deserialize;
use serde_json::{json, Value};

/// Escape a string for use inside a `RegExp` character class (JS `adminEscapeRegex`).
pub fn escape_regex(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '.' | '*' | '+' | '?' | '^' | '$' | '{' | '}' | '(' | ')' | '|' | '[' | ']' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

/// PH-S42 empty state markup (`adminEmptyStateHtml`).
pub fn empty_state_html(
    message: &str,
    hint: Option<&str>,
    icon: &str,
    action_html: Option<&str>,
) -> String {
    let title = escape_html(message);
    let hint_block = hint
        .filter(|h| !h.is_empty())
        .map(|h| {
            format!(
                r#"<p class="admin-empty-state-hint">{}</p>"#,
                escape_html(h)
            )
        })
        .unwrap_or_default();
    let action_block = action_html
        .filter(|a| !a.is_empty())
        .map(|a| format!(r#"<div class="admin-empty-state-action">{a}</div>"#))
        .unwrap_or_default();
    format!(
        r#"<div class="admin-empty-state" role="status"><div class="admin-empty-state-icon" aria-hidden="true">{icon}</div><p class="admin-empty-state-title">{title}</p>{hint_block}{action_block}</div>"#
    )
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum HeaderSpec {
    Str(String),
    Obj {
        label: Option<String>,
        #[serde(default, rename = "noSort")]
        no_sort: bool,
        #[serde(default)]
        actions: bool,
    },
}

#[derive(Debug, Deserialize, Default)]
struct RenderTableOptions {
    #[serde(default, rename = "emptyMessage")]
    empty_message: Option<String>,
    #[serde(default, rename = "emptyOptions")]
    empty_options: Option<EmptyOptions>,
}

#[derive(Debug, Deserialize, Default)]
struct EmptyOptions {
    hint: Option<String>,
    icon: Option<String>,
    #[serde(default, rename = "actionHtml")]
    action_html: Option<String>,
}

/// Build striped admin table HTML (`adminRenderTable`).
pub fn render_table_html(headers_json: &str, rows_json: &str, options_json: &str) -> String {
    let headers: Vec<HeaderSpec> = serde_json::from_str(headers_json).unwrap_or_default();
    let rows: Vec<Vec<Value>> = serde_json::from_str(rows_json).unwrap_or_default();
    let opts: RenderTableOptions = serde_json::from_str(options_json).unwrap_or_default();

    if rows.is_empty() {
        let msg = opts
            .empty_message
            .unwrap_or_else(|| "No data to display".to_string());
        let eo = opts.empty_options.unwrap_or_default();
        return empty_state_html(
            &msg,
            eo.hint.as_deref(),
            eo.icon.as_deref().unwrap_or("📋"),
            eo.action_html.as_deref(),
        );
    }

    let mut html = String::from(
        r#"<div class="admin-table-container"><table class="admin-table admin-table--striped"><thead><tr>"#,
    );
    for h in &headers {
        let (label, no_sort, actions) = match h {
            HeaderSpec::Str(s) => (s.as_str(), false, false),
            HeaderSpec::Obj {
                label,
                no_sort,
                actions,
            } => (label.as_deref().unwrap_or(""), *no_sort, *actions),
        };
        let no_sort_attr = if no_sort { r#" data-no-sort="1""# } else { "" };
        let cls = if actions {
            r#" class="admin-table-actions-col""#
        } else {
            ""
        };
        html.push_str(&format!(
            r#"<th scope="col"{no_sort_attr}{cls}>{}</th>"#,
            escape_html(label)
        ));
    }
    html.push_str("</tr></thead><tbody>");
    for row in &rows {
        html.push_str("<tr>");
        for cell in row {
            let cell_html = match cell {
                Value::Null => String::new(),
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            html.push_str(&format!("<td>{cell_html}</td>"));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    html
}

#[derive(Debug, Deserialize, Default)]
struct FormFieldSpec {
    id: Option<String>,
    name: Option<String>,
    label: Option<String>,
    #[serde(rename = "type", default)]
    field_type: Option<String>,
    #[serde(default)]
    required: bool,
    placeholder: Option<String>,
    #[serde(default)]
    options: Vec<SelectOption>,
}

#[derive(Debug, Deserialize)]
struct SelectOption {
    value: String,
    label: String,
}

/// One labeled form field (`adminFormFieldHtml`). `spec_json` mirrors the JS object; `id` override when empty.
pub fn form_field_html(spec_json: &str, generated_id: &str) -> String {
    let spec: FormFieldSpec = serde_json::from_str(spec_json).unwrap_or_default();
    let id = spec.id.as_deref().unwrap_or(generated_id);
    let name = escape_html(spec.name.as_deref().unwrap_or(id));
    let required_attr = if spec.required {
        " required aria-required=\"true\""
    } else {
        ""
    };
    let mut label = format!(
        r#"<label for="{id}">{}</label>"#,
        escape_html(spec.label.as_deref().unwrap_or(""))
    );
    if spec.required {
        label = format!(
            r#"<label for="{id}">{} <span class="required" aria-hidden="true">*</span></label>"#,
            escape_html(spec.label.as_deref().unwrap_or(""))
        );
    }
    let field_type = spec.field_type.as_deref().unwrap_or("text");
    let control = match field_type {
        "select" => {
            let mut opts = String::new();
            for o in &spec.options {
                opts.push_str(&format!(
                    r#"<option value="{}">{}</option>"#,
                    escape_html(&o.value),
                    escape_html(&o.label)
                ));
            }
            format!(r#"<select id="{id}" name="{name}"{required_attr}>{opts}</select>"#)
        }
        "textarea" => format!(r#"<textarea id="{id}" name="{name}"{required_attr}></textarea>"#),
        _ => {
            let mut input = format!(
                r#"<input type="{}" id="{id}" name="{name}"{required_attr}"#,
                escape_html(field_type)
            );
            if let Some(ph) = &spec.placeholder {
                input.push_str(&format!(r#" placeholder="{}""#, escape_html(ph)));
            }
            input.push_str(" />");
            input
        }
    };
    format!(r#"<div class="form-group">{label}{control}</div>"#)
}

/// CSV cell escape (`adminExportTableCsv` row cells).
pub fn csv_escape_cell(text: &str) -> String {
    if text.contains(['"', ',', '\n']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

/// Build CSV document from headers + row matrix.
pub fn build_csv(headers: &[String], rows: &[Vec<String>]) -> String {
    let header_line = headers
        .iter()
        .map(|h| csv_escape_cell(h.trim()))
        .collect::<Vec<_>>()
        .join(",");
    let mut lines = vec![header_line];
    for row in rows {
        lines.push(
            row.iter()
                .map(|c| csv_escape_cell(c.trim()))
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    lines.join("\n")
}

/// Build JSON export array (`adminExportTableJson` data portion).
pub fn build_json_export(headers: &[String], rows: &[Vec<String>]) -> String {
    let mut objects = Vec::with_capacity(rows.len());
    for row in rows {
        let mut obj = serde_json::Map::new();
        for (i, h) in headers.iter().enumerate() {
            let val = row.get(i).map(|s| s.trim()).unwrap_or("");
            obj.insert(h.clone(), Value::String(val.to_string()));
        }
        objects.push(Value::Object(obj));
    }
    serde_json::to_string_pretty(&objects).unwrap_or_else(|_| "[]".to_string())
}

/// Sort comparator return (`adminSortTable`): -1, 0, 1.
pub fn compare_sort_values(a: &str, b: &str, numeric: bool, ascending: bool) -> i32 {
    let (a_val, b_val) = if numeric {
        (
            a.trim().parse::<f64>().unwrap_or(f64::NAN),
            b.trim().parse::<f64>().unwrap_or(f64::NAN),
        )
    } else {
        (f64::NAN, f64::NAN)
    };
    let ordering = if numeric {
        a_val
            .partial_cmp(&b_val)
            .unwrap_or(std::cmp::Ordering::Equal)
    } else {
        a.trim().to_lowercase().cmp(&b.trim().to_lowercase())
    };
    match ordering {
        std::cmp::Ordering::Less => {
            if ascending {
                -1
            } else {
                1
            }
        }
        std::cmp::Ordering::Greater => {
            if ascending {
                1
            } else {
                -1
            }
        }
        std::cmp::Ordering::Equal => 0,
    }
}

/// Row filter match (`adminFilterTable`).
pub fn row_matches_query(row_text: &str, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return true;
    }
    row_text.to_lowercase().contains(&q)
}

/// Wrap query matches in `<mark>` (`adminFilterTable` highlight).
pub fn highlight_query_html(original: &str, query: &str) -> String {
    let q = query.trim();
    if q.is_empty() {
        return original.to_string();
    }
    let lower_orig = original.to_lowercase();
    let lower_q = q.to_lowercase();
    let mut out = String::with_capacity(original.len() + 32);
    let mut i = 0;
    while let Some(rel) = lower_orig[i..].find(&lower_q) {
        let start = i + rel;
        let end = start + q.len();
        out.push_str(&original[i..start]);
        out.push_str(r#"<mark class="admin-table-highlight">"#);
        out.push_str(&original[start..end]);
        out.push_str("</mark>");
        i = end;
    }
    out.push_str(&original[i..]);
    out
}

/// Export filename from table aria-label (PH-S575).
pub fn export_filename_from_aria(aria_label: &str, extension: &str) -> String {
    let slug: String = aria_label
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-");
    let base = if slug.is_empty() {
        "poolai-table".to_string()
    } else {
        slug
    };
    format!("{base}.{extension}")
}

/// Export CSV/JSON button markup for admin table toolbar (PH-S575).
pub fn table_export_buttons_html(
    export_csv_label: &str,
    export_json_label: &str,
    csv_aria: &str,
    json_aria: &str,
) -> String {
    format!(
        r#"<button type="button" class="btn btn-secondary btn-sm" data-poolai-export="csv" aria-label="{}">{}</button><button type="button" class="btn btn-secondary btn-sm" data-poolai-export="json" aria-label="{}">{}</button>"#,
        escape_html(csv_aria),
        escape_html(export_csv_label),
        escape_html(json_aria),
        escape_html(export_json_label),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_regex_metachars() {
        assert_eq!(escape_regex("a.b*"), r"a\.b\*");
    }

    #[test]
    fn empty_state_html_escapes_title() {
        let html = empty_state_html("No <jobs>", Some("hint"), "📋", None);
        assert!(html.contains("No &lt;jobs&gt;"));
        assert!(html.contains("admin-empty-state-hint"));
    }

    #[test]
    fn render_table_html_basic() {
        let html = render_table_html(r#"["ID","Status"]"#, r#"[["1","ok"]]"#, "{}");
        assert!(html.contains("admin-table--striped"));
        assert!(html.contains("<td>1</td>"));
    }

    #[test]
    fn render_table_empty_uses_empty_state() {
        let html = render_table_html("[]", "[]", r#"{"emptyMessage":"Empty"}"#);
        assert!(html.contains("admin-empty-state"));
        assert!(html.contains("Empty"));
    }

    #[test]
    fn build_csv_quotes_commas() {
        let csv = build_csv(&["a".into()], &[vec!["hello, world".into()]]);
        assert_eq!(csv, "a\n\"hello, world\"");
    }

    #[test]
    fn compare_sort_numeric_and_text() {
        assert_eq!(compare_sort_values("2", "10", true, true), -1);
        assert_eq!(compare_sort_values("b", "a", false, true), 1);
    }

    #[test]
    fn highlight_query_wraps_match() {
        let html = highlight_query_html("Hello World", "world");
        assert!(html.contains("<mark class=\"admin-table-highlight\">World</mark>"));
    }

    #[test]
    fn row_matches_query_case_insensitive() {
        assert!(row_matches_query("Hello", "hel"));
        assert!(!row_matches_query("Hello", "xyz"));
    }

    #[test]
    fn export_filename_from_aria_ph_s575() {
        assert_eq!(
            export_filename_from_aria("Jobs Table", "csv"),
            "jobs-table.csv"
        );
    }
}
