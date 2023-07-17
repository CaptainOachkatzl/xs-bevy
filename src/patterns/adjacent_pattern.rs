use std::sync::Arc;

use super::*;

pub fn adjacent_pattern() -> Arc<GridPattern> {
    cross_pattern(1)
}

pub fn new_adjacent_pattern() -> GridPattern {
    new_cross_pattern(1)
}
