pub mod adjacent_pattern;
pub use adjacent_pattern::*;

pub mod cross_pattern;
pub use cross_pattern::*;

pub mod grid_pattern;
pub use grid_pattern::*;

pub mod surrounding_pattern;
pub use surrounding_pattern::*;

pub mod rectangle_pattern;
pub use rectangle_pattern::*;

use super::{Grid, Position};

pub trait PatternPositions {
    fn get_pattern_positions(&self, center: Position) -> Box<[Position]>;
}

pub fn get_grid_values_from_pattern<T>(grid: &Grid<T>, center: Position, pattern: &dyn PatternPositions) -> Box<[T]>
where
    T: Copy,
{
    pattern
        .get_pattern_positions(center)
        .iter()
        .filter_map(|&pos| grid.get(pos))
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
