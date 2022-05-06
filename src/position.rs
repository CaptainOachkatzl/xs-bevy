use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

impl Position {
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
