use crate::graph::*;

#[test]
fn test_empty_graph() {
    let g = graph_new();
    assert_eq!(graph_node_count(&g), 0);
    assert_eq!(graph_edge_count(&g), 0);
    assert!(graph_is_dag(&g));
    assert_eq!(graph_topo_sort(&g), Some(vec![]));
}

#[test]
fn test_add_node() {
    let mut g = graph_new();
    graph_add_node(&mut g, "A", "Node A", "Rectangle", None);
    assert_eq!(graph_node_count(&g), 1);
    assert_eq!(graph_nodes(&g), vec!["A"]);
}

#[test]
fn test_add_node_dedup() {
    let mut g = graph_new();
    graph_add_node(&mut g, "A", "Node A", "Rectangle", None);
    graph_add_node(&mut g, "A", "duplicate", "Rounded", None);
    assert_eq!(graph_node_count(&g), 1);
    let idx = g.node_index["A"];
    assert_eq!(g.digraph[idx].label, "Node A");
}

#[test]
fn test_add_edge_creates_placeholder_nodes() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    assert_eq!(graph_node_count(&g), 2);
    assert_eq!(graph_edge_count(&g), 1);
}

#[test]
fn test_successors_and_predecessors() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "A", "C", "Arrow", None);

    assert_eq!(graph_successors(&g, "A"), vec!["B", "C"]);
    assert_eq!(graph_predecessors(&g, "B"), vec!["A"]);
    assert_eq!(graph_predecessors(&g, "A"), Vec::<String>::new());
}

#[test]
fn test_successors_missing_node() {
    let g = graph_new();
    assert_eq!(graph_successors(&g, "X"), Vec::<String>::new());
    assert_eq!(graph_predecessors(&g, "X"), Vec::<String>::new());
}

#[test]
fn test_in_and_out_degree() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "C", "B", "Arrow", None);

    assert_eq!(graph_out_degree(&g, "A"), 1);
    assert_eq!(graph_in_degree(&g, "B"), 2);
    assert_eq!(graph_in_degree(&g, "A"), 0);
    assert_eq!(graph_in_degree(&g, "missing"), 0);
    assert_eq!(graph_out_degree(&g, "missing"), 0);
}

#[test]
fn test_edges_list() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "B", "C", "Line", Some("label"));
    let edges = graph_edges(&g);
    assert_eq!(
        edges,
        vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
        ]
    );
}

#[test]
fn test_nodes_sorted() {
    let mut g = graph_new();
    graph_add_node(&mut g, "C", "C", "Rectangle", None);
    graph_add_node(&mut g, "A", "A", "Rectangle", None);
    graph_add_node(&mut g, "B", "B", "Rectangle", None);
    assert_eq!(graph_nodes(&g), vec!["A", "B", "C"]);
}

#[test]
fn test_is_dag_true() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "B", "C", "Arrow", None);
    assert!(graph_is_dag(&g));
}

#[test]
fn test_is_dag_false_with_cycle() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "B", "C", "Arrow", None);
    graph_add_edge(&mut g, "C", "A", "Arrow", None);
    assert!(!graph_is_dag(&g));
}

#[test]
fn test_topo_sort_chain() {
    let mut g = graph_new();
    graph_add_node(&mut g, "A", "A", "Rectangle", None);
    graph_add_node(&mut g, "B", "B", "Rectangle", None);
    graph_add_node(&mut g, "C", "C", "Rectangle", None);
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "B", "C", "Arrow", None);
    let order = graph_topo_sort(&g).unwrap();
    let pos = |id: &str| order.iter().position(|x| x == id).unwrap();
    assert!(pos("A") < pos("B"));
    assert!(pos("B") < pos("C"));
}

#[test]
fn test_topo_sort_returns_none_for_cycle() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "Arrow", None);
    graph_add_edge(&mut g, "B", "A", "Arrow", None);
    assert!(graph_topo_sort(&g).is_none());
}

#[test]
fn test_graph_copy_is_independent() {
    let mut g = graph_new();
    graph_add_node(&mut g, "X", "Node X", "Diamond", None);
    let mut g2 = graph_copy(&g);

    graph_add_node(&mut g, "Y", "Node Y", "Rectangle", None);
    assert_eq!(graph_node_count(&g2), 1);
    assert_eq!(graph_nodes(&g2), vec!["X"]);

    graph_add_node(&mut g2, "Z", "Node Z", "Rounded", None);
    assert_eq!(graph_node_count(&g), 2);
}

#[test]
fn test_node_data_fields() {
    let mut g = graph_new();
    graph_add_node(&mut g, "A", "Label A", "Rounded", Some("mysubgraph"));
    let idx = g.node_index["A"];
    let data = &g.digraph[idx];
    assert_eq!(data.id, "A");
    assert_eq!(data.label, "Label A");
    assert_eq!(data.shape, "Rounded");
    assert_eq!(data.subgraph, Some("mysubgraph".to_string()));
}

#[test]
fn test_edge_data_fields() {
    let mut g = graph_new();
    graph_add_edge(&mut g, "A", "B", "DottedArrow", Some("my label"));
    let eidx = g.digraph.edge_indices().next().unwrap();
    let data = &g.digraph[eidx];
    assert_eq!(data.edge_type, "DottedArrow");
    assert_eq!(data.label, Some("my label".to_string()));
}

#[test]
fn test_ensure_node_creates_placeholder() {
    let mut g = graph_new();
    graph_ensure_node(&mut g, "implicit");
    assert_eq!(graph_node_count(&g), 1);
    let idx = g.node_index["implicit"];
    let data = &g.digraph[idx];
    assert_eq!(data.id, "implicit");
    assert_eq!(data.label, "implicit");
    assert_eq!(data.shape, "Rectangle");
    assert!(data.subgraph.is_none());
}
