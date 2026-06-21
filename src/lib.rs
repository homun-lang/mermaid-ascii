pub mod graph;

#[allow(
    dead_code,
    unused_imports,
    unused_macros,
    unused_assignments,
    unused_mut,
    unused_variables,
    clippy::assign_op_pattern,
    clippy::clone_on_copy,
    clippy::let_and_return,
    clippy::manual_map,
    clippy::needless_return,
    clippy::no_effect,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::redundant_field_names,
    clippy::single_match,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    clippy::unnecessary_mut_passed,
    clippy::unnecessary_to_owned,
    clippy::unused_unit,
    clippy::useless_conversion
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/runtime.rs"));
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
    include!(concat!(env!("OUT_DIR"), "/lexer.rs"));
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
    include!(concat!(env!("OUT_DIR"), "/layout.rs"));
    include!(concat!(env!("OUT_DIR"), "/subgraph.rs"));
    include!(concat!(env!("OUT_DIR"), "/pathfinder.rs"));
    include!(concat!(env!("OUT_DIR"), "/canvas.rs"));
    include!(concat!(env!("OUT_DIR"), "/render_ascii.rs"));
    include!(concat!(env!("OUT_DIR"), "/render_svg.rs"));
}
pub use generated::parse_graph;
pub use generated::{
    BoxChars, Canvas, CharSet, canvas_new, canvas_to_string, charset_ascii, charset_unicode,
    paint_edge, paint_node,
};
pub use generated::{Direction, Edge, EdgeType, Graph, Node, NodeShape, Subgraph};
pub use generated::{
    DummyResult, LayoutNode, OrderedNode, assign_coords, insert_dummies, order_layers,
};
pub use generated::{LayoutEdge, NodeLayer, assign_layers, remove_cycles};
pub use generated::{Point, RoutedEdge, route_edges};
pub use generated::{Token, TokenKind, tokenize};
pub use generated::{
    build_dim_overrides, collapse_subgraphs, expand_compound_nodes, paint_compound,
};
pub use generated::{svg_compound, svg_edge, svg_node};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub fn render_dsl(text: &str, ascii: bool, direction: Option<Direction>) -> String {
    let tokens = tokenize(text.to_string());
    let mut graph = parse_graph(tokens);
    if let Some(dir) = direction {
        graph.direction = dir;
    }
    render(&graph, ascii)
}

pub fn render_dsl_svg(text: &str, direction: Option<Direction>) -> String {
    let tokens = tokenize(text.to_string());
    let mut graph = parse_graph(tokens);
    if let Some(dir) = direction {
        graph.direction = dir;
    }
    render_svg_doc(&graph)
}

// Shape of node `id` (defaults to Rectangle if not found — e.g. dummies, which are
// skipped by paint_node anyway).
fn shape_of(graph: &Graph, id: &str) -> NodeShape {
    for n in &graph.nodes {
        if n.id == id {
            return n.shape.clone();
        }
    }
    NodeShape::Rectangle
}

// Display label for node `id`: the parsed label, falling back to the id itself.
fn label_of(graph: &Graph, id: &str) -> String {
    for n in &graph.nodes {
        if n.id == id {
            if !n.label.is_empty() {
                return n.label.clone();
            }
            return n.id.clone();
        }
    }
    id.to_string()
}

