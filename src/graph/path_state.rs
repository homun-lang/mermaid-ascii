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

