#[allow(
    dead_code,
    unused_imports,
    unused_macros,
    unused_mut,
    clippy::assign_op_pattern,
    clippy::clone_on_copy,
    clippy::manual_map,
    clippy::needless_return,
    clippy::no_effect,
    clippy::ptr_arg,
    clippy::redundant_closure,
    clippy::redundant_field_names,
    clippy::unnecessary_to_owned,
    clippy::unused_unit,
    clippy::useless_conversion
)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/runtime.rs"));
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
    include!(concat!(env!("OUT_DIR"), "/lexer.rs"));
}
pub use generated::{Direction, Edge, EdgeType, Graph, Node, NodeShape, Subgraph};
pub use generated::{Token, TokenKind, tokenize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub fn render_dsl(text: &str, ascii: bool, direction: Option<Direction>) -> String {
    let graph = parse(text, direction);
    let laid_out = layout(&graph);
    render(&laid_out, ascii)
}

fn parse(text: &str, dir_override: Option<Direction>) -> Graph {
    let _ = text;
    let direction = dir_override.unwrap_or(Direction::TD);
    Graph {
        nodes: vec![],
        edges: vec![],
        subgraphs: vec![],
        direction,
    }
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
