/// Layout module — Phase 3 of the Sugiyama pipeline.
///
/// This module implements the 4 phases of Sugiyama-style layout:
///   1. Cycle removal  (this file — greedy-FAS approach)
///   2. Layer assignment (rank each node)
///   3. Crossing minimization (barycenter heuristic)
///   4. Coordinate assignment (x/y positions)
///
/// The public entry point is `Layout::from_graph_ir()`.

use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

use crate::graph::{EdgeData, GraphIR, NodeData};

// ─── Cycle Removal (Greedy-FAS) ───────────────────────────────────────────────

/// Result of cycle removal: a set of edge indices that were reversed to make
/// the graph a DAG. These are "back-edges" in the original graph.
///
/// The caller can use this set to flip arrow directions in the rendering phase
/// (so the displayed arrow still points the "right" way visually).
pub struct CycleRemovalResult {
    /// Edges that were reversed. Their direction in the layout graph is flipped.
    pub reversed_edges: HashSet<EdgeIndex>,
}

/// Remove cycles from a copy of the DiGraph using the greedy-FAS heuristic.
///
/// The greedy-FAS (Feedback Arc Set) algorithm works as follows:
///   - Maintain a sequence `s1` (sinks) and `s2` (sources) being built left/right.
///   - Repeatedly:
///     1. Move all sinks to the front of s2.
///     2. Move all sources to the back of s1.
///     3. Of the remaining nodes, pick the one with max (out-degree - in-degree)
///        and add it to s1.
///   - The final ordering is: s1 + s2 (reversed).
///   - Any edge going against this ordering is a back-edge and is reversed.
///
/// Returns the modified graph and the set of reversed edge indices (relative to
/// the input graph's edge indices).
///
/// Reference: Eades, Lin, Smyth (1993) — "A fast and effective heuristic for
/// the feedback arc set problem", Information Processing Letters.
pub fn remove_cycles(
    graph: &DiGraph<NodeData, EdgeData>,
) -> (DiGraph<NodeData, EdgeData>, HashSet<EdgeIndex>) {
    let node_count = graph.node_count();
    if node_count == 0 {
        return (graph.clone(), HashSet::new());
    }

    // Build node ordering via greedy-FAS.
    let ordering = greedy_fas_ordering(graph);

    // Build position map: NodeIndex → position in the ordering.
    let mut position: HashMap<NodeIndex, usize> = HashMap::new();
    for (pos, &node) in ordering.iter().enumerate() {
        position.insert(node, pos);
    }

    // Identify back-edges: edges where source comes AFTER target in the ordering,
    // or self-loops (source == target). These must be reversed to break cycles.
    let mut reversed_edges: HashSet<EdgeIndex> = HashSet::new();
    for edge in graph.edge_references() {
        let is_self_loop = edge.source() == edge.target();
        let src_pos = position[&edge.source()];
        let tgt_pos = position[&edge.target()];
        if is_self_loop || src_pos > tgt_pos {
            reversed_edges.insert(edge.id());
        }
    }

    // Build the modified graph with back-edges reversed.
    let mut new_graph: DiGraph<NodeData, EdgeData> = DiGraph::new();

    // Add all nodes preserving NodeIndex mapping (petgraph assigns 0..N in order).
    let mut sorted_nodes: Vec<NodeIndex> = graph.node_indices().collect();
    sorted_nodes.sort();
    for &ni in &sorted_nodes {
        new_graph.add_node(graph[ni].clone());
    }

    // Add edges, reversing back-edges. Skip self-loops (they can't be forward
    // edges and reversing them still gives a self-loop — just remove them).
    for edge in graph.edge_references() {
        if edge.source() == edge.target() {
            // Self-loop is in reversed_edges; omit from the DAG entirely.
            continue;
        }
        let (src, tgt) = if reversed_edges.contains(&edge.id()) {
            (edge.target(), edge.source())
        } else {
            (edge.source(), edge.target())
        };
        new_graph.add_edge(src, tgt, edge.weight().clone());
    }

    (new_graph, reversed_edges)
}

