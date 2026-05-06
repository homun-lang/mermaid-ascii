use crate::graph::*;

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
