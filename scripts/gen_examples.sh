#!/usr/bin/env bash
# Generate output for all examples in examples/ directory.
# Usage: ./scripts/gen_examples.sh
#
# Builds the project first, then renders each .txt example
# and writes the output alongside the input as .out.txt.

set -euo pipefail
cd "$(dirname "$0")/.."

echo "Building..."
cargo build --release --quiet

BIN=target/release/text-graph

# Remove old generated files
rm -f examples/*.out.txt

for src in examples/*.txt; do
    # Skip output files and directories
    [[ "$src" == *.out.txt ]] && continue
    [ -d "$src" ] && continue
    name=$(basename "$src" .txt)
    out="examples/${name}.out.txt"
    echo "  $src -> $out"
    "$BIN" "$src" -o "$out"
done

echo ""
echo "Done. Generated output:"
for f in examples/*.out.txt; do
    echo "  $f"
done
