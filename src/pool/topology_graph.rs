//! FM-037 / PH-S157 / PH-S198 — cluster topology graph layout (force layout + heatmap HTML).
//!
//! Parity with legacy `topology_graph.js` build/layout/color; browser JS only paints SVG from JSON.
//! PH-S198: hub label text + label anchor coords computed in Rust.

use crate::pool::topology::NodeResources;
use poolai_ui_core::format::escape_html;
use serde::Serialize;
use std::collections::HashMap;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 360;
const DEFAULT_ITERATIONS: u32 = 80;
const LABEL_OFFSET_Y: f64 = 14.0;
const MAX_HUB_LABEL_LEN: usize = 14;

#[derive(Debug, Clone, Serialize)]
pub struct TopologyGraphNodeDto {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub load: f32,
    pub label_x: f64,
    pub label_y: f64,
    pub is_hub: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TopologyGraphLinkDto {
    pub from: String,
    pub to: String,
    pub latency: f64,
    pub stroke: String,
    pub stroke_width: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TopologyGraphLayout {
    pub width: u32,
    pub height: u32,
    pub nodes: Vec<TopologyGraphNodeDto>,
    pub links: Vec<TopologyGraphLinkDto>,
    pub heatmap_html: String,
    pub empty: bool,
}

#[derive(Debug, Clone)]
struct SimNode {
    id: String,
    label: String,
    load: f32,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    seed_angle: f64,
}

#[derive(Debug, Clone)]
struct SimLink {
    from: String,
    to: String,
    latency: f64,
}

/// Compute graph layout + latency heatmap HTML from topology snapshot data.
pub fn compute_topology_graph_layout(
    node_resources: &HashMap<String, NodeResources>,
    latency_matrix: &HashMap<String, f64>,
    width: Option<u32>,
    height: Option<u32>,
    iterations: Option<u32>,
) -> TopologyGraphLayout {
    let width = width.unwrap_or(DEFAULT_WIDTH).max(320);
    let height = height.unwrap_or(DEFAULT_HEIGHT).max(200);
    let iterations = iterations.unwrap_or(DEFAULT_ITERATIONS);

    let (sim_nodes, sim_links) = layout_graph(
        build_graph(node_resources, latency_matrix),
        width,
        height,
        iterations,
    );

    let heatmap_html = render_latency_heatmap_html(node_resources, latency_matrix);

    if sim_nodes.is_empty() {
        return TopologyGraphLayout {
            width,
            height,
            nodes: Vec::new(),
            links: Vec::new(),
            heatmap_html,
            empty: true,
        };
    }

    let max_lat = sim_links
        .iter()
        .map(|l| l.latency)
        .fold(0.0, f64::max)
        .max(1.0);

    let degrees = node_link_degrees(&sim_links, &sim_nodes);
    let max_degree = degrees.values().copied().max().unwrap_or(0);

    let nodes = sim_nodes
        .iter()
        .map(|n| {
            let radius = 10.0 + f64::min(14.0, n.load as f64 * 18.0);
            let degree = *degrees.get(&n.id).unwrap_or(&0);
            let is_hub = max_degree >= 2 && degree == max_degree;
            let label = topology_hub_label(&n.id, degree, max_degree);
            TopologyGraphNodeDto {
                id: n.id.clone(),
                label,
                x: n.x,
                y: n.y,
                radius,
                load: n.load,
                label_x: n.x,
                label_y: n.y + radius + LABEL_OFFSET_Y,
                is_hub,
            }
        })
        .collect();

    let links = sim_links
        .iter()
        .map(|l| {
            let stroke_width = 1.0 + f64::min(4.0, l.latency / max_lat * 4.0);
            TopologyGraphLinkDto {
                from: l.from.clone(),
                to: l.to.clone(),
                latency: l.latency,
                stroke: latency_color(l.latency, max_lat),
                stroke_width,
            }
        })
        .collect();

    TopologyGraphLayout {
        width,
        height,
        nodes,
        links,
        heatmap_html,
        empty: false,
    }
}

fn build_graph(
    node_resources: &HashMap<String, NodeResources>,
    latency_matrix: &HashMap<String, f64>,
) -> (Vec<SimNode>, Vec<SimLink>) {
    let mut node_ids: Vec<String> = node_resources.keys().cloned().collect();
    node_ids.sort();

    let count = node_ids.len();
    let nodes: Vec<SimNode> = node_ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let resources = node_resources.get(id);
            let load = resources.map(|r| r.current_load).unwrap_or(0.0);
            let angle = if count > 0 {
                (2.0 * std::f64::consts::PI * i as f64) / count as f64
            } else {
                0.0
            };
            SimNode {
                id: id.clone(),
                label: id.clone(),
                load,
                x: 0.0,
                y: 0.0,
                vx: 0.0,
                vy: 0.0,
                seed_angle: angle,
            }
        })
        .collect();

