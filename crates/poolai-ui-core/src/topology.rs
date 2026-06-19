//! Topology graph label helpers (PH-S566) — shared with `topology_graph.rs` / wasm.

const MAX_HUB_LABEL_LEN: usize = 14;

/// Short display id for topology tables and graph labels (PH-S198 / PH-S566).
pub fn short_topology_node_id(node_id: &str) -> String {
    let id = node_id.trim();
    if id.is_empty() {
        return "—".to_string();
    }
    let base = id
        .rsplit(':')
        .next()
        .and_then(|s| s.rsplit('/').next())
        .unwrap_or(id)
        .trim();
    let base = base.strip_prefix("node-").unwrap_or(base);
    if base.len() <= MAX_HUB_LABEL_LEN {
        base.to_string()
    } else {
        format!("{}…", &base[..MAX_HUB_LABEL_LEN.saturating_sub(1)])
    }
}

/// Hub-aware SVG label: highest-degree nodes (degree ≥ 2) get a `hub·` prefix.
pub fn topology_hub_label(node_id: &str, degree: usize, max_degree: usize) -> String {
    let short = short_topology_node_id(node_id);
    if max_degree >= 2 && degree == max_degree {
        format!("hub·{short}")
    } else {
        short
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_topology_node_id_truncates_long_ids_ph_s566() {
        assert_eq!(
            short_topology_node_id("cluster/node-very-long-name-here"),
            "very-long-nam…"
        );
    }

    #[test]
    fn topology_hub_label_marks_max_degree_hub_ph_s566() {
        assert_eq!(topology_hub_label("node-a", 2, 2), "hub·a");
        assert_eq!(topology_hub_label("node-b", 1, 2), "b");
    }
}
