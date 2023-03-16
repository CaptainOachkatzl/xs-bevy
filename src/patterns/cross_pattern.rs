use std::{collections::BTreeMap, sync::Arc};

use once_cell::sync::Lazy;

use crate::*;

use super::*;

pub fn cross_pattern(arm_length: usize) -> Arc<GridPattern> {
    static mut PATTERN_CACHE: Lazy<FactoryCache<usize, GridPattern, BTreeMap<usize, Arc<GridPattern>>>> =
        Lazy::new(|| FactoryCache::new(BTreeMap::new(), Box::new(new_cross_pattern)));

    unsafe { PATTERN_CACHE.get(arm_length) }
}

pub fn new_cross_pattern(arm_length: usize) -> GridPattern {
    let center = Position::new(arm_length as i64, arm_length as i64);

    let grid_size = 2 * arm_length + 1;
    let grid_values: Vec<_> = (0..grid_size * grid_size).map(|_| false).collect();
    let mut mapping = Grid::new(grid_size, grid_size, grid_values.into_boxed_slice());
    for (pos, matches) in mapping.iter_mut_with_position() {
        if pos != center && pos.x == center.x || pos.y == center.y {
            *matches = true;
        }
    }
    GridPattern { mapping, center }
}