// Full ASCII/Unicode render: lay the graph out (Sugiyama → coords → routing) into the
// shared graphIR, then paint nodes and edges onto a character canvas.
// Shared layout: graph → graphIR (LayoutNode[] + RoutedEdge[]). Subgraphs are collapsed
// into compound nodes, laid out, then expanded into container + member boxes (ref-hom-rs).
fn layout_pipeline(graph: &Graph) -> (Vec<LayoutNode>, Vec<RoutedEdge>) {
    let dir = graph.direction.clone();
    if !graph.subgraphs.is_empty() {
        let cr = collapse_subgraphs(graph.clone(), 1);
        let overrides = build_dim_overrides(cr.compounds.clone(), 1);
        let dag = remove_cycles(cr.collapsed.clone());
        let layers = assign_layers(cr.collapsed.nodes.clone(), dag.clone());
        let expanded = insert_dummies(layers, dag);
        let ordered = order_layers(expanded.nodes, expanded.edges.clone());
        let laid = assign_coords(
            ordered,
            cr.collapsed.nodes.clone(),
            expanded.edges.clone(),
            overrides,
            dir.clone(),
        );
        let routed = route_edges(laid.clone(), expanded.edges, dir);
        let nodes = expand_compound_nodes(laid, cr.compounds);
        (nodes, routed)
    } else {
        let dag = remove_cycles(graph.clone());
        let layers = assign_layers(graph.nodes.clone(), dag.clone());
        let expanded = insert_dummies(layers, dag);
        let ordered = order_layers(expanded.nodes, expanded.edges.clone());
        let nodes = assign_coords(
            ordered,
            graph.nodes.clone(),
            expanded.edges.clone(),
            vec![],
            dir.clone(),
        );
        let routed = route_edges(nodes.clone(), expanded.edges, dir);
        (nodes, routed)
    }
}

// True if `id` is a compound (subgraph container) id.
fn is_compound(id: &str) -> bool {
    id.starts_with("__sg_")
}

fn render(graph: &Graph, ascii: bool) -> String {
    let (nodes, routed) = layout_pipeline(graph);

    // Canvas size: bounding box of every box plus every routed waypoint.
    let mut w: i32 = 1;
    let mut h: i32 = 1;
    for n in &nodes {
        if n.x + n.width > w {
            w = n.x + n.width;
        }
        if n.y + n.height > h {
            h = n.y + n.height;
        }
    }
    for e in &routed {
        for p in &e.waypoints {
            if p.x + 1 > w {
                w = p.x + 1;
            }
            if p.y + 1 > h {
                h = p.y + 1;
            }
        }
    }

    let cs = if ascii {
        charset_ascii()
    } else {
        charset_unicode()
    };

    let mut c = canvas_new(w, h);
    // Containers first (behind), then nodes on top.
    for n in &nodes {
        if is_compound(&n.id) {
            paint_compound(&mut c, cs.clone(), n.clone());
        }
    }
    for n in &nodes {
        if !is_compound(&n.id) {
            let shape = shape_of(graph, &n.id);
            let label = label_of(graph, &n.id);
            paint_node(&mut c, cs.clone(), n.clone(), shape, label);
        }
    }
    for e in routed {
        paint_edge(&mut c, cs.clone(), e);
    }

    // canvas_to_string already drops trailing blank rows, but its per-row right-trim
    // is byte/char-inconsistent for rows containing multi-byte box glyphs, so it can
    // leave trailing spaces. Right-trim each line here to match the golden format.
    let s = canvas_to_string(c);
    let mut out: String = s
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    if !out.is_empty() {
        out.push('\n');
    }
    out
}

// Pull a routed polyline's two endpoints in by one cell toward their neighbors. The
// layout's waypoints touch the source/target box borders; the SVG line should stop one
// cell short at each end (the gap the ASCII paint_edge leaves for stubs/arrowheads), so
// the arrowhead marker sits in the gap rather than under the box border.
fn trim_edge(mut e: RoutedEdge) -> RoutedEdge {
    // Target end: land exactly one cell before the box border. The ASCII renderer
    // always paints the arrowhead one cell shy of the border, so two edges entering
    // the same node share that cell and the arm-merge yields a single head. The SVG
    // must end on that same cell or its marker-end sits on the border, one cell off
    // from a sibling edge — the phantom double arrow. When the final segment is a
    // single cell, stepping in would land on the neighbour, so drop the border
    // waypoint instead (the neighbour already IS the one-before-border cell).
    let n = e.waypoints.len();
    if n >= 2 {
        let stepped = step_one(&e.waypoints[n - 1], &e.waypoints[n - 2]);
        if stepped == e.waypoints[n - 2] {
            e.waypoints.pop();
        } else {
            e.waypoints[n - 1] = stepped;
        }
    }
    // Source end: same, one cell out from the source border.
    let n = e.waypoints.len();
    if n >= 2 {
        let stepped = step_one(&e.waypoints[0], &e.waypoints[1]);
        if stepped == e.waypoints[1] {
            e.waypoints.remove(0);
        } else {
            e.waypoints[0] = stepped;
        }
    }
    e
}

