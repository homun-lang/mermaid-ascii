/// Integration tests for edge case graphs — Phase 7.
///
/// Tests use `insta` snapshot testing to lock down expected output.
/// Run with: cargo test
/// Update snapshots: cargo insta review  (or INSTA_UPDATE=always cargo test)

#[cfg(test)]
mod edge_case_tests {
    use crate::{graph, layout, parser, render};

    fn render_dsl(src: &str, unicode: bool) -> String {
        let ast_graph = parser::parse(src).expect("parse failed");
        let gir = graph::GraphIR::from_ast(&ast_graph);
        if gir.node_count() == 0 {
            return String::new();
        }
        let (layout_nodes, routed_edges) = layout::full_layout(&gir);
        render::render(&gir, &layout_nodes, &routed_edges, unicode)
    }

    // ─── Empty graph ──────────────────────────────────────────────────────────

    #[test]
    fn test_empty_graph_produces_no_output() {
        let src = "# Empty graph — only a comment\n";
        let ast_graph = parser::parse(src).expect("parse failed");
        let gir = graph::GraphIR::from_ast(&ast_graph);
        assert_eq!(gir.node_count(), 0);
        assert_eq!(gir.edge_count(), 0);
    }

    // ─── Single node ─────────────────────────────────────────────────────────

    #[test]
    fn test_single_node_unicode() {
        let result = render_dsl("[Alone]\n", true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_single_node_ascii() {
        let result = render_dsl("[Alone]\n", false);
        insta::assert_snapshot!(result);
    }

    // ─── Self-loop ────────────────────────────────────────────────────────────

    #[test]
    fn test_self_loop_renders_as_single_node() {
        // Self-loops are removed during cycle removal; the node still renders.
        let src = "[Loop] --> [Loop]\n";
        let ast_graph = parser::parse(src).expect("parse failed");
        let gir = graph::GraphIR::from_ast(&ast_graph);
        assert_eq!(gir.node_count(), 1, "self-loop should produce exactly 1 node");
        assert_eq!(gir.edge_count(), 1, "self-loop should produce exactly 1 edge");

        let result = render_dsl(src, true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_self_loop_ascii() {
        let result = render_dsl("[Loop] --> [Loop]\n", false);
        insta::assert_snapshot!(result);
    }

    // ─── Very long labels ─────────────────────────────────────────────────────

    #[test]
    fn test_long_label_unicode() {
        let src = "[This is a very long node label that spans many characters] --> [Short]\n";
        let result = render_dsl(src, true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_long_label_chain() {
        let src = concat!(
            "[This is a very long node label that spans many characters] --> [Another quite lengthy label here]\n",
            "[Another quite lengthy label here] --> [Short]\n",
        );
        let result = render_dsl(src, true);
        insta::assert_snapshot!(result);
    }

    // ─── Additional edge cases ────────────────────────────────────────────────

    #[test]
    fn test_disconnected_components() {
        // Two independent nodes with no edge between them.
        let src = "[A]\n[B]\n";
        let ast_graph = parser::parse(src).expect("parse failed");
        let gir = graph::GraphIR::from_ast(&ast_graph);
        assert_eq!(gir.node_count(), 2);
        assert_eq!(gir.edge_count(), 0);
        let result = render_dsl(src, true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_minimal_two_node_chain() {
        let result = render_dsl("[A] --> [B]\n", true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_two_node_chain_ascii() {
        let result = render_dsl("[A] --> [B]\n", false);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_cycle_two_nodes() {
        // A → B → A is a cycle; cycle removal should handle it.
        let src = "[A] --> [B]\n[B] --> [A]\n";
        let ast_graph = parser::parse(src).expect("parse failed");
        let gir = graph::GraphIR::from_ast(&ast_graph);
        assert_eq!(gir.node_count(), 2);
        assert_eq!(gir.edge_count(), 2);
        let result = render_dsl(src, true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_single_node_rounded() {
        let result = render_dsl("(Rounded Node)\n", true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_direction_lr() {
        let result = render_dsl("direction: LR\n[A] --> [B] --> [C]\n", true);
        insta::assert_snapshot!(result);
    }

    #[test]
    fn test_labeled_edge() {
        let result = render_dsl("[A] --> [B] { label: \"connects\" }\n", true);
        insta::assert_snapshot!(result);
    }
}
