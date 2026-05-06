use crate::graph::*;

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
