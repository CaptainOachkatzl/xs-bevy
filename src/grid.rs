use bevy::prelude::Component;
use std::ops::{Index, IndexMut};

use crate::translation::index_translation::to_index;
use crate::{grid_iter::*, Position, Size2D};

#[derive(Component, Clone)]
pub struct Grid<T>
where
  T: Copy,
{
  size: Size2D,
  values: Box<[T]>,
}

impl<T> Grid<T>
where
  T: Copy,
{
  pub fn new(width: usize, height: usize, values: Box<[T]>) -> Self {
    assert!(values.len() == height * width);
    Self {
      size: Size2D { width, height },
      values,
    }
  }

  pub fn iter(&self) -> GridIter<T> {
    GridIter::new(&self)
  }

  pub fn get_size(&self) -> &Size2D {
    &self.size
  }

  pub fn get_width(&self) -> usize {
    self.size.width
  }

  pub fn get_height(&self) -> usize {
    self.size.height
  }

  pub fn get_value(&self, x: usize, y: usize) -> Option<&T> {
    self.get_value_by_position(Position { x: x as i64, y: y as i64 })
  }

  pub fn get_value_by_position(&self, position: Position) -> Option<&T> {
    if self.in_bounds(position) {
      return Some(&self.values[to_index(position, self.size)]);
    } else {
      return None;
    }
  }

  pub fn get_sub_grid(&self, offset: Position, size: Size2D) -> Option<Grid<T>> {
    let mut values = Vec::new();
    for pos in size.iter() {
      let grid_pos = offset + pos;
      if !self.in_bounds(offset + pos) {
        return None;
      }

      values.push(self.values[to_index(grid_pos, self.size)]);
    }

    Some(Grid::new(size.width, size.height, values.into_boxed_slice()))
  }

  fn in_bounds(&self, position: Position) -> bool {
    position.x >= 0 && position.y >= 0 && (position.x as usize) < self.size.width && (position.y as usize) < self.size.height
  }
}

impl<T> IntoIterator for Grid<T>
where
  T: Copy,
{
  type Item = (Position, T);
  type IntoIter = GridIntoIter<T>;
  fn into_iter(self) -> Self::IntoIter {
    GridIntoIter::new(self)
  }
}

impl<T> Index<(usize, usize)> for &Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: (usize, usize)) -> &Self::Output {
    &self.values[to_index(Position::from(index), self.size)]
  }
}

impl<T> Index<Position> for &Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: Position) -> &Self::Output {
    &self.values[to_index(index, self.size)]
  }
}

impl<T> Index<Position> for Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: Position) -> &Self::Output {
    &self.values[to_index(index, self.size)]
  }
}

impl<T> IndexMut<Position> for Grid<T>
where
  T: Copy,
{
  fn index_mut(&mut self, index: Position) -> &mut Self::Output {
    &mut self.values[to_index(index, self.size)]
  }
}
