#[test]
fn placeholder() {}

#[test]
fn tokenize_simple() {
    use mermaid_ascii::{TokenKind, tokenize};
    let input = "graph TD\n    A --> B --> C\n";
    let tokens = tokenize(input.to_string());
    let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &TokenKind::Header,
            &TokenKind::DirTD,
            &TokenKind::Newline,
            &TokenKind::Ident,
            &TokenKind::Arrow,
            &TokenKind::Ident,
            &TokenKind::Arrow,
            &TokenKind::Ident,
            &TokenKind::Newline,
        ]
    );
    assert_eq!(tokens[0].text, "graph");
    assert_eq!(tokens[3].text, "A");
    assert_eq!(tokens[5].text, "B");
    assert_eq!(tokens[7].text, "C");
}

#[test]
fn tokenize_flowchart() {
    use mermaid_ascii::{TokenKind, tokenize};
    let input = "graph TD\n    Start[Start] --> Decision{Decision}\n    Decision -->|yes| ProcessA[Process A]\n    Decision -->|no| ProcessB[Process B]\n    ProcessA --> End[End]\n    ProcessB --> End\n";
    let tokens = tokenize(input.to_string());
    // Should not panic and should produce tokens
    assert!(!tokens.is_empty());
    // First token is header
    assert_eq!(tokens[0].kind, TokenKind::Header);
    assert_eq!(tokens[0].text, "graph");
    // Should contain labels
    let labels: Vec<_> = tokens
        .iter()
        .filter(|t| t.kind == TokenKind::Label)
        .map(|t| t.text.as_str())
        .collect();
    assert!(labels.contains(&"Start"));
    assert!(labels.contains(&"Decision"));
    assert!(labels.contains(&"Process A"));
    assert!(labels.contains(&"End"));
    // Should contain pipe-delimited labels
    let pipes: Vec<_> = tokens
        .iter()
        .filter(|t| t.kind == TokenKind::Pipe)
        .collect();
    assert!(!pipes.is_empty());
}

#[test]
fn tokenize_edges() {
    use mermaid_ascii::{TokenKind, tokenize};
    let input = "graph TD\n    A --> B\n    C --- D\n    E -.-> F\n    G ==> H\n    I <--> J\n";
    let tokens = tokenize(input.to_string());
    let edge_kinds: Vec<_> = tokens
        .iter()
        .filter(|t| {
            matches!(
                t.kind,
                TokenKind::Arrow
                    | TokenKind::Line
                    | TokenKind::DottedArrow
                    | TokenKind::ThickArrow
                    | TokenKind::BidirArrow
            )
        })
        .map(|t| &t.kind)
        .collect();
    assert_eq!(
        edge_kinds,
        vec![
            &TokenKind::Arrow,
            &TokenKind::Line,
            &TokenKind::DottedArrow,
            &TokenKind::ThickArrow,
            &TokenKind::BidirArrow,
        ]
    );
}

#[test]
fn tokenize_shapes() {
    use mermaid_ascii::{TokenKind, tokenize};
    let input = "graph TD\n    A[Rectangle] --> B(Rounded) --> C{Diamond} --> D((Circle))\n";
    let tokens = tokenize(input.to_string());
    assert!(!tokens.is_empty());
    // Should have bracket types for each shape
    let has_bracket = tokens.iter().any(|t| t.kind == TokenKind::BracketOpen);
    let has_paren = tokens.iter().any(|t| t.kind == TokenKind::ParenOpen);
    let has_brace = tokens.iter().any(|t| t.kind == TokenKind::BraceOpen);
    let has_double_paren = tokens.iter().any(|t| t.kind == TokenKind::DoubleParenOpen);
    assert!(has_bracket);
    assert!(has_paren);
    assert!(has_brace);
    assert!(has_double_paren);
}

#[test]
fn tokenize_subgraph() {
    use mermaid_ascii::{TokenKind, tokenize};
    let input = "graph TD\n    subgraph Frontend\n        A[Web App]\n    end\n";
    let tokens = tokenize(input.to_string());
    assert!(tokens.iter().any(|t| t.kind == TokenKind::SubgraphKw));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::EndKw));
}

#[test]
fn tokenize_compact_arrow() {
    use mermaid_ascii::{TokenKind, tokenize};
    let tokens = tokenize("graph TD\n A-->B".to_string());
    let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &TokenKind::Header,
            &TokenKind::DirTD,
            &TokenKind::Newline,
            &TokenKind::Ident,
            &TokenKind::Arrow,
            &TokenKind::Ident,
        ]
    );
    assert_eq!(tokens[0].text, "graph");
    assert_eq!(tokens[1].text, "TD");
    assert_eq!(tokens[3].text, "A");
    assert_eq!(tokens[4].text, "-->");
    assert_eq!(tokens[5].text, "B");
}

