use super::*;

pub fn adjacent_pattern() -> &'static GridPattern {
  return cross_pattern(1);
}

pub fn new_adjacent_pattern() -> GridPattern {
  return new_cross_pattern(1);
}
