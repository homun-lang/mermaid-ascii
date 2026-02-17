/// Integration snapshot tests for text-graph phase 5 and 6 examples.
///
/// Run `cargo test` to execute.  On first run, snapshots are written to
/// `tests/snapshots/`.  Use `cargo insta review` or `INSTA_UPDATE=always
/// cargo test` to update snapshots when intentional output changes occur.
use text_graph::render_dsl;
use text_graph::render_dsl_padded;

// ─── Phase 5 examples ───────────────────────────────────────────────────────

#[test]
fn snapshot_simple_chain() {
    let src = "[A] --> [B] --> [C]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("simple_chain", out);
}

#[test]
fn snapshot_diamond() {
    let src = "[A] --> [B]\n[A] --> [C]\n[B] --> [D]\n[C] --> [D]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("diamond", out);
}

#[test]
fn snapshot_labeled_edges() {
    let src = "[A] --> [B] { label: \"step 1\" }\n[B] --> [C] { label: \"step 2\" }\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("labeled_edges", out);
}

#[test]
fn snapshot_subgraph() {
    let src = "subgraph \"Group\" {\n  [A] --> [B]\n}\n[B] --> [C]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("subgraph", out);
}

#[test]
fn snapshot_td_layout() {
    let src = "direction: TD\n[Start] --> [Middle] --> [End]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("td_layout", out);
}

#[test]
fn snapshot_lr_layout() {
    let src = "direction: LR\n[Start] --> [Middle] --> [End]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("lr_layout", out);
}

#[test]
fn snapshot_ascii_mode() {
    let src = "[A] --> [B] --> [C]\n[A] --> [C]\n";
    let out = render_dsl(src, false).expect("render failed");
    insta::assert_snapshot!("ascii_mode", out);
}

// ─── Phase 6 examples (CLI feature parity) ───────────────────────────────────

#[test]
fn snapshot_padding3() {
    let src = "[A] --> [B] --> [C]\n";
    let out = render_dsl_padded(src, true, 3).expect("render failed");
    insta::assert_snapshot!("padding3", out);
}

#[test]
fn snapshot_empty_graph() {
    let src = "";
    let out = render_dsl(src, true).expect("render failed");
    // Empty graph renders as empty string.
    insta::assert_snapshot!("empty_graph", out);
}

#[test]
fn snapshot_single_node() {
    let src = "[Solo]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("single_node", out);
}

#[test]
fn snapshot_complex_branch() {
    // 6-node graph with multiple branches — tests crossing minimisation.
    let src = "[A] --> [B]\n[A] --> [C]\n[B] --> [D]\n[C] --> [E]\n[D] --> [F]\n[E] --> [F]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("complex_branch", out);
}

#[test]
fn snapshot_long_labels() {
    let src = "[Very Long Label A] --> [Very Long Label B]\n";
    let out = render_dsl(src, true).expect("render failed");
    insta::assert_snapshot!("long_labels", out);
}
