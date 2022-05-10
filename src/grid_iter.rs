use std::mem::{self, MaybeUninit};

use crate::{index_translation::to_position, Position, Size2D};

pub struct GridIter<'a, T>
where
  T: Copy,
{
  values: &'a [T],
  size: Size2D,
  index: i64,
}

impl<'a, T> GridIter<'a, T>
where
  T: Copy,
{
  pub fn new(values: &'a [T], size: Size2D) -> Self {
    GridIter { values, size, index: -1 }
  }
}

impl<'a, T> Iterator for GridIter<'a, T>
where
  T: Copy,
{
  type Item = (Position, &'a T);

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;
    if self.index as usize == self.values.len() {
      return None;
    }
    Some((to_position(self.index as usize, self.size), &self.values[self.index as usize]))
  }
}

pub struct GridIterMut<'a, T>
where
  T: Copy,
{
  values: &'a mut [T],
  size: Size2D,
  index: i64,
}

impl<'a, T> GridIterMut<'a, T>
where
  T: Copy,
{
  pub fn new(values: &'a mut [T], size: Size2D) -> Self {
    GridIterMut { values, size, index: -1 }
  }
}

impl<'a, T> Iterator for GridIterMut<'a, T>
where
  T: Copy,
{
  type Item = (Position, &'a mut T);

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;
    if self.index as usize == self.values.len() {
      return None;
    }
    let val: *mut T = &mut self.values[self.index as usize];
    Some((to_position(self.index as usize, self.size), unsafe { val.as_mut().unwrap() }))
  }
}

pub struct GridIntoIter<T>
where
  T: Copy,
{
  values: Box<[T]>,
  size: Size2D,
  index: i64,
}

impl<'a, T> GridIntoIter<T>
where
  T: Copy,
{
  pub fn new(values: Box<[T]>, size: Size2D) -> Self {
    GridIntoIter { values, size, index: -1 }
  }
}

impl<T> Iterator for GridIntoIter<T>
where
  T: Copy,
{
  type Item = (Position, T);

  fn next(&mut self) -> Option<Self::Item> {
    self.index += 1;
    if self.index as usize == self.values.len() {
      return None;
    }

    Some((
      to_position(self.index as usize, self.size),
      mem::replace(&mut self.values[self.index as usize], unsafe {
        MaybeUninit::<T>::uninit().assume_init()
      }),
    ))
  }
}
