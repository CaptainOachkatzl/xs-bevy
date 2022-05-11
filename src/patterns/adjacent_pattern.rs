use lazy_static::lazy_static;

use crate::*;

use super::*;

pub fn adjacent_pattern() -> &'static GridPattern {
  return &ADJACENT_PATTERN;
}

lazy_static! {
  pub static ref ADJACENT_PATTERN: GridPattern = new_adjacent_pattern();
}

fn new_adjacent_pattern() -> GridPattern {
  let mut mapping = [false; 9];
  for iter in mapping.iter_mut().enumerate() {
    if iter.0 % 2 == 1 {
      *iter.1 = true;
    }
  }
  GridPattern {
    mapping: Grid::new(3, 3, Box::new(mapping)),
    center: Position::new(1, 1),
  }
}
