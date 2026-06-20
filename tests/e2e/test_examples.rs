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
fn assign_layers_diamond_ranks() {
    use mermaid_ascii::{assign_layers, parse_graph, remove_cycles, tokenize};
    // diamond.mm.md: A-->B, A-->C, B-->D, C-->D
    // longest-path ranks: A=0, B=C=1 (one hop from A), D=2 (two hops via B or C).
    let tokens =
        tokenize("graph TD\n    A --> B\n    A --> C\n    B --> D\n    C --> D\n".to_string());
    let graph = parse_graph(tokens);
    let dag = remove_cycles(graph.clone());
    let layers = assign_layers(graph.nodes.clone(), dag);
    let layer_of = |id: &str| layers.iter().find(|nl| nl.id == id).unwrap().layer;
    assert_eq!(layer_of("A"), 0);
    assert_eq!(layer_of("B"), 1);
    assert_eq!(layer_of("C"), 1);
    assert_eq!(layer_of("D"), 2);
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

// True if two axis-aligned boxes overlap (touching edges do not count as overlap).
fn boxes_overlap(a: &mermaid_ascii::LayoutNode, b: &mermaid_ascii::LayoutNode) -> bool {
    a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height
}

fn coords_for(src: &str) -> Vec<mermaid_ascii::LayoutNode> {
    use mermaid_ascii::{
        assign_coords, assign_layers, insert_dummies, order_layers, parse_graph, remove_cycles,
        tokenize,
    };
    let graph = parse_graph(tokenize(src.to_string()));
    let dir = graph.direction.clone();
    let dag = remove_cycles(graph.clone());
    let layers = assign_layers(graph.nodes.clone(), dag.clone());
    let expanded = insert_dummies(layers, dag);
    let ordered = order_layers(expanded.nodes, expanded.edges);
    assign_coords(ordered, graph.nodes.clone(), dir)
}

#[test]
fn assign_coords_no_overlap_simple() {
    let nodes = coords_for("graph TD\n    A --> B\n    B --> C\n");
    assert_eq!(nodes.len(), 3);
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            assert!(
                !boxes_overlap(&nodes[i], &nodes[j]),
                "{} overlaps {}",
                nodes[i].id,
                nodes[j].id
            );
        }
    }
}

#[test]
fn assign_coords_no_overlap_diamond() {
    let nodes = coords_for("graph TD\n    A --> B\n    A --> C\n    B --> D\n    C --> D\n");
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            assert!(
                !boxes_overlap(&nodes[i], &nodes[j]),
                "{} overlaps {}",
                nodes[i].id,
                nodes[j].id
            );
        }
    }
    // Real nodes carry a sized box (3 lines tall, label + frame wide).
    let a = nodes.iter().find(|n| n.id == "A").unwrap();
    assert_eq!(a.height, 3);
    assert!(a.width >= 5);
}

#[test]
fn assign_coords_lr_simple_horizontal() {
    // lr_simple.mm.md: a chain lays out left-to-right — x grows per layer while
    // y stays constant (all nodes share order 0).
    let nodes = coords_for("flowchart LR\n    Start --> Middle --> End\n");
    let s = nodes.iter().find(|n| n.id == "Start").unwrap();
    let m = nodes.iter().find(|n| n.id == "Middle").unwrap();
    let e = nodes.iter().find(|n| n.id == "End").unwrap();
    assert!(s.x < m.x && m.x < e.x, "LR chain should progress along x");
    assert_eq!(s.y, m.y, "LR chain should share a single row");
    assert_eq!(m.y, e.y, "LR chain should share a single row");
}

#[test]
fn assign_coords_lr_fanout_horizontal() {
    // lr_fanout.mm.md: siblings share a layer (same x) and stack vertically
    // (distinct, increasing y); the parent sits to their left.
    let nodes = coords_for("flowchart LR\n    A --> B\n    A --> C\n    A --> D\n");
    let a = nodes.iter().find(|n| n.id == "A").unwrap();
    let b = nodes.iter().find(|n| n.id == "B").unwrap();
    let c = nodes.iter().find(|n| n.id == "C").unwrap();
    let d = nodes.iter().find(|n| n.id == "D").unwrap();
    assert!(a.x < b.x, "parent A should sit left of its children");
    assert_eq!(b.x, c.x, "siblings share a layer column");
    assert_eq!(c.x, d.x, "siblings share a layer column");
    let mut ys = [b.y, c.y, d.y];
    ys.sort();
    assert!(ys[0] < ys[1] && ys[1] < ys[2], "siblings stack along y");
}

