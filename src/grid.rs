use std::ops::Index;
use bevy::prelude::Component;

use crate::{grid_iter::GridIter, Position, Size2D};

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
    Self { size: Size2D { width, height }, values }
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

  pub fn get_value(&self, x: usize, y: usize) -> &T {
    &self.values[x * y]
  }

  pub fn get_value_by_position(&self, position: Position) -> &T {
    &self.values[position.x * position.y]
  }
}

impl<'a, T> IntoIterator for &'a Grid<T>
where
  T: Copy
{
  type Item = (Position, T);
  type IntoIter = GridIter<'a, T>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: (usize, usize)) -> &Self::Output {
    &self.get_value(index.0, index.1)
  }
}

impl<T> Index<Position> for Grid<T>
where
  T: Copy,
{
  type Output = T;
  fn index(&self, index: Position) -> &Self::Output {
    &self.get_value_by_position(index)
  }
}
