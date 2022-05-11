use lazy_static::lazy_static;

use crate::*;

use super::*;

pub fn surrounding_pattern() -> &'static GridPattern {
  &SURROUNDING_PATTERN
}

lazy_static! {
  static ref SURROUNDING_PATTERN: GridPattern = new_surrounding_pattern();
}

fn new_surrounding_pattern() -> GridPattern {
  let mut mapping = [true; 9];
  mapping[4] = false;
  GridPattern {
    mapping: Grid::new(3, 3, Box::new(mapping)),
    center: Position::new(1, 1),
  }
}