#[test]
fn tokenize_labeled_edge() {
    use mermaid_ascii::{TokenKind, tokenize};
    let tokens = tokenize("graph TD\n A-->|yes| B".to_string());
    let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &TokenKind::Header,
            &TokenKind::DirTD,
            &TokenKind::Newline,
            &TokenKind::Ident,
            &TokenKind::Arrow,
            &TokenKind::Pipe,
            &TokenKind::Ident,
            &TokenKind::Pipe,
            &TokenKind::Ident,
        ]
    );
    assert_eq!(tokens[4].text, "-->");
    assert_eq!(tokens[6].text, "yes");
    assert_eq!(tokens[8].text, "B");
}

#[test]
fn parse_graph_td() {
    use mermaid_ascii::{Direction, parse_graph, tokenize};
    let tokens = tokenize("graph TD\n    A --> B\n".to_string());
    let graph = parse_graph(tokens);
    assert_eq!(graph.direction, Direction::TD);
    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 1);
    assert_eq!(graph.edges[0].from_id, "A");
    assert_eq!(graph.edges[0].to_id, "B");
    assert!(graph.subgraphs.is_empty());
}

#[test]
fn parse_graph_lr() {
    use mermaid_ascii::{Direction, parse_graph, tokenize};
    let tokens = tokenize("flowchart LR\n    A --> B\n".to_string());
    let graph = parse_graph(tokens);
    assert_eq!(graph.direction, Direction::LR);
}

#[test]
fn parse_graph_tb_alias() {
    use mermaid_ascii::{Direction, parse_graph, tokenize};
    let tokens = tokenize("graph TB\n    A --> B\n".to_string());
    let graph = parse_graph(tokens);
    assert_eq!(graph.direction, Direction::TD);
}

#[test]
fn parse_graph_default_direction() {
    use mermaid_ascii::{Direction, parse_graph, tokenize};
    let tokens = tokenize("graph\n    A --> B\n".to_string());
    let graph = parse_graph(tokens);
    assert_eq!(graph.direction, Direction::TD);
}

#[test]
fn parse_graph_unsupported_direction() {
    use mermaid_ascii::{Direction, parse_graph, tokenize};
    let tokens = tokenize("graph RL\n    A --> B\n".to_string());
    let graph = parse_graph(tokens);
    assert_eq!(graph.direction, Direction::TD);
}

#[test]
fn assign_layers_simple_chain() {
    use mermaid_ascii::{assign_layers, parse_graph, remove_cycles, tokenize};
    // simple.mm.md: A --> B --> C  ⇒  layers 0 / 1 / 2
    let tokens = tokenize("graph TD\n    A --> B --> C\n".to_string());
    let graph = parse_graph(tokens);
    let dag = remove_cycles(graph.clone());
    let layers = assign_layers(graph.nodes.clone(), dag);
    let layer_of = |id: &str| layers.iter().find(|nl| nl.id == id).unwrap().layer;
    assert_eq!(layer_of("A"), 0);
    assert_eq!(layer_of("B"), 1);
    assert_eq!(layer_of("C"), 2);
}

#[test]
fn order_layers_diamond_no_overlap() {
    use mermaid_ascii::{
        assign_layers, insert_dummies, order_layers, parse_graph, remove_cycles, tokenize,
    };
    // diamond.mm.md: A-->B, A-->C, B-->D, C-->D
    // layers: A=0, B=C=1, D=2. B and C share layer 1 and must get distinct orders.
    let tokens =
        tokenize("graph TD\n    A --> B\n    A --> C\n    B --> D\n    C --> D\n".to_string());
    let graph = parse_graph(tokens);
    let dag = remove_cycles(graph.clone());
    let layers = assign_layers(graph.nodes.clone(), dag.clone());
    let expanded = insert_dummies(layers, dag);
    let ordered = order_layers(expanded.nodes, expanded.edges);
    let find = |id: &str| ordered.iter().find(|o| o.id == id).unwrap();
    // Single node per layer sits at order 0.
    assert_eq!(find("A").order, 0);
    assert_eq!(find("D").order, 0);
    // B and C occupy the same layer with no overlapping order.
    assert_eq!(find("B").layer, find("C").layer);
    assert_ne!(find("B").order, find("C").order);
    let mut layer1: Vec<i64> = ordered
        .iter()
        .filter(|o| o.layer == find("B").layer)
        .map(|o| o.order as i64)
        .collect();
    layer1.sort_unstable();
    assert_eq!(layer1, vec![0, 1]);
}
