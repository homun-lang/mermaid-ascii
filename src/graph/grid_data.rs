// dep/grid_data.rs — Flat boolean grid for A* pathfinding.
//
// With homunc's :: (mutable reference) parameter support, plain Vec<bool>
// can be used directly — no Rc<RefCell<...>> wrapper needed.  Functions
// that mutate take &mut Vec<bool>; read-only functions also take &mut
// (matching .hom's :: convention) to avoid expensive full-Vec clones.
//
// The grid is stored as a flat row-major array:
//   index = row * width + col
//
// .hom modules use :: params and direct array indexing instead of calling
// these helpers.  These functions remain for Rust-side callers (lib.rs)
// and tests.

// ── GridData ──────────────────────────────────────────────────────────────────

/// A flat boolean grid.  Use as the `data` field inside OccupancyGrid.
pub type GridData = Vec<bool>;

/// Create a new GridData of size (width × height), all cells initialised to
/// `false` (i.e. free).
pub fn grid_data_new(width: i32, height: i32) -> GridData {
    let n = (width * height).max(0) as usize;
    vec![false; n]
}

/// Set the cell at (col, row) in a flat row-major grid of the given `width`.
pub fn grid_data_set(data: &mut GridData, row: i32, col: i32, width: i32, val: bool) {
    let idx = (row * width + col) as usize;
    data[idx] = val;
}

/// Get the value of the cell at (col, row) in a flat row-major grid.
pub fn grid_data_get(data: &GridData, row: i32, col: i32, width: i32) -> bool {
    let idx = (row * width + col) as usize;
    data[idx]
}