// Map each node to its (column, row) grid rank: the index of its distinct x among
// all distinct x values, and likewise its y among distinct y values. This collapses
// the absolute pixel pitch to integer grid cells, so two layouts can be compared
// structurally regardless of box sizes.
fn grid_ranks(nodes: &[mermaid_ascii::LayoutNode]) -> Vec<(String, usize, usize)> {
    let mut xs: Vec<i32> = nodes.iter().map(|n| n.x).collect();
    xs.sort_unstable();
    xs.dedup();
    let mut ys: Vec<i32> = nodes.iter().map(|n| n.y).collect();
    ys.sort_unstable();
    ys.dedup();
    nodes
        .iter()
        .map(|n| {
            let col = xs.iter().position(|x| *x == n.x).unwrap();
            let row = ys.iter().position(|y| *y == n.y).unwrap();
            (n.id.clone(), col, row)
        })
        .collect()
}

#[test]
fn lr_transposes_td_layout() {
    // The same graph laid out TD and LR must be grid transposes of each other:
    // a node at TD cell (col, row) lands at LR cell (row, col). TD runs layers
    // down / order across; LR swaps those axes (layout.hom assign_coords).
    let edges = "    A --> B\n    A --> C\n    B --> D\n    C --> D\n";
    let td = grid_ranks(&coords_for(&format!("graph TD\n{edges}")));
    let lr = grid_ranks(&coords_for(&format!("flowchart LR\n{edges}")));
    assert_eq!(td.len(), lr.len());
    for (id, tcol, trow) in &td {
        let (_, lcol, lrow) = lr.iter().find(|(lid, _, _)| lid == id).unwrap();
        assert_eq!(
            (*lcol, *lrow),
            (*trow, *tcol),
            "node {id}: LR grid cell should be the transpose of its TD cell"
        );
    }
    // Sanity: the transpose is non-trivial (B and C actually move off the diagonal).
    let b = td.iter().find(|(id, _, _)| id == "B").unwrap();
    assert_ne!(
        (b.1, b.2),
        (b.2, b.1),
        "diamond TD layout must be off-diagonal"
    );
}

// Full layout through routing: returns (laid-out nodes, routed edges).
fn route_for(
    src: &str,
) -> (
    Vec<mermaid_ascii::LayoutNode>,
    Vec<mermaid_ascii::RoutedEdge>,
) {
    use mermaid_ascii::{
        assign_coords, assign_layers, insert_dummies, order_layers, parse_graph, remove_cycles,
        route_edges, tokenize,
    };
    let graph = parse_graph(tokenize(src.to_string()));
    let dir = graph.direction.clone();
    let dag = remove_cycles(graph.clone());
    let layers = assign_layers(graph.nodes.clone(), dag.clone());
    let expanded = insert_dummies(layers, dag);
    let ordered = order_layers(expanded.nodes, expanded.edges.clone());
    let nodes = assign_coords(ordered, graph.nodes.clone(), dir.clone());
    let routed = route_edges(nodes.clone(), expanded.edges, dir);
    (nodes, routed)
}

// True if cell (x,y) lies strictly inside a real node box other than `from`/`to`.
fn cell_in_other_box(
    nodes: &[mermaid_ascii::LayoutNode],
    from: &str,
    to: &str,
    x: i32,
    y: i32,
) -> Option<String> {
    for n in nodes {
        if n.is_dummy || n.id == from || n.id == to {
            continue;
        }
        if x >= n.x && x < n.x + n.width && y >= n.y && y < n.y + n.height {
            return Some(n.id.clone());
        }
    }
    None
}

