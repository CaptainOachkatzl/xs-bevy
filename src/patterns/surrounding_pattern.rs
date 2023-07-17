use std::sync::Arc;

use super::*;

pub fn surrounding_pattern(radius: usize) -> Arc<GridPattern> {
    rectangle_pattern(radius, radius, radius, radius)
}

pub fn new_surrounding_pattern(radius: usize) -> GridPattern {
    new_rectangle_pattern(radius, radius, radius, radius)
}