    let mut links = Vec::new();
    for (key, lat) in latency_matrix {
        let parts: Vec<&str> = key.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        let from = parts[0];
        let to = parts[1];
        if from.is_empty() || to.is_empty() || from == to {
            continue;
        }
        links.push(SimLink {
            from: from.to_string(),
            to: to.to_string(),
            latency: *lat,
        });
    }

    (nodes, links)
}

/// Short display id for topology tables and graph labels (PH-S198).
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

fn node_link_degrees(links: &[SimLink], nodes: &[SimNode]) -> HashMap<String, usize> {
    let mut degrees: HashMap<String, usize> =
        nodes.iter().map(|n| (n.id.clone(), 0_usize)).collect();
    for link in links {
        *degrees.entry(link.from.clone()).or_insert(0) += 1;
        *degrees.entry(link.to.clone()).or_insert(0) += 1;
    }
    degrees
}

fn layout_graph(
    (mut nodes, links): (Vec<SimNode>, Vec<SimLink>),
    width: u32,
    height: u32,
    iterations: u32,
) -> (Vec<SimNode>, Vec<SimLink>) {
    let w = width as f64;
    let h = height as f64;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let radius = f64::min(w, h) * 0.32;

    let node_count = nodes.len().max(1);
    for (i, n) in nodes.iter_mut().enumerate() {
        let a = if n.seed_angle != 0.0 {
            n.seed_angle
        } else {
            (2.0 * std::f64::consts::PI * i as f64) / node_count as f64
        };
        n.x = cx + radius * f64::cos(a);
        n.y = cy + radius * f64::sin(a);
        n.vx = 0.0;
        n.vy = 0.0;
    }

    let node_by_id: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.clone(), i))
        .collect();

    for _ in 0..iterations {
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let mut dx = nodes[i].x - nodes[j].x;
                let mut dy = nodes[i].y - nodes[j].y;
                let mut dist = (dx * dx + dy * dy).sqrt();
                if dist < 0.01 {
                    dist = 0.01;
                }
                let repulse = 4200.0 / (dist * dist);
                dx = (dx / dist) * repulse;
                dy = (dy / dist) * repulse;
                nodes[i].vx += dx;
                nodes[i].vy += dy;
                nodes[j].vx -= dx;
                nodes[j].vy -= dy;
            }
        }

        for link in &links {
            let ai = node_by_id.get(&link.from).copied();
            let bi = node_by_id.get(&link.to).copied();
            if ai.is_none() || bi.is_none() {
                continue;
            }
            let (ai, bi) = (ai.unwrap(), bi.unwrap());
            let mut dx = nodes[bi].x - nodes[ai].x;
            let mut dy = nodes[bi].y - nodes[ai].y;
            let mut dist = (dx * dx + dy * dy).sqrt();
            if dist < 0.01 {
                dist = 0.01;
            }
            let strength = f64::min(dist * 0.04, 12.0);
            dx = (dx / dist) * strength;
            dy = (dy / dist) * strength;
            nodes[ai].vx += dx;
            nodes[ai].vy += dy;
            nodes[bi].vx -= dx;
            nodes[bi].vy -= dy;
        }

        for n in &mut nodes {
            n.vx += (cx - n.x) * 0.002;
            n.vy += (cy - n.y) * 0.002;
            n.vx *= 0.85;
            n.vy *= 0.85;
            n.x += n.vx;
            n.y += n.vy;
            n.x = f64::max(40.0, f64::min(w - 40.0, n.x));
            n.y = f64::max(40.0, f64::min(h - 40.0, n.y));
        }
    }

    (nodes, links)
}

fn latency_color(latency: f64, max_lat: f64) -> String {
    let max = if max_lat > 0.0 { max_lat } else { 1.0 };
    let t = f64::min(1.0, f64::max(0.0, latency / max));
    let r = (80.0 + t * 175.0).round() as i32;
    let g = (200.0 - t * 140.0).round() as i32;
    let b = (120.0 - t * 80.0).round() as i32;
    format!("rgb({},{},{})", r, g, b)
}