/// Compute a node ordering using the greedy-FAS heuristic.
///
/// Returns a Vec<NodeIndex> ordering that minimizes back-edges.
/// Nodes earlier in the ordering should have outgoing edges going forward.
fn greedy_fas_ordering(graph: &DiGraph<NodeData, EdgeData>) -> Vec<NodeIndex> {
    let mut active: HashSet<NodeIndex> = graph.node_indices().collect();

    // Dynamic degree counters updated as nodes are removed.
    let mut out_deg: HashMap<NodeIndex, i64> = HashMap::new();
    let mut in_deg: HashMap<NodeIndex, i64> = HashMap::new();

    for ni in graph.node_indices() {
        out_deg.insert(
            ni,
            graph
                .neighbors_directed(ni, petgraph::Direction::Outgoing)
                .count() as i64,
        );
        in_deg.insert(
            ni,
            graph
                .neighbors_directed(ni, petgraph::Direction::Incoming)
                .count() as i64,
        );
    }

    // s1: nodes placed at the "left" (sources, high out-degree surplus)
    // s2: nodes placed at the "right" (sinks)
    let mut s1: Vec<NodeIndex> = Vec::new();
    let mut s2: Vec<NodeIndex> = Vec::new();

    while !active.is_empty() {
        // Step 1: Pull all sinks (out_deg == 0) into s2.
        loop {
            let sinks: Vec<NodeIndex> = active
                .iter()
                .copied()
                .filter(|&n| out_deg[&n] == 0)
                .collect();
            if sinks.is_empty() {
                break;
            }
            for sink in sinks {
                active.remove(&sink);
                s2.push(sink);
                for pred in graph.neighbors_directed(sink, petgraph::Direction::Incoming) {
                    if active.contains(&pred) {
                        *out_deg.get_mut(&pred).unwrap() -= 1;
                    }
                }
            }
        }

        // Step 2: Pull all sources (in_deg == 0) into s1.
        loop {
            let sources: Vec<NodeIndex> = active
                .iter()
                .copied()
                .filter(|&n| in_deg[&n] == 0)
                .collect();
            if sources.is_empty() {
                break;
            }
            for source in sources {
                active.remove(&source);
                s1.push(source);
                for succ in graph.neighbors_directed(source, petgraph::Direction::Outgoing) {
                    if active.contains(&succ) {
                        *in_deg.get_mut(&succ).unwrap() -= 1;
                    }
                }
            }
        }

        // Step 3: If nodes remain in cycles, pick max (out - in) node.
        if let Some(&best) = active.iter().max_by_key(|&&n| out_deg[&n] - in_deg[&n]) {
            active.remove(&best);
            s1.push(best);
            for succ in graph.neighbors_directed(best, petgraph::Direction::Outgoing) {
                if active.contains(&succ) {
                    *in_deg.get_mut(&succ).unwrap() -= 1;
                }
            }
            for pred in graph.neighbors_directed(best, petgraph::Direction::Incoming) {
                if active.contains(&pred) {
                    *out_deg.get_mut(&pred).unwrap() -= 1;
                }
            }
        }
    }

    // Final ordering: s1 + reversed(s2)
    s2.reverse();
    s1.extend(s2);
    s1
}

// ─── Layout IR ────────────────────────────────────────────────────────────────

/// A positioned node in the layout.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    /// The original node id (from GraphIR).
    pub id: String,
    /// Layer (rank) — 0 is top/left depending on direction.
    pub layer: usize,
    /// Position within the layer (0-indexed).
    pub order: usize,
    /// Final x coordinate (character column).
    pub x: usize,
    /// Final y coordinate (character row).
    pub y: usize,
    /// Width in characters (includes borders).
    pub width: usize,
    /// Height in characters (includes borders).
    pub height: usize,
}

