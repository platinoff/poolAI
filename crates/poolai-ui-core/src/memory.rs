//! Admin memory / seed inventory panel helpers (PH-S862).

use crate::format::escape_html;

/// Seed inventory meta strip HTML (memory persist + depth labels).
pub fn render_memory_seed_meta_strip_html(
    memory_persist: bool,
    registered_shard_count: u32,
    memory_store_depth: &str,
    memory_layer_depth: &str,
    persist_label: &str,
    shards_label: &str,
) -> String {
    let persist_display = if memory_persist { "JSON" } else { "ephemeral" };
    format!(
        r#"<div class="seed-inventory-meta" data-memory-persist="{persist}" data-store-depth="{store_depth}" data-layer-depth="{layer_depth}"><span class="status-badge {badge_class}" title="{store_depth}">{persist_label} {persist_display}</span> <span class="muted">{shards_label} {count} · {store_depth} · {layer_depth}</span></div>"#,
        persist = if memory_persist { "true" } else { "false" },
        store_depth = escape_html(memory_store_depth),
        layer_depth = escape_html(memory_layer_depth),
        badge_class = if memory_persist { "active" } else { "muted" },
        persist_label = escape_html(persist_label),
        persist_display = escape_html(persist_display),
        shards_label = escape_html(shards_label),
        count = registered_shard_count,
    )
}

/// Format hot-tier RAM bytes for seed inventory table cells (PH-S862).
pub fn format_seed_inventory_ram_bytes(ram_bytes: Option<u64>) -> String {
    match ram_bytes {
        Some(v) => v.to_string(),
        None => "—".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_memory_seed_meta_strip_html_ph_s862() {
        let html = render_memory_seed_meta_strip_html(
            true,
            2,
            "json_restart",
            "full_depth",
            "Memory:",
            "Registered:",
        );
        assert!(html.contains("seed-inventory-meta"));
        assert!(html.contains("data-memory-persist=\"true\""));
        assert!(html.contains("json_restart"));
        assert!(html.contains("full_depth"));
        assert!(html.contains("Registered: 2"));
    }

    #[test]
    fn format_seed_inventory_ram_bytes_ph_s862() {
        assert_eq!(format_seed_inventory_ram_bytes(Some(1024)), "1024");
        assert_eq!(format_seed_inventory_ram_bytes(None), "—");
    }
}
