use std::sync::Arc;

use super::*;

pub fn surrounding_pattern(radius: usize) -> Arc<GridPattern> {
    return rectangle_pattern(radius, radius, radius, radius);
}

pub fn new_surrounding_pattern(radius: usize) -> GridPattern {
    return new_rectangle_pattern(radius, radius, radius, radius);
}