/// The result of the full layout algorithm for a graph.
pub struct Layout {
    /// Positioned nodes.
    pub nodes: Vec<LayoutNode>,
    /// Edge indices that were reversed during cycle removal.
    pub reversed_edges: HashSet<EdgeIndex>,
}

impl Layout {
    /// Run cycle removal on the GraphIR and return a partial Layout.
    /// Coordinates are not yet assigned (that is done in later phases).
    pub fn from_graph_ir(gir: &GraphIR) -> Self {
        let (_, reversed_edges) = remove_cycles(&gir.digraph);

        // Placeholder nodes without coordinates — future phases fill these in.
        let nodes: Vec<LayoutNode> = gir
            .digraph
            .node_indices()
            .map(|ni| {
                let data = &gir.digraph[ni];
                let label_len = data.label.len();
                LayoutNode {
                    id: data.id.clone(),
                    layer: 0,
                    order: 0,
                    x: 0,
                    y: 0,
                    width: label_len + 4, // "[ " + label + " ]"
                    height: 3,            // top border + text + bottom border
                }
            })
            .collect();

        Layout {
            nodes,
            reversed_edges,
        }
    }
}

// ─── Dummy Node Insertion ──────────────────────────────────────────────────────

/// The result of dummy node insertion for a single long edge.
///
/// When an edge spans more than one layer (i.e., `layer[tgt] - layer[src] > 1`),
/// it is replaced by a chain of dummy nodes — one per intermediate layer.
/// Each dummy node gets a synthetic id and is marked with `is_dummy = true`
/// in the augmented graph.
pub struct DummyEdge {
    /// Id of the original source node.
    pub original_src: String,
    /// Id of the original target node.
    pub original_tgt: String,
    /// Ids of the dummy nodes inserted along the path, in order from src to tgt.
    pub dummy_ids: Vec<String>,
    /// Original edge data (edge type, label, attrs) preserved for rendering.
    pub edge_data: EdgeData,
}

/// A graph augmented with dummy nodes for edges that span multiple layers.
///
/// After dummy node insertion, every edge in the augmented graph connects
/// nodes in adjacent layers (layer difference == 1). This is a pre-condition
/// for the crossing-minimisation and coordinate-assignment phases.
pub struct AugmentedGraph {
    /// The augmented DiGraph. Contains original nodes plus dummy nodes.
    /// Dummy nodes have ids starting with `"__dummy_"`.
    pub graph: DiGraph<NodeData, EdgeData>,
    /// Layer assignment for every node (original + dummy).
    pub layers: HashMap<String, usize>,
    /// Total number of layers.
    pub layer_count: usize,
    /// Information about each long edge that was broken up.
    pub dummy_edges: Vec<DummyEdge>,
}

/// Prefix used for dummy node ids. The rendering phase can detect dummy nodes
/// by checking `id.starts_with(DUMMY_PREFIX)`.
pub const DUMMY_PREFIX: &str = "__dummy_";

