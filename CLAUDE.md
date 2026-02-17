# CLAUDE.md — text-graph project instructions

## Project Overview
Rust rewrite of graph-easy: DSL text input → Parse → Graph layout → ASCII/Unicode text output.

## Autonomous Mode
This project runs with autonomous Claude agents. **Never ask the user for permission or clarification. Just work.**

## Development Cycle (CRITICAL — follow every time)

**Small steps → Verify → Commit → Refactor → Commit**

1. **Implement the smallest possible step** — one function, one struct, one module
2. **Verify it works** — `cargo check` minimum, `cargo test` or `cargo run` if applicable
3. **Git commit** — `git add -A && git commit -m "phase N: description" --no-verify`
4. **Refactor** if code smells — improve names, extract functions, simplify logic
5. **Verify again** — `cargo check` / `cargo test`
6. **Git commit the refactor** — `git add -A && git commit -m "refactor: description" --no-verify`

### Error Recovery
- If something breaks and can't be fixed in 3 attempts: `git reset --hard HEAD`
- If a whole approach is wrong: `git log --oneline -10` to find a good checkpoint, then `git reset --hard <hash>`

## Verification Approach
- Unit tests for Rust logic (`cargo test`)
- Visual output verified by generating examples: `bash examples/gen.sh`
- Human reviews `.out.txt` files in `examples/` to confirm rendering correctness
- Do NOT use snapshot tests for rendered output — ASCII art needs human eyes

## Key Files
- `src/` — Rust source code
  - `ast.rs` — AST types (Direction, Node, Edge, Subgraph, etc.)
  - `grammar.pest` — PEG grammar for the DSL
  - `parser.rs` — pest parser → AST
  - `graph.rs` — AST → petgraph IR (GraphIR)
  - `layout.rs` — Sugiyama layout (cycle removal, layers, crossing min, coordinates, edge routing)
  - `render.rs` — Canvas renderer (box-drawing, edge painting, junction merging)
  - `lib.rs` — Library API (`render_dsl`, `render_dsl_padded`)
  - `main.rs` — CLI entry point (clap)
- `examples/` — Example DSL files + gen script
  - `gen.sh` — Builds project and generates `.out.txt` for all examples
  - `*.txt` — Input DSL files
  - `*.out.txt` — Generated output (gitignored)
- `_ref/` — Cloned reference repos (gitignored)

## Tech Stack
- Parser: `pest` (PEG grammar)
- Graph: `petgraph`
- Layout: Sugiyama algorithm (custom implementation)
- CLI: `clap`
- Testing: unit tests (`cargo test`)

## Reference Repos (in `_ref/`)
- `mermaid-ascii` — Closest competitor, A* edge routing
- `beautiful-mermaid` — TS port of mermaid-ascii
- `ascii-dag` — Sugiyama in Rust, zero-dep
- `dagre` — Production Sugiyama algorithm (JS)
- `figurehead` — Rust Mermaid→ASCII
- `d2` — DSL syntax design reference
- `svgbob` — Character rendering techniques

## Pipeline
```
DSL text → pest parser → AST → GraphIR (petgraph) → Sugiyama layout → edge routing → canvas render → text output
```

## Current Feature Status
- [x] TD (top-down) layout — working
- [ ] LR (left-to-right) layout — parsed but NOT rendered (renders as TD)
- [ ] RL, BT — parsed but NOT rendered
- [x] Subgraphs with compound node layout
- [x] Edge labels
- [x] Multiple edge types (arrow, back-arrow, bidirectional, line, thick, dotted)
- [x] Node shapes (rectangle, rounded, diamond, circle)
- [x] Multi-line labels
- [x] ASCII/Unicode character sets
- [x] Barycenter refinement for aligned edges

## Code Style
- Rust 2024 edition
- Keep it simple — no over-engineering, no premature abstraction
- Prefer clear names over comments
- Each module should have a single clear responsibility
- Three similar lines > premature abstraction
