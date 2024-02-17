use std::{cell::OnceCell, collections::BTreeMap, sync::Arc};

use crate::{
    factory_cache::FactoryCache,
    two_dimendions::grid::{Grid, Position},
};

use super::grid_pattern::GridPattern;

pub fn cross_pattern(arm_length: usize) -> Arc<GridPattern> {
    static mut PATTERN_CACHE: OnceCell<FactoryCache<usize, GridPattern, BTreeMap<usize, Arc<GridPattern>>>> = OnceCell::new();
    let cache = unsafe { PATTERN_CACHE.get_or_init(|| FactoryCache::new(BTreeMap::new(), Box::new(new_cross_pattern))) };
    cache.get(arm_length)
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
