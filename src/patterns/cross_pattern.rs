use std::{
  collections::HashMap,
  sync::{Mutex, Once},
};

use crate::*;

use super::*;

pub fn cross_pattern(arm_length: usize) -> &'static GridPattern {
  static INIT: Once = Once::new();
  static mut CROSS_PATTERNS: Option<HashMap<usize, GridPattern>> = None;
  static mut PATTERN_LOCK: Option<Mutex<()>> = None;

  unsafe {
    // initialize hash map if its not
    INIT.call_once(|| {
      PATTERN_LOCK = Some(Mutex::new(()));
      CROSS_PATTERNS = Some(HashMap::new());
    });

    let cross_patterns = CROSS_PATTERNS.as_mut().unwrap();
    // early check without mutex
    if cross_patterns.contains_key(&arm_length) {
      return &cross_patterns[&arm_length];
    }

    {
      // check with lock if the key is still missing
      let _guard = PATTERN_LOCK.as_ref().unwrap().lock().expect("poisoned mutex");
      if !cross_patterns.contains_key(&arm_length) {
        // insert in locked state
        cross_patterns.insert(arm_length, new_cross_pattern(arm_length));
      }
      // can leave the lock here because entry is initialized and other thread can just read value
    }

    cross_patterns.get(&arm_length).unwrap()
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