// Walk every orthogonal segment between consecutive waypoints, asserting no cell
// passes through a node box that isn't this edge's own endpoint.
fn assert_no_box_crossing(src: &str) {
    let (nodes, routed) = route_for(src);
    assert!(!routed.is_empty(), "no edges routed for: {src}");
    for e in &routed {
        let w = &e.waypoints;
        assert!(
            w.len() >= 2,
            "edge {}->{} has < 2 waypoints",
            e.from_id,
            e.to_id
        );
        for seg in w.windows(2) {
            let (a, b) = (&seg[0], &seg[1]);
            let dx = (b.x - a.x).signum();
            let dy = (b.y - a.y).signum();
            assert!(
                dx == 0 || dy == 0,
                "edge {}->{} segment not orthogonal",
                e.from_id,
                e.to_id
            );
            let (mut x, mut y) = (a.x, a.y);
            loop {
                if let Some(hit) = cell_in_other_box(&nodes, &e.from_id, &e.to_id, x, y) {
                    panic!(
                        "edge {}->{} crosses node {} at ({},{})",
                        e.from_id, e.to_id, hit, x, y
                    );
                }
                if x == b.x && y == b.y {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
    }
}

#[test]
fn route_no_box_crossing_chain_td() {
    assert_no_box_crossing("graph TD\n    A --> B\n    B --> C\n");
}

#[test]
fn route_no_box_crossing_diamond_td() {
    assert_no_box_crossing("graph TD\n    A --> B\n    A --> C\n    B --> D\n    C --> D\n");
}

#[test]
fn route_no_box_crossing_chain_lr() {
    assert_no_box_crossing("graph LR\n    A --> B\n    B --> C\n");
}

#[test]
fn route_waypoints_connect_endpoints_td() {
    // First waypoint sits on the source box border, last on the target box border.
    let (nodes, routed) = route_for("graph TD\n    A --> B\n");
    let e = &routed[0];
    let a = nodes.iter().find(|n| n.id == e.from_id).unwrap();
    let b = nodes.iter().find(|n| n.id == e.to_id).unwrap();
    let first = e.waypoints.first().unwrap();
    let last = e.waypoints.last().unwrap();
    // start on A's bottom border row, end on B's top border row
    assert_eq!(first.y, a.y + a.height - 1);
    assert_eq!(last.y, b.y);
}

// --- ASCII renderer: shape-aware node boxes (task-27) ---

fn paint_one(width: i32, shape: mermaid_ascii::NodeShape, label: &str) -> String {
    use mermaid_ascii::{LayoutNode, canvas_new, canvas_to_string, charset_unicode, paint_node};
    let n = LayoutNode {
        id: "n".to_string(),
        x: 0,
        y: 0,
        width,
        height: 3,
        is_dummy: false,
    };
    let mut c = canvas_new(width, 3);
    paint_node(&mut c, charset_unicode(), n, shape, label.to_string());
    canvas_to_string(c)
}

#[test]
fn paint_rectangle_box() {
    // width = len("Rectangle") + 4 = 13
    assert_eq!(
        paint_one(13, mermaid_ascii::NodeShape::Rectangle, "Rectangle"),
        "┌───────────┐\n│ Rectangle │\n└───────────┘\n"
    );
}

#[test]
fn paint_rounded_box() {
    assert_eq!(
        paint_one(11, mermaid_ascii::NodeShape::Rounded, "Rounded"),
        "╭─────────╮\n│ Rounded │\n╰─────────╯\n"
    );
}

#[test]
fn paint_diamond_box() {
    assert_eq!(
        paint_one(11, mermaid_ascii::NodeShape::Diamond, "Diamond"),
        "/─────────\\\n│ Diamond │\n\\─────────/\n"
    );
}

#[test]
fn paint_circle_box() {
    // Circle has no vertical side borders; label centered in inner width.
    assert_eq!(
        paint_one(10, mermaid_ascii::NodeShape::Circle, "Circle"),
        "(────────)\n  Circle\n(────────)\n"
    );
}

#[test]
fn paint_dummy_node_is_skipped() {
    use mermaid_ascii::{LayoutNode, canvas_new, canvas_to_string, charset_unicode, paint_node};
    let n = LayoutNode {
        id: "__d0".to_string(),
        x: 0,
        y: 0,
        width: 1,
        height: 1,
        is_dummy: true,
    };
    let mut c = canvas_new(5, 3);
    paint_node(
        &mut c,
        charset_unicode(),
        n,
        mermaid_ascii::NodeShape::Rectangle,
        "".to_string(),
    );
    assert_eq!(canvas_to_string(c), "");
}

#[test]
fn paint_ascii_charset_rectangle() {
    use mermaid_ascii::{LayoutNode, canvas_new, canvas_to_string, charset_ascii, paint_node};
    let n = LayoutNode {
        id: "n".to_string(),
        x: 0,
        y: 0,
        width: 7,
        height: 3,
        is_dummy: false,
    };
    let mut c = canvas_new(7, 3);
    paint_node(
        &mut c,
        charset_ascii(),
        n,
        mermaid_ascii::NodeShape::Rectangle,
        "Hi".to_string(),
    );
    assert_eq!(canvas_to_string(c), "+-----+\n| Hi  |\n+-----+\n");
}

// --- ASCII renderer: edges + arrows + stubs + labels (task-28) ---

// Right-trim every line. canvas_to_string's rtrim is byte/char inconsistent for rows
// containing multibyte glyphs (a canvas.hom/runtime issue, not the edge painter), so
// these tests normalize trailing whitespace to assert on glyph placement alone.
fn rtrim_lines(s: String) -> String {
    let mut out: String = s
        .lines()
        .map(|l| l.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    if s.ends_with('\n') {
        out.push('\n');
    }
    out
}

// Paint two stacked 3x5 boxes (A over B, separated by 3 rows) plus one vertical edge
// between them, reproducing a single column of edges.expect.txt. The edge runs from
// A's bottom-border centre (2,2) straight down to B's top-border centre (2,6).
fn paint_vertical_edge(et: mermaid_ascii::EdgeType, reversed: bool, label: &str) -> String {
    use mermaid_ascii::{
        LayoutNode, NodeShape, Point, RoutedEdge, canvas_new, canvas_to_string, charset_unicode,
        paint_edge, paint_node,
    };
    let mk = |id: &str, y: i32| LayoutNode {
        id: id.to_string(),
        x: 0,
        y,
        width: 5,
        height: 3,
        is_dummy: false,
    };
    let mut c = canvas_new(5, 9);
    paint_node(
        &mut c,
        charset_unicode(),
        mk("A", 0),
        NodeShape::Rectangle,
        "A".to_string(),
    );
    paint_node(
        &mut c,
        charset_unicode(),
        mk("B", 6),
        NodeShape::Rectangle,
        "B".to_string(),
    );
    let edge = RoutedEdge {
        from_id: "A".to_string(),
        to_id: "B".to_string(),
        edge_type: et,
        label: label.to_string(),
        reversed,
        waypoints: vec![Point { x: 2, y: 2 }, Point { x: 2, y: 6 }],
    };
    paint_edge(&mut c, charset_unicode(), edge);
    rtrim_lines(canvas_to_string(c))
}

#[test]
fn edge_solid_arrow() {
    // A --> B : exit stub ┬ on A, solid line, ▼ arrowhead above B.
    assert_eq!(
        paint_vertical_edge(mermaid_ascii::EdgeType::Arrow, false, ""),
        "┌───┐\n│ A │\n└─┬─┘\n  │\n  │\n  ▼\n┌───┐\n│ B │\n└───┘\n"
    );
}

#[test]
fn edge_plain_line() {
    // C --- D : solid line, no arrowhead, target border left intact.
    assert_eq!(
        paint_vertical_edge(mermaid_ascii::EdgeType::Line, false, ""),
        "┌───┐\n│ A │\n└─┬─┘\n  │\n  │\n  │\n┌───┐\n│ B │\n└───┘\n"
    );
}

#[test]
fn edge_dotted_arrow() {
    // E -.-> F : dotted line glyph ╎ with a ▼ arrowhead.
    assert_eq!(
        paint_vertical_edge(mermaid_ascii::EdgeType::DottedArrow, false, ""),
        "┌───┐\n│ A │\n└─┬─┘\n  ╎\n  ╎\n  ▼\n┌───┐\n│ B │\n└───┘\n"
    );
}

#[test]
fn edge_thick_arrow() {
    // G ==> H : thick line glyph ║ with a ▼ arrowhead.
    assert_eq!(
        paint_vertical_edge(mermaid_ascii::EdgeType::ThickArrow, false, ""),
        "┌───┐\n│ A │\n└─┬─┘\n  ║\n  ║\n  ▼\n┌───┐\n│ B │\n└───┘\n"
    );
}

#[test]
fn edge_bidirectional() {
    // I <--> J : arrowheads at both ends (▲ near source, ▼ near target).
    assert_eq!(
        paint_vertical_edge(mermaid_ascii::EdgeType::BidirArrow, false, ""),
        "┌───┐\n│ A │\n└─┬─┘\n  ▲\n  │\n  ▼\n┌───┐\n│ B │\n└───┘\n"
    );
}

#[test]
fn edge_reversed_flips_arrowhead() {
    // A FAS-reversed arrow flows source->target downward but its head belongs on the
    // source end, so the arrowhead flips up to ▲ and the target border stays clean.
    assert_eq!(
        paint_vertical_edge(mermaid_ascii::EdgeType::Arrow, true, ""),
        "┌───┐\n│ A │\n└─┬─┘\n  ▲\n  │\n  │\n┌───┐\n│ B │\n└───┘\n"
    );
}

#[test]
fn edge_label_at_midpoint() {
    // Edge label sits beside the line at the path midpoint. Wider canvas so it fits.
    use mermaid_ascii::{
        LayoutNode, NodeShape, Point, RoutedEdge, canvas_new, canvas_to_string, charset_unicode,
        paint_edge, paint_node,
    };
    let mk = |id: &str, y: i32| LayoutNode {
        id: id.to_string(),
        x: 0,
        y,
        width: 5,
        height: 3,
        is_dummy: false,
    };
    let mut c = canvas_new(10, 9);
    paint_node(
        &mut c,
        charset_unicode(),
        mk("A", 0),
        NodeShape::Rectangle,
        "A".to_string(),
    );
    paint_node(
        &mut c,
        charset_unicode(),
        mk("B", 6),
        NodeShape::Rectangle,
        "B".to_string(),
    );
    let edge = RoutedEdge {
        from_id: "A".to_string(),
        to_id: "B".to_string(),
        edge_type: mermaid_ascii::EdgeType::Arrow,
        label: "yes".to_string(),
        reversed: false,
        waypoints: vec![Point { x: 2, y: 2 }, Point { x: 2, y: 6 }],
    };
    paint_edge(&mut c, charset_unicode(), edge);
    let out = canvas_to_string(c);
    // Midpoint cell is row 4 (cells 2,3,4,5,6 -> index 2 -> y=4); label starts at col 3.
    assert_eq!(out.lines().nth(4).unwrap().trim_end(), "  │yes");
}

#[test]
fn edge_through_dummy_is_straight_passthrough() {
    // An edge segment ending at a dummy bend point gets no arrowhead and no border
    // skip — the dummy cell is painted as a straight line continuation.
    use mermaid_ascii::{
        LayoutNode, NodeShape, Point, RoutedEdge, canvas_new, canvas_to_string, charset_unicode,
        paint_edge, paint_node,
    };
    let a = LayoutNode {
        id: "A".to_string(),
        x: 0,
        y: 0,
        width: 5,
        height: 3,
        is_dummy: false,
    };
    let mut c = canvas_new(5, 6);
    paint_node(
        &mut c,
        charset_unicode(),
        a,
        NodeShape::Rectangle,
        "A".to_string(),
    );
    let edge = RoutedEdge {
        from_id: "A".to_string(),
        to_id: "__d0".to_string(),
        edge_type: mermaid_ascii::EdgeType::Arrow,
        label: String::new(),
        reversed: false,
        waypoints: vec![Point { x: 2, y: 2 }, Point { x: 2, y: 5 }],
    };
    paint_edge(&mut c, charset_unicode(), edge);
    // Stub on A, then plain vertical line all the way down to the dummy cell (no ▼).
    assert_eq!(
        rtrim_lines(canvas_to_string(c)),
        "┌───┐\n│ A │\n└─┬─┘\n  │\n  │\n  │\n"
    );
}
