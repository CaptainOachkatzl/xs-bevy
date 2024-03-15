pub fn len(x: f32, y: f32) -> f32 {
    f32::sqrt(len_squared(x, y))
}

/// this takes significantly less computing power than `vec_len` and is thus preferable
/// for e.g. length comparisons where the exact length does not matter.
pub fn len_squared(x: f32, y: f32) -> f32 {
    x * x + y * y
}

pub fn rotate_90_degrees_left(x: f32, y: f32) -> (f32, f32) {
    (-y, x)
}

pub fn rotate_90_degrees_right(x: f32, y: f32) -> (f32, f32) {
    (y, -x)
}

#[test]
fn test_rotation_left() {
    // up
    let x = 0.;
    let y = 1.;
    // left
    let expect_x = -1.;
    let expect_y = 0.;
    let (res_x, res_y) = rotate_90_degrees_left(x, y);

    assert_eq!(res_x, expect_x);
    assert_eq!(res_y, expect_y)
}

#[test]
fn test_rotation_right() {
    // up
    let x = 0.;
    let y = 1.;
    // right
    let expect_x = 1.;
    let expect_y = 0.;
    let (res_x, res_y) = rotate_90_degrees_right(x, y);

    assert_eq!(res_x, expect_x);
    assert_eq!(res_y, expect_y)
}
