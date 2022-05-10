use std::mem::{self, MaybeUninit};

use crate::{Grid, Position, Size2D};

pub struct GridIter<'a, T>
where
  T: Copy,
{
  grid: &'a Grid<T>,
  next_position: Position,
}

impl<'a, T> GridIter<'a, T>
where
  T: Copy,
{
  pub fn new(grid: &'a Grid<T>) -> Self {
    GridIter {
      grid,
      next_position: Position { x: 0, y: 0 },
    }
  }
}

impl<'a, T> Iterator for GridIter<'a, T>
where
  T: Copy,
{
  type Item = (Position, &'a T);

  fn next(&mut self) -> Option<Self::Item> {
    let current_position = set_to_next_position(&mut self.next_position, self.grid.get_size())?;
    Some((current_position, self.grid.get_value_by_position(current_position).unwrap()))
  }
}

pub struct GridIntoIter<T>
where
  T: Copy,
{
  grid: Grid<T>,
  next_position: Position,
}

impl<'a, T> GridIntoIter<T>
where
  T: Copy,
{
  pub fn new(grid: Grid<T>) -> Self {
    GridIntoIter {
      grid,
      next_position: Position { x: 0, y: 0 },
    }
  }
}

impl<T> Iterator for GridIntoIter<T>
where
  T: Copy,
{
  type Item = (Position, T);

  fn next(&mut self) -> Option<Self::Item> {
    let current_position = set_to_next_position(&mut self.next_position, self.grid.get_size())?;
    Some((
      current_position,
      mem::replace(&mut self.grid[current_position], unsafe {
        MaybeUninit::<T>::uninit().assume_init()
      }),
    ))
  }
}

// increment to next position
// returns None if there is no next position
// returns current position if there is a next position
fn set_to_next_position(position: &mut Position, grid_size: &Size2D) -> Option<Position> {
  if position.y as usize >= grid_size.height {
    return None;
  }

  let current_position = position.clone();

  if position.x as usize >= grid_size.width - 1 {
    position.y += 1;
    position.x = 0;
  } else {
    position.x += 1;
  }

  return Some(current_position);
}
