use std::ops::{Add, Sub, AddAssign, SubAssign};

use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

impl Position {
  pub fn new(x: usize, y: usize) -> Self {
    Position { x, y }
  }

  pub fn zero() -> Self {
    Position { x: 0, y: 0 }
  }

  pub fn from_tuple(position: (usize, usize)) -> Self {
    Position {
      x: position.0,
      y: position.1,
    }
  }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Position {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Position::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Position {
  fn sub_assign(&mut self, rhs: Self) {
      self.x -= rhs.x;
      self.y -= rhs.y;
  }
}
