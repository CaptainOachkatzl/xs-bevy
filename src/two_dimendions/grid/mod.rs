use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use grid_iter::{GridIntoIter, GridIter, GridIterMut};
use translation::index_translation::to_index;

pub mod field;
pub use field::Field;
pub mod grid_iter;
pub mod pathing;
pub mod patterns;
pub mod position;
pub use position::Position;
pub mod size_2d;
pub use size_2d::Size2D;
pub mod translation;

#[derive(Clone)]
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

    pub fn get_size(&self) -> &Size2D {
        &self.size
    }

    pub fn get_width(&self) -> usize {
        self.size.width
    }

    pub fn get_height(&self) -> usize {
        self.size.height
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_value_by_position(&self, position: Position) -> Option<T> {
        if self.in_bounds(position) {
            Some(self.values[to_index(position, self.size)])
        } else {
            None
        }
    }

    pub fn get_ref(&self, x: usize, y: usize) -> Option<&T> {
        self.get_ref_by_position(Position { x: x as i64, y: y as i64 })
    }

    pub fn get_ref_by_position(&self, position: Position) -> Option<&T> {
        if self.in_bounds(position) {
            Some(&self.values[to_index(position, self.size)])
        } else {
            None
        }
    }

    pub fn get_mut_value_by_position(&mut self, position: Position) -> Option<&mut T> {
        if self.in_bounds(position) {
            Some(&mut self.values[to_index(position, self.size)])
        } else {
            None
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

    pub fn into_array(self) -> Box<[T]> {
        self.values
    }

    pub fn iter_with_position(&self) -> GridIter<T> {
        GridIter::new(&self.values, self.size)
    }

    pub fn iter_mut_with_position(&mut self) -> GridIterMut<T> {
        GridIterMut::new(&mut self.values, self.size)
    }

    pub fn into_iter_with_position(self) -> GridIntoIter<T> {
        GridIntoIter::new(self.values, self.size)
    }

    pub fn iter(&self) -> Iter<T> {
        self.values.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.values.iter_mut()
    }

    fn in_bounds(&self, position: Position) -> bool {
        position.x >= 0 && position.y >= 0 && (position.x as usize) < self.size.width && (position.y as usize) < self.size.height
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
