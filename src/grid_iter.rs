use crate::{Position, Grid};

#[derive(Clone, Copy)]
pub struct GridIter<'a, T> where T: Copy {
  grid: &'a Grid<T>,
  next_coords: Position,
}

impl<'a, T> GridIter<'a, T> where T: Copy {
    pub fn new(grid: &'a Grid<T>) -> Self {
      GridIter { grid, next_coords: Position { x: 0, y: 0 } }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> where T: Copy {
  type Item = (Position, T);

  fn next(&mut self) -> Option<Self::Item> {
    if self.next_coords.y >= self.grid.get_height() {
      return None;
    }

    let current_coords = self.next_coords.clone();

    if self.next_coords.x >= self.grid.get_width() - 1 {
      self.next_coords.y += 1;
      self.next_coords.x = 0;
    } else {
      self.next_coords.x += 1;
    }

    Some((
      current_coords,
      self.grid[(current_coords.x, current_coords.y)],
    ))
  }
}