/// Insert dummy nodes into the cycle-free, layer-assigned graph.
///
/// For each edge (u → v) where `layer[v] - layer[u] > 1`, the edge is removed
/// and replaced by the chain:
///   u → d₁ → d₂ → … → dₖ → v
/// where each dᵢ lives in layer `layer[u] + i`.
///
/// # Arguments
/// * `dag`   — The cycle-free DiGraph produced by `remove_cycles`.
/// * `la`    — The layer assignment produced by `LayerAssignment::assign`.
///
/// # Returns
/// An `AugmentedGraph` where every edge connects adjacent-layer nodes.
pub fn insert_dummy_nodes(dag: &DiGraph<NodeData, EdgeData>, la: &LayerAssignment) -> AugmentedGraph {
    use crate::ast::NodeShape;

    // Clone the DAG — we will mutate it by removing long edges and adding dummies.
    let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();

    // Rebuild the graph from scratch so we control NodeIndex ordering.
    // Map original NodeIndex → new NodeIndex.
    let mut old_to_new: HashMap<NodeIndex, NodeIndex> = HashMap::new();

    // Add all original nodes.
    let mut sorted_nodes: Vec<NodeIndex> = dag.node_indices().collect();
    sorted_nodes.sort();
    for &ni in &sorted_nodes {
        let new_ni = g.add_node(dag[ni].clone());
        old_to_new.insert(ni, new_ni);
    }

    // Layer map: node_id → layer (starts with original nodes, extended with dummies).
    let mut layers: HashMap<String, usize> = la.layers.clone();

    let mut dummy_edges: Vec<DummyEdge> = Vec::new();
    // Counter to generate unique dummy ids per long edge.
    let mut edge_counter: usize = 0;

    // Collect all edges upfront so we can iterate without borrowing `g` mutably.
    let all_edges: Vec<(NodeIndex, NodeIndex, EdgeData)> = dag
        .edge_references()
        .map(|e| (e.source(), e.target(), e.weight().clone()))
        .collect();

    for (src_old, tgt_old, edge_data) in all_edges {
        let src_new = old_to_new[&src_old];
        let tgt_new = old_to_new[&tgt_old];

        let src_id = g[src_new].id.clone();
        let tgt_id = g[tgt_new].id.clone();

        let src_layer = layers[&src_id];
        let tgt_layer = layers[&tgt_id];

        // Edges within the same layer or adjacent layers need no dummies.
        // (Same-layer edges are unusual but can arise from bidirectional edges
        //  after cycle removal; we keep them as-is.)
        let layer_diff = if tgt_layer > src_layer {
            tgt_layer - src_layer
        } else {
            // Edge goes "upward" (reversed back-edge in display); treat as span 1.
            1
        };

        if layer_diff <= 1 {
            // Adjacent-layer edge — copy as-is.
            g.add_edge(src_new, tgt_new, edge_data);
            continue;
        }

        // Long edge: replace with a chain of dummy nodes.
        // Each dummy id is "__dummy_{edge_counter}_{i}" for uniqueness.
        let steps = layer_diff - 1; // number of intermediate layers
        let this_edge = edge_counter;
        edge_counter += 1;

        let mut dummy_ids: Vec<String> = Vec::with_capacity(steps);
        let mut chain_prev = src_new;

        for i in 0..steps {
            let dummy_layer = src_layer + i + 1;
            let dummy_id = format!("{}{}_{}", DUMMY_PREFIX, this_edge, i);

            let dummy_data = NodeData {
                id: dummy_id.clone(),
                label: String::new(),
                shape: NodeShape::Rectangle,
                attrs: Vec::new(),
                subgraph: None,
            };
            let dummy_ni = g.add_node(dummy_data);
            layers.insert(dummy_id.clone(), dummy_layer);
            dummy_ids.push(dummy_id);

            // Edge from previous node to this dummy.
            let segment_edge = EdgeData {
                edge_type: edge_data.edge_type.clone(),
                label: None, // label only on the last segment (see below)
                attrs: Vec::new(),
            };
            g.add_edge(chain_prev, dummy_ni, segment_edge);
            chain_prev = dummy_ni;
        }

        // Final segment: dummy → original target, carry the label.
        let last_segment = EdgeData {
            edge_type: edge_data.edge_type.clone(),
            label: edge_data.label.clone(),
            attrs: edge_data.attrs.clone(),
        };
        g.add_edge(chain_prev, tgt_new, last_segment);

        dummy_edges.push(DummyEdge {
            original_src: src_id,
            original_tgt: tgt_id,
            dummy_ids,
            edge_data,
        });
    }

    let layer_count = layers.values().copied().max().unwrap_or(0) + 1;

    AugmentedGraph {
        graph: g,
        layers,
        layer_count,
        dummy_edges,
    }
}

// ─── Layer Assignment ─────────────────────────────────────────────────────────

/// Result of layer assignment: each node is assigned a layer (rank).
///
/// Layer 0 is the "first" layer (top for TD, left for LR).
/// This is computed on a cycle-free copy of the graph produced by `remove_cycles`.
pub struct LayerAssignment {
    /// Maps node id → layer index.
    pub layers: HashMap<String, usize>,
    /// Total number of layers.
    pub layer_count: usize,
    /// Edges that were reversed during cycle removal (for display).
    pub reversed_edges: HashSet<EdgeIndex>,
}

