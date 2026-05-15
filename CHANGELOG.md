# Changelog

## v0.18 — 2026-05-15 — Drop low-level Rust wrappers

- Delete `src/graph/grid_data.rs`: `OccupancyGrid.data` switches to `@[bool]`, grid constructor inlined into `pathfinder.hom`
- Delete `src/graph/path_state.rs`: `pos_to_key` / `key_to_x` / `key_to_y` / `key_to_str` / `str_to_key` and `cost_data_new` ported into `pathfinder.hom`; `PointList` becomes `@[(int, int)]` inline
- Drop `IntList` / `EdgePairList` / `StrList` struct wrappers + their 16 helper functions from `layout_state.rs`
- `layout.hom` now uses native `@[int]` / `@[(str, str)]` / `@[str]` with `push` / `len` / index / index-assign; new `contains_pair` .hom helper replaces `edge_pair_list_contains`
- Net −552 / +197 lines of hand-written Rust across 7 files

## v0.17 — homunc v0.87 syntax upgrade

- Fix runtime deduplication in `build.rs`: key by function name (not full signature) so new `chars` module predicates (`is_alpha`, `is_digit`, `is_alnum`, `is_upper`) don't duplicate old stdlib versions
- Strip `#[cfg(test)] mod tests_*` blocks from generated `.hom` module files so companion `.rs` tests (grid_data, path_state) run once via the `graph` module, not twice with broken scope
- Add `clippy::unnecessary_mut_passed` allow to `lib.rs` for v0.87 `heap_is_empty(&Heap)` codegen pattern
- Requires homunc v0.87.0+

## v0.16 — Embedded Runtime + Examples

- Use `homunc --emit-runtime` instead of `src/hom` submodule — runtime is now embedded in the compiler
- Remove `hom-std` submodule dependency
- Simplify `build.rs`: no more manual file concatenation
- Move examples to `_site/examples/` for web playground
- Add example dropdown selector to playground (like Homun Playground)
- Update GitHub URL to `homun-lang/mermaid-ascii`
- Requires homunc v0.79.1+

## v0.15 — Kill `Rc<RefCell<>>`

- Eliminate all `Rc<RefCell<>>` wrapper types in `layout_state.rs`, replacing with plain structs and return-value mutation
- Convert 11 types: DegMap, NodeSet, StrList, EdgePairList, PosMap, FloatMap, IntList, EdgeInfoList, OrderingList, DummyEdgeList, MutableGraph
- Remove ~900 lines of boilerplate wrapper code
- Clean up dead code (`float_map_new`, `float_map_get_or_inf`)

## v0.14 — Layout IR Refactor

- Refactor layout pipeline into dedicated layout IR modules
- Introduce `::` mutable-reference param convention (following `pathfinder.hom` pattern)
- Add `.hom` source files for layout modules
- Remove duplicated code, clean up module structure

## v0.13 — Syntax Upgrade

- Upgrade all Homun source files to latest `homunc` syntax
- Adopt `::` namespace operator throughout codebase

## v0.12 — SVG Renderer

- Add real geometry-based SVG renderer (`render_svg_dsl`)
- Add `--svg` CLI flag to `main.rs`
- Update `gen.sh` to generate and verify SVG golden files

## v0.10 — Homun + Rust

- Restructure to Homun (.hom) + Rust architecture
- Full Sugiyama layout pipeline in hand-written Rust (`src/lib.rs`)
- Hand-written `graph/` module: petgraph wrapper, `Rc<RefCell<...>>` mutable state types
- Homun modules: types, config, canvas, charset, pathfinder, parser, layout
- `build.rs` compiles `.hom` → `.rs` via `homunc` at build time
- Add `#[wasm_bindgen]` exports (`render`, `renderWithOptions`, `renderSvg`)
- 35 tests passing

## v0.5 — SVG Renderer

- Add SVG output mode to playground (ASCII + SVG tabs)

## v0.4 — A* Edge Routing

- Port A* pathfinding edge routing from Python to Rust

## v0.3 — Full Rust Port + CI/CD

- Complete Python → Rust port (1:1 module map)
- Parser: recursive descent tokenizer + flowchart parser
- GraphIR: petgraph DiGraph wrapper with `from_ast()`
- Sugiyama layout engine: cycle removal, layering, crossing minimization, coordinates, routing
- ASCII renderer: shape-aware box drawing, edge painting, direction transforms
- API + CLI: `render_dsl()`, clap with `--ascii`, `--direction`, `--padding`, `--output`
- E2E tests: Python pytest against Rust binary (golden file comparison)
- CI/CD: cross-platform binaries (linux x86_64/aarch64, windows) + WASM tarball
- GitHub Pages playground with interactive WASM demo

## v0.2 — Python Package + PyPI

- CI/CD: GitHub Actions for test, build, release
- Dockerfile multi-stage build, PyPI publishing

## v0.1 — Python Implementation

- Recursive descent parser, GraphIR (networkx), Sugiyama layout, ASCII/Unicode renderer
- `render_dsl()` public API, 232 Python tests
