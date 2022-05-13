use std::{collections::BTreeMap, lazy::SyncLazy, sync::Arc};

use crate::*;

use super::*;

pub fn cross_pattern(arm_length: usize) -> Arc<GridPattern> {
  static mut PATTERN_CACHE: SyncLazy<FactoryCache<usize, GridPattern, BTreeMap<usize, Arc<GridPattern>>>> = SyncLazy::new(|| {
    let factory_fn: Box<dyn Fn(&usize) -> GridPattern> = Box::new(|x| new_cross_pattern(*x));
    FactoryCache::new(BTreeMap::new(), factory_fn)
  });

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