impl LayerAssignment {
    /// Assign layers to all nodes using fixed-point iteration.
    ///
    /// Algorithm: for each edge u→v, rank[v] = max(rank[v], rank[u] + 1).
    /// Repeat until stable. Runs in O(V * E) worst case, fast in practice.
    pub fn assign(gir: &GraphIR) -> Self {
        let (dag, reversed_edges) = remove_cycles(&gir.digraph);

        // Initialize all layers to 0.
        let mut layers: HashMap<String, usize> = gir
            .digraph
            .node_indices()
            .map(|ni| (gir.digraph[ni].id.clone(), 0usize))
            .collect();

        // Fixed-point iteration: propagate ranks along DAG edges.
        let mut changed = true;
        while changed {
            changed = false;
            for edge in dag.edge_references() {
                let src_id = &dag[edge.source()].id;
                let tgt_id = &dag[edge.target()].id;
                let src_rank = layers[src_id];
                let tgt_rank = layers[tgt_id];
                if tgt_rank < src_rank + 1 {
                    *layers.get_mut(tgt_id).unwrap() = src_rank + 1;
                    changed = true;
                }
            }
        }

        let layer_count = layers.values().copied().max().unwrap_or(0) + 1;

        LayerAssignment {
            layers,
            layer_count,
            reversed_edges,
        }
    }

