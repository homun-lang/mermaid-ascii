use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct LayoutGraph {
    inner: DiGraph<String, String>,
    id_map: HashMap<String, NodeIndex>,
}

impl LayoutGraph {
    pub fn new() -> Self {
        Self {
            inner: DiGraph::new(),
            id_map: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: &str) -> NodeIndex {
        if let Some(&idx) = self.id_map.get(id) {
            return idx;
        }
        let idx = self.inner.add_node(id.to_string());
        self.id_map.insert(id.to_string(), idx);
        idx
    }

    pub fn add_edge(&mut self, from: &str, to: &str, label: String) {
        let from_idx = self.add_node(from);
        let to_idx = self.add_node(to);
        self.inner.add_edge(from_idx, to_idx, label);
    }

    pub fn node_index(&self, id: &str) -> Option<NodeIndex> {
        self.id_map.get(id).copied()
    }

    pub fn contains_node(&self, id: &str) -> bool {
        self.id_map.contains_key(id)
    }

    pub fn node_id(&self, idx: NodeIndex) -> &str {
        &self.inner[idx]
    }

    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    pub fn node_ids(&self) -> Vec<&str> {
        self.inner
            .node_indices()
            .map(|idx| self.inner[idx].as_str())
            .collect()
    }

    pub fn successors(&self, id: &str) -> Vec<&str> {
        let Some(&idx) = self.id_map.get(id) else {
            return Vec::new();
        };
        self.inner
            .neighbors_directed(idx, petgraph::Direction::Outgoing)
            .map(|n| self.inner[n].as_str())
            .collect()
    }

    pub fn predecessors(&self, id: &str) -> Vec<&str> {
        let Some(&idx) = self.id_map.get(id) else {
            return Vec::new();
        };
        self.inner
            .neighbors_directed(idx, petgraph::Direction::Incoming)
            .map(|n| self.inner[n].as_str())
            .collect()
    }

    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        let (Some(&from_idx), Some(&to_idx)) = (self.id_map.get(from), self.id_map.get(to)) else {
            return false;
        };
        self.inner.contains_edge(from_idx, to_idx)
    }

    pub fn inner(&self) -> &DiGraph<String, String> {
        &self.inner
    }
}

impl Default for LayoutGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_lookup_nodes() {
        let mut g = LayoutGraph::new();
        let a = g.add_node("A");
        let b = g.add_node("B");
        assert_ne!(a, b);
        assert_eq!(g.add_node("A"), a);
        assert_eq!(g.node_count(), 2);
        assert!(g.contains_node("A"));
        assert!(!g.contains_node("C"));
        assert_eq!(g.node_id(a), "A");
    }

    #[test]
    fn add_and_lookup_edges() {
        let mut g = LayoutGraph::new();
        g.add_edge("A", "B", String::new());
        g.add_edge("A", "C", "label".into());
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        assert!(g.has_edge("A", "B"));
        assert!(g.has_edge("A", "C"));
        assert!(!g.has_edge("B", "A"));
    }

    #[test]
    fn successors_and_predecessors() {
        let mut g = LayoutGraph::new();
        g.add_edge("A", "B", String::new());
        g.add_edge("A", "C", String::new());
        g.add_edge("B", "C", String::new());

        let mut succ: Vec<&str> = g.successors("A");
        succ.sort();
        assert_eq!(succ, vec!["B", "C"]);

        let pred: Vec<&str> = g.predecessors("C");
        let mut pred_sorted = pred.clone();
        pred_sorted.sort();
        assert_eq!(pred_sorted, vec!["A", "B"]);

        assert!(g.successors("C").is_empty());
        assert!(g.predecessors("A").is_empty());
    }

    #[test]
    fn node_ids_lists_all() {
        let mut g = LayoutGraph::new();
        g.add_node("X");
        g.add_node("Y");
        g.add_node("Z");
        let mut ids = g.node_ids();
        ids.sort();
        assert_eq!(ids, vec!["X", "Y", "Z"]);
    }

    #[test]
    fn unknown_node_returns_empty() {
        let g = LayoutGraph::new();
        assert!(g.successors("nope").is_empty());
        assert!(g.predecessors("nope").is_empty());
        assert!(!g.has_edge("a", "b"));
        assert_eq!(g.node_index("x"), None);
    }
}