fn render_latency_heatmap_html(
    node_resources: &HashMap<String, NodeResources>,
    latency_matrix: &HashMap<String, f64>,
) -> String {
    let mut node_ids: Vec<String> = node_resources.keys().cloned().collect();
    node_ids.sort();
    if node_ids.is_empty() {
        return String::new();
    }

    let mut values = Vec::new();
    for row in &node_ids {
        for col in &node_ids {
            if row == col {
                continue;
            }
            let key = format!("{}:{}", row, col);
            let rev = format!("{}:{}", col, row);
            let v = latency_matrix
                .get(&key)
                .or_else(|| latency_matrix.get(&rev));
            if let Some(lat) = v {
                values.push(*lat);
            }
        }
    }
    let max_lat = values.iter().copied().fold(0.0, f64::max).max(1.0);

    let mut html =
        String::from("<table class=\"admin-table topology-heatmap-table\"><thead><tr><th></th>");
    for id in &node_ids {
        html.push_str(&format!(
            "<th scope=\"col\">{}</th>",
            escape_html(&short_topology_node_id(id))
        ));
    }
    html.push_str("</tr></thead><tbody>");

    for row in &node_ids {
        html.push_str(&format!(
            "<tr><th scope=\"row\">{}</th>",
            escape_html(&short_topology_node_id(row))
        ));
        for col in &node_ids {
            if row == col {
                html.push_str("<td class=\"topo-heat-diagonal\">—</td>");
                continue;
            }
            let key = format!("{}:{}", row, col);
            let rev = format!("{}:{}", col, row);
            let raw = latency_matrix
                .get(&key)
                .or_else(|| latency_matrix.get(&rev));
            if raw.is_none() {
                html.push_str("<td class=\"topo-heat-empty\">—</td>");
                continue;
            }
            let lat = *raw.unwrap();
            let t = f64::min(1.0, f64::max(0.0, lat / max_lat));
            let bg = latency_color(lat, max_lat);
            let title = escape_html(format!("{} → {}: {:.2} ms", row, col, lat));
            html.push_str(&format!(
                "<td class=\"topo-heat-cell\" style=\"background:{}22\" title=\"{}\">{}</td>",
                bg,
                title,
                escape_html(format!("{:.1}", lat))
            ));
            let _ = t; // parity with JS heatmap alpha via bg + "22" suffix
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table>");
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nodes() -> HashMap<String, NodeResources> {
        let mut nodes = HashMap::new();
        for (id, load) in [("node-a", 0.2), ("node-b", 0.5), ("node-c", 0.1)] {
            nodes.insert(
                id.to_string(),
                NodeResources {
                    node_id: id.to_string(),
                    available_gpu_memory_mb: 8192,
                    total_gpu_memory_mb: 8192,
                    available_cpu_cores: 8,
                    total_cpu_cores: 8,
                    available_memory_mb: 16384,
                    total_memory_mb: 16384,
                    current_load: load,
                },
            );
        }
        nodes
    }

    #[test]
    fn layout_produces_nodes_and_links() {
        let nodes = sample_nodes();
        let mut latency = HashMap::new();
        latency.insert("node-a:node-b".to_string(), 12.5);
        latency.insert("node-b:node-c".to_string(), 4.0);

        let layout =
            compute_topology_graph_layout(&nodes, &latency, Some(640), Some(360), Some(40));
        assert!(!layout.empty);
        assert_eq!(layout.nodes.len(), 3);
        assert_eq!(layout.links.len(), 2);
        assert!(layout.nodes[0].radius >= 10.0);
        assert!(layout.heatmap_html.contains("topology-heatmap-table"));
    }

    #[test]
    fn empty_nodes_yields_empty_layout() {
        let layout =
            compute_topology_graph_layout(&HashMap::new(), &HashMap::new(), None, None, None);
        assert!(layout.empty);
        assert!(layout.nodes.is_empty());
    }

    #[test]
    fn latency_color_warmer_for_higher_latency() {
        let low = latency_color(1.0, 100.0);
        let high = latency_color(99.0, 100.0);
        assert_ne!(low, high);
        assert!(high.contains("rgb("));
    }

    #[test]
    fn short_topology_node_id_truncates_long_ids() {
        assert_eq!(short_topology_node_id("node-a"), "a");
        assert_eq!(
            short_topology_node_id("cluster/coordinator-hub-east-1"),
            "coordinator-h…"
        );
    }

    #[test]
    fn topology_hub_label_marks_max_degree_hub() {
        assert_eq!(topology_hub_label("node-a", 2, 2), "hub·a");
        assert_eq!(topology_hub_label("node-b", 1, 2), "b");
    }

    #[test]
    fn layout_includes_rust_hub_label_coords() {
        let nodes = sample_nodes();
        let mut latency = HashMap::new();
        latency.insert("node-a:node-b".to_string(), 12.5);
        latency.insert("node-b:node-c".to_string(), 4.0);
        latency.insert("node-a:node-c".to_string(), 8.0);

        let layout =
            compute_topology_graph_layout(&nodes, &latency, Some(640), Some(360), Some(40));
        let hub = layout.nodes.iter().find(|n| n.is_hub).expect("hub node");
        assert!(hub.label.starts_with("hub·"));
        assert_eq!(hub.label_x, hub.x);
        assert!((hub.label_y - (hub.y + hub.radius + LABEL_OFFSET_Y)).abs() < f64::EPSILON);
    }
}
