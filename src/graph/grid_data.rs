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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_grid_data {
    use super::*;

    #[test]
    fn test_grid_data_new_all_false() {
        let d = grid_data_new(4, 3);
        for row in 0..3i32 {
            for col in 0..4i32 {
                assert!(!grid_data_get(&d, row, col, 4));
            }
        }
    }

    #[test]
    fn test_grid_data_new_zero_size() {
        let d = grid_data_new(0, 0);
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_grid_data_set_get() {
        let mut d = grid_data_new(5, 5);
        grid_data_set(&mut d, 2, 3, 5, true);
        assert!(grid_data_get(&d, 2, 3, 5));
        assert!(!grid_data_get(&d, 2, 4, 5));
        assert!(!grid_data_get(&d, 3, 3, 5));
    }

    #[test]
    fn test_grid_data_set_multiple_cells() {
        let mut d = grid_data_new(6, 4);
        grid_data_set(&mut d, 0, 0, 6, true);
        grid_data_set(&mut d, 1, 2, 6, true);
        grid_data_set(&mut d, 3, 5, 6, true);
        assert!(grid_data_get(&d, 0, 0, 6));
        assert!(grid_data_get(&d, 1, 2, 6));
        assert!(grid_data_get(&d, 3, 5, 6));
        assert!(!grid_data_get(&d, 0, 1, 6));
        assert!(!grid_data_get(&d, 2, 3, 6));
    }
}
