pub mod graph;

#[allow(
    dead_code,
    unused_imports,
    unused_macros,
    unused_mut,
    unused_variables,
    clippy::assign_op_pattern,
    clippy::clone_on_copy,
    clippy::manual_map,
    clippy::needless_return,
    clippy::no_effect,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::redundant_field_names,
    clippy::single_match,
    clippy::too_many_arguments,
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
    include!(concat!(env!("OUT_DIR"), "/pathfinder.rs"));
    include!(concat!(env!("OUT_DIR"), "/canvas.rs"));
    include!(concat!(env!("OUT_DIR"), "/render_ascii.rs"));
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
fn render(graph: &Graph, ascii: bool) -> String {
    let dir = graph.direction.clone();

    // Layout pipeline → graphIR (LayoutNode[] + RoutedEdge[]).
    let dag = remove_cycles(graph.clone());
    let layers = assign_layers(graph.nodes.clone(), dag.clone());
    let expanded = insert_dummies(layers, dag);
    let ordered = order_layers(expanded.nodes, expanded.edges.clone());
    let nodes = assign_coords(ordered, graph.nodes.clone(), dir.clone());
    let routed = route_edges(nodes.clone(), expanded.edges, dir);

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
    for n in &nodes {
        let shape = shape_of(graph, &n.id);
        let label = label_of(graph, &n.id);
        paint_node(&mut c, cs.clone(), n.clone(), shape, label);
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

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_wasm(text: &str, ascii: bool) -> String {
    render_dsl(text, ascii, None)
}
