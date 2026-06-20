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
    let laid_out = layout(&graph);
    render(&laid_out, ascii)
}

fn layout(graph: &Graph) -> Graph {
    graph.clone()
}

fn render(_graph: &Graph, _ascii: bool) -> String {
    String::new()
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_wasm(text: &str, ascii: bool) -> String {
    render_dsl(text, ascii, None)
}
