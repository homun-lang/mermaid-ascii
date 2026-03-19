// dep/path_state.rs — State primitives for A* pathfinding.
//
// With homunc's :: (mutable reference) parameter support, plain Vec types
// can be used directly — no Rc<RefCell<...>> wrapper needed.
//
// .hom modules import this via `use path_state` and use:
//
//   Position encoding (grid width required for encode/decode):
//     pos_to_key(x, y, width) -> i32        flat row-major index
//     key_to_x(key, width)    -> i32        column from flat index
//     key_to_y(key, width)    -> i32        row    from flat index
//     key_to_str(key)         -> String     for heap item storage
//     str_to_key(s)           -> i32        String (not &str) input
//
//   CostData — Vec<i32>, initialised to -1 (= unvisited):
//     cost_data_new(size)             -> CostData
//     cost_data_set(d, idx, val)      // takes &mut
//     cost_data_get(d, idx)           -> i32   (-1 if out-of-bounds)
//
//   PointList — Vec<(i32,i32)>, accumulates path points:
//     point_list_new()                -> PointList
//     point_list_push(pl, x, y)       // takes &mut
//     point_list_len(pl)              -> i32
//     point_list_get_x(pl, idx)       -> i32
//     point_list_get_y(pl, idx)       -> i32
//     point_list_copy(pl)             -> PointList   (independent copy)
//     point_list_reversed(pl)         -> PointList   (reversed copy)

// ── Position encoding ─────────────────────────────────────────────────────────

pub fn pos_to_key(x: i32, y: i32, width: i32) -> i32 {
    y * width + x
}

pub fn key_to_x(key: i32, width: i32) -> i32 {
    if width <= 0 { 0 } else { key % width }
}

pub fn key_to_y(key: i32, width: i32) -> i32 {
    if width <= 0 { 0 } else { key / width }
}

pub fn key_to_str(key: i32) -> String {
    key.to_string()
}

pub fn str_to_key(s: String) -> i32 {
    s.trim().parse::<i32>().unwrap_or(-1)
}

// ── CostData ─────────────────────────────────────────────────────────────────

pub type CostData = Vec<i32>;

pub fn cost_data_new(size: i32) -> CostData {
    let n = size.max(0) as usize;
    vec![-1i32; n]
}

pub fn cost_data_set(d: &mut CostData, idx: i32, val: i32) {
    if idx >= 0 {
        if let Some(slot) = d.get_mut(idx as usize) {
            *slot = val;
        }
    }
}

pub fn cost_data_get(d: &CostData, idx: i32) -> i32 {
    if idx < 0 {
        return -1;
    }
    d.get(idx as usize).copied().unwrap_or(-1)
}

// ── PointList ─────────────────────────────────────────────────────────────────

pub type PointList = Vec<(i32, i32)>;

pub fn point_list_new() -> PointList {
    Vec::new()
}

pub fn point_list_push(pl: &mut PointList, x: i32, y: i32) {
    pl.push((x, y));
}

pub fn point_list_len(pl: &PointList) -> i32 {
    pl.len() as i32
}

pub fn point_list_get_x(pl: &PointList, idx: i32) -> i32 {
    pl[idx as usize].0
}

pub fn point_list_get_y(pl: &PointList, idx: i32) -> i32 {
    pl[idx as usize].1
}

pub fn point_list_copy(pl: &PointList) -> PointList {
    pl.clone()
}

pub fn point_list_reversed(pl: &PointList) -> PointList {
    pl.iter().cloned().rev().collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests_path_state {
    use super::*;

    #[test]
    fn test_pos_encoding_roundtrip() {
        let width = 10;
        for y in 0..5i32 {
            for x in 0..10i32 {
                let key = pos_to_key(x, y, width);
                assert_eq!(key_to_x(key, width), x, "x mismatch at ({},{})", x, y);
                assert_eq!(key_to_y(key, width), y, "y mismatch at ({},{})", x, y);
            }
        }
    }

    #[test]
    fn test_pos_to_key_values() {
        assert_eq!(pos_to_key(0, 0, 5), 0);
        assert_eq!(pos_to_key(4, 0, 5), 4);
        assert_eq!(pos_to_key(0, 1, 5), 5);
        assert_eq!(pos_to_key(2, 3, 5), 17);
    }

    #[test]
    fn test_key_str_roundtrip() {
        for k in [0i32, 1, 42, 999, -1] {
            assert_eq!(str_to_key(key_to_str(k)), k);
        }
    }

    #[test]
    fn test_str_to_key_accepts_string() {
        assert_eq!(str_to_key(String::from("17")), 17);
        assert_eq!(str_to_key(String::from("-1")), -1);
        assert_eq!(str_to_key(String::from("bad")), -1);
    }

    #[test]
    fn test_cost_data_init_all_minus_one() {
        let d = cost_data_new(6);
        for i in 0..6i32 {
            assert_eq!(cost_data_get(&d, i), -1);
        }
    }

    #[test]
    fn test_cost_data_set_get() {
        let mut d = cost_data_new(10);
        cost_data_set(&mut d, 3, 7);
        assert_eq!(cost_data_get(&d, 3), 7);
        assert_eq!(cost_data_get(&d, 2), -1);
    }

    #[test]
    fn test_cost_data_oob_returns_minus_one() {
        let d = cost_data_new(4);
        assert_eq!(cost_data_get(&d, 10), -1);
        assert_eq!(cost_data_get(&d, -1), -1);
    }

    #[test]
    fn test_point_list_push_len() {
        let mut pl = point_list_new();
        assert_eq!(point_list_len(&pl), 0);
        point_list_push(&mut pl, 3, 4);
        assert_eq!(point_list_len(&pl), 1);
        point_list_push(&mut pl, 7, 2);
        assert_eq!(point_list_len(&pl), 2);
    }

    #[test]
    fn test_point_list_get() {
        let mut pl = point_list_new();
        point_list_push(&mut pl, 10, 20);
        point_list_push(&mut pl, 30, 40);
        assert_eq!(point_list_get_x(&pl, 0), 10);
        assert_eq!(point_list_get_y(&pl, 0), 20);
        assert_eq!(point_list_get_x(&pl, 1), 30);
        assert_eq!(point_list_get_y(&pl, 1), 40);
    }

    #[test]
    fn test_point_list_copy_independent() {
        let mut pl = point_list_new();
        point_list_push(&mut pl, 1, 2);
        let copy = point_list_copy(&pl);
        point_list_push(&mut pl, 3, 4);
        assert_eq!(point_list_len(&copy), 1);
    }

    #[test]
    fn test_point_list_reversed() {
        let mut pl = point_list_new();
        point_list_push(&mut pl, 1, 10);
        point_list_push(&mut pl, 2, 20);
        point_list_push(&mut pl, 3, 30);
        let rev = point_list_reversed(&pl);
        assert_eq!(point_list_get_x(&rev, 0), 3);
        assert_eq!(point_list_get_x(&rev, 1), 2);
        assert_eq!(point_list_get_x(&rev, 2), 1);
    }
}
