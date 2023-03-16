use crate::*;

pub fn to_index(position: Position, size: Size2D) -> usize {
    position.y as usize * size.width + position.x as usize
}

pub fn to_position(index: usize, size: Size2D) -> Position {
    Position {
        x: (index % size.width) as i64,
        y: (index / size.height) as i64,
    }
}
