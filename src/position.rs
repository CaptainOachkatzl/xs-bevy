use std::ops::{Add, AddAssign, Sub, SubAssign};
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
  pub x: i64,
  pub y: i64,
}

impl Position {
  pub fn new(x: i64, y: i64) -> Self {
    Position { x, y }
  }

  pub fn zero() -> Self {
    Position { x: 0, y: 0 }
  }
}

impl From<(usize, usize)> for Position {
  fn from(position: (usize, usize)) -> Self {
    Position {
      x: position.0 as i64,
      y: position.1 as i64,
    }
  }
}

impl Into<(usize, usize)> for Position {
  fn into(self) -> (usize, usize) {
    (self.x as usize, self.y as usize)
  }
}

impl From<(i64, i64)> for Position {
  fn from(position: (i64, i64)) -> Self {
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
