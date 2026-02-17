# text-graph

A Rust CLI that renders directed graphs as ASCII/Unicode art from a simple DSL.

```
echo '[A] --> [B] --> [C]' | text-graph

┌───┐
│ A │
└─┼─┘
  │
  │
  │
┌─▼─┐
│ B │
└─┼─┘
  │
  │
  │
┌─▼─┐
│ C │
└───┘
```

## Install

```sh
cargo install --path .
```

## Usage

```
text-graph [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file (reads from stdin if omitted)

Options:
  -a, --ascii            Use plain ASCII characters instead of Unicode
  -d, --direction <DIR>  Override graph direction (LR, RL, TD, BT)
  -p, --padding <N>      Node padding [default: 1]
  -o, --output <FILE>    Write output to file instead of stdout
```

Read from file:

```sh
text-graph examples/diamond.txt
```

Pipe from stdin:

```sh
echo '[A] --> [B]' | text-graph
```

ASCII mode:

```
echo '[A] --> [B] --> [C]' | text-graph --ascii

+---+
| A |
+-+-+
  |
  |
  |
+-v-+
| B |
+-+-+
  |
  |
  |
+-v-+
| C |
+---+
```

## DSL Syntax

### Edges

```
[A] --> [B]       # directed arrow
[A] -- [B]        # undirected line
[A] <-- [B]       # back arrow (arrow points to A)
[A] <--> [B]      # bidirectional
[A] ==> [B]       # thick arrow
[A] ..> [B]       # dotted arrow
[A] --> [B] --> [C]   # chained edges
```

### Edge labels

```
[Login] --> [Dashboard] { label: "success" }
[Login] --> [Error] { label: "failed" }
```

### Node shapes

```
[Rectangle]       # square brackets
(Rounded)         # parentheses
{Diamond}         # curly braces
((Circle))        # double parens
```

### Multi-line labels

```
["Line 1\nLine 2"]   # use \n for newlines
```

### Direction

```
direction: TD     # top-down (default)
direction: LR     # left-to-right
direction: BT     # bottom-to-top
direction: RL     # right-to-left
```

### Subgraphs

```
subgraph "Backend" {
  [API] --> [DB]
}

subgraph "Empty Group" {
  desc: "description text shown inside"
}
```

### Comments

```
# This is a comment
// This is also a comment
```

## Examples

### Simple pipeline

```
cat <<'EOF' | text-graph
[Start] --> [Build]
[Build] --> [Test]
[Test] --> [Deploy]
[Build] --> [Lint]
[Lint] --> [Deploy]
EOF

      ┌───────┐
      │ Start │
      └───┼───┘
          │
          │
          │
      ┌───▼───┐
      │ Build │
      └───┼───┘
          │
    ┼─────┼─────┼
    │           │
┌───▼──┐    ┌───▼──┐
│ Lint │    │ Test │
└───┼──┘    └───┼──┘
    │           │
    ┼─────┼─────┼
          │
     ┌────▼───┐
     │ Deploy │
     └────────┘
```

### Architecture diagram with subgraphs

```
text-graph examples/sysarch.txt

     ┌────────────────────────────────────────────────────────┐
     │                   Svelte + Tailwind                    │
     │ ┌───────────┐ ┌──────────┐ ┌────────────┐ ┌──────────┐ │
     │ │ Grid View │ │ Timeline │ │ Board View │ │ LLM Chat │ │
     │ └───────────┘ └──────────┘ └────────────┘ └──────────┘ │
     └────────────────────────────┼───────────────────────────┘
                                  HTTP
                                  │
                                  │
                     ┌────────────▼───────────┐
                     │   FastAPI + SQLModel   │
                     └────────────┼───────────┘
                                  │
                ┼─────────────────┼──────────────────┼
                │                 │                  │
         ┌──────▼─────┐    ┌──────▼─────┐    ┌───────▼──────┐
         │ PostgreSQL │    │ Claude API │    │    Minio     │
         └──────▲─────┘    │  tool_use  │    │ (blob store) │
                │          └────────────┘    └──────────────┘
                │                 writes
                ┼─────────────────┼
                                  │
┌─────────────────────────────────┼─────────────────────────────────┐
│                   Git Sync Worker (background)                    │
│   git fetch -> parse branches/tags -> openapi plugin -> sync DB   │
└───────────────────────────────────────────────────────────────────┘
```

Generate all example outputs:

```sh
bash examples/gen.sh
```

## Architecture

Pipeline: **DSL text** → **pest parser** → **AST** → **petgraph IR** → **Sugiyama layout** → **edge routing** → **canvas render** → **text output**

- Parser: [pest](https://pest.rs/) PEG grammar
- Graph: [petgraph](https://docs.rs/petgraph/) directed graph
- Layout: Sugiyama algorithm (cycle removal, layer assignment, crossing minimization, coordinate assignment with barycenter refinement)
- Rendering: 2D character canvas with Unicode box-drawing character merging

## License

MIT