    /// Format a human-readable report for the out/phase3/ files.
    pub fn format_report(&self, gir: &GraphIR) -> String {
        let mut out = String::new();

        out.push_str(&format!(
            "Reversed edges (cycle removal): {}\n",
            self.reversed_edges.len()
        ));
        out.push_str(&format!("Layer count: {}\n\n", self.layer_count));

        // Group nodes by layer.
        let mut by_layer: Vec<Vec<&str>> = vec![vec![]; self.layer_count];
        let mut node_ids: Vec<&str> = self.layers.keys().map(|s| s.as_str()).collect();
        node_ids.sort();
        for id in &node_ids {
            let layer = self.layers[*id];
            by_layer[layer].push(id);
        }

        for (i, nodes) in by_layer.iter().enumerate() {
            let mut sorted = nodes.clone();
            sorted.sort();
            out.push_str(&format!("  Layer {}: {}\n", i, sorted.join(", ")));
        }

        // Show which edges were reversed (by node id pairs).
        if !self.reversed_edges.is_empty() {
            out.push_str("\nReversed back-edges:\n");
            for &eidx in &self.reversed_edges {
                if let Some(edge) = gir.digraph.edge_endpoints(eidx) {
                    let src = &gir.digraph[edge.0].id;
                    let tgt = &gir.digraph[edge.1].id;
                    out.push_str(&format!("  {} ← {} (displayed reversed)\n", src, tgt));
                }
            }
        }

        out
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{self, NodeShape};

    fn make_node(id: &str) -> NodeData {
        NodeData {
            id: id.to_string(),
            label: id.to_string(),
            shape: NodeShape::Rectangle,
            attrs: vec![],
            subgraph: None,
        }
    }

    fn make_edge() -> EdgeData {
        EdgeData {
            edge_type: ast::EdgeType::Arrow,
            label: None,
            attrs: vec![],
        }
    }

    #[test]
    fn test_dag_has_no_reversed_edges() {
        // A → B → C  (simple DAG, no cycles)
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let b = g.add_node(make_node("B"));
        let c = g.add_node(make_node("C"));
        g.add_edge(a, b, make_edge());
        g.add_edge(b, c, make_edge());

        let (_, reversed) = remove_cycles(&g);
        assert!(
            reversed.is_empty(),
            "DAG should have no reversed edges, got: {:?}",
            reversed
        );
    }

    #[test]
    fn test_single_cycle_reversed() {
        // A → B → A  (2-cycle)
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let b = g.add_node(make_node("B"));
        g.add_edge(a, b, make_edge());
        g.add_edge(b, a, make_edge());

        let (result, reversed) = remove_cycles(&g);
        assert_eq!(reversed.len(), 1, "Should reverse exactly one edge");
        assert!(
            !petgraph::algo::is_cyclic_directed(&result),
            "Result should be a DAG"
        );
    }

    #[test]
    fn test_self_loop_reversed() {
        // A → A  (self-loop)
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        g.add_edge(a, a, make_edge());

        let (result, reversed) = remove_cycles(&g);
        assert_eq!(reversed.len(), 1, "Self-loop should be counted as reversed");
        assert!(
            !petgraph::algo::is_cyclic_directed(&result),
            "Result should be a DAG"
        );
    }

    #[test]
    fn test_complex_cycle() {
        // A → B → C → A  (3-cycle) plus D → B
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let b = g.add_node(make_node("B"));
        let c = g.add_node(make_node("C"));
        let d = g.add_node(make_node("D"));
        g.add_edge(a, b, make_edge());
        g.add_edge(b, c, make_edge());
        g.add_edge(c, a, make_edge()); // back-edge
        g.add_edge(d, b, make_edge());

        let (result, _) = remove_cycles(&g);
        assert!(
            !petgraph::algo::is_cyclic_directed(&result),
            "Result should be a DAG"
        );
    }

    #[test]
    fn test_empty_graph() {
        let g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let (result, reversed) = remove_cycles(&g);
        assert_eq!(result.node_count(), 0);
        assert!(reversed.is_empty());
    }

    // ─── insert_dummy_nodes tests ──────────────────────────────────────────

    /// Build a minimal LayerAssignment from a map of id→layer.
    fn make_layer_assignment(layers: HashMap<String, usize>) -> LayerAssignment {
        let layer_count = layers.values().copied().max().unwrap_or(0) + 1;
        LayerAssignment {
            layers,
            layer_count,
            reversed_edges: HashSet::new(),
        }
    }

    #[test]
    fn test_adjacent_edge_no_dummy() {
        // A(0) → B(1): span 1 — no dummy needed
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let b = g.add_node(make_node("B"));
        g.add_edge(a, b, make_edge());

        let mut layers = HashMap::new();
        layers.insert("A".to_string(), 0);
        layers.insert("B".to_string(), 1);
        let la = make_layer_assignment(layers);

        let aug = insert_dummy_nodes(&g, &la);

        // Should have exactly 2 nodes (A and B) and 1 edge.
        assert_eq!(aug.graph.node_count(), 2, "No dummy nodes should be added");
        assert_eq!(aug.graph.edge_count(), 1);
        assert!(aug.dummy_edges.is_empty());
    }

    #[test]
    fn test_long_edge_one_dummy() {
        // A(0) → C(2): span 2 — one dummy in layer 1
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let c = g.add_node(make_node("C"));
        g.add_edge(a, c, make_edge());

        let mut layers = HashMap::new();
        layers.insert("A".to_string(), 0);
        layers.insert("C".to_string(), 2);
        let la = make_layer_assignment(layers);

        let aug = insert_dummy_nodes(&g, &la);

        // 3 nodes total: A, C, and 1 dummy
        assert_eq!(aug.graph.node_count(), 3, "Should have 1 dummy node");
        // 2 edges: A→dummy and dummy→C
        assert_eq!(aug.graph.edge_count(), 2);
        assert_eq!(aug.dummy_edges.len(), 1);

        let de = &aug.dummy_edges[0];
        assert_eq!(de.original_src, "A");
        assert_eq!(de.original_tgt, "C");
        assert_eq!(de.dummy_ids.len(), 1);

        // The dummy node should be in layer 1
        let dummy_id = &de.dummy_ids[0];
        assert_eq!(aug.layers[dummy_id], 1);

        // All edges must connect adjacent layers
        for edge in aug.graph.edge_references() {
            let src_id = &aug.graph[edge.source()].id;
            let tgt_id = &aug.graph[edge.target()].id;
            let src_layer = aug.layers[src_id];
            let tgt_layer = aug.layers[tgt_id];
            assert!(
                tgt_layer >= src_layer && tgt_layer - src_layer <= 1,
                "Edge {}→{} spans {} layers (expected ≤1)",
                src_id, tgt_id, tgt_layer.saturating_sub(src_layer)
            );
        }
    }

    #[test]
    fn test_long_edge_two_dummies() {
        // A(0) → D(3): span 3 — two dummies in layers 1 and 2
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let d = g.add_node(make_node("D"));
        g.add_edge(a, d, make_edge());

        let mut layers = HashMap::new();
        layers.insert("A".to_string(), 0);
        layers.insert("D".to_string(), 3);
        let la = make_layer_assignment(layers);

        let aug = insert_dummy_nodes(&g, &la);

        // 4 nodes: A, D, dummy_0, dummy_1
        assert_eq!(aug.graph.node_count(), 4);
        // 3 edges: A→d0, d0→d1, d1→D
        assert_eq!(aug.graph.edge_count(), 3);
        assert_eq!(aug.dummy_edges.len(), 1);

        let de = &aug.dummy_edges[0];
        assert_eq!(de.dummy_ids.len(), 2);
        assert_eq!(aug.layers[&de.dummy_ids[0]], 1);
        assert_eq!(aug.layers[&de.dummy_ids[1]], 2);
    }

    #[test]
    fn test_multiple_long_edges_independent() {
        // A(0) → C(2) and B(0) → D(2): two independent long edges
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let b = g.add_node(make_node("B"));
        let c = g.add_node(make_node("C"));
        let d = g.add_node(make_node("D"));
        g.add_edge(a, c, make_edge());
        g.add_edge(b, d, make_edge());

        let mut layers = HashMap::new();
        layers.insert("A".to_string(), 0);
        layers.insert("B".to_string(), 0);
        layers.insert("C".to_string(), 2);
        layers.insert("D".to_string(), 2);
        let la = make_layer_assignment(layers);

        let aug = insert_dummy_nodes(&g, &la);

        // 6 nodes: A, B, C, D + 2 dummies
        assert_eq!(aug.graph.node_count(), 6);
        // 4 edges: A→d0, d0→C, B→d1, d1→D
        assert_eq!(aug.graph.edge_count(), 4);
        assert_eq!(aug.dummy_edges.len(), 2);
    }

    #[test]
    fn test_mixed_short_and_long_edges() {
        // A(0) → B(1) [short] and A(0) → C(2) [long]
        let mut g: DiGraph<NodeData, EdgeData> = DiGraph::new();
        let a = g.add_node(make_node("A"));
        let b = g.add_node(make_node("B"));
        let c = g.add_node(make_node("C"));
        g.add_edge(a, b, make_edge());
        g.add_edge(a, c, make_edge());

        let mut layers = HashMap::new();
        layers.insert("A".to_string(), 0);
        layers.insert("B".to_string(), 1);
        layers.insert("C".to_string(), 2);
        let la = make_layer_assignment(layers);

        let aug = insert_dummy_nodes(&g, &la);

        // 4 nodes: A, B, C, 1 dummy
        assert_eq!(aug.graph.node_count(), 4);
        // 3 edges: A→B (short), A→dummy, dummy→C
        assert_eq!(aug.graph.edge_count(), 3);
        assert_eq!(aug.dummy_edges.len(), 1);

        // Verify all edges are adjacent-layer
        for edge in aug.graph.edge_references() {
            let src_id = &aug.graph[edge.source()].id;
            let tgt_id = &aug.graph[edge.target()].id;
            let diff = aug.layers[tgt_id].saturating_sub(aug.layers[src_id]);
            assert!(diff <= 1, "Edge {}→{} not adjacent-layer", src_id, tgt_id);
        }
    }
}
