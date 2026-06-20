use mermaid_ascii::{Direction, EdgeType, NodeShape, parse_graph, tokenize};

fn parse(s: &str) -> mermaid_ascii::Graph {
    parse_graph(tokenize(s.to_string()))
}

#[test]
fn diamond_counts() {
    let g = parse("graph TD\n    A --> B\n    A --> C\n    B --> D\n    C --> D\n");
    assert_eq!(g.direction, Direction::TD);
    assert_eq!(g.nodes.len(), 4, "nodes");
    assert_eq!(g.edges.len(), 4, "edges");
}

#[test]
fn shapes_and_chain() {
    let g = parse("graph TD\n    A[Rectangle] --> B(Rounded) --> C{Diamond} --> D((Circle))\n");
    assert_eq!(g.nodes.len(), 4);
    assert_eq!(g.edges.len(), 3, "chained edges");
    assert_eq!(g.nodes[0].shape, NodeShape::Rectangle);
    assert_eq!(g.nodes[0].label, "Rectangle");
    assert_eq!(g.nodes[1].shape, NodeShape::Rounded);
    assert_eq!(g.nodes[2].shape, NodeShape::Diamond);
    assert_eq!(g.nodes[3].shape, NodeShape::Circle);
}

#[test]
fn edge_types_and_lr() {
    let g =
        parse("flowchart LR\n    A --> B\n    C --- D\n    E -.-> F\n    G ==> H\n    I <--> J\n");
    assert_eq!(g.direction, Direction::LR);
    assert_eq!(g.nodes.len(), 10);
    assert_eq!(g.edges.len(), 5);
    assert_eq!(g.edges[0].edge_type, EdgeType::Arrow);
    assert_eq!(g.edges[1].edge_type, EdgeType::Line);
    assert_eq!(g.edges[2].edge_type, EdgeType::DottedArrow);
    assert_eq!(g.edges[3].edge_type, EdgeType::ThickArrow);
    assert_eq!(g.edges[4].edge_type, EdgeType::BidirArrow);
}

#[test]
fn edge_label() {
    let g = parse("graph TD\n    A -->|yes| B\n");
    assert_eq!(g.edges.len(), 1);
    assert_eq!(g.edges[0].label, "yes");
}
