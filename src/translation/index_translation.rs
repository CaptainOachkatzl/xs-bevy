use crate::*;

pub fn to_index(position: Position, size: Size2D) -> usize {
  position.y * size.width + position.x
}

pub fn to_position(index: usize, size: Size2D) -> Position {
  Position { x: index % size.width, y: index / size.height }
}