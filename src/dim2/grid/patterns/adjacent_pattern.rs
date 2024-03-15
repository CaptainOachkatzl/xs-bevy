use std::sync::Arc;

use super::{
    cross_pattern::{cross_pattern, new_cross_pattern},
    grid_pattern::GridPattern,
};

pub fn adjacent_pattern() -> Arc<GridPattern> {
    cross_pattern(1)
}

pub fn new_adjacent_pattern() -> GridPattern {
    new_cross_pattern(1)
}
