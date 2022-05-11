use std::{
  collections::HashMap,
  sync::{Mutex, Once},
};

use crate::*;

use super::*;

pub fn cross_pattern(size: usize) -> &'static GridPattern {
  assert!(size % 2 == 1);

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
    if cross_patterns.contains_key(&size) {
      return &cross_patterns[&size];
    }

    {
      // check with lock if the key is still missing
      let _guard = PATTERN_LOCK.as_ref().unwrap().lock().expect("poisoned mutex");
      if !cross_patterns.contains_key(&size) {
        // insert in locked state
        cross_patterns.insert(size, new_cross_pattern(size));
      }
      // can leave the lock here because entry is initialized and other thread can just read value
    }

    cross_patterns.get(&size).unwrap()
  }
}

pub fn new_cross_pattern(size: usize) -> GridPattern {
  assert!(size % 2 == 1);

  let center = Position::new(size as i64 / 2, size as i64 / 2);

  let grid_values: Vec<_> = (0..size * size).map(|_| false).collect();
  let mut mapping = Grid::new(size, size, grid_values.into_boxed_slice());
  for (pos, matches) in mapping.iter_mut_with_position() {
    if pos.x == center.x || pos.y == center.y {
      *matches = true;
    }
  }
  GridPattern { mapping, center }
}
