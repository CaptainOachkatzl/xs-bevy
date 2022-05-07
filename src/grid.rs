use bevy::prelude::Component;
use std::ops::Index;

use crate::{grid_iter::GridIter, Position, Size2D};
use crate::translation::index_translation::to_index;

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
    self.get_value_by_position(Position { x, y })
  }

  pub fn get_value_by_position(&self, position: Position) -> Option<&T> {
    if self.in_bounds(position) {
      return Some(&self.values[to_index(position, self.size)]);
    }
    else {
      return None;
    } 
  }

  fn in_bounds(&self, position: Position) -> bool {
    position.x < self.size.width && position.y < self.size.height
  }
}

impl<'a, T> IntoIterator for &'a Grid<T>
where
  T: Copy,
{
  type Item = (Position, T);
  type IntoIter = GridIter<'a, T>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> Index<(usize, usize)> for &'a Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: (usize, usize)) -> &Self::Output {
    &self.values[to_index(Position::from_tuple(index), self.size)]
  }
}

impl<'a, T> Index<Position> for &'a Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: Position) -> &Self::Output {
    &self.values[to_index(index, self.size)]
  }
}
