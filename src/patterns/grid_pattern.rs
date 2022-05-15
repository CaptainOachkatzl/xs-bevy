use crate::*;

pub trait PatternPositions {
  fn get_pattern_positions(&self, center: Position) -> Box<[Position]>;
}

pub struct GridPattern {
  pub mapping: Grid<bool>,
  pub center: Position,
}

impl PatternPositions for GridPattern {
  fn get_pattern_positions(&self, center: Position) -> Box<[Position]> {
    let mut result = vec![];

    let offset = center - self.center;

    for (pos, &matches) in self.mapping.iter_with_position() {
      if matches {
        result.push(pos + offset);
      }
    }

    return result.into_boxed_slice();
  }
}

pub fn get_grid_values_from_pattern<T>(grid: &Grid<T>, center: Position, pattern: &dyn PatternPositions) -> Box<[T]>
where
  T: Copy,
{
  pattern
    .get_pattern_positions(center)
    .iter()
    .filter_map(|&pos| grid.get_value_by_position(pos))
    .collect::<Vec<_>>()
    .into_boxed_slice()
}