// One orthogonal cell from `p` toward `toward`.
fn step_one(p: &Point, toward: &Point) -> Point {
    Point {
        x: p.x + (toward.x - p.x).signum(),
        y: p.y + (toward.y - p.y).signum(),
    }
}

// Full SVG render: lay the graph out (Sugiyama → coords → routing) into the shared
// graphIR, then emit one SVG document. Cell-grid coords map to pixels via the same
// transform the .hom svg_* helpers use (MARGIN=20, CELL_W=10, CELL_H=20); the document
// width/height pad the cell bounding box by one cell on every side before scaling.
// Edges are drawn before nodes so node boxes paint on top.
fn render_svg_doc(graph: &Graph) -> String {
    let (nodes, routed) = layout_pipeline(graph);

    // Cell bounding box of every box plus every routed waypoint (same as render()).
    let mut w: i32 = 1;
    let mut h: i32 = 1;
    for n in &nodes {
        if n.x + n.width > w {
            w = n.x + n.width;
        }
        if n.y + n.height > h {
            h = n.y + n.height;
        }
    }
    for e in &routed {
        for p in &e.waypoints {
            if p.x + 1 > w {
                w = p.x + 1;
            }
            if p.y + 1 > h {
                h = p.y + 1;
            }
        }
    }

    // Document size in pixels: pad the cell box by one cell each side, scale, then add
    // the 20px margin on each side (2*MARGIN). Reverse-engineered from the goldens.
    let pw = 2 * 20 + (w + 2) * 10;
    let ph = 2 * 20 + (h + 2) * 20;

    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{pw}\" height=\"{ph}\" viewBox=\"0 0 {pw} {ph}\">"
    ));
    lines.push(
        "<defs>\n  <marker id=\"arrowhead\" markerWidth=\"10\" markerHeight=\"7\" refX=\"10\" refY=\"3.5\" orient=\"auto\">\n    <polygon points=\"0 0, 10 3.5, 0 7\" fill=\"black\"/>\n  </marker>\n  <marker id=\"arrowhead-rev\" markerWidth=\"10\" markerHeight=\"7\" refX=\"0\" refY=\"3.5\" orient=\"auto\">\n    <polygon points=\"10 0, 0 3.5, 10 7\" fill=\"black\"/>\n  </marker>\n</defs>".to_string(),
    );
    lines.push(format!(
        "<rect width=\"{pw}\" height=\"{ph}\" fill=\"white\"/>"
    ));

    for e in routed {
        lines.push(svg_edge(trim_edge(e)));
    }
    // Containers behind, member/regular nodes on top.
    for n in &nodes {
        if is_compound(&n.id) {
            lines.push(svg_compound(n.clone(), n.id[5..].to_string()));
        }
    }
    for n in &nodes {
        if !is_compound(&n.id) {
            let shape = shape_of(graph, &n.id);
            let label = label_of(graph, &n.id);
            let s = svg_node(n.clone(), shape, label);
            if !s.is_empty() {
                lines.push(s);
            }
        }
    }
    lines.push("</svg>".to_string());

    lines.join("\n")
}

// Parse a direction override from the playground ("" = use the graph's own header).
#[cfg(feature = "wasm")]
fn dir_from_str(s: &str) -> Option<Direction> {
    match s {
        "TD" | "td" | "TB" | "tb" => Some(Direction::TD),
        "LR" | "lr" => Some(Direction::LR),
        _ => None,
    }
}

// WASM bindings consumed by _site/index.html (renderWithOptions / renderSvg).
// `padding` is accepted for forward-compat but not yet wired into the layout.
// js_name forces the camelCase export the playground imports — wasm-bindgen keeps
// free-function names snake_case by default.
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = renderWithOptions)]
pub fn render_with_options(text: &str, unicode: bool, _padding: i32, direction: &str) -> String {
    render_dsl(text, !unicode, dir_from_str(direction))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = renderSvg)]
pub fn render_svg(text: &str, _padding: i32, direction: &str) -> String {
    render_dsl_svg(text, dir_from_str(direction))
}
