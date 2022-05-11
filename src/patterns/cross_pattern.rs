use std::sync::{Once};

use crate::{*, patterns::factory_cache::FactoryCache};

use super::*;

pub fn cross_pattern(arm_length: usize) -> &'static GridPattern {
  static INIT: Once = Once::new();
  static mut PATTERN_CACHE: Option<FactoryCache<usize, GridPattern>> = None;

  unsafe {
    // initialize hash map if its not
    INIT.call_once(|| {
      let factory_fn = Box::new(|x| new_cross_pattern(x));
      PATTERN_CACHE = Some(FactoryCache::new(factory_fn));
    });

    PATTERN_CACHE.as_mut().unwrap().get(arm_length)
  }
}

pub fn new_cross_pattern(arm_length: usize) -> GridPattern {
  let center = Position::new(arm_length as i64, arm_length as i64);

  let grid_size = 2*arm_length + 1;
  let grid_values: Vec<_> = (0..grid_size * grid_size).map(|_| false).collect();
  let mut mapping = Grid::new(grid_size, grid_size, grid_values.into_boxed_slice());
  for (pos, matches) in mapping.iter_mut_with_position() {
    if pos.x == center.x || pos.y == center.y {
      *matches = true;
    }
  }
  GridPattern { mapping, center }
}
