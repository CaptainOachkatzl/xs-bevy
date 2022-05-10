use std::mem::{self, MaybeUninit};

use crate::{index_translation::to_index, Position, Size2D};

pub struct GridIter<'a, T>
where
  T: Copy,
{
  values: &'a [T],
  size: Size2D,
  next_position: Position,
}

impl<'a, T> GridIter<'a, T>
where
  T: Copy,
{
  pub fn new(values: &'a [T], size: Size2D) -> Self {
    GridIter {
      values,
      size,
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
    let current_position = set_to_next_position(&mut self.next_position, self.size)?;
    Some((current_position, &self.values[to_index(current_position, self.size)]))
  }
}

pub struct GridIterMut<'a, T>
where
  T: Copy,
{
  values: &'a mut [T],
  size: Size2D,
  next_position: Position,
}

impl<'a, T> GridIterMut<'a, T>
where
  T: Copy,
{
  pub fn new(values: &'a mut [T], size: Size2D) -> Self {
    GridIterMut {
      values,
      size,
      next_position: Position { x: 0, y: 0 },
    }
  }
}

impl<'a, T> Iterator for GridIterMut<'a, T>
where
  T: Copy,
{
  type Item = (Position, &'a mut T);

  fn next(&mut self) -> Option<Self::Item> {
    let current_position = set_to_next_position(&mut self.next_position, self.size)?;
    let val: *mut T = &mut self.values[to_index(current_position, self.size)];
    Some((current_position, unsafe { val.as_mut().unwrap() }))
  }
}

pub struct GridIntoIter<T>
where
  T: Copy,
{
  values: Box<[T]>,
  size: Size2D,
  next_position: Position,
}

impl<'a, T> GridIntoIter<T>
where
  T: Copy,
{
  pub fn new(values: Box<[T]>, size: Size2D) -> Self {
    GridIntoIter {
      values,
      size,
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
    let current_position = set_to_next_position(&mut self.next_position, self.size)?;
    Some((
      current_position,
      mem::replace(&mut self.values[to_index(current_position, self.size)], unsafe {
        MaybeUninit::<T>::uninit().assume_init()
      }),
    ))
  }
}

// increment to next position
// returns None if there is no next position
// returns current position if there is a next position
fn set_to_next_position(position: &mut Position, grid_size: Size2D) -> Option<Position> {
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
