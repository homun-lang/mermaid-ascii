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
